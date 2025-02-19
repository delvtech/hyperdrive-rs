use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, int256, FixedPoint};

use crate::{State, YieldSpace};

impl State {
    /// Calculates the pool's max spot price.
    ///
    /// Hyperdrive has assertions to ensure that traders don't purchase bonds at
    /// negative interest rates. The maximum spot price that longs can push the
    /// market to is given by:
    ///
    /// ```math
    /// p_{\text{max}} = \frac{1 - \phi_f}{1 + \phi_c \cdot
    /// \left( p_0^{-1} - 1 \right) \cdot \left( \phi_f - 1 \right)}
    /// ```
    pub fn calculate_max_spot_price(&self) -> Result<FixedPoint<U256>> {
        Ok((fixed!(1e18) - self.flat_fee())
            / (fixed!(1e18)
                + self
                    .curve_fee()
                    .mul_up(fixed!(1e18).div_up(self.calculate_spot_price_down()?) - fixed!(1e18)))
            .mul_up(fixed!(1e18) - self.flat_fee()))
    }

    /// Calculates the max long that can be opened given a budget.
    ///
    /// We start by calculating the long that brings the pool's spot price to 1.
    /// If we are solvent at this point, then we're done. Otherwise, we approach
    /// the max long iteratively using Newton's method.
    pub fn calculate_max_long<F: Into<FixedPoint<U256>>, I: Into<I256>>(
        &self,
        budget: F,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let budget = budget.into();
        let checkpoint_exposure = checkpoint_exposure.into();

        // Check the spot price after opening a minimum long is less than the
        // max spot price
        let spot_price_after_min_long =
            self.calculate_spot_price_after_long(self.minimum_transaction_amount(), None)?;
        if spot_price_after_min_long > self.calculate_max_spot_price()? {
            return Ok(fixed!(0));
        }

        // Calculate the maximum long that brings the spot price to 1.
        // If the pool is solvent after opening this long, then we're done.
        let (absolute_max_base_amount, absolute_max_bond_amount) = self.absolute_max_long()?;
        if self
            .solvency_after_long(
                absolute_max_base_amount,
                absolute_max_bond_amount,
                checkpoint_exposure,
            )
            .is_ok()
            && absolute_max_base_amount >= self.minimum_transaction_amount()
        {
            return Ok(absolute_max_base_amount.min(budget));
        }

        // Use Newton's method to iteratively approach a solution. We use pool's
        // solvency `$S(x)$` as our objective function, which will converge to the
        // amount of base that needs to be paid to open the maximum long. The
        // derivative of `$S(x)$` is negative (since solvency decreases as more
        // longs are opened). The fixed point library doesn't support negative
        // numbers, so we use the negation of the derivative to side-step the
        // issue.
        //
        // Given the current guess of `$x_n$`, Newton's method gives us an updated
        // guess of `$x_{n+1}$`:
        //
        // ```math
        // x_{n+1} = x_n - \tfrac{S(x_n)}{S'(x_n)} = x_n + \tfrac{S(x_n)}{-S'(x_n)}
        // ```
        //
        // The guess that we make is very important in determining how quickly
        // we converge to the solution.
        let mut max_base_amount =
            self.max_long_guess(absolute_max_base_amount, checkpoint_exposure)?;

        // possible_max_base_amount might be less than minimum transaction amount.
        // we clamp here if so
        if max_base_amount < self.minimum_transaction_amount() {
            max_base_amount = self.minimum_transaction_amount();
        }
        let mut solvency = match self.solvency_after_long(
            max_base_amount,
            self.calculate_open_long(max_base_amount)?,
            checkpoint_exposure,
        ) {
            Ok(solvency) => solvency,
            Err(err) => {
                return Err(eyre!(
                    "Initial guess in `calculate_max_long` is insolvent with error:\n{:#?}",
                    err
                ))
            }
        };
        for _ in 0..maybe_max_iterations.unwrap_or(7) {
            // If the max base amount is equal to or exceeds the absolute max,
            // we've gone too far and the calculation deviated from reality at
            // some point.
            if max_base_amount >= absolute_max_base_amount {
                return Err(eyre!(
                    "Reached absolute max bond amount in `calculate_max_long`."
                ));
            }

            // If the max base amount exceeds the budget, we know that the
            // entire budget can be consumed without running into solvency
            // constraints.
            if max_base_amount >= budget {
                return Ok(budget);
            }

            // TODO: It may be better to gracefully handle crossing over the
            // root by extending the fixed point math library to handle negative
            // numbers or even just using an if-statement to handle the negative
            // numbers.
            //
            // Proceed to the next step of Newton's method. Once we have a
            // candidate solution, we check to see if the pool is solvent after
            // a long is opened with the candidate amount. If the pool isn't
            // solvent, then we're done.
            let derivative = match self.solvency_after_long_derivative_negation(max_base_amount) {
                Ok(d) => d,
                Err(_) => break,
            };

            let mut possible_max_base_amount = max_base_amount + solvency / derivative;

            // possible_max_base_amount might be less than minimum transaction amount.
            // we clamp here if so
            if possible_max_base_amount < self.minimum_transaction_amount() {
                possible_max_base_amount = self.minimum_transaction_amount();
            }

            solvency = match self.solvency_after_long(
                possible_max_base_amount,
                self.calculate_open_long(possible_max_base_amount)?,
                checkpoint_exposure,
            ) {
                Ok(solvency) => solvency,
                Err(_) => break,
            };
            max_base_amount = possible_max_base_amount;
        }

        // If the max base amount is less than the minimum transaction amount, we return 0 as the max long.
        if max_base_amount <= self.minimum_transaction_amount() {
            return Ok(fixed!(0));
        }

        // Ensure that the final result is less than the absolute max and clamp
        // to the budget.
        if max_base_amount >= absolute_max_base_amount {
            return Err(eyre!(
                "Reached absolute max bond amount in `calculate_max_long`."
            ));
        }

        Ok(max_base_amount.min(budget))
    }

