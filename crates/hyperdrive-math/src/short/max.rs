use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::{calculate_effective_share_reserves, State, YieldSpace};

impl State {
    /// Calculates the minimum price that the pool can support.
    ///
    /// YieldSpace intersects the y-axis with a finite slope, so there is a
    /// minimum price that the pool can support. This is the price at which the
    /// share reserves are equal to the minimum share reserves.
    ///
    /// We can solve for the bond reserves `$y_{\text{max}}$` implied by the share reserves
    /// being equal to `$z_{\text{min}}$` using the current k value:
    ///
    /// ```math
    /// k = \tfrac{c}{\mu} \cdot \left( \mu \cdot z_{min} \right)^{1 - t_s}
    /// + y_{max}^{1 - t_s} \\
    /// \implies \\
    /// y_{max} = \left( k - \tfrac{c}{\mu} \cdot \left(
    /// \mu \cdot z_{min} \right)^{1 - t_s} \right)^{\tfrac{1}{1 - t_s}}
    /// ```
    ///
    /// From there, we can calculate the spot price as normal as:
    ///
    /// ```math
    /// p = \left( \tfrac{\mu \cdot z_{min}}{y_{max}} \right)^{t_s}
    /// ```
    pub fn calculate_min_spot_price(&self) -> Result<FixedPoint<U256>> {
        let y_max = (self.k_up()?
            - (self.vault_share_price() / self.initial_vault_share_price())
                * (self.initial_vault_share_price() * self.minimum_share_reserves())
                    .pow(fixed!(1e18) - self.time_stretch())?)
        .pow(fixed!(1e18).div_up(fixed!(1e18) - self.time_stretch()))?;

        ((self.initial_vault_share_price() * self.minimum_share_reserves()) / y_max)
            .pow(self.time_stretch())
    }

