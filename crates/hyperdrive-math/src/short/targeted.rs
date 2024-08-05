use ethers::types::I256;
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::{State, YieldSpace};

impl State {
    /// Gets a target short that can be opened given a budget to achieve a
    /// desired fixed rate.
    ///
    /// If the short amount to reach the target is greater than the budget,
    /// the budget is returned. If the short amount to reach the target is
    /// invalid (i.e. it would produce an insolvent pool), then an error is
    /// thrown, and the user is advised to use
    /// [calculate_max_short](State::calculate_max_short).
    ///
    pub fn calculate_targeted_short_with_budget<
        F1: Into<FixedPoint>,
        F2: Into<FixedPoint>,
        F3: Into<FixedPoint>,
        F4: Into<FixedPoint>,
        I: Into<I256>,
    >(
        &self,
        budget: F1,
        target_rate: F2,
        open_vault_share_price: F3,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<F4>,
    ) -> Result<FixedPoint> {
        // TODO: use budget
        // We need to do Newton's method again here to iteratively find the bond_amount input to
        // calc_open_short such that the base_amount required is <= budget.
        self.calculate_targeted_short(
            target_rate,
            open_vault_share_price,
            checkpoint_exposure,
            maybe_max_iterations,
            maybe_allowable_error,
        )
    }

    /// Gets a target short that can be opened to achieve a desired fixed rate.
    fn calculate_targeted_short<
        F1: Into<FixedPoint>,
        F2: Into<FixedPoint>,
        F3: Into<FixedPoint>,
        I: Into<I256>,
    >(
        &self,
        target_rate: F1,
        open_vault_share_price: F2,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<F3>,
    ) -> Result<FixedPoint> {
        let target_rate = target_rate.into();
        let checkpoint_exposure = checkpoint_exposure.into();
        let open_vault_share_price = open_vault_share_price.into();
        let allowable_error = match maybe_allowable_error {
            Some(allowable_error) => allowable_error.into(),
            None => fixed!(1e14),
        };

        // Check input args.
        let current_rate = self.calculate_spot_rate()?;
        if target_rate <= current_rate {
            return Err(eyre!(
                "target_rate = {} argument must be greater than the current_rate = {} for a targeted short.",
                target_rate, current_rate,
            ));
        }

        // Estimate the short that achieves a target rate.
        let (target_share_reserves, target_bond_reserves) =
            self.reserves_given_rate_ignoring_exposure(target_rate)?;
        let (target_base_delta, mut target_bond_delta) =
            self.short_trade_deltas_from_reserves(target_share_reserves, target_bond_reserves)?;

        // Determine what rate was achieved.
        let resulting_rate =
            self.calculate_spot_rate_after_short(target_bond_delta, Some(target_base_delta))?;

        // The estimated short will usually underestimate because the realized price
        // should always be greater than the spot price.
        //
        // However, if we overshot the zero-crossing (due to errors arising from FixedPoint arithmetic),
        // then either return or reduce the starting bond amount and start on Newton's method.
        if target_rate > resulting_rate {
            let rate_error = target_rate - resulting_rate;

            // If we were still close enough and solvent, return.
            if self
                .solvency_after_short(target_bond_delta, checkpoint_exposure)
                .is_ok()
                && rate_error < allowable_error
            {
                return Ok(target_bond_delta);
            }
            // Else, cut the initial guess down by an order of magnitude and go to Newton's method.
            else {
                target_bond_delta /= fixed!(10e18);
            }
        }
        // Else check if we are close enough to return.
        else {
            // If solvent & within the allowable error, stop here.
            let rate_error = target_rate - resulting_rate;
            if self
                .solvency_after_short(target_bond_delta, checkpoint_exposure)
                .is_ok()
                && rate_error < allowable_error
            {
                return Ok(target_bond_delta);
            }
        }

        // Iterate to find a solution.
        // We can use the initial guess as a starting point since we know it is less than the target.
        let mut possible_target_bond_delta = target_bond_delta;

        // Iteratively find a solution
        for _ in 0..maybe_max_iterations.unwrap_or(7) {
            let possible_target_base_delta =
                self.calculate_open_short(possible_target_bond_delta, open_vault_share_price)?;
            let resulting_rate = self.calculate_spot_rate_after_short(
                possible_target_bond_delta,
                Some(possible_target_base_delta),
            )?;

            // We assume that the loss is positive only because Newton's
            // method will always underestimate.
            if target_rate < resulting_rate {
                let rate_error = target_rate - resulting_rate;

                // If we were still close enough and solvent, return.
                if self
                    .solvency_after_short(target_bond_delta, checkpoint_exposure)
                    .is_ok()
                    && rate_error < allowable_error
                {
                    return Ok(target_bond_delta);
                }
                // Else, cut the initial guess down by an order of magnitude and go to Newton's method.
                else {
                    target_bond_delta /= fixed!(10e18);
                }
                return Err(eyre!(
                    "We overshot the zero-crossing during Newton's method.",
                ));
            }
            // We choose the difference between the rates as the loss because it
            // is convex given the above check, differentiable almost everywhere,
            // and has a simple derivative.
            let loss = target_rate - resulting_rate;

            // If we've done it (solvent & within error), then return the value.
            if self
                .solvency_after_short(possible_target_bond_delta, checkpoint_exposure)
                .is_ok()
                && loss < allowable_error
            {
                return Ok(possible_target_bond_delta);
            }
            // Otherwise perform another iteration.
            else {
                // The derivative of the loss is $l'(x) = r'(x)$.
                // We return $-l'(x)$ because $r'(x)$ is negative, which
                // can't be represented with FixedPoint.
                let negative_loss_derivative = self.rate_after_short_derivative_negation(
                    possible_target_bond_delta,
                    possible_target_base_delta,
                )?;

                // Adding the negative loss derivative instead of subtracting the loss derivative
                // ∆y_{n+1} = ∆y_{n} - l / l'
                //          = ∆y_{n} + l / (-l')
                possible_target_bond_delta += loss / negative_loss_derivative;
            }
        }

        // Final solvency check.
        if self
            .solvency_after_short(possible_target_bond_delta, checkpoint_exposure)
            .is_ok()
        {
            return Err(eyre!("Guess in `calculate_targeted_short` is insolvent."));
        }

        // Final accuracy check.
        let possible_target_base_delta =
            self.calculate_open_short(possible_target_bond_delta, open_vault_share_price)?;
        let resulting_rate = self.calculate_spot_rate_after_short(
            possible_target_bond_delta,
            Some(possible_target_base_delta),
        )?;
        if target_rate < resulting_rate {
            return Err(eyre!(
                "We overshot the zero-crossing after Newton's method.",
            ));
        }
        let loss = target_rate - resulting_rate;
        if loss >= allowable_error {
            return Err(eyre!(
                "Unable to find an acceptable loss with max iterations. Final loss = {}.",
                loss
            ));
        }

        Ok(possible_target_bond_delta)
    }