    /// Calculates the largest long that can be opened without buying bonds at a
    /// negative interest rate. This calculation does not take Hyperdrive's
    /// solvency constraints into account and shouldn't be used directly.
    fn absolute_max_long(&self) -> Result<(FixedPoint<U256>, FixedPoint<U256>)> {
        // We are targeting the pool's max spot price of:
        //
        // p_max = (1 - flatFee) / (1 + curveFee * (1 / p_0 - 1) * (1 - flatFee))
        //
        // We can derive a formula for the target bond reserves y_t in
        // terms of the target share reserves z_t as follows:
        //
        // p_max = ((mu * z_t) / y_t) ** t_s
        //
        //                       =>
        //
        // y_t = (mu * z_t) * ((1 + curveFee * (1 / p_0 - 1) * (1 - flatFee)) / (1 - flatFee)) ** (1 / t_s)
        //
        // Our equation for price is the inverse of that used by YieldSpace, which must be considered when
        // deriving the invariant from the price equation.
        // With this in mind, we can use this formula to solve our YieldSpace invariant for z_t:
        //
        // k = (c / mu) * (mu * z_t) ** (1 - t_s) +
        //     (
        //         (mu * z_t) * ((1 + curveFee * (1 / p_0 - 1) * (1 - flatFee)) / (1 - flatFee)) ** (1 / t_s)
        //     ) ** (1 - t_s)
        //
        //                       =>
        //
        // z_t = (1 / mu) * (
        //           k / (
        //               (c / mu) +
        //               ((1 + curveFee * (1 / p_0 - 1) * (1 - flatFee)) / (1 - flatFee)) ** ((1 - t_s) / t_s))
        //           )
        //       ) ** (1 / (1 - t_s))
        let inner = self
            .k_down()?
            .div_down(
                self.vault_share_price()
                    .div_up(self.initial_vault_share_price())
                    + ((fixed!(1e18)
                        + self
                            .curve_fee()
                            .mul_up(
                                fixed!(1e18).div_up(self.calculate_spot_price_down()?)
                                    - fixed!(1e18),
                            )
                            .mul_up(fixed!(1e18) - self.flat_fee()))
                    .div_up(fixed!(1e18) - self.flat_fee()))
                    .pow((fixed!(1e18) - self.time_stretch()).div_down(self.time_stretch()))?,
            )
            .pow(fixed!(1e18).div_down(fixed!(1e18) - self.time_stretch()))?;
        let target_share_reserves = inner.div_down(self.initial_vault_share_price());

        // Now that we have the target share reserves, we can calculate the
        // target bond reserves using the formula:
        //
        // y_t = (mu * z_t) * ((1 + curveFee * (1 / p_0 - 1) * (1 - flatFee)) / (1 - flatFee)) ** (1 / t_s)
        //
        // `inner` as defined above is `mu * z_t` so we calculate y_t as
        //
        // y_t = inner * ((1 + curveFee * (1 / p_0 - 1) * (1 - flatFee)) / (1 - flatFee)) ** (1 / t_s)
        let fee_adjustment = self.curve_fee()
            * (fixed!(1e18) / self.calculate_spot_price_down()? - fixed!(1e18))
            * (fixed!(1e18) - self.flat_fee());
        let target_bond_reserves = ((fixed!(1e18) + fee_adjustment)
            / (fixed!(1e18) - self.flat_fee()))
        .pow(fixed!(1e18).div_up(self.time_stretch()))?
            * inner;

        // Catch if the target share reserves are smaller than the effective share reserves.
        let effective_share_reserves = self.effective_share_reserves()?;
        if target_share_reserves < effective_share_reserves {
            return Err(eyre!(
                "target share reserves less than effective share reserves"
            ));
        }

        // The absolute max base amount is given by:
        // absolute_max_base_amount = (z_t - z_e) * c
        let absolute_max_base_amount =
            (target_share_reserves - effective_share_reserves) * self.vault_share_price();

        // The absolute max bond amount is given by:
        // absolute_max_bond_amount = (y - y_t) - Phi_c(absolute_max_base_amount)
        let absolute_max_bond_amount = (self.bond_reserves() - target_bond_reserves)
            - self.open_long_curve_fee(absolute_max_base_amount)?;

        Ok((absolute_max_base_amount, absolute_max_bond_amount))
    }

