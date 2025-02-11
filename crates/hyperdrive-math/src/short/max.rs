use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, fixed_i256, FixedPoint};
use rand::{thread_rng, Rng};

use crate::{calculate_effective_share_reserves, State, YieldSpace};

impl State {
    /// Calculates the minimum price that the pool can support.
    ///
    /// YieldSpace intersects the y-axis with a finite slope, so there is a
    /// minimum price that the pool can support. This is the price at which the
    /// share reserves are equal to the minimum share reserves.
    ///
    /// We can solve for the bond reserves `$y_{\text{max}}$` implied by the
    /// share reserves being equal to `$z_{\text{min}}$` using the current `$k$`
    /// value:
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

    /// Calculate the minimum pool share reserves assuming a short trade is
    /// next, given the current exposure and share adjustment.
    ///
    /// Given the pool config parameter `z_{min}`, we have the twin constraints
    /// that `$z \geq z_{min} + \frac{e}{c}$` and `$z - \zeta \geq z_{min}$`.
    /// These are checked independently, so they can be combined via a `max`
    /// operation, yielding
    /// `$min_share_reserves = z_{min} + max(max(0, \zeta), \frac{e}{c}$`).
    ///
    /// This approach ignores that the short will subtract from long exposure.
    pub fn calculate_min_share_reserves_given_exposure(&self) -> Result<FixedPoint<I256>> {
        // We want the current total exposure, with no netting from shorts.
        // exposure_shares = e/c
        let exposure_shares = self
            .exposure_after_short_shares(fixed!(0))?
            .change_type::<I256>()?;
        // min_share_reserves = z_min + max(max(zeta, 0), e/c)
        let min_share_reserves = self.minimum_share_reserves().change_type::<I256>()?
            + FixedPoint::<I256>::try_from(self.share_adjustment().max(I256::zero()))?
                .max(exposure_shares);
        Ok(min_share_reserves)
    }

    /// Calculate the current exposure minus new short exposure, in shares.
    ///
    /// The long exposure will account for any executed trades. Any new short
    /// nets previous longs by subtracting bonds from the long exposure.
    /// This increases solvency until the checkpoint exposure goes negative.
    /// Past that point, shorts will impact solvency by decreasing share
    /// reserves.
    fn exposure_after_short_shares(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        let exposure_shares = {
            // The short will net out all long exposure.
            if self.long_exposure() < bond_amount {
                fixed!(0)
            }
            // The short will leave some exposure.
            else {
                (self.long_exposure() - bond_amount).div_up(self.vault_share_price())
            }
        };
        Ok(exposure_shares)
    }