    /// TODO:
    /// The derivative of the equation for calculating the rate after a short.
    ///
    /// For some $r = (1 - p(x)) / (p(x) \cdot t)$, where $p(x)$
    /// is the spot price after a long of `delta_base`$= x$ was opened and $t$
    /// is the annualized position duration, the rate derivative is:
    ///
    /// $$
    /// r'(x) = \frac{(-p'(x) \cdot p(x) t - (1 - p(x)) (p'(x) \cdot t))}{(p(x) \cdot t)^2} //
    /// r'(x) = \frac{-p'(x)}{t \cdot p(x)^2}
    /// $$
    ///
    /// We return $-r'(x)$ because negative numbers cannot be represented by FixedPoint.
    fn rate_after_short_derivative_negation(
        &self,
        base_amount: FixedPoint,
        bond_amount: FixedPoint,
    ) -> Result<FixedPoint> {
        let price = self.calculate_spot_price_after_short(base_amount, Some(bond_amount))?;
        let price_derivative = self.price_after_short_derivative(base_amount, bond_amount)?;
        // The actual equation we want to represent is:
        // r' = -p' / (t p^2)
        // We can do a trick to return a positive-only version and
        // indicate that it should be negative in the fn name.
        // We use price * price instead of price.pow(fixed!(2e18)) to avoid error introduced by pow.
        Ok(price_derivative / (self.annualized_position_duration() * price * price))
    }