    /// Calculates an initial guess of the max long that can be opened. This is a
    /// reasonable estimate that is guaranteed to be less than the true max
    /// long. We use this to get a reasonable starting point for Newton's
    /// method.
    fn max_long_guess(
        &self,
        absolute_max_base_amount: FixedPoint<U256>,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        // Calculate an initial estimate of the max long by using the spot price as
        // our conservative price.
        let spot_price = self.calculate_spot_price_down()?;
        let guess = self.max_long_estimate(spot_price, spot_price, checkpoint_exposure)?;

        // We know that the spot price is 1 when the absolute max base amount is
        // used to open a long. We also know that our spot price isn't a great
        // estimate (conservative or otherwise) of the realized price that the
        // max long will pay, so we calculate a better estimate of the realized
        // price by interpolating between the spot price and 1 depending on how
        // large the estimate is.
        let t = (guess / absolute_max_base_amount)
            .pow(fixed!(1e18).div_up(fixed!(1e18) - self.time_stretch()))?
            * fixed!(0.8e18);
        let estimate_price = spot_price * (fixed!(1e18) - t) + fixed!(1e18) * t;

        // Recalculate our intial guess using the bootstrapped conservative
        // estimate of the realized price.
        self.max_long_estimate(estimate_price, spot_price, checkpoint_exposure)
    }