    // TODO: Make it clear to the consumer that the maximum number of iterations
    // is 2 * max_iterations.
    //
    /// Calculates the max short that can be opened with the given budget.
    ///
    /// We start by finding the largest possible short (irrespective of budget),
    /// and then we iteratively approach a solution using Newton's method if the
    /// budget isn't satisified.
    ///
    /// The user can provide `maybe_conservative_price`, which is a lower bound
    /// on the realized price that the short will pay. This is used to help the
    /// algorithm converge faster in real world situations. If this is `None`,
    /// then we'll use the theoretical worst case realized price.
    pub fn calculate_max_short<
        F1: Into<FixedPoint<U256>>,
        F2: Into<FixedPoint<U256>>,
        I: Into<I256>,
    >(
        &self,
        budget: F1,
        open_vault_share_price: F2,
        checkpoint_exposure: I,
        maybe_conservative_price: Option<FixedPoint<U256>>, // TODO: Is there a nice way of abstracting the inner type?
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let budget = budget.into();
        let open_vault_share_price = open_vault_share_price.into();
        let checkpoint_exposure = checkpoint_exposure.into();

        // Sanity check that we can open any shorts at all.
        if self
            .solvency_after_short(self.minimum_transaction_amount(), checkpoint_exposure)
            .is_err()
        {
            return Err(eyre!("No solvent short is possible."));
        }

        // To avoid the case where Newton's method overshoots and stays on
        // the invalid side of the optimization equation (i.e., when deposit > budget),
        // we artificially set the target budget to be less than the actual budget.
        //
        // If the budget is less than the minimum transaction amount, then we return early.
        let target_budget = if budget < self.minimum_transaction_amount() {
            return Err(eyre!(
                "expected budget={} >= min_transaction_amount={}",
                budget,
                self.minimum_transaction_amount(),
            ));
        }
        // If the budget equals the minimum transaction amount, then we return.
        // We know it is ok because we already checked solvency after opening a
        // short with the minimum txn amount.
        else if budget == self.minimum_transaction_amount() {
            return Ok(self.minimum_transaction_amount());
        }
        // If the budget is greater than the minimum transaction amount, then we set the target budget.
        else {
            budget - self.minimum_transaction_amount()
        };

        // If the open share price is zero, then we'll use the current share
        // price since the checkpoint hasn't been minted yet.
        let open_vault_share_price = if open_vault_share_price != fixed!(0) {
            open_vault_share_price
        } else {
            self.vault_share_price()
        };

        // Assuming the budget is infinite, find the largest possible short that
        // can be opened. If the short satisfies the budget, this is the max
        // short amount.
        let spot_price = self.calculate_spot_price()?;
        // The initial guess should be guaranteed correct, and we should only get better from there.
        let absolute_max_bond_amount = self.calculate_absolute_max_short(
            spot_price,
            checkpoint_exposure,
            maybe_max_iterations,
        )?;
        // The max bond amount might be below the pool's minimum. If so, no short can be opened.
        if absolute_max_bond_amount < self.minimum_transaction_amount() {
            return Err(eyre!("No solvent short is possible."));
        }

        // Figure out the base deposit for the absolute max bond amount.
        let absolute_max_deposit =
            self.calculate_open_short(absolute_max_bond_amount, open_vault_share_price)?;
        if absolute_max_deposit <= target_budget {
            return Ok(absolute_max_bond_amount);
        }

        // Make an initial guess to refine.
        let mut max_bond_amount = self
            .max_short_guess(
                target_budget,
                spot_price,
                open_vault_share_price,
                maybe_conservative_price,
            )
            .max(self.minimum_transaction_amount());
        let mut best_valid_max_bond_amount =
            match self.solvency_after_short(max_bond_amount, checkpoint_exposure) {
                Ok(_) => max_bond_amount,
                Err(_) => self.minimum_transaction_amount(),
            };

        // Use Newton's method to iteratively approach a solution. We use the
        // short deposit in base minus the budget as our objective function,
        // which will converge to the amount of bonds that need to be shorted
        // for the short deposit to consume the entire budget. Using the
        // notation from the function comments, we can write our objective
        // function as:
        //
        // ```math
        // F(x) = B - D(x)
        // ```
        //
        // Since `$B$` is just a constant, `$F'(x) = -D'(x)$`. Given the current guess
        // of `$x_n$`, Newton's method gives us an updated guess of `$x_{n+1}$`:
        //
        // ```math
        // \begin{aligned}
        // x_{n+1} &= x_n - \tfrac{F(x_n)}{F'(x_n)} \\
        // &= x_n + \tfrac{B - D(x_n)}{D'(x_n)}
        // \end{aligned}
        // ```
        //
        // The guess that we make is very important in determining how quickly
        // we converge to the solution.
        //
        // TODO: This can get stuck in a loop if the Newton update pushes the bond amount to be too large.
        for _ in 0..maybe_max_iterations.unwrap_or(7) {
            let deposit = match self.calculate_open_short(max_bond_amount, open_vault_share_price) {
                Ok(valid_deposit) => valid_deposit,
                Err(_) => {
                    // The pool is insolvent for the guess at this point.
                    // We use the absolute max bond amount and deposit
                    // for the next guess iteration
                    max_bond_amount = best_valid_max_bond_amount;
                    // Should work this time.
                    self.calculate_open_short(max_bond_amount, open_vault_share_price)?
                }
            };

            // We update the best valid max bond amount if the deposit amount
            // is valid and the current guess is bigger than the previous best.
            if deposit <= target_budget && max_bond_amount >= best_valid_max_bond_amount {
                best_valid_max_bond_amount = max_bond_amount;
                // Stop if we found the exact solution.
                if deposit == target_budget {
                    break;
                }
            }

            // Iteratively update max_bond_amount via Newton's method.
            let derivative = self.calculate_open_short_derivative(
                max_bond_amount,
                open_vault_share_price,
                Some(spot_price),
            )?;
            if deposit < target_budget {
                max_bond_amount += (target_budget - deposit) / derivative
            }
            // deposit > target_budget
            else {
                max_bond_amount -= (deposit - target_budget) / derivative
            }

            // TODO this always iterates for max_iterations unless
            // it makes the pool insolvent. Likely want to check an
            // epsilon to early break
        }

        // Verify that the max short satisfies the budget.
        if target_budget
            < self.calculate_open_short(best_valid_max_bond_amount, open_vault_share_price)?
        {
            return Err(eyre!("max short exceeded budget"));
        }

        // Ensure that the max bond amount is within the absolute max bond amount.
        if best_valid_max_bond_amount > absolute_max_bond_amount {
            return Err(eyre!(
                "max short bond amount exceeded absolute max bond amount"
            ));
        }

        Ok(best_valid_max_bond_amount)
    }