    /// TODO:
    /// The derivative of the price after a short.
    ///
    /// The price after a short that moves shares by $\Delta z$ and bonds by $\Delta y$
    /// is equal to
    ///
    /// $$
    /// $$
    ///
    /// where $t_{s}$ is the time stretch constant and $z_{e,0}$ is the initial
    /// effective share reserves, and $\zeta$ is the zeta adjustment.
    /// The zeta adjustment is constant when opening a short, i.e.
    /// $\Delta \zeta = 0$, so we drop the subscript. Equivalently, for some
    /// amount of `delta_base`$= \Delta x$ provided to open a long, we can write:
    ///
    /// $$
    /// p(\Delta x) = \left(
    ///     \frac{\mu (z_{0} + \frac{1}{c}
    ///     \cdot \left( \Delta x - \Phi_{g,ol}(\Delta x) \right) - \zeta)}
    ///     {y_0 - y(\Delta x)}
    /// \right)^{t_{s}}
    /// $$
    ///
    /// where $\Phi_{g,ol}(\Delta x)$ is the [open_long_governance_fee](long::fees::open_long_governance_fee),
    /// $y(\Delta x)$ is the [bond_amount](long::open::calculate_open_long),
    ///
    /// To compute the derivative, we first define some auxiliary variables:
    ///
    /// $$
    /// a(\Delta x) &= \mu (z_{0} + \frac{\Delta x}{c} - \frac{\Phi_{g,ol}(\Delta x)}{c} - \zeta) \\
    ///     &= \mu \left( z_{e,0} + \frac{\Delta x}{c} - \frac{\Phi_{g,ol}(\Delta x)}{c} \right) \\
    /// b(\Delta x) &= y_0 - y(\Delta x) \\
    /// v(\Delta x) &= \frac{a(\Delta x)}{b(\Delta x)}
    /// $$
    ///
    /// and thus $p(\Delta x) = v(\Delta x)^{t_{s}}$.
    /// Given these, we can write out intermediate derivatives:
    ///
    /// $$
    /// a'(\Delta x) &= \frac{\mu}{c} (1 - \Phi_{g,ol}'(\Delta x)) \\
    /// b'(\Delta x) &= -y'(\Delta x) \\
    /// v'(\Delta x) &= \frac{b(\Delta x) \cdot a'(\Delta x) - a(\Delta x) \cdotb'(\Delta x)}{b(\Delta x)^2}
    /// $$
    ///
    /// And finally, the price after long derivative is:
    ///
    /// $$
    /// p'(\Delta x) = v'(\Delta x) \cdot t_{s} \cdot v(\Delta x)^{(t_{s} - 1)}
    /// $$
    ///
    fn price_after_short_derivative(
        &self,
        bond_amount: FixedPoint,
        base_amount: FixedPoint,
    ) -> Result<FixedPoint> {
        Ok(fixed!(1))
        // // g'(x) = \phi_g \phi_c (1 - p_0)
        // let gov_fee_derivative = self.governance_lp_fee()
        //     * self.curve_fee()
        //     * (fixed!(1e18) - self.calculate_spot_price()?);

        // // a(x) = mu * (z_{e,0} + 1/c (x - g(x))
        // let inner_numerator = self.mu()
        //     * (self.ze()?
        //         + (base_amount - self.open_long_governance_fee(base_amount, None)?)
        //             .div_down(self.vault_share_price()));

        // // a'(x) = (mu / c) (1 - g'(x))
        // let inner_numerator_derivative = self
        //     .mu()
        //     .mul_div_down(fixed!(1e18) - gov_fee_derivative, self.vault_share_price());
        // //(self.mu() / self.vault_share_price()) * (fixed!(1e18) - gov_fee_derivative);

        // // b(x) = y_0 - y(x)
        // let inner_denominator = self.bond_reserves() - bond_amount;

        // // b'(x) = -y'(x)
        // // -b'(x) = y'(x)
        // let long_amount_derivative = match self.calculate_open_long_derivative(base_amount)? {
        //     Some(derivative) => derivative,
        //     None => return Err(eyre!("long_amount_derivative failure.")),
        // };

        // // v(x) = a(x) / b(x)
        // // v'(x) = ( b(x) * a'(x) - a(x) * b'(x) ) / b(x)^2
        // //       = ( b(x) * a'(x) + a(x) * -b'(x) ) / b(x)^2
        // // Note that we are adding the negative b'(x) to avoid negative fixedpoint numbers
        // let inner_derivative = (inner_denominator * inner_numerator_derivative
        //     + inner_numerator * long_amount_derivative)
        //     / (inner_denominator * inner_denominator);

        // // p'(x) = v'(x) * t_s * v(x)^(t_s - 1)
        // // p'(x) = v'(x) * t_s * v(x)^(-1)^(1 - t_s)
        // // v(x) is flipped to (denominator / numerator) to avoid a negative exponent
        // Ok(inner_derivative
        //     * self.time_stretch()
        //     * (inner_denominator / inner_numerator).pow(fixed!(1e18) - self.time_stretch())?)
    }