    /// Use Newton's method to find the amount of bonds shorted for a given base
    /// deposit amount.
    ///
    /// If the result is Ok then the answer is guaranteed to be within
    /// `maybe_base_tolerance` of the target base amount (default is 1e10).
    ///
    /// Increasing `maybe_max_iterations` will increase the accuracy of the
    /// result (default is 1_000).
    pub fn calculate_short_bonds_given_deposit(
        &self,
        target_base_amount: FixedPoint<U256>,
        open_vault_share_price: FixedPoint<U256>,
        absolute_max_bond_amount: FixedPoint<U256>,
        maybe_base_tolerance: Option<FixedPoint<U256>>,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let base_tolerance = maybe_base_tolerance.unwrap_or(fixed!(1e10));
        let max_iterations = maybe_max_iterations.unwrap_or(1_000);

        // The max bond amount might be below the pool's minimum.
        // If so, no short can be opened.
        if absolute_max_bond_amount < self.minimum_transaction_amount() {
            return Err(eyre!("No solvent short is possible."));
        }

        // Figure out the base deposit for the absolute max bond amount.
        let absolute_max_base_amount =
            self.calculate_open_short(absolute_max_bond_amount, open_vault_share_price)?;
        if target_base_amount > absolute_max_base_amount {
            return Err(eyre!(
                "Input too large.
                target_base_amount={:#?}
                max_base_amount   ={:#?}",
                target_base_amount,
                absolute_max_base_amount
            ));
        }
        // If the absolute max is within tolerance, return it.
        if absolute_max_base_amount - target_base_amount <= base_tolerance {
            return Ok(absolute_max_bond_amount);
        }

        // Compute a conservative estimate of the bonds shorted & base paid.
        let mut last_good_bond_amount = self.calculate_approximate_short_bonds_given_base_deposit(
            target_base_amount,
            open_vault_share_price,
        )?;

        // Run Newton's Method to adjust the bond amount.
        let mut rng = thread_rng();
        let mut loss = FixedPoint::from(U256::MAX);
        for _ in 0..max_iterations {
            // Calculate the current deposit.
            let current_base_amount =
                self.calculate_open_short(last_good_bond_amount, open_vault_share_price)?;

            // Calculate the current loss.
            loss = if current_base_amount < target_base_amount {
                // It's possible that a nudge from failure cases in the previous
                // iteration put us within the tolerance.
                if (target_base_amount - current_base_amount) <= base_tolerance {
                    return Ok(last_good_bond_amount);
                }
                target_base_amount - current_base_amount
            } else {
                current_base_amount - target_base_amount
            };

            // Calculate the update amount.
            let base_amount_derivative = self.calculate_open_short_derivative(
                last_good_bond_amount,
                open_vault_share_price,
                Some(self.calculate_spot_price_down()?),
            )?;
            let dy = loss.div_up(base_amount_derivative); // div up to discourage dy == 0

            // Calculate the new bond amount.
            // The update rule is: y_1 = y_0 - L(x, x_t) / dL(x, x_t)/dy,
            // where y is the bond amount, x is the base deposit returned by
            // calculate_open_short(y), x_t is the target deposit, L is the loss
            // (x_t - x), and dL(x, x_t)/dy = -base_amount_derivative. We avoid
            // negative numbers using a conditional.
            let new_bond_amount = if current_base_amount < target_base_amount {
                last_good_bond_amount + dy
            } else {
                last_good_bond_amount - dy
            };

            // Check solvency with the latest bond amount.
            match self.calculate_open_short(new_bond_amount, open_vault_share_price) {
                Ok(new_base_amount) => {
                    if new_base_amount <= target_base_amount {
                        last_good_bond_amount = new_bond_amount;
                    }
                    // We overshot the zero crossing & the amount was solvent.
                    // It's possible that we are so close in base amounts that
                    // dy is zero, but we are still overshooting. In this case,
                    // nudge the bond amount down by the error and continue.
                    else {
                        let error = new_base_amount - target_base_amount;
                        last_good_bond_amount = new_bond_amount - error;
                    }
                }
                // New bond amount is not solvent. Start again from slightly
                // below the absolute max.
                Err(_) => {
                    // We know abs max is solvent and we know the target bond
                    // amount is less than the absolute max. So we overshot, but
                    // we can safely overshoot by less.
                    if new_bond_amount >= absolute_max_bond_amount {
                        last_good_bond_amount = absolute_max_bond_amount
                            - base_tolerance * rng.gen_range(fixed!(1e18)..=fixed!(2e18));
                    } else {
                        return Err(eyre!(
                            "current_bond_amount={:#?} is less than the expected absolute_max={:#?}, but still not solvent.",
                            new_bond_amount,
                            absolute_max_bond_amount
                        ));
                    }
                }
            }
        }
        // Did not hit tolerance in max iterations.
        return Err(eyre!(
            "Could not converge to a bond amount given
            Max iterations      = {:#?}
            Target base deposit = {:#?}
            Error               = {:#?}
            Tolerance           = {:#?}",
            max_iterations,
            target_base_amount,
            loss,
            base_tolerance,
        ));
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
    pub fn calculate_max_short<F1: Into<FixedPoint<U256>>, F2: Into<FixedPoint<U256>>>(
        &self,
        budget: F1,
        open_vault_share_price: F2,
        maybe_conservative_price: Option<FixedPoint<U256>>, // TODO: Is there a nice way of abstracting the inner type?
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let budget = budget.into();
        let open_vault_share_price = open_vault_share_price.into();

        // Sanity check that we can open any shorts at all.
        if self
            .solvency_after_short(self.minimum_transaction_amount())
            .is_err()
        {
            return Err(eyre!("No solvent short is possible."));
        }

        // To avoid the case where Newton's method overshoots and stays on
        // the invalid side of the optimization equation (i.e., when deposit >
        // budget), we artificially set the target budget to be less than the
        // actual budget.
        //
        // If the budget is less than the minimum transaction amount, then we
        // return early.
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
        // If the budget is greater than the minimum transaction amount, then we
        // set the target budget.
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
        let spot_price = self.calculate_spot_price_down()?;
        // The initial guess should be guaranteed correct, and we should only
        // get better from there.
        let absolute_max_bond_amount =
            self.calculate_absolute_max_short(None, maybe_max_iterations)?;
        // The max bond amount might be below the pool's minimum. If so, no
        // short can be opened.
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
        let mut best_valid_max_bond_amount = match self.solvency_after_short(max_bond_amount) {
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
        // F(\Delta y) = B - D(\Delta y)
        // ```
        //
        // Since `$B$` is just a constant, `$F'(\Delta y) = -D'(\Delta y)$`.
        // Given the current guess of `$x_n$`, Newton's method gives us an
        // updated guess of `$x_{n+1}$`:
        //
        // ```math
        // \begin{aligned}
        // \Delta y_{n+1} &= x_n - \tfrac{F(\Delta y_n)}{F'(\Delta y_n)} \\
        // &= \Delta y_n + \tfrac{B - D(\Delta y_n)}{D'(\Delta y_n)}
        // \end{aligned}
        // ```
        //
        // The guess that we make is very important in determining how quickly
        // we converge to the solution.
        //
        // TODO: This can get stuck in a loop if the Newton update pushes the
        // bond amount to be too large.
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
            // Given our conservative price `$p_c$`, we can write the short
            // deposit function as:
            //
            // ```math
            // D(\Delta y) = \left( \tfrac{c}{c_0} - $p_c$ \right)
            //        \cdot \Delta y + \phi_{flat} \cdot \Delta y
            //        + \phi_{curve} \cdot (1 - p) \cdot \Delta y
            // ```
            //
            // We then solve for $\Delta y^*$ such that $D(\Delta y^*) = B$,
            // which gives us a guess of:
            //
            // ```math
            // \Delta y^* = \tfrac{B}{\tfrac{c}{c_0} - $p_c$ + \phi_{flat}
            // + \phi_{curve} \cdot (1 - p)}
            // ```
            //
            // If the budget can cover the actual short deposit on
            // `$\Delta y^*$`, we return it as our guess. Otherwise, we revert
            // to the worst case scenario.
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

    /// Calculates the absolute max short that can be opened without violating
    /// the pool's solvency constraints.
    ///
    /// We use Newton's method to iteratively approach a solution. Our objective
    /// function is the difference between the pool's minimum allowable share
    /// reserves and the share reserves after opening a max short, which will
    /// converge to zero from the negative quadrant.
    ///
    /// Given the current guess of `$x_n$`, Newton's method gives us an updated
    /// guess of `$x_{n+1}$`:
    ///
    /// ```math
    /// \begin{aligned}
    /// l(x_n) &= z_t - (z_0 - \Delta z(x_n)) \\
    /// x_{n+1} &= x_n - \tfrac{l(x_n)}{l'(x_n)}
    /// \end{aligned}
    /// ```
    ///
    /// The tolerance parameter is in bonds, and indicates how close to
    /// insolvency the pool should be.
    pub fn calculate_absolute_max_short(
        &self,
        maybe_bond_tolerance: Option<FixedPoint<U256>>,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let bond_tolerance = maybe_bond_tolerance.unwrap_or(fixed!(1e9));
        let max_iterations = maybe_max_iterations.unwrap_or(500);
        // The exposure solvency constraint varies with the short amount.
        // We will ignore it in the loss, but exit early if it is hit during
        // optimization.
        let target_pool_shares = self.minimum_share_reserves().change_type::<I256>()?
            + FixedPoint::from(self.share_adjustment().max(0.into()));
        let mut last_good_bond_amount = self.absolute_max_short_guess()?;
        for _ in 0..max_iterations {
            // Return if we are within tolerance bonds of insolvency.
            if self
                .solvency_after_short(last_good_bond_amount + bond_tolerance)
                .is_err()
            {
                return Ok(last_good_bond_amount);
            }

            // Calculate what the pool shares would be if we opened the short.
            // z_1 = z_0 - ∆z(∆y)
            let current_pool_shares = self.share_reserves().change_type::<I256>()?
                - self.calculate_pool_share_delta_after_open_short(last_good_bond_amount)?;

            // The loss function is z_t - z_1(∆y),
            // and its derivative is -dz_1(∆y)/dy.
            let loss = target_pool_shares - current_pool_shares;
            let loss_derivative =
                -self.calculate_pool_share_delta_after_short_derivative(last_good_bond_amount)?;
            let delta_y = loss.div_up(loss_derivative);

            // Bond amount must increase.
            // Negative dy indicates that the bond amount would go up with a
            // proper step of Newton's Method.
            let mut new_bond_amount = if delta_y < fixed!(0) {
                // x_{n+1} = x_n - l(x_n) / l'(x_n)
                //         = x_n + -l(x_n) / l'(x_n)
                last_good_bond_amount + (-delta_y).change_type::<U256>()?
            }
            // Positive dy indicates that the bond amount would go down with a
            // proper step of Newton's Method. Instead, we will invert the sign
            // of dy to ensure increasing bond amount. This will probably
            // require a binary search in the next step.
            else {
                // x_{n+1} = x_n + l(x_n) / l'(x_n)
                last_good_bond_amount + delta_y.change_type::<U256>()?
            };

            // Refine the new bond amount if necessary by binary search.
            let mut num_tries = 0;
            let mut rng = thread_rng();
            loop {
                match self.solvency_after_short(new_bond_amount) {
                    // New bond amount is good; assign it & get out of the loop.
                    Ok(_) => {
                        last_good_bond_amount = new_bond_amount;
                        break;
                    }
                    // New bond amount is too large.
                    Err(_) => {
                        new_bond_amount -= (new_bond_amount - last_good_bond_amount) / fixed!(2e18);
                    }
                }
                num_tries += 1;
                // We've tried enough; back up to a new random starting point
                // and try again.
                if num_tries >= 1_000 {
                    last_good_bond_amount -= delta_y.abs().change_type::<U256>()?
                        * rng.gen_range(fixed!(1e5)..=fixed!(1e12));
                    break;
                }
            }
        }
        // We did not find a solution within tolerance in the provided number of
        // iterations.
        return Err(eyre!(
            "Could not converge to a bond amount given max iterations = {:#?} and tolerance = {:#?}.",
            max_iterations,
            bond_tolerance
        ));
    }

    /// Calculates an initial guess for the absolute max short that is always
    /// solvent.
    ///
    /// This is a conservative guess that will be less than the true absolute
    /// max short, which is what we need to start Newton's method. To calculate
    /// our guess, we start from the equation for computing the final share
    /// reserves given an open short for some delta bonds:
    ///
    /// ```math
    ///z_1 = \frac{1}{\mu} \cdot \left( \frac{\mu}{c} \cdot \left( k
    /// - (y_0 + \Delta y)^{1 - t_s} \right) \right)^{\frac{1}{1 - t_s}}
    /// + \phi_c \cdot (1 - p) \cdot (1 - \phi_g) \cdot \frac{\Delta y}{c}
    /// ```
    ///
    /// After the open short, this must be greater than or equal to the minimum
    /// share reserves,
    /// `$z_{\text{min}} + \text{max}(\zeta, \frac{e}{c}, 0)$`,
    /// where `$e$` is the pool's current total exposure.
    ///
    /// We can solve for an open short bond amount by using a conservative
    /// linear approximation of the share reserve equation. The Taylor Expansion
    /// provides such an approximation.
    ///
    /// ```math
    /// \Delta y_{\text{max}} \ge \Tilde{\Delta y} = \frac{c \cdot
    /// \left( z_0 - \zeta
    /// - \left( z_{\text{min}} + \text{max}(\zeta, \tfrac{e}{c}, 0 \right)
    /// \right)}{p - \phi_c \cdot (1 - p) \cdot (1 - \phi_g)}
    /// ```
    ///
    /// While this should always provide a conservative estimate, we include
    /// an iterative loop that checks solvency and refines the result as a
    /// precautionary measure.
    pub fn absolute_max_short_guess(&self) -> Result<FixedPoint<U256>> {
        // Check that a short is possible.
        if self
            .solvency_after_short(self.minimum_transaction_amount())
            .is_err()
        {
            return Err(eyre!("No short is possible."));
        }
        // min_share_reseves = z_1 = z_min + max(zeta, e/c, 0)
        let min_share_reserves = self.calculate_min_share_reserves_given_exposure()?;
        // share_adjustment = zeta
        let share_adjustment = FixedPoint::<I256>::try_from(self.share_adjustment())?;
        // effective_share_reserves = z_0 - zeta
        let effective_share_reserves =
            self.share_reserves().change_type::<I256>()? - share_adjustment;
        // min_shares_minus_effective = z_1 - (z_0 - zeta)
        let min_minus_effective_shares = min_share_reserves - effective_share_reserves;
        // fee_component = ø_c * (1 - p) * (1 - ø_g)
        let fee_component = (self
            .curve_fee()
            .mul_up(fixed!(1e18) - self.calculate_spot_price_up()?)
            .mul_up(fixed!(1e18) - self.governance_lp_fee()))
        .change_type::<I256>()?;

        // If zeta >= e/c, then the share adjustment is the limiting factor.
        // ∆y = (c * (z_min + zeta - (z_0 - zeta))) / (ø_c * (1 - p) * (1 - ø_g) - p)
        let delta_zeta_limiting = {
            let divisor =
                fee_component - self.calculate_spot_price_down()?.change_type::<I256>()?;
            self.vault_share_price()
                .change_type::<I256>()?
                .mul_div_down(min_minus_effective_shares, divisor)
        }
        .max(fixed!(0))
        .change_type::<U256>()?;
        // If zeta < e/c, then the exposure is the limiting factor.
        // ∆y = (c * (z_min + e/c - (z_0 - zeta))) / (ø_c * (1 - p) * (1 - ø_g) - p + 1)
        let delta_exposure_limiting = {
            let divisor = fee_component
                - self.calculate_spot_price_down()?.change_type::<I256>()?
                + fixed!(1e18);
            self.vault_share_price()
                .change_type::<I256>()?
                .mul_div_down(min_minus_effective_shares, divisor)
        }
        .max(fixed!(0))
        .change_type::<U256>()?;

        // Due to the nature of the linear estimate, it is possible that the
        // true max short is greater than the minimum transaction amount, but
        // less than delta_max. To account for this, we will bias our initial
        // guess to the largest of the possible solutions, and then use a
        // iterative approach to refine the result.
        let mut conservative_bond_delta = {
            let delta_max = delta_zeta_limiting.max(delta_exposure_limiting);
            let delta_min = delta_zeta_limiting.min(delta_exposure_limiting);
            // If delta_max is solvent, use it.
            if self.solvency_after_short(delta_max).is_ok() {
                delta_max
            }
            // Otherwise, choose between delta_min and a known-bad delta_max.
            else {
                // If delta_min is greater than the minimum, use it.
                if delta_min > self.minimum_transaction_amount() {
                    delta_min
                }
                // Otherwise, we choose a known-bad delta_max and expect the
                // iterative refinement to find a valid solution.
                else {
                    delta_max
                }
            }
        };

        // Iteratively adjust to ensure solvency.
        // In fuzz testing this happens about 15% of the time.
        loop {
            match self.solvency_after_short(conservative_bond_delta) {
                Ok(_) => break,
                Err(_) => {
                    conservative_bond_delta *= fixed!(0.9e18); // 10% drop
                    if conservative_bond_delta < self.minimum_transaction_amount() {
                        return Ok(self.minimum_transaction_amount());
                    }
                }
            }
        }
        Ok(conservative_bond_delta)
    }

    /// Calculates the pool's solvency after opening a short.
    ///
    /// We can express the pool's solvency after opening a short of `$\Delta y$`
    /// bonds as:
    ///
    /// ```math
    /// s(\Delta y) = z_1 - \tfrac{e(\Delta y)}{c} - z_{min}
    /// ```
    ///
    /// where `$z_1$` represents the pool's share reserves after opening
    /// the short:
    ///
    /// ```math
    /// z_1 = z_0 - \left(
    ///     P_{\text{lp}}(\Delta y) - \left( \tfrac{\Phi_{c,os}(\Delta y)}{c}
    ///     - \tfrac{\Phi_{g,os}(\Delta y)}{c} \right)
    /// \right)
    /// ```
    ///
    /// and `$e(\Delta y)$` represents the pool's exposure after opening the
    /// short:
    ///
    /// ```math
    /// e(\Delta y) = max(e_0 - \Delta y, 0)
    /// ```
    pub fn solvency_after_short(&self, bond_amount: FixedPoint<U256>) -> Result<FixedPoint<U256>> {
        // LP portion must be greater than the curve fee amount.
        let share_reserves_delta = self.calculate_short_principal(bond_amount)?;
        let curve_fee_shares = self
            .open_short_curve_fee(bond_amount)?
            .div_up(self.vault_share_price());
        if share_reserves_delta < curve_fee_shares {
            return Err(eyre!(
                "bond_amount={:#?} results in share_reserves_delta={:#?} < curve_fee_shares={:#?}.",
                bond_amount,
                share_reserves_delta,
                curve_fee_shares,
            ));
        }
        // If the share reserves would underflow when the short is opened,
        // then we revert with an insufficient liquidity error.
        let pool_share_delta = self
            .calculate_pool_share_delta_after_open_short(bond_amount)?
            .change_type::<U256>()?;
        if self.share_reserves() < pool_share_delta {
            return Err(eyre!(
                "Insufficient liquidity. Expected share_reserves={:#?} >= pool_share_delta={:#?}",
                self.share_reserves(),
                pool_share_delta
            ));
        }
        // Need to check that z - zeta >= z_min.
        let new_share_reserves = self.share_reserves() - pool_share_delta;
        let new_effective_share_reserves =
            calculate_effective_share_reserves(new_share_reserves, self.share_adjustment())?;
        if new_effective_share_reserves < self.minimum_share_reserves() {
            return Err(eyre!("Insufficient liquidity. Expected effective_share_reserves={:#?} >= min_share_reserves={:#?}",
            new_effective_share_reserves, self.minimum_share_reserves()));
        }
        // Check global exposure. This also ensures z >= z_min.
        let exposure_shares = self.exposure_after_short_shares(bond_amount)?;
        if new_share_reserves >= exposure_shares + self.minimum_share_reserves() {
            Ok((new_share_reserves - exposure_shares) - self.minimum_share_reserves())
        } else {
            Err(eyre!("Insiffucient liquidity. Expected share_reserves={:#?} >= exposure_shares={:#?} + min_share_reserves={:#?} = {:#?}",
            new_share_reserves, exposure_shares, self.minimum_share_reserves(), exposure_shares + self.minimum_share_reserves()))
        }
    }

    /// Calculates the derivative of the pool's share delta w.r.t. the short
    /// amount.
    ///
    /// The pool share delta after opening a short is
    ///
    /// ```math
    /// \Delta z = z_0
    ///     - \frac{1}{\mu} \cdot \left(
    ///     \frac{\mu}{c} \cdot (k - (y + \Delta y)^{1 - t_s})
    ///     \right)^{\frac{1}{1 - t_s}}
    ///     - \frac{\phi_c \cdot (1 - p) \cdot (1 - \phi_g) \cdot \Delta y}{c}
    /// ```
    /// where the second term is `$P_{\text{lp}}(\Delta y)$` and the remaining
    /// terms are from fees. Therefore, the derivative is
    ///
    /// ```math
    /// \frac{\partial \Delta z}{\partial \Delta y} = - P_{\text{lp}}'(\Delta y)
    ///         - \left(\phi_c \cdot (1 - p) \cdot (1 - \phi_g) \right)
    /// ```
    fn calculate_pool_share_delta_after_short_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<I256>> {
        let lhs = self.calculate_short_principal_derivative(bond_amount)?;
        let rhs = (self.curve_fee()
            * (fixed!(1e18) - self.calculate_spot_price_down()?)
            * (fixed!(1e18) - self.governance_lp_fee())
            / self.vault_share_price())
        .change_type::<I256>()?;
        Ok(lhs - rhs)
    }

    /// Calculates the derivative of the pool's solvency w.r.t. the short
    /// amount.
    ///
    /// ```math
    /// s'(\Delta y) = 0 - \left( P'(\Delta y) - \frac{(c'(\Delta y)
    ///          - g'(\Delta y))}{c} \right) - \frac{e'(\Delta y)}{c}
    /// ```
    ///
    /// where `$e'(\Delta y)$` is piecewise linear:
    ///
    /// ```math
    /// e'(\Delta y) =
    /// \begin{cases}
    ///     -\frac{1}{c} & \text{if} e_0 <\Delta y \\
    ///     0 & \text{otherwise}
    /// \end{cases}
    /// ```
    fn solvency_after_short_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<I256>> {
        let lhs = self.calculate_short_principal_derivative(bond_amount)?;
        let rhs = (self.curve_fee()
            * (fixed!(1e18) - self.calculate_spot_price_down()?)
            * (fixed!(1e18) - self.governance_lp_fee())
            / self.vault_share_price())
        .change_type::<I256>()?;
        let exposure_prime = if self.long_exposure() < bond_amount {
            fixed_i256!(0)
        } else {
            -(fixed!(1e18).div_up(self.vault_share_price())).change_type::<I256>()?
        };
        Ok(-(lhs - rhs) - exposure_prime)
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use ethers::types::U256;
    use fixedpointmath::{fixed, fixed_u256, uint256, FixedPoint};
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS, SLOW_FUZZ_RUNS},
    };
    use hyperdrive_wrappers::wrappers::ihyperdrive::Checkpoint;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::test_utils::{
        agent::HyperdriveMathAgent, preamble::initialize_pool_with_random_state,
    };

    #[tokio::test]
    async fn fuzz_calculate_pool_share_delta_after_short_derivative() -> Result<()> {
        let empirical_derivative_epsilon = fixed!(1e14);
        let test_tolerance = fixed!(1e14);
        let mut rng = thread_rng();

        // Run the fuzz tests.
        let mut count = 0;
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Min trade amount should be > 1,000x the derivative epsilon.
            // Max trade amount is bounded by the pool, but we will pick a big
            // number to speed up this test.
            let bond_amount =
                rng.gen_range(empirical_derivative_epsilon * fixed!(1_000)..=fixed!(10_000_000e18));
            // Ensure a short is possible.
            if state
                .solvency_after_short(bond_amount + empirical_derivative_epsilon)
                .is_err()
            {
                continue;
            }
            // Compute function at two close points.
            let f_x = state.calculate_pool_share_delta_after_open_short(bond_amount)?;
            let f_x_plus_delta = state.calculate_pool_share_delta_after_open_short(
                bond_amount + empirical_derivative_epsilon,
            )?;
            // Compute the empirical and analytical derivatives.
            let pool_share_delta_derivative =
                state.calculate_pool_share_delta_after_short_derivative(bond_amount)?;
            let empirical_derivative =
                (f_x_plus_delta - f_x) / empirical_derivative_epsilon.change_type::<I256>()?;
            // Ensure that the empirical and analytical derivatives match.
            let derivative_diff = (empirical_derivative - pool_share_delta_derivative)
                .abs()
                .change_type::<U256>()?;
            assert!(
                derivative_diff < test_tolerance,
                "expected abs(derivative_diff)={:#?} < test_tolerance={:#?};
                calculated_derivative={:#?}, empirical_derivative={:#?}",
                derivative_diff,
                test_tolerance,
                pool_share_delta_derivative,
                empirical_derivative
            );
            count += 1;
        }
        // Assert that at least 50% of runs passed.
        assert!(count >= 5_000, "Not enough runs passed.");
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_solvency_after_short_derivative() -> Result<()> {
        let empirical_derivative_epsilon = fixed!(1e14);
        let test_tolerance = fixed!(1e14);
        let mut rng = thread_rng();

        // Run the fuzz tests.
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Min trade amount should be at least 1,000x the derivative epsilon
            let bond_amount = rng.gen_range(fixed!(1e18)..=fixed!(10_000_000e18));

            let f_x = match panic::catch_unwind(|| state.solvency_after_short(bond_amount)) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };
            let f_x_plus_delta = match panic::catch_unwind(|| {
                state.solvency_after_short(bond_amount + empirical_derivative_epsilon)
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };

            // Compute the absolute value of the empirical and analytical
            // derivatives.
            let solvency_after_short_derivative =
                state.solvency_after_short_derivative(bond_amount)?;
            let empirical_derivative = if f_x < f_x_plus_delta {
                // Adding delta bonds results in lower solvency.
                // This implies a positive derivative.
                assert!(solvency_after_short_derivative >= fixed!(0));
                (f_x_plus_delta - f_x) / empirical_derivative_epsilon
            } else {
                // Adding delta bonds results in higher solvency.
                // This implies a negative derivative.
                assert!(solvency_after_short_derivative <= fixed!(0));
                (f_x - f_x_plus_delta) / empirical_derivative_epsilon
            };
            let solvency_after_short_derivative = solvency_after_short_derivative
                .abs()
                .change_type::<U256>()?;

            // Ensure that the empirical and analytical derivatives match.
            let derivative_diff = if empirical_derivative > solvency_after_short_derivative {
                empirical_derivative - solvency_after_short_derivative
            } else {
                solvency_after_short_derivative - empirical_derivative
            };
            assert!(
                derivative_diff < test_tolerance,
                "expected abs(derivative_diff)={:#?} < test_tolerance={:#?};
                calculated_derivative={:#?}, empirical_derivative={:#?}",
                derivative_diff,
                test_tolerance,
                solvency_after_short_derivative,
                empirical_derivative
            );
        }
        Ok(())
    }

    /// Test to ensure that the absolute max short guess is always solvent.
    #[tokio::test]
    async fn fuzz_calculate_absolute_max_short_guess() -> Result<()> {
        let solvency_tolerance = fixed!(100_000_000e18);
        let mut rng = thread_rng();

        // Run the fuzz tests.
        for _ in 0..*FAST_FUZZ_RUNS {
            // Compute a random state and checkpoint exposure.
            let state = rng.gen::<State>();
            // Check that a short is possible.
            match state.solvency_after_short(state.minimum_transaction_amount()) {
                Ok(_) => (),
                Err(_) => continue,
            }

            // Compute the guess, check that it is solvent.
            let max_short_guess = state.absolute_max_short_guess()?;
            assert!(
                state.solvency_after_short(max_short_guess).is_ok(),
                "max_short_guess={:#?} is not solvent",
                max_short_guess
            );
            assert!(
                state
                    .calculate_open_short(max_short_guess, state.vault_share_price())
                    .is_ok(),
                "cannot open short with max_short_guess={:#?}",
                max_short_guess
            );
            let solvency = state.solvency_after_short(max_short_guess)?;

            // Check that the remaining available shares in the pool are below a
            // solvency tolerance.
            assert!(
                solvency <= solvency_tolerance,
                "solvency={:#?} > solvency_tolerance={:#?}",
                solvency,
                solvency_tolerance
            );
        }
        Ok(())
    }

    /// This test ensures that a pool is fully drained after opening a short for
    /// the absolute maximum amount. It also verifies that the absolute maximum
    /// trade returned is always valid.
    #[tokio::test]
    async fn fuzz_calculate_absolute_max_short() -> Result<()> {
        let bonds_tolerance = fixed_u256!(1e9);
        let max_iterations = 500;

        // Run the fuzz tests.
        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Make sure a short is possible.
            match state.solvency_after_short(state.minimum_transaction_amount()) {
                Ok(_) => (),
                Err(_) => continue,
            }

            // Get the max short.
            let absolute_max_short =
                state.calculate_absolute_max_short(Some(bonds_tolerance), Some(max_iterations))?;

            // The short should be valid.
            assert!(absolute_max_short >= state.minimum_transaction_amount());
            assert!(state.solvency_after_short(absolute_max_short).is_ok());
            assert!(state
                .calculate_open_short(absolute_max_short, state.vault_share_price())
                .is_ok());
            // Adding tolerance more bonds should be insolvent.
            assert!(state
                .solvency_after_short(absolute_max_short + bonds_tolerance)
                .is_err());
            assert!(state
                .calculate_open_short(
                    absolute_max_short + bonds_tolerance,
                    state.vault_share_price()
                )
                .is_err());
        }
        Ok(())
    }

    /// This test ensures that the short functions for converting between a
    /// deposit in shares and bonds to be shorted are invertible.
    #[tokio::test]
    async fn fuzz_open_short_inversion() -> Result<()> {
        let abs_max_bonds_tolerance = fixed_u256!(1e9);
        let budget_base_tolerance = fixed_u256!(1e10);
        let max_iterations = 1_000;
        let mut rng = thread_rng();

        // Run the fuzz tests.
        for _ in 0..*SLOW_FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Make sure a short is possible.
            match state.solvency_after_short(state.minimum_transaction_amount()) {
                Ok(_) => (),
                Err(_) => continue,
            }

            // Check invertibility near the max.
            // Get the max short.
            let absolute_max_short_bonds = state.calculate_absolute_max_short(
                Some(abs_max_bonds_tolerance),
                Some(max_iterations),
            )?;
            assert!(state.solvency_after_short(absolute_max_short_bonds).is_ok());
            // Get the deposit for this short.
            let max_short_deposit_base =
                state.calculate_open_short(absolute_max_short_bonds, state.vault_share_price())?;
            // Verify the bonds shorted when the deposit is a constraint.
            let user_max_short_bonds = state.calculate_short_bonds_given_deposit(
                max_short_deposit_base,
                state.vault_share_price(),
                absolute_max_short_bonds,
                Some(budget_base_tolerance),
                Some(max_iterations),
            )?;
            // Check that the outputs match up.
            assert!(state.solvency_after_short(user_max_short_bonds).is_ok());
            if user_max_short_bonds > absolute_max_short_bonds {
                assert!(user_max_short_bonds - absolute_max_short_bonds <= abs_max_bonds_tolerance);
            } else {
                assert!(absolute_max_short_bonds - user_max_short_bonds <= abs_max_bonds_tolerance);
            }

            // Check invertibility near the min.
            // Get the min short.
            let absolute_min_short_bonds = state.minimum_transaction_amount();
            assert!(state.solvency_after_short(absolute_min_short_bonds).is_ok());
            // Get the deposit for this short.
            let min_short_deposit_base =
                state.calculate_open_short(absolute_min_short_bonds, state.vault_share_price())?;
            // Verify the bonds shorted when the deposit is a constraint.
            let user_min_short_bonds = state.calculate_short_bonds_given_deposit(
                min_short_deposit_base,
                state.vault_share_price(),
                absolute_min_short_bonds,
                Some(budget_base_tolerance),
                Some(max_iterations),
            )?;
            // Check that the outputs match up.
            assert!(state.solvency_after_short(user_min_short_bonds).is_ok());
            if user_min_short_bonds > absolute_min_short_bonds {
                assert!(user_min_short_bonds - absolute_min_short_bonds <= abs_max_bonds_tolerance);
            } else {
                assert!(absolute_min_short_bonds - user_min_short_bonds <= abs_max_bonds_tolerance);
            }

            // Check invertibility at a random point.
            let random_short_bonds =
                rng.gen_range(absolute_min_short_bonds..=absolute_max_short_bonds);
            // Get the deposit for this short.
            let random_short_deposit_base =
                state.calculate_open_short(random_short_bonds, state.vault_share_price())?;
            // Verify the bonds shorted when the deposit is a constraint.
            let user_random_short_bonds = state.calculate_short_bonds_given_deposit(
                random_short_deposit_base,
                state.vault_share_price(),
                random_short_bonds,
                Some(budget_base_tolerance),
                Some(max_iterations),
            )?;
            // Check that the outputs match up.
            assert!(state.solvency_after_short(user_random_short_bonds).is_ok());
            if user_random_short_bonds > random_short_bonds {
                assert!(user_random_short_bonds - random_short_bonds <= abs_max_bonds_tolerance);
            } else {
                assert!(random_short_bonds - user_random_short_bonds <= abs_max_bonds_tolerance);
            }
        }
        Ok(())
    }

    /// This test ensures that `calculate_short_bonds_given_deposit` returns a
    /// short bond amount that results consuming the agent's budget, ignoring
    /// the slippage guard.
    /// TODO: see Issue #136 for whiy this is failing.
    // #[ignore]
    #[tokio::test]
    async fn fuzz_calculate_max_short_budget_consumed() -> Result<()> {
        let abs_max_bond_tolerance = fixed!(1e9);
        let remaining_balance_tolerance = fixed!(1e10);
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

        // Run the fuzz tests.
        let mut num_tests = 0;
        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;
            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;
            // Get the current state of the pool.
            let state = alice.get_state().await?;
            // Check that a short is possible.
            if state
                .solvency_after_short(state.minimum_transaction_amount())
                .is_err()
            {
                chain.revert(id).await?;
                alice.reset(Default::default()).await?;
                bob.reset(Default::default()).await?;
                celine.reset(Default::default()).await?;
                continue;
            }
            let Checkpoint {
                vault_share_price: open_vault_share_price,
                weighted_spot_price: _,
                last_weighted_spot_price_update_time: _,
            } = alice
                .get_checkpoint(state.to_checkpoint(alice.now().await?))
                .await?;

            // Celine should always be budget constrained when trying to open the short.
            let global_max_short_bonds =
                state.calculate_absolute_max_short(Some(abs_max_bond_tolerance), None)?;
            let global_max_base_required = state
                .calculate_open_short(global_max_short_bonds, open_vault_share_price.into())?;
            if global_max_base_required - fixed!(1e18) <= state.minimum_transaction_amount() {
                chain.revert(id).await?;
                alice.reset(Default::default()).await?;
                bob.reset(Default::default()).await?;
                celine.reset(Default::default()).await?;
                continue; // Avoid case where max is within 1e18 of min.
            }
            let budget = rng.gen_range(
                state.minimum_transaction_amount()..=global_max_base_required - fixed!(1e18),
            );
            println!("celine wallet base before funding: {}", celine.base());
            celine.fund(budget - celine.base()).await?;
            println!("celine wallet base after funding:  {}", celine.base());

            // Celine opens a max short position.
            let slippage_tolerance = fixed!(0.01e18);
            let max_short_bonds = celine.calculate_max_short(Some(slippage_tolerance)).await?;
            
            println!("run #{}", num_tests);
            println!("opening short with budget: {}", budget);
            println!("global_max_short_bonds:    {}", global_max_short_bonds);
            println!("global_max_base_required:  {}", global_max_base_required);

            let (maturity_time, base_paid) = celine
                .open_short(max_short_bonds, Some(slippage_tolerance), None)
                .await?;

            println!("full_budget:     {}", budget);
            println!("base_paid:       {}", base_paid);
            println!("intended_budget: {}", budget * (fixed!(1e18) - slippage_tolerance));
            println!("short_bonds:     {}", max_short_bonds);
            println!("maturity_time:   {}", maturity_time);

            // celine
            //     .open_short(max_short_bonds, Some(slippage_tolerance), None)
            //     .await?;

            // The max short should consume the budget up to the slippage
            // tolerance.
            let remaining_balance = celine.base();
            assert!(
                remaining_balance <= remaining_balance_tolerance + budget * slippage_tolerance,
                "expected remaining_balance={:#?} <= remaining_balance_tolerance={:#?}
                global_max_short_bonds={:#?}
                global_max_base_required={:#?}
                budget={:#?}
                max_short_bonds={:#?}",
                remaining_balance,
                format!(
                    "{}",
                    remaining_balance_tolerance + budget * slippage_tolerance
                ),
                global_max_short_bonds,
                global_max_base_required,
                budget,
                max_short_bonds,
            );

            // Revert to the snapshot and reset the agents' wallets.
            num_tests += 1;
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }
        // Assert that we've run at least 50% of the tests.
        assert!(num_tests > 50);
        Ok(())
    }

    /// Test to ensure that the rust computed absolute max short can always be
    /// opened in the smart contracts.
    #[tokio::test]
    async fn fuzz_absolute_max_short_valid() -> Result<()> {
        let bond_tolerance = fixed!(1e9);
        let max_iterations = 500;
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

        // Run the fuzz tests.
        let mut num_tests = 0;
        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;
            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;
            // Get the current state of the pool.
            let state = alice.get_state().await?;
            // Check that a short is possible.
            if state
                .solvency_after_short(state.minimum_transaction_amount())
                .is_err()
            {
                chain.revert(id).await?;
                alice.reset(Default::default()).await?;
                bob.reset(Default::default()).await?;
                celine.reset(Default::default()).await?;
                continue;
            }

            // Get the max short.
            let global_max_short_bonds =
                state.calculate_absolute_max_short(Some(bond_tolerance), Some(max_iterations))?;
            // Celine opens a max short position.
            // Test passes if this does not revert or throw any errors.
            let slippage_tolerance = fixed!(0.01e18);
            celine.fund(global_max_short_bonds * fixed!(100e18)).await?; // plenty of money
            celine
                .open_short(global_max_short_bonds, Some(slippage_tolerance), None)
                .await?;

            // Revert to the snapshot and reset the agents' wallets.
            num_tests += 1;
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }
        // TODO: increase this back to 50 when Issue #136 is resolved
        // Assert that we've run at least 10% of the tests.
        assert!(num_tests > 10);
        Ok(())
    }

    /// Estimate solvency after short, open the short, verify solvency.
    #[ignore]
    #[tokio::test]
    async fn fuzz_sol_solvency_after_short() -> Result<()> {
        let solvency_tolerance = fixed!(1e18);
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

        // Run the fuzz tests.
        for iter in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;
            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;
            // Get the current state from solidity.
            let mut state = alice.get_state().await?;
            // Check that a short is possible.
            if state
                .solvency_after_short(state.minimum_transaction_amount())
                .is_err()
            {
                chain.revert(id).await?;
                alice.reset(Default::default()).await?;
                bob.reset(Default::default()).await?;
                celine.reset(Default::default()).await?;
                continue;
            }

            // Open the max short.
            let max_short = state.calculate_absolute_max_short(None, None)?;
            let max_short_deposit_shares =
                state.calculate_open_short(max_short, state.vault_share_price())?;
            celine
                .fund((max_short_deposit_shares + fixed!(100e18)).into())
                .await?;
            let solvency_after_short_guess = state.solvency_after_short(max_short)?;
            celine.open_short(max_short, None, None).await?;
            alice
                .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
                .await?;

            // Check solvency values.
            state = alice.get_state().await?;
            let solvency_after_short_rs = state.calculate_solvency()?;
            // TODO: add solvency function to mock hyperdrive so we can call it.
            let solvency_after_short_sol = (state.share_reserves()
                - state.long_exposure().div_up(state.vault_share_price()))
                - state.minimum_share_reserves();

            // Ensure solidity & rust solvency values are within tolerance.
            let error = if solvency_after_short_guess > solvency_after_short_rs {
                solvency_after_short_guess - solvency_after_short_rs
            } else {
                solvency_after_short_rs - solvency_after_short_guess
            };
            assert!(
                error <= solvency_tolerance,
                "rust error={:#?} > tolerance={:#?}",
                error,
                solvency_tolerance
            );
            let error = if solvency_after_short_guess > solvency_after_short_sol {
                solvency_after_short_guess - solvency_after_short_sol
            } else {
                solvency_after_short_sol - solvency_after_short_guess
            };
            assert!(
                error <= solvency_tolerance,
                "solidity error={:#?} > tolerance={:#?}",
                error,
                solvency_tolerance
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_solvency_monotonicity() -> Result<()> {
        // Define a range of increments to test.
        let mut rng = thread_rng();
        let mut increments = Vec::new();
        // 500 points between 1 and 1B.
        for i in (1..=1_000_000_000).step_by(5_000_000) {
            let i_val = FixedPoint::<U256>::from(i);
            let inc = i_val * fixed!(1e9); // range is 1e9->1e19
            increments.push(inc);
        }

        // Run the fuzz tests.
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Ensure a short is possible.
            if state
                .solvency_after_short(state.minimum_transaction_amount())
                .is_err()
            {
                continue;
            }

            // Vary the baseline scale factor by adjusting the initial bond amount.
            let bond_amount = rng.gen_range(fixed!(1e10)..=fixed!(100_000_000e18));
            // Compute f_x at the baseline bond_amount.
            let f_x = match panic::catch_unwind(|| state.solvency_after_short(bond_amount)) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // Start value is not solvent; skip the test.
                },
                Err(_) => continue, // Start value caused panic; skip the test.
            };
            // Compute f_x for each increment.
            let mut f_x_values = Vec::new();
            for &inc in &increments {
                match panic::catch_unwind(|| state.solvency_after_short(bond_amount + inc)) {
                    Ok(result) => match result {
                        Ok(val) => f_x_values.push((inc, val)),
                        Err(_) => {
                            continue; // Increment makes the pool insolvent, skip.
                        }
                    },
                    Err(_) => continue, // Overflow or underflow
                };
            }
            // If we didn't get enough data points, skip this iteration.
            if f_x_values.len() < 2 {
                continue;
            }
            // Check monotonicity: determine if all differences f_x - f_x_values[i] have the same sign.
            let diffs: Vec<bool> = f_x_values
                .iter()
                .map(|(_inc, val)| *val < f_x) // Compare each f_x_plus_delta to f_x
                .collect();
            // We want either all "true" or all "false" in diffs for strict monotonicity.
            let all_decreasing = diffs.iter().all(|&x| x == true);
            let all_increasing = diffs.iter().all(|&x| x == false);
            // If not strictly monotonic, assert fails.
            assert!(
                all_decreasing || all_increasing,
                "Non-monotonic behavior detected. diffs={:?}",
                diffs
            );
        }
        Ok(())
    }
}
