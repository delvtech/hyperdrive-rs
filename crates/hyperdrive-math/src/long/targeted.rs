use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::{State, YieldSpace};

impl State {
    /// Gets a target long that can be opened given a budget to achieve a
    /// desired fixed rate.
    ///
    /// If the long amount to reach the target is greater than the budget,
    /// the budget is returned.
    /// If the long amount to reach the target is invalid (i.e. it would produce
    /// an insolvent pool), then an error is thrown, and the user is advised to
    /// use [calculate_max_long](State::calculate_max_long).
    pub fn calculate_targeted_long_with_budget<
        F1: Into<FixedPoint<U256>>,
        F2: Into<FixedPoint<U256>>,
        F3: Into<FixedPoint<U256>>,
        I: Into<I256>,
    >(
        &self,
        budget: F1,
        target_rate: F2,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<F3>,
    ) -> Result<FixedPoint<U256>> {
        let budget = budget.into();
        match self.calculate_targeted_long(
            target_rate,
            checkpoint_exposure,
            maybe_max_iterations,
            maybe_allowable_error,
        ) {
            Ok(long_amount) => Ok(long_amount.min(budget)),
            Err(error) => Err(error),
        }
    }

    /// Gets a target long that can be opened to achieve a desired fixed rate.
    fn calculate_targeted_long<
        F1: Into<FixedPoint<U256>>,
        F2: Into<FixedPoint<U256>>,
        I: Into<I256>,
    >(
        &self,
        target_rate: F1,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<F2>,
    ) -> Result<FixedPoint<U256>> {
        // Check input args.
        let target_rate = target_rate.into();
        let checkpoint_exposure = checkpoint_exposure.into();
        let allowable_error = match maybe_allowable_error {
            Some(allowable_error) => allowable_error.into(),
            None => fixed!(1e15),
        };
        let current_rate = self.calculate_spot_rate()?;
        if target_rate >= current_rate {
            return Err(eyre!(
                "target_rate = {} argument must be less than the current_rate = {} for a targeted long.",
                target_rate, current_rate,
            ));
        }

        // Estimate the long that achieves a target rate.
        let (target_pool_share_reserves, target_pool_bond_reserves) =
            self.reserves_given_rate_ignoring_exposure(target_rate)?;
        let (mut target_user_base_delta, target_user_bond_delta) = self
            .long_trade_needed_given_reserves(
                target_pool_share_reserves,
                target_pool_bond_reserves,
            )?;
        // Determine what rate was achieved.
        let resulting_rate = self
            .calculate_spot_rate_after_long(target_user_base_delta, Some(target_user_bond_delta))?;

        // The estimated long will usually underestimate because the realized price
        // should always be greater than the spot price.
        //
        // However, if we overshot the zero-crossing (due to errors arising from FixedPoint<U256> arithmetic),
        // then either return or reduce the starting base amount and start on Newton's method.
        if target_rate > resulting_rate {
            let rate_error = target_rate - resulting_rate;

            // If we were still close enough and solvent, return.
            if self
                .solvency_after_long(
                    target_user_base_delta,
                    target_user_bond_delta,
                    checkpoint_exposure,
                )
                .is_ok()
                && rate_error < allowable_error
            {
                return Ok(target_user_base_delta);
            }
            // Else, cut the initial guess down by an order of magnitude and go to Newton's method.
            else {
                target_user_base_delta /= fixed!(10e18);
            }
        }
        // Else check if we are close enough to return.
        else {
            // If solvent & within the allowable error, stop here.
            let rate_error = resulting_rate - target_rate;
            if self
                .solvency_after_long(
                    target_user_base_delta,
                    target_user_bond_delta,
                    checkpoint_exposure,
                )
                .is_ok()
                && rate_error < allowable_error
            {
                return Ok(target_user_base_delta);
            }
        }

        // Iterate to find a solution.
        // We can use the initial guess as a starting point since we know it is less than the target.
        let mut possible_target_base_delta = target_user_base_delta;

        // Iteratively find a solution
        for _ in 0..maybe_max_iterations.unwrap_or(7) {
            let possible_target_bond_delta =
                self.calculate_open_long(possible_target_base_delta)?;
            let resulting_rate = self.calculate_spot_rate_after_long(
                possible_target_base_delta,
                Some(possible_target_bond_delta),
            )?;

            // We assume that the loss is positive only because Newton's
            // method will always underestimate.
            if target_rate > resulting_rate {
                return Err(eyre!(
                    "We overshot the zero-crossing during Newton's method.",
                ));
            }
            // We choose the difference between the rates as the loss because it
            // is convex given the above check, differentiable almost everywhere,
            // and has a simple derivative.
            let loss = resulting_rate - target_rate;

            // If solvent & within error, then return the value.
            if self
                .solvency_after_long(
                    possible_target_base_delta,
                    possible_target_bond_delta,
                    checkpoint_exposure,
                )
                .is_ok()
                && loss < allowable_error
            {
                return Ok(possible_target_base_delta);
            }
            // Otherwise perform another iteration.
            else {
                // The derivative of the loss is $l'(x) = r'(x)$.
                // We return $-l'(x)$ because $r'(x)$ is negative, which
                // can't be represented with FixedPoint<U256>.
                let negative_loss_derivative = self.rate_after_long_derivative_negation(
                    possible_target_base_delta,
                    possible_target_bond_delta,
                )?;

                // Adding the negative loss derivative instead of subtracting the loss derivative
                // ∆x_{n+1} = ∆x_{n} - l / l'
                //          = ∆x_{n} + l / (-l')
                possible_target_base_delta += loss / negative_loss_derivative;
            }
        }

        // Final solvency check.
        if self
            .solvency_after_long(
                possible_target_base_delta,
                self.calculate_open_long(possible_target_base_delta)?,
                checkpoint_exposure,
            )
            .is_err()
        {
            return Err(eyre!("Guess in `calculate_targeted_long` is insolvent."));
        }

        // Final accuracy check.
        let possible_target_bond_delta = self.calculate_open_long(possible_target_base_delta)?;
        let resulting_rate = self.calculate_spot_rate_after_long(
            possible_target_base_delta,
            Some(possible_target_bond_delta),
        )?;
        if target_rate > resulting_rate {
            return Err(eyre!(
                "We overshot the zero-crossing after Newton's method.",
            ));
        }
        let loss = resulting_rate - target_rate;
        if loss >= allowable_error {
            return Err(eyre!(
                "Unable to find an acceptable loss with max iterations. Final loss = {}.",
                loss
            ));
        }

        Ok(possible_target_base_delta)
    }

