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

    /// Calculate the minimum share reserves allowed by the pool given the
    /// current exposure and share adjustment.
    pub fn calculate_min_share_reserves(
        &self,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        // We have the twin constraints that `$z \geq z_{min} + exposure$` and
        // `$z - \zeta \geq z_{min}$`. Combining these together, we calculate
        // the share reserves after a max short as
        // `$z_{optimal} = z_{min} + max(0, \zeta) + exposure$`.
        let exposure_shares = self.long_minus_checkpoint_exposure_shares(checkpoint_exposure)?;
        let min_share_reserves = self.minimum_share_reserves()
            + FixedPoint::try_from(self.share_adjustment().max(I256::zero()))?
            + exposure_shares;
        Ok(min_share_reserves)
    }

    /// Calculate the long exposure minus the checkpoint exposure, in shares.
    ///
    /// The long exposure will account for any executed trades. Any new short
    /// net previous longs by subtracting from the long exposure. This increases
    /// solvency until the checkpoint exposure goes negative. Past that point,
    /// shorts will impact solvency by decreasing share reserves.
    ///
    /// This function is useful when working with shorts because it converts
    /// a piece-wise linear calculation into a linear one by computing the net
    /// exposure.
    fn long_minus_checkpoint_exposure_shares(
        &self,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        let exposure_shares = {
            let checkpoint_exposure = FixedPoint::try_from(checkpoint_exposure.max(I256::zero()))?;
            if self.long_exposure() < checkpoint_exposure {
                return Err(eyre!(
                    "expected long_exposure={:#?} >= checkpoint_exposure={:#?}.",
                    self.long_exposure(),
                    checkpoint_exposure
                ));
            } else {
                (self.long_exposure() - checkpoint_exposure).div_up(self.vault_share_price())
            }
        };
        Ok(exposure_shares)
    }

    /// Use Newton's method to find the amount of bonds shorted for a given base
    /// deposit amount.
    pub fn calculate_short_bonds_given_deposit(
        &self,
        target_base_amount: FixedPoint<U256>,
        open_vault_share_price: FixedPoint<U256>,
        absolute_max_bond_amount: FixedPoint<U256>,
        maybe_base_tolerance: Option<FixedPoint<U256>>,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let base_tolerance = maybe_base_tolerance.unwrap_or(fixed!(1e9));
        let max_iterations = maybe_max_iterations.unwrap_or(500);

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
                        last_good_bond_amount = absolute_max_bond_amount - base_tolerance;
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
            "Could not converge to a bond amount given max iterations = {:#?}.
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
            self.calculate_absolute_max_short(checkpoint_exposure, None, maybe_max_iterations)?;
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
        checkpoint_exposure: I256,
        maybe_bond_tolerance: Option<FixedPoint<U256>>,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let bond_tolerance = maybe_bond_tolerance.unwrap_or(fixed!(1e9));
        let max_iterations = maybe_max_iterations.unwrap_or(500);
        // Use a conservative guess to get us close and speed up the process.
        let target_pool_shares = calculate_effective_share_reserves(
            self.calculate_min_share_reserves(checkpoint_exposure)?,
            self.share_adjustment(),
        )?
        .change_type::<I256>()?;
        let mut last_good_bond_amount = self.absolute_max_short_guess(checkpoint_exposure)?;
        for _ in 0..max_iterations {
            // Return if we are within tolerance bonds of insolvency.
            let solvency_within_tolerance = self
                .solvency_after_short(last_good_bond_amount + bond_tolerance, checkpoint_exposure);
            if solvency_within_tolerance.is_err() {
                return Ok(last_good_bond_amount);
            }

            // Calculate what the pool shares would be if we opened the short.
            let current_pool_shares = self.effective_share_reserves()?.change_type::<I256>()?
                - self
                    .calculate_pool_share_delta_after_open_short(last_good_bond_amount)?
                    .change_type::<I256>()?;

            // The loss function is z_t - z, and its derivative is -dz/dy.
            let loss = target_pool_shares - current_pool_shares;
            let loss_derivative =
                -self.calculate_pool_share_delta_after_short_derivative(last_good_bond_amount)?;
            let dy = loss.div_up(loss_derivative);
            // Bond amount must increase.
            // Negative dy indicates that the bond amount would go up with a
            // proper step of Newton's Method.
            let new_bond_amount = if dy < fixed!(0) {
                // x_{n+1} = x_n - l(x_n) / l'(x_n)
                //         = x_n + -l(x_n) / l'(x_n)
                last_good_bond_amount + (-dy).change_type::<U256>()?
            }
            // Positive dy indicates that the bond amount would go down with a
            // proper step of Newton's Method. Instead, we will invert the sign
            // of dy to ensure increasing bond amount. This will probably
            // require a binary search in the next step.
            else {
                // x_{n+1} = x_n + l(x_n) / l'(x_n)
                last_good_bond_amount + dy.change_type::<U256>()?
            };

            // Calculate the current solvency & iterate w/ a good bond amount.
            last_good_bond_amount =
                match self.solvency_after_short(new_bond_amount, checkpoint_exposure) {
                    Ok(solvency) => new_bond_amount,
                    // New bond amount is too large. Use a binary search to find a
                    // new starting point.
                    Err(_) => {
                        // Start from halfway between the new bond amount and the
                        // last good bond amount.
                        let mut return_amount = new_bond_amount
                            - ((new_bond_amount - last_good_bond_amount) / fixed!(2e18));
                        let mut num_tries = 0;
                        loop {
                            if self
                                .solvency_after_short(return_amount, checkpoint_exposure)
                                .is_ok()
                            {
                                break return_amount;
                            }
                            // Return amount is still too large, try again.
                            // U256 ensures return_amount > last_good_bond_amount
                            return_amount -= (return_amount - last_good_bond_amount) / fixed!(2e18);
                            num_tries += 1;
                            // Avoid running forever by returning a guaranteed good
                            // bond amount.
                            if num_tries >= 100_000 {
                                break last_good_bond_amount * fixed!(0.99e18);
                            }
                        }
                    }
                };
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
    /// share reserves, `$z_{\text{min}} + \text{max}_(\zeta, 0) + e$`, where
    /// `$e$` is the pool's current total exposure.
    ///
    /// We can solve for a conservative open short bond amount by using a
    /// conservative linear approximation of the nonlinear YieldSpace term. The
    /// Taylor Expansion provides such an approximation:
    ///
    /// ```math
    /// z_{1,ys} \ge z_0 - \zeta - \frac{p}{c} \cdot \Delta y_{\text{max}}
    /// ```
    ///
    /// Using this, we can produce a conservative delta bond estimate:
    ///
    /// ```math
    /// \Delta y_{\text{max}} \ge \Tilde{\Delta y} = \frac{c \cdot \left( z_0
    ///  \zeta - \left( z_{\text{min}} + \text{max}(\zeta, 0)
    /// + e \right) \right)}{p + \phi_c \cdot (1 - p) \cdot (1 - \phi_g)}
    /// ```
    ///
    /// While this should always provide a conservative estimate, we include
    /// an iterative loop that checks solvency and refines the result as a
    /// precautionary measure.
    fn absolute_max_short_guess(&self, checkpoint_exposure: I256) -> Result<FixedPoint<U256>> {
        // We cannot directly solve for a valid delta y that produces the
        // minimum effective share reserves, so instead we use a linear
        // approximation of the YieldSpace component.
        let min_share_reserves = self.calculate_min_share_reserves(checkpoint_exposure)?;
        if self.effective_share_reserves()? < min_share_reserves {
            return Err(eyre!(
                "Current effective pool share reserves = {:#?} are below the minimum = {:#?}.",
                self.effective_share_reserves()?,
                min_share_reserves
            ));
        }
        // Use a linear estimate that lies below the YieldSpace curve.
        // z0 - zeta - z1
        let effective_shares_minus_min_shares =
            self.effective_share_reserves()? - min_share_reserves;
        // ø_c * (1 - ø_g) * (1 - p)
        let fee_component = self
            .curve_fee()
            .mul_up(fixed!(1e18) - self.governance_lp_fee())
            .mul_up(fixed!(1e18) - self.calculate_spot_price_down()?);
        // (c * (z0 - zeta - z1)) / (p + ø_c * (1 - ø_g) * (1 - p))
        let mut conservative_bond_delta = self.vault_share_price().mul_div_up(
            effective_shares_minus_min_shares,
            self.calculate_spot_price_up()? + fee_component,
        );
        // Iteratively adjust to ensure solvency.
        loop {
            match self.solvency_after_short(conservative_bond_delta, checkpoint_exposure) {
                Ok(_) => break,
                Err(_) => {
                    conservative_bond_delta /= fixed!(2e18);
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
    /// e(\Delta y) = e_0 - min(\Delta y + D(\Delta y), max(e_{c}, 0))
    /// ```
    ///
    /// We simplify our `$e(\Delta y)$` formula by noting that the max short is
    /// only constrained by solvency when
    /// `$\Delta y + D(\Delta y) > max(e_{c}, 0)$` since
    /// `$\Delta y + D(\Delta y)$` grows faster than
    /// `$P(\Delta y) - \tfrac{\phi_{c}}{c} \cdot \left( 1 - p \right) \cdot \Delta y$`.
    /// With this in mind,
    /// `$min(\Delta y + D(\Delta y), max(e_{c}, 0)) = max(e_{c}, 0)$` whenever
    /// solvency is actually a constraint, so we can write:
    ///
    /// ```math
    /// e(\Delta y) = e_0 - max(e_{c}, 0)
    /// ```
    fn solvency_after_short(
        &self,
        bond_amount: FixedPoint<U256>,
        checkpoint_exposure: I256,
    ) -> Result<FixedPoint<U256>> {
        let pool_share_delta = self.calculate_pool_share_delta_after_open_short(bond_amount)?;
        // If the share reserves would underflow when the short is opened,
        // then we revert with an insufficient liquidity error.
        if self.share_reserves() < pool_share_delta {
            return Err(eyre!(
                "Insufficient liquidity. Expected share_reserves={:#?} >= pool_share_delta={:#?}",
                self.share_reserves(),
                pool_share_delta
            ));
        }
        // Need to check that z - zeta >= z_min
        let new_share_reserves = self.share_reserves() - pool_share_delta;
        let new_effective_share_reserves =
            calculate_effective_share_reserves(new_share_reserves, self.share_adjustment())?;
        if new_effective_share_reserves < self.minimum_share_reserves() {
            return Err(eyre!("Insufficient liquidity. Expected effective_share_reserves={:#?} >= min_share_reserves={:#?}",
            new_effective_share_reserves, self.minimum_share_reserves()));
        }
        // Check global exposure. This also ensures z >= z_min.
        let exposure_shares = self.long_minus_checkpoint_exposure_shares(checkpoint_exposure)?;
        if new_share_reserves >= exposure_shares + self.minimum_share_reserves() {
            Ok(new_share_reserves - exposure_shares - self.minimum_share_reserves())
        } else {
            Err(eyre!("Insiffucient liquidity. Expected share_reserves={:#?} >= {:#?} = exposure_shares={:#?} + min_share_reserves={:#?}",
            new_share_reserves, exposure_shares + self.minimum_share_reserves(), exposure_shares, self.minimum_share_reserves()))
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
    /// where the second term is P_{\text{lp}}(\Delta y) and the remaining terms
    /// are from fees. Therefore, the derivative is
    ///
    /// ```math
    /// \frac{\partial \Delta z}{\partial \Delta y} = - P_{\text{lp}}'(\Delta y)
    ///         - \left(\phi_c \cdot (1 - p) \cdot (1 - \phi_g) \right)
    /// ```
    fn calculate_pool_share_delta_after_short_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<I256>> {
        let lhs = self
            .calculate_short_principal_derivative(bond_amount)?
            .change_type::<I256>()?;
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
    ///          - g'(\Delta y))}{c} \right)
    /// ```
    fn solvency_after_short_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<I256>> {
        let lhs = self
            .calculate_short_principal_derivative(bond_amount)?
            .change_type::<I256>()?;
        let rhs = (self.curve_fee()
            * (fixed!(1e18) - self.calculate_spot_price_down()?)
            * (fixed!(1e18) - self.governance_lp_fee())
            / self.vault_share_price())
        .change_type::<I256>()?;
        Ok(-(lhs - rhs))
    }
}

#[cfg(test)]
mod tests {
    use std::{panic, path::absolute};

    use ethers::types::{I256, U128, U256};
    use fixedpointmath::{fixed, fixed_u256, uint256, FixedPoint};
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
        agent::HyperdriveMathAgent,
        preamble::{get_max_short, initialize_pool_with_random_state},
    };

    #[tokio::test]
    async fn fuzz_calculate_pool_share_delta_after_short_derivative() -> Result<()> {
        let empirical_derivative_epsilon = fixed!(1e14);
        let test_tolerance = fixed!(1e14);
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

            // Min trade amount should be at least 1,000x the derivative epsilon
            let bond_amount = rng.gen_range(fixed!(1e18)..=fixed!(10_000_000e18));

            let f_x = match panic::catch_unwind(|| {
                state.calculate_pool_share_delta_after_open_short(bond_amount)
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };
            let f_x_plus_delta = match panic::catch_unwind(|| {
                state.calculate_pool_share_delta_after_open_short(
                    bond_amount + empirical_derivative_epsilon,
                )
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };

            // Compute the absolute value of the empirical and analytical
            // derivatives.
            let pool_share_delta_derivative =
                state.calculate_pool_share_delta_after_short_derivative(bond_amount)?;
            let empirical_derivative = if f_x < f_x_plus_delta {
                // Adding delta bonds results in smaller pool share delta.
                // This implies a positive derivative.
                assert!(pool_share_delta_derivative >= fixed!(0));
                (f_x_plus_delta - f_x) / empirical_derivative_epsilon
            } else {
                // Adding delta bonds results in larger pool share delta.
                // This implies a negative derivative.
                assert!(pool_share_delta_derivative <= fixed!(0));
                (f_x - f_x_plus_delta) / empirical_derivative_epsilon
            };
            let pool_share_delta_derivative =
                pool_share_delta_derivative.abs().change_type::<U256>()?;

            // Ensure that the empirical and analytical derivatives match.
            let derivative_diff = if empirical_derivative > pool_share_delta_derivative {
                empirical_derivative - pool_share_delta_derivative
            } else {
                pool_share_delta_derivative - empirical_derivative
            };
            assert!(
                derivative_diff < test_tolerance,
                "expected abs(derivative_diff)={:#?} < test_tolerance={:#?};
                calculated_derivative={:#?}, empirical_derivative={:#?}",
                derivative_diff,
                test_tolerance,
                pool_share_delta_derivative,
                empirical_derivative
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_solvency_after_short_derivative() -> Result<()> {
        let empirical_derivative_epsilon = fixed!(1e14);
        let test_tolerance = fixed!(1e14);
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

            // Min trade amount should be at least 1,000x the derivative epsilon
            let bond_amount = rng.gen_range(fixed!(1e18)..=fixed!(10_000_000e18));

            let f_x = match panic::catch_unwind(|| {
                state.solvency_after_short(bond_amount, checkpoint_exposure)
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };
            let f_x_plus_delta = match panic::catch_unwind(|| {
                state.solvency_after_short(
                    bond_amount + empirical_derivative_epsilon,
                    checkpoint_exposure,
                )
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

    #[tokio::test]
    async fn fuzz_calculate_short_bonds_given_deposit() -> Result<()> {
        let test_tolerance = fixed!(1e9);
        let max_iterations = 500;
        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::from(U256::from(U128::MAX)));
                if rng.gen() {
                    -I256::try_from(value)?
                } else {
                    I256::try_from(value)?
                }
            };
            // TODO: Absolute max doesn't always work. Sometimes the random
            // state causes an overflow when calculating absolute max.
            // Unlikely: fix the state generator so that the random state always has a valid max.
            // Likely: fix absolute max short such that the output is guaranteed to be solvent.
            match panic::catch_unwind(|| {
                state.calculate_absolute_max_short(
                    checkpoint_exposure,
                    Some(test_tolerance),
                    Some(max_iterations),
                )
            }) {
                Ok(max_bond_no_panic) => {
                    match max_bond_no_panic {
                        Ok(absolute_max_bond_amount) => {
                            // Get random input parameters.
                            let open_vault_share_price =
                                rng.gen_range(fixed!(1e5)..=state.vault_share_price());
                            match panic::catch_unwind(|| {
                                state.calculate_open_short(
                                    absolute_max_bond_amount,
                                    open_vault_share_price,
                                )
                            }) {
                                Ok(result_no_panic) => match result_no_panic {
                                    Ok(_) => (),
                                    Err(_) => continue,
                                },
                                Err(_) => continue,
                            }
                            let max_short_bonds =
                                match get_max_short(state.clone(), checkpoint_exposure, None) {
                                    Ok(max_short_trade) => {
                                        max_short_trade.min(absolute_max_bond_amount)
                                    }
                                    Err(_) => continue,
                                };
                            let max_short_base = state
                                .calculate_open_short(max_short_bonds, open_vault_share_price)?;
                            let target_base_amount =
                                rng.gen_range(state.minimum_transaction_amount()..=max_short_base);
                            // Run the function to be tested.
                            let bond_amount = state.calculate_short_bonds_given_deposit(
                                target_base_amount,
                                open_vault_share_price,
                                absolute_max_bond_amount,
                                Some(test_tolerance),
                                Some(max_iterations),
                            )?;
                            // Verify outputs.
                            let computed_base_amount =
                                state.calculate_open_short(bond_amount, open_vault_share_price)?;
                            assert!(
                                target_base_amount >= computed_base_amount,
                                "target is less than computed base amount:
                                target_base_amount  ={:#?}
                                computed_base_amount={:#?}",
                                target_base_amount,
                                computed_base_amount
                            );
                            assert!(
                                target_base_amount - computed_base_amount <= test_tolerance,
                                "target - computed base amounts are greater than tolerance:
                                error     = {:#?}
                                tolerance = {:#?}",
                                target_base_amount - computed_base_amount,
                                test_tolerance
                            );
                        }
                        Err(_) => continue, // absolute max threw an error
                    }
                }
                Err(_) => continue, // absolute max threw a panic
            }
        }
        Ok(())
    }

    /// Test to ensure that the absolute max short guess is always solvent.
    #[tokio::test]
    async fn fuzz_calculate_absolute_max_short_guess() -> Result<()> {
        let solvency_tolerance = fixed!(100_000_000e18);
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            // Compute a random state and checkpoint exposure.
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::from(U256::from(U128::MAX)));
                if rng.gen() {
                    -I256::try_from(value)?
                } else {
                    I256::try_from(value)?
                }
            }
            .min(I256::try_from(state.long_exposure())?);

            // Check that a short is possible.
            if state
                .effective_share_reserves()?
                .min(state.share_reserves())
                < state.calculate_min_share_reserves(checkpoint_exposure)?
            {
                continue;
            }
            match state
                .solvency_after_short(state.minimum_transaction_amount(), checkpoint_exposure)
            {
                Ok(_) => (),
                Err(_) => continue,
            }

            // Compute the guess, check that it is solvent.
            let max_short_guess = state.absolute_max_short_guess(checkpoint_exposure)?;
            let solvency = state.solvency_after_short(max_short_guess, checkpoint_exposure)?;

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
        // Run the fuzz tests
        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::from(U256::from(U128::MAX)));
                if rng.gen() {
                    -I256::try_from(value)?
                } else {
                    I256::try_from(value)?
                }
            }
            .min(I256::try_from(state.long_exposure())?);

            // Make sure a short is possible.
            if state
                .effective_share_reserves()?
                .min(state.share_reserves())
                < state.calculate_min_share_reserves(checkpoint_exposure)?
            {
                continue;
            }
            match state
                .solvency_after_short(state.minimum_transaction_amount(), checkpoint_exposure)
            {
                Ok(_) => (),
                Err(_) => continue,
            }

            // Get the max short.
            let absolute_max_short = state.calculate_absolute_max_short(
                checkpoint_exposure,
                Some(bonds_tolerance),
                Some(max_iterations),
            )?;

            // The short should be valid.
            assert!(absolute_max_short >= state.minimum_transaction_amount());
            assert!(state
                .solvency_after_short(absolute_max_short, checkpoint_exposure)
                .is_ok());

            // Adding tolerance more bonds should be insolvent.
            assert!(state
                .solvency_after_short(absolute_max_short + bonds_tolerance, checkpoint_exposure)
                .is_err());
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

            // Check that a short is possible.
            if state.effective_share_reserves()?
                < state.calculate_min_share_reserves(checkpoint_exposure)?
            {
                chain.revert(id).await?;
                alice.reset(Default::default()).await?;
                // Don't need to reset bob because he hasn't done anything.
                continue;
            }
            let global_max_short_bonds =
                state.calculate_absolute_max_short(checkpoint_exposure, None, None)?;

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
                    // Check that a short is possible.
                    // TODO: We will remove this check in the future; this a failure case in rust that is not
                    // checked in solidity.
                    if state.effective_share_reserves()?
                        < state.calculate_min_share_reserves(checkpoint_exposure)?
                    {
                        chain.revert(id).await?;
                        alice.reset(Default::default()).await?;
                        bob.reset(Default::default()).await?;
                        celine.reset(Default::default()).await?;
                        continue;
                    }

                    // Solidity reports everything is good, so we run the Rust fns.
                    let rust_max_bonds = panic::catch_unwind(|| {
                        state.calculate_absolute_max_short(
                            checkpoint_exposure,
                            None,
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
                            checkpoint_exposure,
                            None,
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
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::from(U256::from(U128::MAX)));
                if rng.gen() {
                    -I256::try_from(value)?
                } else {
                    I256::try_from(value)?
                }
            };
            // Vary the baseline scale factor by adjusting the initial bond amount.
            let bond_amount = rng.gen_range(fixed!(1e10)..=fixed!(100_000_000e18));
            // Compute f_x at the baseline bond_amount.
            let f_x = match panic::catch_unwind(|| {
                state.solvency_after_short(bond_amount, checkpoint_exposure)
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // Start value is not solvent; skip the test.
                },
                Err(_) => continue, // Start value caused panic; skip the test.
            };
            // Compute f_x for each increment.
            let mut f_x_values = Vec::new();
            for &inc in &increments {
                match panic::catch_unwind(|| {
                    state.solvency_after_short(bond_amount + inc, checkpoint_exposure)
                }) {
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