    /// Calculates an initial guess for the max short calculation.
    ///
    /// The user can specify a conservative price that they know is less than
    /// the worst-case realized price. This significantly improves the speed of
    /// convergence of Newton's method.
    fn max_short_guess(
        &self,
        budget: FixedPoint<U256>,
        spot_price: FixedPoint<U256>,
        open_vault_share_price: FixedPoint<U256>,
        maybe_conservative_price: Option<FixedPoint<U256>>,
    ) -> FixedPoint<U256> {
        // If a conservative price is given, we can use it to solve for an
        // initial guess for what the max short is. If this conservative price
        // is an overestimate or if a conservative price isn't given, we revert
        // to using the theoretical worst case scenario as our guess.
        if let Some(conservative_price) = maybe_conservative_price {
            // Given our conservative price `$p_c$`, we can write the short deposit
            // function as:
            //
            // ```math
            // D(x) = \left( \tfrac{c}{c_0} - $p_c$ \right) \cdot x
            //        + \phi_{flat} \cdot x + \phi_{curve} \cdot (1 - p) \cdot x
            // ```
            //
            // We then solve for $x^*$ such that $D(x^*) = B$, which gives us a
            // guess of:
            //
            // ```math
            // x^* = \tfrac{B}{\tfrac{c}{c_0} - $p_c$ + \phi_{flat}
            // + \phi_{curve} \cdot (1 - p)}
            // ```
            //
            // If the budget can cover the actual short deposit on `$x^*$`, we
            // return it as our guess. Otherwise, we revert to the worst case
            // scenario.
            let guess = budget
                / (self.vault_share_price().div_up(open_vault_share_price)
                    + self.flat_fee()
                    + self.curve_fee() * (fixed!(1e18) - spot_price)
                    - conservative_price);
            if let Ok(deposit) = self.calculate_open_short(guess, open_vault_share_price) {
                if budget >= deposit {
                    return guess;
                }
            }
        }

        // We know that the max short's bond amount is greater than 0 which
        // gives us an absolute lower bound, but we can do better most of the
        // time. If the fixed rate was infinite, the max loss for shorts would
        // be 1 per bond since the spot price would be 0. With this in mind, the
        // max short amount would be equal to the budget before we consider the
        // flat fee, curve fee, and back-paid interest. Considering that the
        // budget also needs to cover the fees and back-paid interest, we
        // subtract these components from the budget to get a better estimate of
        // the max bond amount. If subtracting these components results in a
        // negative number, we just 0 as our initial guess.
        let worst_case_deposit = match self.calculate_open_short(budget, open_vault_share_price) {
            Ok(d) => d,
            Err(_) => return fixed!(0),
        };
        if budget >= worst_case_deposit {
            budget - worst_case_deposit
        } else {
            fixed!(0)
        }
    }

    /// Calculates the max short that can be opened on the YieldSpace curve
    /// without considering solvency constraints.
    fn calculate_max_short_upper_bound(&self) -> Result<FixedPoint<U256>> {
        // We have the twin constraints that $z \geq z_{min}$ and
        // $z - \zeta \geq z_{min}$. Combining these together, we calculate
        // the optimal share reserves as $z_{optimal} = z_{min} + max(0, \zeta)$.
        let optimal_share_reserves = self.minimum_share_reserves()
            + FixedPoint::try_from(self.share_adjustment().max(I256::zero()))?;

        // We calculate the optimal bond reserves by solving for the bond
        // reserves that is implied by the optimal share reserves. We can do
        // this as follows:
        //
        // k = (c / mu) * (mu * (z' - zeta)) ** (1 - t_s) + y' ** (1 - t_s)
        //                              =>
        // y' = (k - (c / mu) * (mu * (z' - zeta)) ** (1 - t_s)) ** (1 / (1 - t_s))
        let optimal_effective_share_reserves =
            calculate_effective_share_reserves(optimal_share_reserves, self.share_adjustment())?;
        let optimal_bond_reserves = self.k_down()?
            - self.vault_share_price().mul_div_up(
                self.initial_vault_share_price()
                    .mul_up(optimal_effective_share_reserves)
                    .pow(fixed!(1e18) - self.time_stretch())?,
                self.initial_vault_share_price(),
            );
        let optimal_bond_reserves = if optimal_bond_reserves >= fixed!(1e18) {
            // Rounding the exponent down results in a smaller outcome.
            optimal_bond_reserves.pow(fixed!(1e18).div_down(fixed!(1e18) - self.time_stretch()))?
        } else {
            // Rounding the exponent up results in a smaller outcome.
            optimal_bond_reserves.pow(fixed!(1e18).div_up(fixed!(1e18) - self.time_stretch()))?
        };

        Ok(optimal_bond_reserves - self.bond_reserves())
    }