    /// The derivative of the equation for calculating the rate after a long.
    ///
    /// For some `$r = \frac{(1 - p(x))}{(p(x) \cdot t)}$`, where $p(x)$
    /// is the spot price after a long of `delta_base``$= x$` was opened and
    /// `$t$` is the annualized position duration, the rate derivative is:
    ///
    /// ```math
    /// r'(x) = \frac{(-p'(x) \cdot p(x) t
    /// - (1 - p(x)) (p'(x) \cdot t))}{(p(x) \cdot t)^2} //
    /// r'(x) = \frac{-p'(x)}{t \cdot p(x)^2}
    /// ```
    ///
    /// We return `$-r'(x)$` because negative numbers cannot be represented by FixedPoint<U256>.
    fn rate_after_long_derivative_negation(
        &self,
        base_amount: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        let price = self.calculate_spot_price_after_long(base_amount, Some(bond_amount))?;
        let price_derivative = self.price_after_long_derivative(base_amount, bond_amount)?;
        // The actual equation we want to represent is:
        // r' = -p' / (t p^2)
        // We can do a trick to return a positive-only version and
        // indicate that it should be negative in the fn name.
        // We use price * price instead of price.pow(fixed!(2e18)) to avoid error introduced by pow.
        Ok(price_derivative / (self.annualized_position_duration() * price * price))
    }