    /// Calculate the base & bond deltas for a short trade that moves the
    /// current state to the given desired ending reserve levels.
    ///
    /// Given a target ending pool share reserves, `$z_t$`, and bond reserves,
    /// `$y_t$`, the trade deltas to achieve that state would be:
    ///
    /// ```math
    /// \Delta y = y_t - y \\
    /// \Delta x = c \cdot \left(
    ///   z_{e,0} - z_t - (\Phi_c(\Delta y) - \Phi_g(\Delta y))
    /// \right)
    /// ```
    ///
    /// where `$c$` is the vault share price, `$\Phi_c(\Delta y)$` is the
    /// (open_short_curve_fee)[State::open_short_curve_fee], and
    /// `$\Phi_g(\Delta y)$` is the
    /// (open_short_governance_fee)[State::open_short_governance_fee].
    fn short_trade_deltas_from_reserves(
        &self,
        ending_share_reserves: FixedPoint,
        ending_bond_reserves: FixedPoint,
    ) -> Result<(FixedPoint, FixedPoint)> {
        if ending_bond_reserves < self.bond_reserves() {
            return Err(eyre!(
                "Expected ending_bond_reserves={} >= bond_reserves={} for a short trade.",
                ending_bond_reserves,
                self.bond_reserves()
            ));
        }
        let bond_delta = ending_bond_reserves - self.bond_reserves();
        let curve_fee_base = self.open_short_curve_fee(bond_delta)?;
        let curve_fee_shares = curve_fee_base.div_up(self.vault_share_price());
        let gov_curve_fee_shares = self
            .open_short_governance_fee(bond_delta, Some(curve_fee_base))?
            .div_up(self.vault_share_price());
        let fees = curve_fee_shares - gov_curve_fee_shares;
        let shares_delta = self.effective_share_reserves()? - ending_share_reserves - fees;
        let base_delta = self.vault_share_price() * shares_delta;
        Ok((base_delta, bond_delta))
    }
}