    /// Calculates the absolute max short that can be opened without violating the
    /// pool's solvency constraints.
    pub fn calculate_absolute_max_short(
        &self,
        spot_price: FixedPoint<U256>,
        checkpoint_exposure: I256,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        // We start by calculating the maximum short that can be opened on the
        // YieldSpace curve.
        let absolute_max_bond_amount = self.calculate_max_short_upper_bound()?;
        if self
            .solvency_after_short(absolute_max_bond_amount, checkpoint_exposure)
            .is_ok()
        {
            return Ok(absolute_max_bond_amount);
        }

        // Use Newton's method to iteratively approach a solution. We use pool's
        // solvency $S(x)$ w.r.t. the amount of bonds shorted $x$ as our
        // objective function, which will converge to the maximum short amount
        // when $S(x) = 0$. The derivative of $S(x)$ is negative (since solvency
        // decreases as more shorts are opened). The fixed point library doesn't
        // support negative numbers, so we use the negation of the derivative to
        // side-step the issue.
        //
        // Given the current guess of $x_n$, Newton's method gives us an updated
        // guess of $x_{n+1}$:
        //
        // ```math
        // \begin{aligned}
        // x_{n+1} &= x_n - \tfrac{S(x_n)}{S'(x_n)} \\
        // &= x_n + \tfrac{S(x_n)}{-S'(x_n)}
        // \end{aligned}
        // ```
        //
        // The guess that we make is very important in determining how quickly
        // we converge to the solution.
        let mut max_bond_guess = self.absolute_max_short_guess(spot_price, checkpoint_exposure)?;
        // If the initial guess is insolvent, we need to throw an error.
        let mut solvency = self.solvency_after_short(max_bond_guess, checkpoint_exposure)?;
        for _ in 0..maybe_max_iterations.unwrap_or(7) {
            // TODO: It may be better to gracefully handle crossing over the
            // root by extending the fixed point math library to handle negative
            // numbers or even just using an if-statement to handle the negative
            // numbers.
            //
            // Calculate the next iteration of Newton's method. If the candidate
            // is larger than the absolute max, we've gone too far and something
            // has gone wrong.
            let derivative = match self.solvency_after_short_derivative(max_bond_guess, spot_price)
            {
                Ok(derivative) => derivative,
                Err(_) => break,
            };
            let possible_max_bond_amount = max_bond_guess + solvency / derivative;
            if possible_max_bond_amount > absolute_max_bond_amount {
                break;
            }

            // If the candidate is insolvent, we've gone too far and can stop
            // iterating. Otherwise, we update our guess and continue.
            solvency =
                match self.solvency_after_short(possible_max_bond_amount, checkpoint_exposure) {
                    Ok(solvency) => {
                        max_bond_guess = possible_max_bond_amount;
                        solvency
                    }
                    Err(_) => break,
                };
        }

        Ok(max_bond_guess)
    }