    /// The derivative of the price after a long.
    ///
    /// The price after a long that moves shares by $\Delta z$ and bonds by
    /// `$\Delta y$` is equal to:
    ///
    /// ```math
    /// p(\Delta z) = \left( \frac{\mu \cdot
    ///     (z_{0} + \Delta z - (\zeta_{0} + \Delta \zeta))}
    ///     {y - \Delta y} \right)^{t_{s}}
    /// ```
    ///
    /// where `$t_s$` is the time stretch constant and `$z_{e,0}$` is the
    /// initial effective share reserves, and `$\zeta$` is the zeta adjustment.
    /// The zeta adjustment is constant when opening a long, i.e.
    /// `$\Delta \zeta = 0$`, so we drop the subscript. Equivalently, for some
    /// amount of `delta_base` `$= \Delta x$` provided to open a long,
    /// we can write:
    ///
    /// ```math
    /// p(\Delta x) = \left(
    ///     \frac{\mu (z_{0} + \frac{1}{c}
    ///     \cdot \left( \Delta x - \Phi_{g,ol}(\Delta x) \right) - \zeta)}
    ///     {y_0 - y(\Delta x)}
    /// \right)^{t_{s}}
    /// ```
    ///
    /// where `$\Phi_{g,ol}(\Delta x)$` is the
    /// [open_long_governance_fee](State::open_long_governance_fee),
    /// `$y(\Delta x)$` is the [bond_amount](State::calculate_open_long),
    ///
    /// To compute the derivative, we first define some auxiliary variables:
    ///
    /// ```math
    /// a(\Delta x) &= \mu (z_{0} + \frac{\Delta x}{c} - \frac{\Phi_{g,ol}(\Delta x)}{c} - \zeta) \\
    ///     &= \mu \left( z_{e,0} + \frac{\Delta x}{c} - \frac{\Phi_{g,ol}(\Delta x)}{c} \right) \\
    /// b(\Delta x) &= y_0 - y(\Delta x) \\
    /// v(\Delta x) &= \frac{a(\Delta x)}{b(\Delta x)}
    /// ```
    ///
    /// and thus `$p(\Delta x) = v(\Delta x)^{t_{s}}$`.
    /// Given these, we can write out intermediate derivatives:
    ///
    /// ```math
    /// \begin{aligned}
    /// a'(\Delta x) &= \frac{\mu}{c} (1 - \Phi_{g,ol}'(\Delta x)) \\
    /// b'(\Delta x) &= -y'(\Delta x) \\
    /// v'(\Delta x) &= \frac{b(\Delta x) \cdot a'(\Delta x) - a(\Delta x) \cdotb'(\Delta x)}{b(\Delta x)^2}
    /// \end{aligned}
    /// ```
    ///
    /// And finally, the price after long derivative is:
    ///
    /// ```math
    /// p'(\Delta x) = v'(\Delta x) \cdot t_{s} \cdot v(\Delta x)^{(t_{s} - 1)}
    /// ```
    ///
    fn price_after_long_derivative(
        &self,
        base_amount: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // g'(x) = \phi_g \phi_c (1 - p_0)
        let gov_fee_derivative = self.governance_lp_fee()
            * self.curve_fee()
            * (fixed!(1e18) - self.calculate_spot_price()?);

        // a(x) = mu * (z_{e,0} + 1/c (x - g(x))
        let inner_numerator = self.mu()
            * (self.ze()?
                + (base_amount - self.open_long_governance_fee(base_amount, None)?)
                    .div_down(self.vault_share_price()));

        // a'(x) = (mu / c) (1 - g'(x))
        let inner_numerator_derivative = self
            .mu()
            .mul_div_down(fixed!(1e18) - gov_fee_derivative, self.vault_share_price());
        //(self.mu() / self.vault_share_price()) * (fixed!(1e18) - gov_fee_derivative);

        // b(x) = y_0 - y(x)
        let inner_denominator = self.bond_reserves() - bond_amount;

        // b'(x) = -y'(x)
        // -b'(x) = y'(x)
        let long_amount_derivative = self.calculate_open_long_derivative(base_amount)?;

        // v(x) = a(x) / b(x)
        // v'(x) = ( b(x) * a'(x) - a(x) * b'(x) ) / b(x)^2
        //       = ( b(x) * a'(x) + a(x) * -b'(x) ) / b(x)^2
        // Note that we are adding the negative b'(x) to avoid negative fixedpoint numbers
        let inner_derivative = (inner_denominator * inner_numerator_derivative
            + inner_numerator * long_amount_derivative)
            / (inner_denominator * inner_denominator);

        // p'(x) = v'(x) * t_s * v(x)^(t_s - 1)
        // p'(x) = v'(x) * t_s * v(x)^(-1)^(1 - t_s)
        // v(x) is flipped to (denominator / numerator) to avoid a negative exponent
        Ok(inner_derivative
            * self.time_stretch()
            * (inner_denominator / inner_numerator).pow(fixed!(1e18) - self.time_stretch())?)
    }