    /// Estimates the max long based on the pool's current solvency and a
    /// conservative price estimate, `$p_r$`.
    ///
    /// We can use our estimate price `$p_r$` to approximate `$y(x)$` as
    /// `$y(x) \approx p_r^{-1} \cdot x - c(x)$`. Plugging this into our
    /// solvency function $s(x)$, we can calculate the share reserves and
    /// exposure after opening a long with `$x$` base as:
    ///
    /// ```math
    /// \begin{aligned}
    /// z(x) &= z_0 + \tfrac{x - g(x)}{c} \\
    /// e(x) &= e_0 + min(\text{exposure}_{c}, 0) + 2 \cdot y(x) - x + g(x) \\
    ///      &= e_0 + min(\text{exposure}_{c}, 0) + 2 \cdot p_r^{-1} \cdot x -
    ///             2 \cdot c(x) - x + g(x)
    /// \end{aligned}
    /// ```
    ///
    /// We debit and negative checkpoint exposure from $e_0$ since the
    /// global exposure doesn't take into account the negative exposure
    /// from non-netted shorts in the checkpoint. These forumulas allow us
    /// to calculate the approximate ending solvency of:
    ///
    /// ```math
    /// s(x) \approx z(x) - \tfrac{e(x) - min(exposure_{c}, 0)}{c} - z_{min}
    /// ```
    ///
    /// If we let the initial solvency be given by `$s_0$`, we can solve for
    /// `$x$` as:
    ///
    /// ```math
    /// x = \frac{c}{2} \cdot \frac{s_0 + min(exposure_{c}, 0)}{
    ///         p_r^{-1} +
    ///         \phi_{g} \cdot \phi_{c} \cdot \left( 1 - p \right) -
    ///         1 -
    ///         \phi_{c} \cdot \left( p^{-1} - 1 \right)
    ///     }
    /// ```
    fn max_long_estimate(
        &self,
        estimate_price: FixedPoint<U256>,
        spot_price: FixedPoint<U256>,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        let checkpoint_exposure = FixedPoint::try_from(-checkpoint_exposure.min(int256!(0)))?;
        let mut estimate =
            self.calculate_solvency()? + checkpoint_exposure / self.vault_share_price();
        estimate = estimate.mul_div_down(self.vault_share_price(), fixed!(2e18));
        estimate /= fixed!(1e18) / estimate_price
            + self.governance_lp_fee() * self.curve_fee() * (fixed!(1e18) - spot_price)
            - fixed!(1e18)
            - self.curve_fee() * (fixed!(1e18) / spot_price - fixed!(1e18));
        Ok(estimate)
    }

    /// Calculates the solvency of the pool `$S(x)$` after a long is opened with
    /// a base amount `$x$`.
    ///
    /// Since longs can net out with shorts in this checkpoint, we decrease
    /// the global exposure variable by any negative exposure we have
    /// in the checkpoint. The pool's solvency is calculated as:
    ///
    /// ```math
    /// s = z - \tfrac{exposure + min(\text{exposure}_{c}, 0)}{c} - z_{min}
    /// ```
    ///
    /// When a long is opened, the share reserves `$z$` increase by:
    ///
    /// ```math
    /// \Delta z = \tfrac{x - g(x)}{c}
    /// ```
    ///
    /// Opening the long increases the non-netted longs by the bond amount. From
    /// this, the change in the exposure is given by:
    ///
    /// ```math
    /// \Delta exposure = y(x)
    /// ```
    ///
    /// From this, we can calculate `$S(x)$` as:
    ///
    /// ```math
    /// S(x) = \left( z + \Delta z \right) - \left(
    ///   \tfrac{exposure + min(exposure_{checkpoint}, 0) + \Delta exposure}{c}
    /// \right) - z_{min}
    /// ```
    ///
    /// It's possible that the pool is insolvent after opening a long. In this
    /// case, we return `None` since the fixed point library can't represent
    /// negative numbers.
    pub(super) fn solvency_after_long(
        &self,
        base_amount: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        let governance_fee_shares =
            self.open_long_governance_fee(base_amount, None)? / self.vault_share_price();
        let share_amount = base_amount / self.vault_share_price();
        if self.share_reserves() + share_amount < governance_fee_shares {
            return Err(eyre!(
                "expected new_share_amount={:#?} >= governance_fee_shares={:#?}",
                self.share_reserves() + share_amount,
                governance_fee_shares
            ));
        }
        let share_reserves = (self.share_reserves() + share_amount) - governance_fee_shares;
        let exposure = self.long_exposure() + bond_amount;
        // Netting allows us to remove any negative checkpoint exposure from the
        // long exposure.
        let checkpoint_exposure = FixedPoint::try_from(-checkpoint_exposure.min(int256!(0)))?;
        if share_reserves + checkpoint_exposure / self.vault_share_price()
            >= exposure / self.vault_share_price() + self.minimum_share_reserves()
        {
            Ok(
                share_reserves + checkpoint_exposure / self.vault_share_price()
                    - exposure / self.vault_share_price()
                    - self.minimum_share_reserves(),
            )
        } else {
            Err(eyre!("Long would result in an insolvent pool."))
        }
    }