#[cfg(test)]
mod tests {
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, SLOW_FUZZ_RUNS},
    };
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::test_utils::{
        agent::HyperdriveMathAgent, preamble::initialize_pool_with_random_state,
    };

    #[tokio::test]
    async fn fuzz_short_trade_deltas_from_reserves() -> Result<()> {
        let test_tolerance = fixed!(1e7);
        let mut rng = thread_rng();
        let mut counter = 0;
        for _ in 0..*FAST_FUZZ_RUNS {
            // Get the reserves before the short.
            let old_state = rng.gen::<State>();
            let base_reserves = old_state.vault_share_price() * old_state.share_reserves();
            let bond_reserves = old_state.bond_reserves();
            // Update the state with a random short.
            let max_short = old_state.calculate_absolute_max_short(
                old_state.calculate_spot_price()?,
                I256::from(0),
                None,
            )?;
            let bond_amount = rng.gen_range(old_state.minimum_transaction_amount()..=max_short);
            let new_state = old_state.calculate_pool_state_after_open_short(bond_amount, None)?;
            // Get the reserves after the short.
            let target_share_reserves = new_state.share_reserves();
            let target_bond_reserves = new_state.bond_reserves();
            // Compute new state deltas.
            let base_delta =
                (new_state.vault_share_price() * target_share_reserves) - base_reserves;
            let bond_delta = bond_reserves - target_bond_reserves;
            // Calculate the old state deltas to achieve the short.
            let (target_base_delta, target_bond_delta) = old_state
                .short_trade_deltas_from_reserves(target_share_reserves, target_bond_reserves)?;
            // By what amount does the resulting deltas differ from the target?
            let base_error = if base_delta > target_base_delta {
                base_delta - target_base_delta
            } else {
                target_base_delta - base_delta
            };
            assert!(
                base_error <= test_tolerance,
                "expected abs(base_delta-target_base_delta)={} <= test_tolerance={}",
                base_error,
                test_tolerance
            );
            let bond_error = if bond_delta > target_bond_delta {
                bond_delta - target_bond_delta
            } else {
                target_bond_delta - bond_delta
            };
            assert!(
                bond_error <= test_tolerance,
                "expected abs(bond_delta-target_bond_delta)={} <= test_tolerance={}",
                bond_error,
                test_tolerance
            );
            counter += 1;
        }
        assert!(counter >= 1_000); // this passed at least 1,000 times
        Ok(())
    }

    // TODO:
    #[tokio::test]
    async fn test_calculate_targeted_short_with_budget() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob.
        // Alice is funded with a large amount of capital so that she can initialize
        // the pool. Bob is funded with a random amount of capital so that we
        // can test `calculate_targeted_short` when budget is the primary constraint
        // and when it is not.

        let allowable_solvency_error = fixed!(1e5);
        let allowable_budget_error = fixed!(1e5);
        let allowable_rate_error = fixed!(1e11);
        let num_newton_iters = 7;

        // Set up a random number generator. We use ChaCha8Rng with a randomly
        // generated seed, which makes it easy to reproduce test failures given
        // the seed.
        let mut rng = {
            let mut rng = thread_rng();
            let seed = rng.gen();
            ChaCha8Rng::seed_from_u64(seed)
        };

        // Initialize a test chain and agents.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let mut celine = chain.celine().await?;

        // Fuzz test
        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Run the preamble and get state information.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;
            let current_state = alice.get_state().await?;
            let min_spot_price_before_short = current_state.calculate_min_spot_price()?;

            // Get a targeted short amount.
            let target_rate = current_state.calculate_spot_rate()?
                * rng.gen_range(fixed!(1.0001e18)..=fixed!(10e18));
            let targeted_short_result = bob
                .calculate_targeted_short(
                    target_rate,
                    Some(num_newton_iters),
                    Some(allowable_rate_error),
                )
                .await;

            // Bob opens a targeted short.
            match targeted_short_result {
                // If the code ran without error, open the short
                Ok(targeted_short_bond_amount) => {
                    bob.open_short(targeted_short_bond_amount, None, None)
                        .await?;
                }

                // Else parse the error for a to improve error messaging.
                Err(e) => {
                    // If the fn failed it's possible that the target rate would be insolvent.
                    if e.to_string()
                        .contains("Unable to find an acceptable loss with max iterations.")
                    {
                        let max_short_bond_amount = bob.calculate_max_short(None).await?;
                        let rate_after_max_short = current_state
                            .calculate_spot_rate_after_short(max_short_bond_amount, None)?;
                        // If the rate after the max short is at or above the target, then we could have hit it.
                        if rate_after_max_short >= target_rate {
                            return Err(eyre!(
                                "ERROR {}\nA short that hits the target rate exists but was not found.",
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

            // Three things should be true after opening the short:
            //
            // 1. The pool's spot price is over the min spot price prior to
            //    considering fees
            // 2. The pool's solvency is above zero.
            // 3. IF Bob's budget is not consumed; then new rate is close to the target rate

            // Check that our resulting price is over the min
            let current_state = bob.get_state().await?;
            let spot_price_after_short = current_state.calculate_spot_price()?;
            assert!(
                min_spot_price_before_short < spot_price_after_short,
                "expected min_spot_price_before_short={} < spot_price_after_short={}",
                min_spot_price_before_short,
                spot_price_after_short
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
                    "expected new_rate={} >= target_rate={} when budget constrained.",
                    new_rate,
                    target_rate
                );
            }

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }

        Ok(())
    }
}