    /// Calculate the base & bond trade amount for an open long trade that moves the
    /// current state to the given desired ending reserve levels.
    ///
    /// Given a target ending pool share reserves, `$z_1$`, and bond reserves,
    /// `$y_1$`, the trade deltas to achieve that state would be:
    ///
    /// From the pool's perspective:
    /// ```math
    /// z_1 = z_0 + \Delta z \\
    /// \Delta z = z_1 - z_0
    /// ```
    ///
    /// From the trader's perspective, for a provided `base_amount`
    /// `$= \Delta x$`, the pool share reserves update, `$\Delta z$`, would be:
    /// ```math
    /// \Delta z = \frac{1}{c} (\Delta x - \Phi_{g,ol}(\Delta x))
    /// ```
    ///
    /// Solving for the change in base:
    /// ```math
    /// \begin{aligned}
    /// c \cdot \Delta z
    ///   &= \Delta x - \Phi_{g,ol}(\Delta x) \\
    /// c \cdot \Delta z
    ///   &= \Delta x - (1 - p) \cdot \phi_c \cdot \phi_g \cdot \Delta x \\
    /// c \cdot \Delta z
    ///   &= \Delta x \cdot (1 - (1 - p) \cdot \phi_c \cdot \phi_g) \\
    /// &\therefore \\
    /// \Delta x
    ///   &= \frac{c \cdot \Delta z}{(1 - (1 - p) \cdot \phi_c \cdot \phi_g)}
    /// \end{aligned}
    /// ```
    ///
    /// These should be equal, therefore:
    /// ```math
    /// \Delta x = \frac{c \cdot \Delta z}{(1 - (1 - p) \cdot \phi_c \cdot \phi_g)} \\
    /// \Delta x = \frac{c \cdot (z_1 - z_0)}{(1 - (1 - p) \cdot \phi_c \cdot \phi_g)} \\
    /// ```
    /// where `$c$` is the vault share price, `$p$` is the original spot price,
    /// and `$\Phi_{g,ol}(\Delta x)$` is the
    /// (open long governance fee)[State::open_long_governance_fee].
    ///
    /// The change in bonds, $\Delta y$ is equal for the trader and the pool and
    /// can be determined with (`calculate_open_long`)[State::calculate_open_long].
    fn long_trade_needed_given_reserves(
        &self,
        ending_share_reserves: FixedPoint<U256>,
        ending_bond_reserves: FixedPoint<U256>,
    ) -> Result<(FixedPoint<U256>, FixedPoint<U256>)> {
        if self.bond_reserves() < ending_bond_reserves {
            return Err(eyre!(
                "expected bond_reserves={} >= ending_bond_reserves={}",
                self.bond_reserves(),
                ending_bond_reserves,
            ));
        }
        if ending_share_reserves < self.share_reserves() {
            return Err(eyre!(
                "expected ending_share_reserves={} >= share_reserves={}",
                ending_share_reserves,
                self.share_reserves(),
            ));
        }
        let share_delta = ending_share_reserves - self.share_reserves();
        let fees = fixed!(1e18)
            - (fixed!(1e18) - self.calculate_spot_price()?)
                * self.curve_fee()
                * self.governance_lp_fee();
        let base_delta = self.vault_share_price().mul_div_down(share_delta, fees);
        let bond_delta = self.calculate_open_long(base_delta)?;
        Ok((base_delta, bond_delta))
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use ethers::types::U256;
    use fixedpointmath::{uint256, FixedPointValue};
    use hyperdrive_test_utils::{chain::TestChain, constants::FUZZ_RUNS};
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::test_utils::agent::HyperdriveMathAgent;

    #[tokio::test]
    async fn fuzz_long_trade_needed_given_reserves() -> Result<()> {
        let base_reserve_test_tolerance = fixed!(1e10);
        let bond_reserve_test_tolerance = fixed!(1e10);
        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Get a random long trade amount.
            let checkpoint_exposure = rng
                .gen_range(fixed!(0)..=FixedPoint::<I256>::MAX)
                .raw()
                .flip_sign_if(rng.gen());
            let max_long_trade = match panic::catch_unwind(|| {
                state.calculate_max_long(U256::MAX, checkpoint_exposure, None)
            }) {
                Ok(max_trade) => match max_trade {
                    Ok(max_trade) => max_trade,
                    Err(_) => continue, // Max threw an Err
                },
                Err(_) => continue, // Max threw a panic
            };
            let long_base_amount =
                rng.gen_range(state.minimum_transaction_amount()..=max_long_trade);
            // Do the long to see the bond delta (same amount for the user & pool in this case).
            let long_bond_amount = state.calculate_open_long(long_base_amount)?;
            // Get the reserve levels after the state was updated from the open long.
            let updated_state = state
                .calculate_pool_state_after_open_long(long_base_amount, Some(long_bond_amount))?;
            let (final_share_reserves, final_bond_reserves) = (
                FixedPoint::from(updated_state.info.share_reserves),
                FixedPoint::from(updated_state.info.bond_reserves),
            );
            // Estimate the trade amounts from the final reserve levels.
            let (estimated_base_amount, estimated_bond_amount) = state
                .long_trade_needed_given_reserves(final_share_reserves, final_bond_reserves)?;
            // Make sure the estimates match the realized transaction amounts.
            let base_error = if estimated_base_amount > long_base_amount {
                estimated_base_amount - long_base_amount
            } else {
                long_base_amount - estimated_base_amount
            };
            assert!(
                base_error <= base_reserve_test_tolerance,
                "expected abs(estimated_base_amount={}-long_base_amount={})={} <= test_tolerance={}",
                estimated_base_amount,
                long_base_amount,
                base_error,
                base_reserve_test_tolerance,
            );
            let bond_error = if estimated_bond_amount > long_bond_amount {
                estimated_bond_amount - long_bond_amount
            } else {
                long_bond_amount - estimated_bond_amount
            };
            assert!(
                bond_error <= bond_reserve_test_tolerance,
                "expected abs(estimated_bond_amount={}-long_bond_amount={})={} <= test_tolerance={}",
                estimated_bond_amount,
                long_bond_amount,
                bond_error,
                bond_reserve_test_tolerance,
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_calculate_targeted_long_with_budget() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob.
        // Alice is funded with a large amount of capital so that she can initialize
        // the pool. Bob is funded with a random amount of capital so that we
        // can test `calculate_targeted_long` when budget is the primary constraint
        // and when it is not.
        let allowable_solvency_error = fixed!(1e5);
        let allowable_budget_error = fixed!(1e5);
        let allowable_rate_error = fixed!(1e11);
        let num_newton_iters = 7;

        // Initialize a test chain and agents.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let config = bob.get_config().clone();

        // Fuzz test
        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Alice initializes the pool.
            // Large budget for initializing the pool.
            let contribution = fixed!(1_000_000e18);
            alice.fund(contribution).await?;
            let initial_fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
            alice
                .initialize(initial_fixed_rate, contribution, None)
                .await?;

            // Small lower bound on Bob's budget for resource-constrained targeted longs.
            let budget = rng.gen_range(fixed!(10e18)..=fixed!(500_000_000e18));
            // Half the time we will open a long & let it mature.
            if rng.gen_range(0..=1) == 0 {
                // Open a long.
                let max_long =
                    bob.get_state()
                        .await?
                        .calculate_max_long(U256::MAX, I256::from(0), None)?;
                let long_amount =
                    (max_long / fixed!(100e18)).max(config.minimum_transaction_amount.into());
                bob.fund(long_amount + budget).await?;
                bob.open_long(long_amount, None, None).await?;
                // Advance time to just after maturity.
                let variable_rate = rng.gen_range(fixed!(0)..=fixed!(0.5e18));
                let time_amount = FixedPoint::from(config.position_duration) * fixed!(1.05e18); // 1.05 * position_duraiton
                alice.advance_time(variable_rate, time_amount).await?;
                // Checkpoint to auto-close the position.
                alice
                    .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
                    .await?;
            }
            // Else we will just fund a random budget amount and do the targeted long.
            else {
                bob.fund(budget).await?;
            }

            // Some of the checkpoint passes and variable interest accrues.
            alice
                .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
                .await?;
            let variable_rate = rng.gen_range(fixed!(0)..=fixed!(0.5e18));
            alice
                .advance_time(
                    variable_rate,
                    FixedPoint::from(config.checkpoint_duration) * fixed!(0.5e18),
                )
                .await?;

            // Get a targeted long amount.
            let target_rate = bob.get_state().await?.calculate_spot_rate()?
                / rng.gen_range(fixed!(1.0001e18)..=fixed!(10e18));
            let targeted_long_result = bob
                .calculate_targeted_long(
                    target_rate,
                    Some(num_newton_iters),
                    Some(allowable_rate_error),
                )
                .await;

            // Bob opens a targeted long.
            let current_state = bob.get_state().await?;
            let max_spot_price_before_long = current_state.calculate_max_spot_price()?;
            match targeted_long_result {
                // If the code ran without error, open the long
                Ok(targeted_long) => {
                    bob.open_long(targeted_long, None, None).await?;
                }

                // Else parse the error for a to improve error messaging.
                Err(e) => {
                    // If the fn failed it's possible that the target rate would be insolvent.
                    if e.to_string()
                        .contains("Unable to find an acceptable loss with max iterations")
                    {
                        let max_long = bob.calculate_max_long(None).await?;
                        let rate_after_max_long =
                            current_state.calculate_spot_rate_after_long(max_long, None)?;
                        // If the rate after the max long is at or below the target, then we could have hit it.
                        if rate_after_max_long <= target_rate {
                            return Err(eyre!(
                                "ERROR {}\nA long that hits the target rate exists but was not found.",
                                e
                            ));
                        }
                        // Otherwise the target would have resulted in insolvency and wasn't possible.
                        else {
                            return Err(eyre!(
                                "ERROR {}\nThe target rate would result in insolvency.",
                                e
                            ));
                        }
                    }
                    // If the error is not the one we're looking for, return it, causing the test to fail.
                    else {
                        return Err(e);
                    }
                }
            }

            // Three things should be true after opening the long:
            //
            // 1. The pool's spot price is under the max spot price prior to
            //    considering fees
            // 2. The pool's solvency is above zero.
            // 3. IF Bob's budget is not consumed; then new rate is close to the target rate

            // Check that our resulting price is under the max
            let current_state = alice.get_state().await?;
            let spot_price_after_long = current_state.calculate_spot_price()?;
            assert!(
                max_spot_price_before_long > spot_price_after_long,
                "Resulting price is greater than the max."
            );

            // Check solvency
            let is_solvent = { current_state.calculate_solvency()? > allowable_solvency_error };
            assert!(is_solvent, "Resulting pool state is not solvent.");

            let new_rate = current_state.calculate_spot_rate()?;
            // If the budget was NOT consumed, then we assume the target was hit.
            if bob.base() > allowable_budget_error {
                // Actual price might result in long overshooting the target.
                let abs_error = if target_rate > new_rate {
                    target_rate - new_rate
                } else {
                    new_rate - target_rate
                };
                assert!(
                    abs_error <= allowable_rate_error,
                    "target_rate was {}, realized rate is {}. abs_error={} was not <= {}.",
                    target_rate,
                    new_rate,
                    abs_error,
                    allowable_rate_error
                );
            }
            // Else, we should have undershot,
            // or by some coincidence the budget was the perfect amount
            // and we hit the rate exactly.
            else {
                assert!(
                    new_rate >= target_rate,
                    "The new_rate={} should be >= target_rate={} when budget constrained.",
                    new_rate,
                    target_rate
                );
            }

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }
}