    /// Calculates the negation of the derivative of the pool's solvency with respect
    /// to the base amount that the long pays.
    ///
    /// The derivative of the pool's solvency `$S(x)$` with respect to the base
    /// amount that the long pays is given by:
    ///
    /// ```math
    /// S'(x) = \tfrac{1}{c} \cdot \left( 1 - y'(x) - \phi_{g} \cdot p \cdot c'(x) \right) \\
    ///       = \tfrac{1}{c} \cdot \left(
    ///             1 - y'(x) - \phi_{g} \cdot \phi_{c} \cdot \left( 1 - p \right)
    ///         \right)
    /// ```
    ///
    /// This derivative is negative since solvency decreases as more longs are
    /// opened. We use the negation of the derivative to stay in the positive
    /// domain, which allows us to use the fixed point library.
    pub(super) fn solvency_after_long_derivative_negation(
        &self,
        base_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        let derivative = self.calculate_open_long_derivative(base_amount)?;
        let spot_price = self.calculate_spot_price_down()?;
        Ok(
            (derivative
                + self.governance_lp_fee() * self.curve_fee() * (fixed!(1e18) - spot_price)
                - fixed!(1e18))
            .div_down(self.vault_share_price()),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use ethers::types::U256;
    use fixedpointmath::{uint256, FixedPointValue};
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS},
    };
    use hyperdrive_wrappers::wrappers::mock_hyperdrive_math::MaxTradeParams;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::{calculate_effective_share_reserves, test_utils::agent::HyperdriveMathAgent};