    /// Calculates an initial guess for the absolute max short. This is a conservative
    /// guess that will be less than the true absolute max short, which is what
    /// we need to start Newton's method.
    ///
    /// To calculate our guess, we assume an unrealistically good realized
    /// price `$p_r$` for opening the short. This allows us to approximate
    /// `$P(x) \approx \tfrac{1}{c} \cdot p_r \cdot x$`. Plugging this
    /// into our solvency function `$S(x)$`, we get an approximation of our
    /// solvency as:
    ///
    /// ```math
    /// S(x) \approx (z_0 - \tfrac{1}{c} \cdot (
    ///                  p_r - \phi_{c} \cdot (1 - p) + \phi_{g} \cdot \phi_{c} \cdot (1 - p)
    ///              )) - \tfrac{e_0 - max(e_{c}, 0)}{c} - z_{min}
    /// ```
    ///
    /// Setting this equal to zero, we can solve for our initial guess:
    ///
    /// ```math
    /// x = \frac{c \cdot (s_0 + \tfrac{max(e_{c}, 0)}{c})}{
    ///         p_r - \phi_{c} \cdot (1 - p) + \phi_{g} \cdot \phi_{c} \cdot (1 - p)
    ///     }
    /// ```
    fn absolute_max_short_guess(
        &self,
        spot_price: FixedPoint<U256>,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        let checkpoint_exposure_shares =
            FixedPoint::try_from(checkpoint_exposure.max(I256::zero()))?
                .div_down(self.vault_share_price());
        // solvency = share_reserves - long_exposure / vault_share_price - min_share_reserves
        let solvency = self.calculate_solvency()? + checkpoint_exposure_shares;
        let guess = self.vault_share_price().mul_down(solvency);
        let curve_fee = self.curve_fee().mul_down(fixed!(1e18) - spot_price);
        let gov_curve_fee = self.governance_lp_fee().mul_down(curve_fee);
        Ok(guess.div_down(spot_price - curve_fee + gov_curve_fee))
    }

    /// Calculates the pool's solvency after opening a short.
    ///
    /// We can express the pool's solvency after opening a short of `$x$` bonds
    /// as:
    ///
    /// ```math
    /// s(x) = z(x) - \tfrac{e(x)}{c} - z_{min}
    /// ```
    ///
    /// where `$z(x)$` represents the pool's share reserves after opening the
    /// short:
    ///
    /// ```math
    /// z(x) = z_0 - \left(
    ///            P(x) - \left( \tfrac{c(x)}{c} - \tfrac{g(x)}{c} \right)
    ///        \right)
    /// ```
    ///
    /// and `$e(x)$` represents the pool's exposure after opening the short:
    ///
    /// ```math
    /// e(x) = e_0 - min(x + D(x), max(e_{c}, 0))
    /// ```
    ///
    /// We simplify our `$e(x)$` formula by noting that the max short is only
    /// constrained by solvency when `$x + D(x) > max(e_{c}, 0)$` since
    /// `$x + D(x)$` grows faster than
    /// `$P(x) - \tfrac{\phi_{c}}{c} \cdot \left( 1 - p \right) \cdot x$`.
    /// With this in mind, `$min(x + D(x), max(e_{c}, 0)) = max(e_{c}, 0)$`
    /// whenever solvency is actually a constraint, so we can write:
    ///
    /// ```math
    /// e(x) = e_0 - max(e_{c}, 0)
    /// ```
    fn solvency_after_short(
        &self,
        bond_amount: FixedPoint<U256>,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        let share_delta = self.calculate_pool_share_delta_after_open_short(bond_amount)?;
        if self.share_reserves() < share_delta {
            return Err(eyre!(
                "expected share_reserves={:#?} >= share_delta={:#?}",
                self.share_reserves(),
                share_delta
            ));
        }
        let new_share_reserves = self.share_reserves() - share_delta;
        let exposure_shares = {
            let checkpoint_exposure = FixedPoint::try_from(checkpoint_exposure.max(I256::zero()))?;
            if self.long_exposure() < checkpoint_exposure {
                return Err(eyre!(
                    "expected long_exposure={:#?} >= checkpoint_exposure={:#?}.",
                    self.long_exposure(),
                    checkpoint_exposure
                ));
            } else {
                (self.long_exposure() - checkpoint_exposure) / self.vault_share_price()
            }
        };
        if new_share_reserves >= exposure_shares + self.minimum_share_reserves() {
            Ok(new_share_reserves - exposure_shares - self.minimum_share_reserves())
        } else {
            Err(eyre!("Short would result in an insolvent pool."))
        }
    }