    /// This test differentially fuzzes the `absolute_max_long` function against
    /// the Solidity analogue `calculateAbsoluteMaxLong`.
    #[tokio::test]
    async fn fuzz_sol_absolute_max_long() -> Result<()> {
        let chain = TestChain::new().await?;
        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let rust_absolute_max_long = panic::catch_unwind(|| state.absolute_max_long());
            match chain
                .mock_hyperdrive_math()
                .calculate_absolute_max_long(
                    MaxTradeParams {
                        share_reserves: state.info.share_reserves,
                        bond_reserves: state.info.bond_reserves,
                        longs_outstanding: state.info.longs_outstanding,
                        long_exposure: state.info.long_exposure,
                        share_adjustment: state.info.share_adjustment,
                        time_stretch: state.config.time_stretch,
                        vault_share_price: state.info.vault_share_price,
                        initial_vault_share_price: state.config.initial_vault_share_price,
                        minimum_share_reserves: state.config.minimum_share_reserves,
                        curve_fee: state.config.fees.curve,
                        flat_fee: state.config.fees.flat,
                        governance_lp_fee: state.config.fees.governance_lp,
                    },
                    calculate_effective_share_reserves(
                        state.info.share_reserves.into(),
                        state.info.share_adjustment,
                    )?
                    .into(),
                    state.calculate_spot_price_down()?.into(),
                )
                .call()
                .await
            {
                Ok((sol_base_amount, sol_bond_amount)) => {
                    let (rust_base_amount, rust_bond_amount) =
                        rust_absolute_max_long.unwrap().unwrap();
                    assert_eq!(rust_base_amount, FixedPoint::from(sol_base_amount));
                    assert_eq!(rust_bond_amount, FixedPoint::from(sol_bond_amount));
                }
                Err(_) => assert!(
                    rust_absolute_max_long.is_err() || rust_absolute_max_long.unwrap().is_err()
                ),
            }
        }
        Ok(())
    }

    /// This test differentially fuzzes the `calculate_max_long` function against the
    /// Solidity analogue `calculateMaxLong`. `calculateMaxLong` doesn't take
    /// a trader's budget into account, so it only provides a subset of
    /// `calculate_max_long`'s functionality. With this in mind, we provide
    /// `calculate_max_long` with a budget of `U256::MAX` to ensure that the two
    /// functions are equivalent.
    #[tokio::test]
    async fn fuzz_sol_calculate_max_long() -> Result<()> {
        let test_tolerance = fixed!(1e13);
        let chain = TestChain::new().await?;
        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;
            // Gen a random state.
            let state = rng.gen::<State>();
            // Generate a random checkpoint exposure.
            let checkpoint_exposure = rng.gen_range(0..=i128::MAX).flip_sign_if(rng.gen()).into();
            // Check Solidity against Rust.
            let max_iterations = 8usize;
            // We need to catch panics because of overflows.
            let rust_base_amount = panic::catch_unwind(|| {
                state.calculate_max_long(
                    U256::MAX,
                    checkpoint_exposure,
                    Some(max_iterations.into()),
                )
            });
            match chain
                .mock_hyperdrive_math()
                .calculate_max_long(
                    MaxTradeParams {
                        share_reserves: state.info.share_reserves,
                        bond_reserves: state.info.bond_reserves,
                        longs_outstanding: state.info.longs_outstanding,
                        long_exposure: state.info.long_exposure,
                        share_adjustment: state.info.share_adjustment,
                        time_stretch: state.config.time_stretch,
                        vault_share_price: state.info.vault_share_price,
                        initial_vault_share_price: state.config.initial_vault_share_price,
                        minimum_share_reserves: state.config.minimum_share_reserves,
                        curve_fee: state.config.fees.curve,
                        flat_fee: state.config.fees.flat,
                        governance_lp_fee: state.config.fees.governance_lp,
                    },
                    checkpoint_exposure,
                    max_iterations.into(),
                )
                .call()
                .await
            {
                Ok((sol_base_amount, ..)) => {
                    let rust_base_amount = rust_base_amount.unwrap().unwrap();
                    let sol_base_amount = FixedPoint::from(sol_base_amount);
                    let error = if rust_base_amount > sol_base_amount {
                        rust_base_amount - sol_base_amount
                    } else {
                        sol_base_amount - rust_base_amount
                    };
                    assert!(
                        error <= test_tolerance,
                        "abs(rust_base_amount={:#?}-sol_base_amount={:#?})={:#?} > tolerance={:#?}",
                        rust_base_amount,
                        sol_base_amount,
                        error,
                        test_tolerance
                    );
                }
                Err(_) => assert!(rust_base_amount.is_err() || rust_base_amount.unwrap().is_err()),
            }
            // Reset chain snapshot.
            chain.revert(id).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_calculate_max_long() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob. Alice
        // is funded with a large amount of capital so that she can initialize
        // the pool. Bob is funded with a small amount of capital so that we
        // can test `calculate_max_long` when budget is the primary constraint.
        let mut rng = thread_rng();
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;

        let config = bob.get_config().clone();

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Fund Alice and Bob.
            let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
            let contribution = rng.gen_range(fixed!(10_000e18)..=fixed!(500_000_000e18));
            let budget = rng.gen_range(fixed!(10e18)..=fixed!(500_000_000e18));
            alice.fund(contribution).await?;
            bob.fund(budget).await?;

            // Alice initializes the pool.
            alice.initialize(fixed_rate, contribution, None).await?;

            // Some of the checkpoint passes and variable interest accrues.
            alice
                .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
                .await?;
            let rate = rng.gen_range(fixed!(0)..=fixed!(0.5e18));
            alice
                .advance_time(
                    rate,
                    FixedPoint::from(config.checkpoint_duration) * fixed!(0.5e18),
                )
                .await?;

            // Bob opens a max long.
            let max_spot_price = bob.get_state().await?.calculate_max_spot_price()?;
            let max_long = bob.calculate_max_long(None).await?;
            let state = alice.get_state().await?;
            // Ensure max long is valid.
            if state.calculate_open_long(max_long).is_err() {
                continue;
            }
            // Get the andicipated spot price & open the log.
            let spot_price_after_long = bob
                .get_state()
                .await?
                .calculate_spot_price_after_long(max_long, None)?;
            bob.open_long(max_long, None, None).await?;

            // One of three things should be true after opening the long:
            //
            // 1. The pool's spot price reached the max spot price prior to
            //    considering fees.
            // 2. The pool's solvency is close to zero.
            // 3. Bob's budget is consumed.
            let is_max_price =
                max_spot_price - spot_price_after_long.min(max_spot_price) < fixed!(1e15);
            let is_solvency_consumed = {
                let state = alice.get_state().await?;
                let error_tolerance = fixed!(1_000e18).mul_div_down(fixed_rate, fixed!(0.1e18));
                state.calculate_solvency()? < error_tolerance
            };
            let is_budget_consumed = {
                let error_tolerance = fixed!(1e18);
                bob.base() < error_tolerance
            };
            assert!(
                is_max_price || is_solvency_consumed || is_budget_consumed,
                "Invalid max long."
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }
}