    /// Calculates the derivative of the pool's solvency w.r.t. the short
    /// amount.
    ///
    /// The derivative is calculated as:
    ///
    /// ```math
    /// \begin{aligned}
    /// s'(x) &= z'(x) - 0 - 0
    ///       &= 0 - \left( P'(x) - \frac{(c'(x) - g'(x))}{c} \right)
    ///       &= -P'(x) + \frac{
    ///              \phi_{c} \cdot (1 - p) \cdot (1 - \phi_{g})
    ///          }{c}
    /// \end{aligned}
    /// ```
    ///
    /// Since solvency decreases as the short amount increases, we negate the
    /// derivative. This avoids issues with the fixed point library which
    /// doesn't support negative values.
    fn solvency_after_short_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
        spot_price: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        let lhs = self.calculate_short_principal_derivative(bond_amount)?;
        let rhs = self.curve_fee()
            * (fixed!(1e18) - spot_price)
            * (fixed!(1e18) - self.governance_lp_fee())
            / self.vault_share_price();
        if lhs >= rhs {
            Ok(lhs - rhs)
        } else {
            Err(eyre!("Invalid derivative."))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use ethers::types::{U128, U256};
    use fixedpointmath::{fixed, uint256};
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS, SLOW_FUZZ_RUNS},
    };
    use hyperdrive_wrappers::wrappers::{
        ihyperdrive::{Checkpoint, Options},
        mock_hyperdrive_math::MaxTradeParams,
    };
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::test_utils::{
        agent::HyperdriveMathAgent, preamble::initialize_pool_with_random_state,
    };

    /// This test differentially fuzzes the `calculate_max_short` function against
    /// the Solidity analogue `calculateMaxShort`. `calculateMaxShort` doesn't take
    /// a trader's budget into account, so it only provides a subset of
    /// `calculate_max_short`'s functionality. With this in mind, we provide
    /// `calculate_max_short` with a budget of `U256::MAX` to ensure that the two
    /// functions are equivalent.
    #[tokio::test]
    async fn fuzz_sol_calculate_max_short_without_budget() -> Result<()> {
        // TODO: We should be able to pass these tests with a much lower (if not zero) tolerance.
        let sol_correctness_tolerance = fixed!(1e17);

        // Fuzz the rust and solidity implementations against each other.
        let chain = TestChain::new().await?;
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::from(U256::from(U128::MAX)));
                if rng.gen() {
                    -I256::try_from(value)?
                } else {
                    I256::try_from(value)?
                }
            };
            let max_iterations = 7;
            // We need to catch panics because of overflows.
            let rust_max_bond_amount = panic::catch_unwind(|| {
                state.calculate_absolute_max_short(
                    state.calculate_spot_price()?,
                    checkpoint_exposure,
                    Some(max_iterations),
                )
            });
            // Run the solidity function & compare outputs.
            match chain
                .mock_hyperdrive_math()
                .calculate_max_short(
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
                Ok(sol_max_bond_amount) => {
                    // Make sure the solidity & rust runctions gave the same value.
                    let rust_max_bonds_unwrapped = rust_max_bond_amount.unwrap().unwrap();
                    let sol_max_bonds_fp = FixedPoint::from(sol_max_bond_amount);
                    let error = if sol_max_bonds_fp > rust_max_bonds_unwrapped {
                        sol_max_bonds_fp - rust_max_bonds_unwrapped
                    } else {
                        rust_max_bonds_unwrapped - sol_max_bonds_fp
                    };
                    assert!(
                        error < sol_correctness_tolerance,
                        "expected abs(solidity_amount={} - rust_amount={})={} < tolerance={}",
                        sol_max_bonds_fp,
                        rust_max_bonds_unwrapped,
                        error,
                        sol_correctness_tolerance,
                    );
                }
                // Hyperdrive Solidity calculate_max_short threw an error
                Err(sol_err) => {
                    assert!(
                        rust_max_bond_amount.is_err()
                            || rust_max_bond_amount.as_ref().unwrap().is_err(),
                        "expected rust_max_short={:#?} to have an error.\nsolidity error={:#?}",
                        rust_max_bond_amount,
                        sol_err
                    );
                }
            };
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_max_short_budget_consumed() -> Result<()> {
        // TODO: This should be fixed!(0.0001e18) == 0.01%
        let budget_tolerance = fixed!(1e18);

        // Spawn a test chain and create two agents -- Alice and Bob. Alice
        // is funded with a large amount of capital so that she can initialize
        // the pool. Bob is funded with a small amount of capital so that we
        // can test `calculate_max_short` when budget is the primary constraint.
        let mut rng = thread_rng();

        // Initialize the chain and the agents.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let config = alice.get_config().clone();

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Fund Alice and Bob.
            let contribution = rng.gen_range(fixed!(100_000e18)..=fixed!(100_000_000e18));
            alice.fund(contribution).await?;

            // Alice initializes the pool.
            let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
            alice.initialize(fixed_rate, contribution, None).await?;

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

            // Get the current state of the pool.
            let state = alice.get_state().await?;
            let Checkpoint {
                vault_share_price: open_vault_share_price,
                weighted_spot_price: _,
                last_weighted_spot_price_update_time: _,
            } = alice
                .get_checkpoint(state.to_checkpoint(alice.now().await?))
                .await?;
            let checkpoint_exposure = alice
                .get_checkpoint_exposure(state.to_checkpoint(alice.now().await?))
                .await?;

            let global_max_short_bonds = state.calculate_absolute_max_short(
                state.calculate_spot_price()?,
                checkpoint_exposure,
                None,
            )?;

            // Bob should always be budget constrained when trying to open the short.
            let global_max_base_required = state
                .calculate_open_short(global_max_short_bonds, open_vault_share_price.into())?;
            let budget = rng.gen_range(
                state.minimum_transaction_amount()..=global_max_base_required - fixed!(1e18),
            );
            bob.fund(budget).await?;

            // Bob opens a max short position. We allow for a very small amount
            // of slippage to account for interest accrual between the time the
            // calculation is performed and the transaction is submitted.
            let slippage_tolerance = fixed!(0.0001e18); // 0.01%
            let max_short_bonds = bob.calculate_max_short(Some(slippage_tolerance)).await?;
            bob.open_short(max_short_bonds, None, None).await?;

            // Bob used a slippage tolerance of 0.01%, which means
            // that the max short is always consuming at least 99.99% of
            // the budget.
            let max_allowable_balance =
                budget * (fixed!(1e18) - slippage_tolerance) * budget_tolerance;
            let remaining_balance = bob.base();
            assert!(remaining_balance < max_allowable_balance,
                "expected {}% of budget consumed, or remaining_balance={} < max_allowable_balance={}
                global_max_short_bonds = {}; max_short_bonds = {}; global_max_base_required={}",
                format!("{}", fixed!(100e18)*(fixed!(1e18) - budget_tolerance)).trim_end_matches("0"),
                remaining_balance,
                max_allowable_balance,
                global_max_short_bonds,
                max_short_bonds,
                global_max_base_required,
            );

            // Revert to the snapshot and reset the agents' wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_sol_calculate_max_short_without_budget_then_open_short() -> Result<()> {
        let max_bonds_tolerance = fixed!(1e10);
        let max_base_tolerance = fixed!(1e10);
        let reserves_drained_tolerance = fixed!(1e27);

        // Set up a random number generator. We use ChaCha8Rng with a randomly
        // generated seed, which makes it easy to reproduce test failures given
        // the seed.
        let mut rng = {
            let mut rng = thread_rng();
            let seed = rng.gen();
            ChaCha8Rng::seed_from_u64(seed)
        };

        // Initialize the test chain.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let mut celine = chain.celine().await?;

        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;

            // Get the current state from solidity.
            let mut state = alice.get_state().await?;

            // Get the current checkpoint exposure.
            let checkpoint_exposure = alice
                .get_checkpoint_exposure(state.to_checkpoint(alice.now().await?))
                .await?;

            // Get the global max short from Solidity.
            let max_iterations = 7;
            match chain
                .mock_hyperdrive_math()
                .calculate_max_short(
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
                Ok(sol_max_bonds) => {
                    // Solidity reports everything is good, so we run the Rust fns.
                    let rust_max_bonds = panic::catch_unwind(|| {
                        state.calculate_absolute_max_short(
                            state.calculate_spot_price()?,
                            checkpoint_exposure,
                            Some(max_iterations),
                        )
                    });

                    // Compare the max bond amounts.
                    let rust_max_bonds_unwrapped = rust_max_bonds.unwrap().unwrap();
                    let sol_max_bonds_fp = FixedPoint::from(sol_max_bonds);
                    let error = if rust_max_bonds_unwrapped > sol_max_bonds_fp {
                        rust_max_bonds_unwrapped - sol_max_bonds_fp
                    } else {
                        sol_max_bonds_fp - rust_max_bonds_unwrapped
                    };
                    assert!(
                        error < max_bonds_tolerance,
                        "expected abs(rust_bonds - sol_bonds)={} >= max_bonds_tolerance={}",
                        error,
                        max_bonds_tolerance
                    );

                    // The amount Celine has to pay will always be less than the bond amount.
                    celine.fund(sol_max_bonds.into()).await?;
                    match celine
                        .hyperdrive()
                        .open_short(
                            sol_max_bonds.into(),
                            FixedPoint::from(U256::MAX).into(),
                            fixed!(0).into(),
                            Options {
                                destination: celine.address(),
                                as_base: true,
                                extra_data: [].into(),
                            },
                        )
                        .call()
                        .await
                    {
                        Ok((_, sol_max_base)) => {
                            // Calling any Solidity Hyperdrive transaction causes the
                            // mock yield source to accrue some interest. We want to use
                            // the state before the Solidity OpenShort, but with the
                            // vault share price after the block tick.
                            // Get the current vault share price & update state.
                            let vault_share_price = alice.get_state().await?.vault_share_price();
                            state.info.vault_share_price = vault_share_price.into();

                            // Get the open vault share price.
                            let Checkpoint {
                                weighted_spot_price: _,
                                last_weighted_spot_price_update_time: _,
                                vault_share_price: open_vault_share_price,
                            } = alice
                                .get_checkpoint(state.to_checkpoint(alice.now().await?))
                                .await?;

                            // Compare the open short call outputs.
                            let rust_max_base = state.calculate_open_short(
                                rust_max_bonds_unwrapped,
                                open_vault_share_price.into(),
                            );

                            let rust_max_base_unwrapped = rust_max_base.unwrap();
                            let sol_max_base_fp = FixedPoint::from(sol_max_base);
                            let error = if rust_max_base_unwrapped > sol_max_base_fp {
                                rust_max_base_unwrapped - sol_max_base_fp
                            } else {
                                sol_max_base_fp - rust_max_base_unwrapped
                            };
                            assert!(
                                error < max_base_tolerance,
                                "expected abs(rust_base - sol_base)={} >= max_base_tolerance={}",
                                error,
                                max_base_tolerance
                            );

                            // Make sure the pool was drained.
                            let pool_shares = state
                                .effective_share_reserves()?
                                .min(state.share_reserves());
                            let min_share_reserves = state.minimum_share_reserves();
                            assert!(pool_shares >= min_share_reserves,
                                "effective_share_reserves={} should always be greater than the minimum_share_reserves={}.",
                                state.effective_share_reserves()?,
                                min_share_reserves,
                            );
                            let reserve_amount_above_minimum = pool_shares - min_share_reserves;
                            assert!(reserve_amount_above_minimum < reserves_drained_tolerance,
                                "share_reserves={} - minimum_share_reserves={} (diff={}) should be < tolerance={}",
                                pool_shares,
                                min_share_reserves,
                                reserve_amount_above_minimum,
                                reserves_drained_tolerance,
                            );
                        }

                        // Solidity calculate_max_short worked, but passing that bond amount to open_short failed.
                        Err(_) => assert!(
                            false,
                            "Solidity calculate_max_short produced an insolvent answer!"
                        ),
                    }
                }

                // Solidity calculate_max_short failed; verify that rust calculate_max_short fails.
                Err(_) => {
                    // Get the current vault share price & update state.
                    let vault_share_price = alice.get_state().await?.vault_share_price();
                    state.info.vault_share_price = vault_share_price.into();

                    // Get the current checkpoint exposure.
                    let checkpoint_exposure = alice
                        .get_checkpoint_exposure(state.to_checkpoint(alice.now().await?))
                        .await?;

                    // Solidity reports everything is good, so we run the Rust fns.
                    let rust_max_bonds = panic::catch_unwind(|| {
                        state.calculate_absolute_max_short(
                            state.calculate_spot_price()?,
                            checkpoint_exposure,
                            Some(max_iterations),
                        )
                    });

                    assert!(rust_max_bonds.is_err() || rust_max_bonds.unwrap().is_err());
                }
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
