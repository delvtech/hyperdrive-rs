use std::cmp::{max, min, Ordering};

use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, int256, FixedPoint};

use crate::{calculate_effective_share_reserves, State, YieldSpace};

pub static SHARE_PROCEEDS_MAX_ITERATIONS: u64 = 4;
pub static SHARE_PROCEEDS_SHORT_CIRCUIT_TOLERANCE: u128 = 1_000_000_000; // 1e9
pub static SHARE_PROCEEDS_TOLERANCE: u128 = 100_000_000_000_000; // 1e14

impl State {
    /// Calculates the initial reserves. We solve for the initial reserves
    /// by solving the following equations simultaneously:
    ///
    /// (1) `$c \cdot z = c \cdot z_e + p_{\text{target}} \cdot y$`
    ///
    /// (2) `$p_{\text{target}} = \left(\tfrac{\mu \cdot z_e}{y}\right)^{t_s}$`
    ///
    /// where `$p_{\text{target}}$` is the target spot price implied by the
    /// target spot rate.
    pub fn calculate_initial_reserves(
        &self,
        share_amount: FixedPoint<U256>,
        target_apr: FixedPoint<U256>,
    ) -> Result<(FixedPoint<U256>, I256, FixedPoint<U256>)> {
        // NOTE: Round down to underestimate the initial bond reserves.
        //
        // Normalize the time to maturity to fractions of a year since the provided
        // rate is an APR.
        let t = self
            .position_duration()
            .div_down(U256::from(60 * 60 * 24 * 365).into());

        // NOTE: Round up to underestimate the initial bond reserves.
        //
        // Calculate the target price implied by the target rate.
        let one = fixed!(1e18);
        let target_price = one.div_up(one + target_apr.mul_down(t));

        // The share reserves is just the share amount since we are initializing
        // the pool.
        let share_reserves = share_amount;

        // NOTE: Round down to underestimate the initial bond reserves.
        //
        // Calculate the initial bond reserves. This is given by:
        //
        // y = (mu * c * z) / (c * p_target ** (1 / t_s) + mu * p_target)
        let bond_reserves = self.initial_vault_share_price().mul_div_down(
            self.vault_share_price().mul_down(share_reserves),
            self.vault_share_price()
                .mul_down(target_price.pow(one.div_down(self.time_stretch()))?)
                + self.initial_vault_share_price().mul_up(target_price),
        );

        // NOTE: Round down to underestimate the initial share adjustment.
        //
        // Calculate the initial share adjustment. This is given by:
        //
        // zeta = (p_target * y) / c
        let share_adjustment =
            I256::try_from(bond_reserves.mul_div_down(target_price, self.vault_share_price()))?;

        Ok((share_reserves, share_adjustment, bond_reserves))
    }

    /// Calculates the resulting share_reserves, share_adjustment, and
    /// bond_reserves when updating liquidity with a `share_reserves_delta`.
    pub fn calculate_update_liquidity(
        &self,
        share_reserves: FixedPoint<U256>,
        share_adjustment: I256,
        bond_reserves: FixedPoint<U256>,
        minimum_share_reserves: FixedPoint<U256>,
        share_reserves_delta: I256,
    ) -> Result<(FixedPoint<U256>, I256, FixedPoint<U256>)> {
        // If the share reserves delta is zero, we can return early since no
        // action is needed.
        if share_reserves_delta == I256::zero() {
            return Ok((share_reserves, share_adjustment, bond_reserves));
        }

        // Update the share reserves by applying the share reserves delta. We
        // ensure that our minimum share reserves invariant is still maintained.
        let new_share_reserves = I256::try_from(share_reserves)? + share_reserves_delta;
        if new_share_reserves < I256::try_from(minimum_share_reserves).unwrap() {
            return Err(eyre!(
                "update would result in share reserves below minimum."
            ));
        }
        let new_share_reserves = FixedPoint::try_from(new_share_reserves)?;

        // Update the share adjustment by holding the ratio of share reserves
        // to share adjustment proportional. In general, our pricing model cannot
        // support negative values for the z coordinate, so this is important as
        // it ensures that if z - zeta starts as a positive value, it ends as a
        // positive value. With this in mind, we update the share adjustment as:
        //
        // zeta_old / z_old = zeta_new / z_new
        //                  =>
        // zeta_new = zeta_old * (z_new / z_old)
        let new_share_adjustment = if share_adjustment >= I256::zero() {
            let share_adjustment_fp = FixedPoint::try_from(share_adjustment)?;
            I256::try_from(new_share_reserves.mul_div_down(share_adjustment_fp, share_reserves))?
        } else {
            let share_adjustment_fp = FixedPoint::try_from(-share_adjustment)?;
            -I256::try_from(new_share_reserves.mul_div_up(share_adjustment_fp, share_reserves))?
        };

        // NOTE: Rounding down to avoid introducing dust into the computation.
        //
        // The liquidity update should hold the spot price invariant. The spot
        // price of base in terms of bonds is given by:
        //
        // p = (mu * (z - zeta) / y) ** tau
        //
        // This formula implies that holding the ratio of share reserves to bond
        // reserves constant will hold the spot price constant. This allows us
        // to calculate the updated bond reserves as:
        //
        // (z_old - zeta_old) / y_old = (z_new - zeta_new) / y_new
        //                          =>
        // y_new = (z_new - zeta_new) * (y_old / (z_old - zeta_old))
        let old_effective_share_reserves =
            calculate_effective_share_reserves(self.share_reserves(), self.share_adjustment())?;
        let new_effective_share_reserves =
            calculate_effective_share_reserves(new_share_reserves, new_share_adjustment)?;
        let new_bond_reserves =
            bond_reserves.mul_div_down(new_effective_share_reserves, old_effective_share_reserves);

        Ok((new_share_reserves, new_share_adjustment, new_bond_reserves))
    }

    /// Calculates the present value in shares of LP's capital in the pool.
    pub fn calculate_present_value(
        &self,
        current_block_timestamp: U256,
    ) -> Result<FixedPoint<U256>> {
        // Calculate the average time remaining for the longs and shorts.

        // To keep precision of long and short average maturity time (from contract call)
        // we scale the block timestamp and position duration by 1e18 to calculate
        // the normalized time remaining.
        let long_average_time_remaining = self.calculate_scaled_normalized_time_remaining(
            self.long_average_maturity_time(),
            current_block_timestamp,
        );
        let short_average_time_remaining = self.calculate_scaled_normalized_time_remaining(
            self.short_average_maturity_time(),
            current_block_timestamp,
        );
        let net_curve_trade = self
            .calculate_net_curve_trade(long_average_time_remaining, short_average_time_remaining)?;
        let net_flat_trade = self
            .calculate_net_flat_trade(long_average_time_remaining, short_average_time_remaining)?;

        let present_value: I256 =
            I256::try_from(self.share_reserves())? + net_curve_trade + net_flat_trade
                - I256::try_from(self.minimum_share_reserves())?;

        if present_value < int256!(0) {
            return Err(eyre!("Negative present value!"));
        }

        Ok(present_value.try_into()?)
    }

    pub fn calculate_net_curve_trade(
        &self,
        long_average_time_remaining: FixedPoint<U256>,
        short_average_time_remaining: FixedPoint<U256>,
    ) -> Result<I256> {
        // NOTE: To underestimate the impact of closing the net curve position,
        // we round up the long side of the net curve position (since this
        // results in a larger value removed from the share reserves) and round
        // down the short side of the net curve position (since this results in
        // a smaller value added to the share reserves).
        //
        // The net curve position is the net of the longs and shorts that are
        // currently tradeable on the curve. Given the amount of outstanding
        // longs `y_l` and shorts `y_s` as well as the average time remaining
        // of outstanding longs `t_l` and shorts `t_s`, we can
        // compute the net curve position as:
        //
        // netCurveTrade = y_l * t_l - y_s * t_s.
        let net_curve_position =
            I256::try_from(self.longs_outstanding().mul_up(long_average_time_remaining))?
                - I256::try_from(
                    self.shorts_outstanding()
                        .mul_down(short_average_time_remaining),
                )?;
        match self.effective_share_reserves() {
            Ok(_) => {}
            // NOTE: Return 0 to indicate that the net curve trade couldn't be
            // computed.
            Err(_) => return Ok(I256::zero()),
        }

        // If the net curve position is positive, then the pool is net long.
        // Closing the net curve position results in the longs being paid out
        // from the share reserves, so we negate the result.
        match net_curve_position.cmp(&int256!(0)) {
            Ordering::Greater => {
                let net_curve_position: FixedPoint<U256> =
                    FixedPoint::try_from(net_curve_position)?;
                let max_curve_trade =
                    match self.calculate_max_sell_bonds_in(self.minimum_share_reserves()) {
                        Ok(max_curve_trade) => max_curve_trade,
                        Err(_) => {
                            // NOTE: Return 0 to indicate that the net curve trade couldn't
                            // be computed.
                            return Ok(I256::zero());
                        }
                    };

                if max_curve_trade >= net_curve_position {
                    match self.calculate_shares_out_given_bonds_in_down(net_curve_position) {
                        Ok(net_curve_trade) => Ok(-I256::try_from(net_curve_trade)?),
                        Err(err) => {
                            // If the net curve position is smaller than the
                            // minimum transaction amount and the trade fails,
                            // we mark it to 0. This prevents liveness problems
                            // when the net curve position is very small.
                            if net_curve_position < self.minimum_transaction_amount() {
                                Ok(I256::zero())
                            } else {
                                Err(err)
                            }
                        }
                    }
                } else {
                    // If the share adjustment is greater than or equal to zero,
                    // then the effective share reserves are less than or equal to
                    // the share reserves. In this case, the maximum amount of
                    // shares that can be removed from the share reserves is
                    // `effectiveShareReserves - minimumShareReserves`.
                    if self.share_adjustment() >= I256::from(0) {
                        Ok(-I256::try_from(
                            self.effective_share_reserves()? - self.minimum_share_reserves(),
                        )?)

                    // Otherwise, the effective share reserves are greater than the
                    // share reserves. In this case, the maximum amount of shares
                    // that can be removed from the share reserves is
                    // `shareReserves - minimumShareReserves`.
                    } else {
                        Ok(-I256::try_from(
                            self.share_reserves() - self.minimum_share_reserves(),
                        )?)
                    }
                }
            }
            Ordering::Less => {
                let net_curve_position: FixedPoint<U256> =
                    FixedPoint::try_from(-net_curve_position)?;
                let max_curve_trade = match self.calculate_max_buy_bonds_out() {
                    Ok(max_curve_trade) => max_curve_trade,
                    Err(_) => {
                        // NOTE: Return 0 to indicate that the net curve trade couldn't
                        // be computed.
                        return Ok(I256::zero());
                    }
                };
                if max_curve_trade >= net_curve_position {
                    match self.calculate_shares_in_given_bonds_out_up(net_curve_position) {
                        Ok(net_curve_trade) => Ok(I256::try_from(net_curve_trade)?),
                        Err(err) => {
                            // If the net curve position is smaller than the
                            // minimum transaction amount and the trade fails,
                            // we mark it to 0. This prevents liveness problems
                            // when the net curve position is very small.
                            if net_curve_position < self.minimum_transaction_amount() {
                                Ok(I256::zero())
                            } else {
                                Err(err)
                            }
                        }
                    }
                } else {
                    let max_share_payment = match self.calculate_max_buy_shares_in() {
                        Ok(max_share_payment) => max_share_payment,
                        Err(_) => {
                            // NOTE: Return 0 to indicate that the net curve trade couldn't
                            // be computed.
                            return Ok(I256::zero());
                        }
                    };

                    // NOTE: We round the difference down to underestimate the
                    // impact of closing the net curve position.
                    Ok(I256::try_from(
                        max_share_payment
                            + (net_curve_position - max_curve_trade)
                                .div_down(self.vault_share_price()),
                    )?)
                }
            }
            Ordering::Equal => Ok(int256!(0)),
        }
    }

    /// Calculates the result of closing the net flat position.
    pub fn calculate_net_flat_trade(
        &self,
        long_average_time_remaining: FixedPoint<U256>,
        short_average_time_remaining: FixedPoint<U256>,
    ) -> Result<I256> {
        if self.vault_share_price() == fixed!(0) {
            return Err(eyre!("Vault share price is zero."));
        }
        if short_average_time_remaining > fixed!(1e18) || long_average_time_remaining > fixed!(1e18)
        {
            return Err(eyre!("Average time remaining is greater than 1e18."));
        }
        // NOTE: In order to underestimate the impact of closing all of the
        // flat trades, we round the impact of closing the shorts down and round
        // the impact of closing the longs up.
        //
        // Compute the net of the longs and shorts that will be traded flat and
        // apply this net to the reserves.
        let net_flat_trade = I256::try_from(self.shorts_outstanding().mul_div_down(
            fixed!(1e18) - short_average_time_remaining,
            self.vault_share_price(),
        ))? - I256::try_from(self.longs_outstanding().mul_div_up(
            fixed!(1e18) - long_average_time_remaining,
            self.vault_share_price(),
        ))?;

        Ok(net_flat_trade)
    }

    /// Calculates the amount of withdrawal shares that can be redeemed and
    /// the share proceeds the withdrawal pool should receive given the
    /// pool's current idle liquidity. We use the following algorithm to
    /// ensure that the withdrawal pool receives the correct amount of
    /// shares to (1) preserve the LP share price and (2) pay out as much
    /// of the idle liquidity as possible to the withdrawal pool:
    ///
    /// 1. If `$y_s \cdot t_s <= y_l \cdot t_l$` or
    ///    `$y_{\text{max\_out}}(I) >= y_s \cdot t_s - y_l \cdot t_l$` ,
    ///    set `$dz_{\text{max}} = I$` and proceed to step (3).
    ///    Otherwise, proceed to step (2).
    /// 2. Solve
    ///    `$y_{\text{max\_out}}(dz_{\text{max}}) = y_s \cdot t_s - y_l \cdot t_l$`
    ///    for `$dz_{\text{max}}$` using Newton's method.
    /// 3. Set `$dw = (1 - \tfrac{PV(dz_{\text{max}})}{PV(0)}) \cdot l$`.
    ///    If `$dw <= w$`, then proceed to step (5). Otherwise, set `$dw = w$`
    ///    and continue to step (4).
    /// 4. Solve `$\tfrac{PV(0)}{l} = \tfrac{PV(dz)}{(l - dw)}$` for `$dz$`
    ///    using Newton's method if `$y_l \cdot t_l ~= y_s \cdot t_s$` or
    ///    directly otherwise.
    /// 5. Return `$dw$` and `$dz$`.
    ///
    ///  Returns `(withdrawal_shares_redeemed, share_proceeds, success)`
    pub fn calculate_distribute_excess_idle(
        &self,
        current_block_timestamp: U256,
        active_lp_total_supply: FixedPoint<U256>,
        withdrawal_shares_total_supply: FixedPoint<U256>, // withdraw shares - ready to withdraw
        max_iterations: u64,
    ) -> Result<(FixedPoint<U256>, FixedPoint<U256>)> {
        // Steps 1 and 2: Calculate the maximum amount the share reserves can be
        // debited. If the effective share reserves or the maximum share
        // reserves delta can't be calculated or if the maximum share reserves
        // delta is zero, idle can't be distributed.
        let success = match self.effective_share_reserves() {
            Ok(_) => true,
            // The error is safe from the calculation, panics are not.
            Err(_) => false,
        };
        if success == false {
            return Ok((fixed!(0), fixed!(0)));
        }
        let (max_share_reserves_delta, success) =
            self.calculate_max_share_reserves_delta_safe(current_block_timestamp)?;
        if success == false || max_share_reserves_delta == fixed!(0) {
            return Ok((fixed!(0), fixed!(0)));
        }

        // Step 3: Calculate the amount of withdrawal shares that can be
        // redeemed given the maximum share reserves delta.  Otherwise, we
        // proceed to calculating the amount of shares that should be paid out
        // to redeem all of the withdrawal shares.
        let withdrawal_shares_redeemed = {
            let withdrawal_shares_redeemed = self
                .calculate_distribute_excess_idle_withdrawal_shares_redeemed(
                    current_block_timestamp,
                    max_share_reserves_delta,
                    active_lp_total_supply,
                    withdrawal_shares_total_supply,
                )?;

            // Step 3: If none of the withdrawal shares could be redeemed, then
            // we're done and we pay out nothing.
            if withdrawal_shares_redeemed == fixed!(0) {
                return Ok((fixed!(0), fixed!(0)));
            }
            // Step 3: Otherwise if this amount is less than or equal to the amount
            // of withdrawal shares outstanding, then we're done and we pay out the
            // full maximum share reserves delta.
            else if withdrawal_shares_redeemed <= withdrawal_shares_total_supply {
                return Ok((withdrawal_shares_redeemed, max_share_reserves_delta));
            }
            // Step 3: Otherwise, all of the withdrawal shares are redeemed, and we
            // need to calculate the amount of shares the withdrawal pool should
            // receive.
            else {
                withdrawal_shares_total_supply
            }
        };

        // Step 4: Solve for the share proceeds that hold the LP share price
        // invariant after all of the withdrawal shares are redeemed. If the
        // calculation returns a share proceeds of zero, we can't pay out
        // anything.
        let share_proceeds = self.calculate_distribute_excess_idle_share_proceeds(
            current_block_timestamp,
            active_lp_total_supply,
            withdrawal_shares_total_supply,
            max_share_reserves_delta,
            max_iterations,
        )?;
        if share_proceeds == fixed!(0) {
            return Ok((fixed!(0), fixed!(0)));
        }

        // Step 4: If the share proceeds are greater than or equal to the
        // maximum share reserves delta that was previously calculated, then
        // we can't distribute excess idle since we ruled out the possibility
        // of paying out the full maximum share reserves delta in step 3.
        if share_proceeds >= max_share_reserves_delta {
            return Ok((fixed!(0), fixed!(0)));
        }

        // Step 5: Return the amount of withdrawal shares redeemed and the
        // share proceeds.
        Ok((withdrawal_shares_redeemed, share_proceeds))
    }

    /// Calculates the amount of withdrawal shares that can be redeemed
    /// given an amount of shares to remove from the share reserves.
    /// Assuming that dz is the amount of shares to remove from the
    /// reserves and dl is the amount of LP shares to be burned, we can
    /// derive the calculation as follows:
    ///
    ///  PV(0) / l = PV(dz) / (l - dl)
    ///                =>
    ///  dl = l - l * (PV(dz) / PV(0))
    ///
    ///  We round this calculation up to err on the side of slightly too
    ///  many withdrawal shares being redeemed.
    fn calculate_distribute_excess_idle_withdrawal_shares_redeemed(
        &self,
        current_block_timestamp: U256,
        share_reserves_delta: FixedPoint<U256>,
        active_lp_total_supply: FixedPoint<U256>,
        withdrawal_shares_total_supply: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // Calculate the present value after debiting the share reserves delta.
        let updated_state =
            match self.get_state_after_liquidity_update(-I256::try_from(share_reserves_delta)?) {
                Ok(state) => state,
                // NOTE: Return zero to indicate that the withdrawal shares redeemed
                // couldn't be calculated.
                Err(_) => return Ok(fixed!(0)),
            };
        let starting_present_value = match self.calculate_present_value(current_block_timestamp) {
            Ok(present_value) => present_value,
            // NOTE: Return zero to indicate that the withdrawal shares redeemed
            // couldn't be calculated.
            // Errors are safe from this calculation, panics are not.
            Err(_) => return Ok(fixed!(0)),
        };
        let ending_present_value =
            match updated_state.calculate_present_value(current_block_timestamp) {
                Ok(present_value) => present_value,
                // NOTE: Return zero to indicate that the withdrawal shares redeemed
                // couldn't be calculated.
                // Errors are safe from this calculation, panics are not.
                Err(_) => return Ok(fixed!(0)),
            };

        // If the ending present value is greater than or equal to the starting
        // present value, we short-circuit to avoid distributing excess idle.
        // This edge-case can occur when the share reserves is very close to the
        // minimum share reserves with a large value of k.
        if ending_present_value >= starting_present_value {
            return Ok(fixed!(0));
        }

        // NOTE: This subtraction is safe since the ending present value is less
        // than the starting present value and the rhs is rounded down.
        //
        // Calculate the amount of withdrawal shares that can be redeemed.
        let lp_total_supply = active_lp_total_supply + withdrawal_shares_total_supply;
        Ok(lp_total_supply
            - lp_total_supply.mul_div_down(ending_present_value, starting_present_value))
    }

    /// Calculates the share proceeds to distribute to the withdrawal pool
    /// assuming that all of the outstanding withdrawal shares will be
    /// redeemed. The share proceeds are calculated such that the LP share
    /// price is conserved. When we need to round, we round down to err on
    /// the side of slightly too few shares being paid out.
    fn calculate_distribute_excess_idle_share_proceeds(
        &self,
        current_block_timestamp: U256,
        active_lp_total_supply: FixedPoint<U256>, // just the number of lp tokens
        withdrawal_shares_total_supply: FixedPoint<U256>,
        max_share_reserves_delta: FixedPoint<U256>,
        max_iterations: u64,
    ) -> Result<FixedPoint<U256>> {
        // Calculate the LP total supply.
        let lp_total_supply = active_lp_total_supply + withdrawal_shares_total_supply;

        // NOTE: Round the initial guess down to avoid overshooting.
        //
        // We make an initial guess for Newton's method by assuming that the
        // ratio of the share reserves delta to the withdrawal shares
        // outstanding is equal to the LP share price. In reality, the
        // withdrawal pool should receive more than this, but it's a good
        // starting point. The calculation is:
        //
        // x_0 = w * (PV(0) / l)
        let starting_present_value = self.calculate_present_value(current_block_timestamp)?;
        let mut share_proceeds =
            withdrawal_shares_total_supply.mul_div_down(starting_present_value, lp_total_supply);

        // If the pool is net neutral, the initial guess is equal to the final
        // result.
        let net_curve_trade =
            self.calculate_net_curve_trade_from_timestamp(current_block_timestamp)?;
        if net_curve_trade == int256!(0) {
            return Ok(share_proceeds);
        }

        // Proceed with Newton's method. The objective function, `F(x)`, is
        // given by:
        //
        // F(x) = PV(x) * l - PV(0) * (l - w)
        //
        // Newton's method will terminate as soon as the current iteration is
        // within the minimum tolerance or the maximum number of iterations has
        // been reached.
        let mut smallest_delta = I256::zero();
        let mut closest_share_proceeds = fixed!(0);
        let mut closest_present_value = fixed!(0);
        for _ in 0..max(max_iterations, SHARE_PROCEEDS_MAX_ITERATIONS) {
            // Clamp the share proceeds to the max share reserves delta since
            // values above this threshold are always invalid.
            share_proceeds = min(share_proceeds, max_share_reserves_delta);

            // Simulate applying the share proceeds to the reserves.
            //
            // NOTE: We are calling this with 'self' so that original values are
            // used with the updated value of share_proceeds
            let updated_state =
                match self.get_state_after_liquidity_update(-I256::try_from(share_proceeds)?) {
                    Ok(state) => state,
                    // NOTE: If the updated reserves can't be calculated,  we can't
                    // continue the calculation. Return 0 to indicate that the share
                    // proceeds couldn't be calculated.
                    // Errors are safe from this calculation, panics are not.
                    Err(_) => return Ok(fixed!(0)),
                };

            // Recalculate the present value.
            let present_value = match updated_state.calculate_present_value(current_block_timestamp)
            {
                Ok(present_value) => present_value,
                // NOTE: If the present value can't be calculated,  we can't
                // continue the calculation. Return 0 to indicate that the share
                // proceeds couldn't be calculated.
                // Errors are safe from this calculation, panics are not.
                Err(_) => return Ok(fixed!(0)),
            };

            // Short-circuit if we are within the minimum tolerance.
            if self.should_short_circuit_distribute_excess_idle_share_proceeds(
                active_lp_total_supply,
                starting_present_value,
                lp_total_supply,
                present_value,
            )? {
                return Ok(share_proceeds);
            }

            // If the pool is net long, we can solve for the next iteration of
            // Newton's method directly when the net curve trade is greater than
            // or equal to the max bond amount.
            if net_curve_trade > I256::zero() {
                // Calculate the max bond amount. If the calculation fails, we
                // return a failure flag.
                let max_bond_amount = match updated_state
                    .calculate_max_sell_bonds_in(self.minimum_share_reserves())
                {
                    Ok(max_bond_amount) => max_bond_amount,
                    // NOTE: If the max bond amount couldn't be calculated, we
                    // can't continue the calculation. Return 0 to indicate that
                    // the share proceeds couldn't be calculated.
                    // Errors are safe from this calculation, panics are not.
                    Err(_) => return Ok(fixed!(0)),
                };

                // If the net curve trade is greater than or equal to the max
                // bond amount, we can solve directly for the share proceeds.
                let net_curve_trade = FixedPoint::from(U256::try_from(net_curve_trade)?);
                if net_curve_trade >= max_bond_amount {
                    // Solve the objective function directly assuming that it is
                    // linear with respect to the share proceeds.

                    let (share_proceeds, success) = updated_state
                        .calculate_distribute_excess_idle_share_proceeds_net_long_edge_case_safe(
                            current_block_timestamp,
                            self.share_adjustment(),
                            self.share_reserves(),
                            starting_present_value,
                            active_lp_total_supply,
                            withdrawal_shares_total_supply,
                        )?;
                    if success == false {
                        // NOTE: Return 0 to indicate that the share proceeds
                        // couldn't be calculated.
                        return Ok(share_proceeds);
                    }

                    // Simulate applying the share proceeds to the reserves and
                    // recalculate the max bond amount.
                    //
                    // NOTE: We are calling this with 'self' so that original
                    // values are used with the updated value of share_proceeds.
                    let updated_state = match self
                        .get_state_after_liquidity_update(-I256::try_from(share_proceeds)?)
                    {
                        Ok(state) => state,
                        // NOTE: Return 0 to indicate that the share proceeds
                        // couldn't be calculated.
                        // Errors are safe from this calculation, panics are not.
                        Err(_) => return Ok(fixed!(0)),
                    };
                    let max_bond_amount = match updated_state
                        .calculate_max_sell_bonds_in(self.minimum_share_reserves())
                    {
                        Ok(max_bond_amount) => max_bond_amount,
                        // NOTE: Return 0 to indicate that the share proceeds
                        // couldn't be calculated.
                        // Errors are safe from this calculation, panics are not.
                        Err(_) => return Ok(fixed!(0)),
                    };

                    // If the max bond amount is less than or equal to the net
                    // curve trade, then Newton's method has terminated since
                    // proceeding to the next step would result in reaching the
                    // same point.
                    if max_bond_amount <= net_curve_trade {
                        return Ok(share_proceeds);
                    }
                    // Otherwise, we continue to the next iteration of Newton's
                    // method.
                    else {
                        continue;
                    }
                }
            }

            // We calculate the derivative of F(x) using the derivative of
            // `calculateSharesOutGivenBondsIn` when the pool is net long or
            // the derivative of `calculateSharesInGivenBondsOut`. when the pool
            // is net short.
            let derivative = match updated_state
                .calculate_shares_delta_given_bonds_delta_derivative(
                    net_curve_trade,
                    self.share_reserves(),
                    self.bond_reserves(),
                    self.effective_share_reserves()?,
                    self.share_adjustment(),
                ) {
                Ok(derivative) => derivative,
                // NOTE: Return 0 to indicate that the share proceeds
                // couldn't be calculated.
                // Errors are safe from this calculation, panics are not.
                Err(_) => return Ok(fixed!(0)),
            };
            if derivative >= fixed!(1e18) {
                return Ok(fixed!(0));
            }
            let derivative = fixed!(1e18) - derivative;

            // NOTE: Round the delta down to avoid overshooting.
            //
            // Calculate the objective function's value. If the value's magnitude
            // is smaller than the previous smallest value, then we update the
            // value and record the share proceeds. We'll ultimately return the
            // share proceeds that resulted in the smallest value.
            let delta = I256::try_from(present_value.mul_down(lp_total_supply))?
                - I256::try_from(starting_present_value.mul_up(active_lp_total_supply))?;
            if smallest_delta == I256::from(0) || delta.abs() < smallest_delta.abs() {
                smallest_delta = delta;
                closest_share_proceeds = share_proceeds;
                closest_present_value = present_value;
            }

            // We calculate the updated share proceeds `x_n+1` by proceeding
            // with Newton's method. This is given by:
            //
            // x_n+1 = x_n - F(x_n) / F'(x_n)
            if delta > I256::zero() {
                // NOTE: Round the quotient down to avoid overshooting.
                share_proceeds = share_proceeds
                    + FixedPoint::try_from(delta)?
                        .div_down(derivative)
                        .div_down(lp_total_supply);
            } else if delta < I256::zero() {
                let delta = FixedPoint::try_from(-delta)?
                    .div_down(derivative)
                    .div_down(lp_total_supply);
                if delta < share_proceeds {
                    share_proceeds = share_proceeds - delta;
                } else {
                    // NOTE: Returning 0 to indicate that the share proceeds
                    // couldn't be calculated.
                    return Ok(fixed!(0));
                }
            } else {
                break;
            }
        }

        // Get the updated present value after applying the share proceeds.
        let updated_state =
            match self.get_state_after_liquidity_update(-I256::try_from(share_proceeds)?) {
                Ok(state) => state,
                // NOTE: Return 0 to indicate that the share proceeds couldn't
                // be calculated.
                // Errors are safe from this calculation, panics are not.
                Err(_) => return Ok(fixed!(0)),
            };
        let present_value = updated_state.calculate_present_value(current_block_timestamp)?;

        // Check to see if the current share proceeds is closer to the optimal
        // value than the previous closest value. We'll choose whichever of the
        // share proceeds that is closer to the optimal value.
        let last_delta = I256::try_from(present_value.mul_down(lp_total_supply))?
            - I256::try_from(starting_present_value.mul_up(active_lp_total_supply))?;
        if last_delta.abs() < smallest_delta.abs() {
            closest_share_proceeds = share_proceeds;
            closest_present_value = present_value;
        }

        // Verify that the LP share price was conserved within a reasonable
        // tolerance.
        if
        // NOTE: Round down to make the check stricter.
        closest_present_value.div_down(active_lp_total_supply)
                < starting_present_value.mul_div_up(
                    fixed!(1e18) - FixedPoint::from(SHARE_PROCEEDS_TOLERANCE),
                    lp_total_supply,
                ) ||
            // NOTE: Round up to make the check stricter.
            closest_present_value.div_up(active_lp_total_supply)
                > starting_present_value.mul_div_down(
                    fixed!(1e18) + FixedPoint::from(SHARE_PROCEEDS_TOLERANCE),
                    lp_total_supply,
                )
        {
            return Err(eyre!("LP share price was not conserved within tolerance."));
        }

        Ok(closest_share_proceeds)
    }

    /// One of the edge cases that occurs when using Newton's method for
    /// the share proceeds while distributing excess idle is when the net
    /// curve trade is larger than the max bond amount. In this case, the
    /// the present value simplifies to the following:
    ///
    /// PV(dz) = (z - dz) + net_c(dz) + net_f - z_min
    ///        = (z - dz) - z_max_out(dz) + net_f - z_min
    ///
    /// There are two cases to evaluate:
    ///
    /// (1) zeta > 0:
    ///
    /// z_max_out(dz) = ((z - dz) / z) * (z - zeta) - z_min
    ///
    /// =>
    ///
    /// PV(dz) = zeta * ((z - dz) / z) + net_f
    ///
    /// (2) zeta <= 0:
    ///
    /// z_max_out(dz) = (z - dz) - z_min
    ///
    /// =>
    ///
    /// PV(dz) = net_f
    ///
    /// Since the present value is constant with respect to the share
    /// proceeds in case 2, Newton's method has achieved a stationary point
    /// and can't proceed. On the other hand, the present value is linear
    /// with respect to the share proceeds, and we can solve for the next
    /// step of Newton's method directly as follows:
    ///
    /// PV(0) / l = PV(dz) / (l - w)
    ///
    /// =>
    ///
    /// dz = z - ((PV(0) / l) * (l - w) - net_f) / (zeta / z)
    ///
    /// We round the share proceeds down to err on the side of the
    /// withdrawal pool receiving slightly less shares.
    fn calculate_distribute_excess_idle_share_proceeds_net_long_edge_case_safe(
        &self,
        current_block_timestamp: U256,
        original_share_adjustment: I256,
        original_share_reserves: FixedPoint<U256>,
        starting_present_value: FixedPoint<U256>,
        active_lp_total_supply: FixedPoint<U256>,
        withdrawal_shares_total_supply: FixedPoint<U256>,
    ) -> Result<(FixedPoint<U256>, bool)> {
        // If the original share adjustment is zero or negative, we cannot
        // calculate the share proceeds. This should never happen, but for
        // safety we return a failure flag and break the loop at this point.
        if original_share_adjustment <= I256::zero() {
            return Ok((fixed!(0), false));
        }
        let original_share_adjustment: U256 = original_share_adjustment.abs().try_into().unwrap();

        // Calculate the net flat trade.
        let net_flat_trade =
            self.calculate_net_flat_trade_from_timestamp(current_block_timestamp)?;

        // Avoid panic: make sure we don't divide by zero before calculating the rhs.
        if active_lp_total_supply + withdrawal_shares_total_supply == fixed!(0) {
            return Err(eyre!(
                "active_lp_total_supply + withdrawal_shares_total_supply is zero"
            ));
        }

        // NOTE: Round up since this is the rhs of the final subtraction.
        //
        // rhs = (PV(0) / l) * (l - w) - net_f
        let rhs = {
            let rhs: FixedPoint<U256> = starting_present_value.mul_div_up(
                active_lp_total_supply,
                active_lp_total_supply + withdrawal_shares_total_supply,
            );
            if net_flat_trade >= I256::zero() {
                if net_flat_trade < I256::try_from(rhs)? {
                    rhs - net_flat_trade.try_into()?
                } else {
                    // NOTE: Return a failure flag if computing the rhs would
                    // underflow.
                    return Ok((fixed!(0), false));
                }
            } else {
                rhs + (-net_flat_trade).try_into()?
            }
        };

        // NOTE: Round up since this is the rhs of the final subtraction.
        //
        // rhs = ((PV(0) / l) * (l - w) - net_f) / (zeta / z)
        let rhs =
            original_share_reserves.mul_div_up(rhs, FixedPoint::from(original_share_adjustment));

        // share proceeds = z - rhs
        if original_share_reserves < rhs {
            return Ok((fixed!(0), false));
        }

        Ok((original_share_reserves - rhs, true))
    }

    /// Checks to see if we should short-circuit the iterative calculation of
    /// the share proceeds when distributing excess idle liquidity. This
    /// verifies that the ending LP share price is greater than or equal to the
    /// starting LP share price and less than or equal to the starting LP share
    /// price plus the minimum tolerance.
    fn should_short_circuit_distribute_excess_idle_share_proceeds(
        &self,
        active_lp_total_supply: FixedPoint<U256>, // just the total number of lp shares
        starting_present_value: FixedPoint<U256>,
        lp_total_supply: FixedPoint<U256>, // total lp shares and withdrawal shares - w.s. ready to withraw
        present_value: FixedPoint<U256>,
    ) -> Result<bool> {
        Ok(
            // Ensure that new LP share price is greater than or equal to the
            // previous LP share price:
            //
            // PV_1 / l_1 >= PV_0 / l_0
            //
            // NOTE: Round the LHS down to make the check stricter.
            present_value.div_down(active_lp_total_supply) >=
            starting_present_value.div_up(lp_total_supply)
            // Ensure that new LP share price is less than or equal to the
            // previous LP share price plus the minimum tolerance:
            //
            // PV_1 / l_1 <= (PV_0 / l_0) * (1 + tolerance)
            //
            // NOTE: Round the LHS up to make the check stricter.
            && present_value.div_up(active_lp_total_supply) <=
            (fixed!(1e18) + FixedPoint::from(SHARE_PROCEEDS_SHORT_CIRCUIT_TOLERANCE)).mul_div_down(
                starting_present_value,
                lp_total_supply),
        )
    }

    /// Calculates the upper bound on the share proceeds of distributing
    /// excess idle. When the pool is net long or net neutral, the upper
    /// bound is the amount of idle liquidity. When the pool is net short,
    /// the upper bound is the share reserves delta that results in the
    /// maximum amount of bonds that can be purchased being equal to the
    /// net short position.
    fn calculate_max_share_reserves_delta_safe(
        &self,
        current_block_timestamp: U256,
    ) -> Result<(FixedPoint<U256>, bool)> {
        let net_curve_trade =
            self.calculate_net_curve_trade_from_timestamp(current_block_timestamp)?;
        let idle = self.calculate_idle_share_reserves();
        // If the net curve position is zero or net long, then the maximum
        // share reserves delta is equal to the pool's idle.
        if net_curve_trade >= I256::from(0) {
            return Ok((idle, true));
        }
        let net_curve_trade = FixedPoint::try_from(-net_curve_trade)?;

        // Calculate the max bond amount. if the calculation fails, we return a
        // failure flag. if the calculation succeeds but the max bond amount
        // is zero, then we return a failure flag since we can't divide by zero.
        let max_bond_amount = match self.calculate_max_buy_bonds_out() {
            Ok(result) => result,
            // Errors are safe from the calculation, panics are not.
            Err(_) => fixed!(0),
        };
        if max_bond_amount == fixed!(0) {
            return Ok((fixed!(0), false));
        }

        // We can solve for the maximum share reserves delta in one shot using
        // the fact that the maximum amount of bonds that can be purchased is
        // linear with respect to the scaling factor applied to the reserves.
        // In other words, if s > 0 is a factor scaling the reserves, we have
        // the following relationship:
        //
        // y_out^max(s * z, s * y, s * zeta) = s * y_out^max(z, y, zeta)
        //
        // We solve for the maximum share reserves delta by finding the scaling
        // factor that results in the maximum amount of bonds that can be
        // purchased being equal to the net curve trade. We can derive this
        // maximum using the linearity property mentioned above as follows:
        //
        // y_out^max(s * z, s * y, s * zeta) - netCurveTrade = 0
        //                        =>
        // s * y_out^max(z, y, zeta) - netCurveTrade = 0
        //                        =>
        // s = netCurveTrade / y_out^max(z, y, zeta)
        let max_scaling_factor = net_curve_trade.div_up(max_bond_amount);

        // Using the maximum scaling factor, we can calculate the maximum share
        // reserves delta as:
        //
        // maxShareReservesDelta = z * (1 - s)
        let max_share_reserves_delta = if max_scaling_factor <= fixed!(1e18) {
            (fixed!(1e18) - max_scaling_factor).mul_down(self.share_reserves())
        } else {
            // NOTE: If the max scaling factor is greater than one, the
            // calculation fails and we return a failure flag.
            return Ok((fixed!(0), false));
        };

        // If the maximum share reserves delta is greater than the idle, then
        // the maximum share reserves delta is equal to the idle.
        if max_share_reserves_delta > idle {
            return Ok((idle, true));
        }
        return Ok((max_share_reserves_delta, true));
    }

    /// TODO: https://github.com/delvtech/hyperdrive/issues/965
    ///
    /// Note that the state is the present state of the pool and original values
    /// passed in as parameters.  Present sate variables are not expressly
    /// paased in because so that downstream function like kUp() can still be
    /// used.
    ///
    /// Given a signed bond amount, this function calculates the negation
    /// of the derivative of `calculateSharesOutGivenBondsIn` when the
    /// bond amount is positive or the derivative of
    /// `calculateSharesInGivenBondsOut` when the bond amount is negative.
    /// In both cases, the calculation is given by:
    ///
    ///  derivative = (1 - zeta / z) * (
    ///      1 - (1 / c) * (
    ///          c * (mu * z_e(x)) ** -t_s +
    ///          (y / z_e) * y(x) ** -t_s  -
    ///          (y / z_e) * (y(x) + dy) ** -t_s
    ///      ) * (
    ///          (mu / c) * (k(x) - (y(x) + dy) ** (1 - t_s))
    ///      ) ** (t_s / (1 - t_s))
    ///  )
    ///
    ///  This quantity is used in Newton's method to search for the optimal
    ///  share proceeds. When the pool is net long, We can express the
    ///  derivative of the objective function F(x) by the derivative
    ///  -z_out'(x) that this function returns:
    ///
    ///  -F'(x) = l * -PV'(x)
    ///         = l * (1 - net_c'(x))
    ///         = l * (1 + z_out'(x))
    ///         = l * (1 - derivative)
    ///
    ///  When the pool is net short, we can express the derivative of the
    ///  objective function F(x) by the derivative z_in'(x) that this
    ///  function returns:
    ///
    ///  -F'(x) = l * -PV'(x)
    ///         = l * (1 - net_c'(x))
    ///         = l * (1 - z_in'(x))
    ///         = l * (1 - derivative)
    ///
    ///  With these calculations in mind, this function rounds its result
    ///  down so that F'(x) is overestimated. Since F'(x) is in the
    ///  denominator of Newton's method, overestimating F'(x) helps to avoid
    ///  overshooting the optimal solution.
    fn calculate_shares_delta_given_bonds_delta_derivative(
        &self,
        bond_amount: I256,
        original_share_reserves: FixedPoint<U256>,
        original_bond_reserves: FixedPoint<U256>,
        original_effective_share_reserves: FixedPoint<U256>,
        original_share_adjustment: I256,
    ) -> Result<FixedPoint<U256>> {
        // Calculate the bond reserves after the bond amount is applied.
        let bond_reserves_after = if bond_amount >= I256::zero() {
            self.bond_reserves() + bond_amount.try_into()?
        } else {
            let bond_amount = FixedPoint::from(U256::try_from(-bond_amount)?);
            if bond_amount < self.bond_reserves() {
                self.bond_reserves() - bond_amount
            } else {
                return Err(eyre!("Calculating the bond reserves underflows"));
            }
        };

        // NOTE: Round up since this is on the rhs of the final subtraction.
        //
        // derivative = c * (mu * z_e(x)) ** -t_s +
        //              (y / z_e) * (y(x)) ** -t_s -
        //              (y / z_e) * (y(x) + dy) ** -t_s
        let effective_share_reserves = self.effective_share_reserves()?;
        // NOTE: The exponent is positive and base is flipped to handle the negative value.
        let derivative = self.vault_share_price().div_up(
            self.initial_vault_share_price()
                .mul_down(effective_share_reserves)
                .pow(self.time_stretch())?,
        ) + original_bond_reserves.div_up(
            original_effective_share_reserves
                .mul_down(self.bond_reserves().pow(self.time_stretch())?),
        );

        // NOTE: Rounding this down rounds the subtraction up.
        let rhs = original_bond_reserves.div_down(
            original_effective_share_reserves.mul_up(bond_reserves_after.pow(self.time_stretch())?),
        );
        if derivative < rhs {
            return Err(eyre!("Derivative is less than right hand side"));
        }
        let derivative = derivative - rhs;

        // NOTE: Round up since this is on the rhs of the final subtraction.
        //
        // inner = (
        //             (mu / c) * (k(x) - (y(x) + dy) ** (1 - t_s))
        //         ) ** (t_s / (1 - t_s))
        let k = self.k_up()?;
        let inner = bond_reserves_after.pow(fixed!(1e18) - self.time_stretch())?;
        if k < inner {
            return Err(eyre!("k is less than inner"));
        }
        let inner = k - inner;
        let inner = inner.mul_div_up(self.initial_vault_share_price(), self.vault_share_price());
        let inner = if inner >= fixed!(1e18) {
            // NOTE: Round the exponent up since this rounds the result up.
            inner.pow(
                self.time_stretch()
                    .div_up(fixed!(1e18) - self.time_stretch()),
            )?
        } else {
            // NOTE: Round the exponent down since this rounds the result up.
            inner.pow(
                self.time_stretch()
                    .div_down(fixed!(1e18) - self.time_stretch()),
            )?
        };
        let derivative = derivative.mul_div_up(inner, self.vault_share_price());
        let derivative = if fixed!(1e18) > derivative {
            fixed!(1e18) - derivative
        } else {
            // NOTE: Small rounding errors can result in the derivative being
            // slightly (on the order of a few wei) greater than 1. In this case,
            // we return 0 since we should proceed with Newton's method.
            return Ok(fixed!(0));
        };
        // NOTE: Round down to round the final result down.
        //
        // derivative = derivative * (1 - (zeta / z))
        let derivative = if original_share_adjustment >= I256::zero() {
            let right_hand_side =
                FixedPoint::try_from(original_share_adjustment)?.div_up(original_share_reserves);
            if right_hand_side > fixed!(1e18) {
                return Err(eyre!("Right hand side is greater than 1e18"));
            }
            let right_hand_side = fixed!(1e18) - right_hand_side;
            derivative.mul_down(right_hand_side)
        } else {
            derivative.mul_down(
                fixed!(1e18)
                    + FixedPoint::try_from(-original_share_adjustment)?
                        .div_down(original_share_reserves),
            )
        };

        Ok(derivative)
    }
}

#[cfg(test)]
mod tests {
    use fixedpointmath::{fixed_i256, uint256};
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS},
    };
    use hyperdrive_wrappers::wrappers::mock_lp_math::{
        DistributeExcessIdleParams, PresentValueParams,
    };
    use rand::{thread_rng, Rng};

    use super::*;

    #[tokio::test]
    async fn fuzz_calculate_initial_reserves() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let initial_contribution = rng.gen_range(fixed!(0)..=state.bond_reserves());
            let initial_rate = rng.gen_range(fixed!(0)..=fixed!(1));
            let (actual_share_reserves, actual_share_adjustment, actual_bond_reserves) =
                state.calculate_initial_reserves(initial_contribution, initial_rate)?;
            match chain
                .mock_lp_math()
                .calculate_initial_reserves(
                    initial_contribution.into(),
                    state.vault_share_price().into(),
                    state.initial_vault_share_price().into(),
                    initial_rate.into(),
                    state.position_duration().into(),
                    state.time_stretch().into(),
                )
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual_share_reserves, expected.0.into());
                    assert_eq!(actual_share_adjustment, expected.1);
                    assert_eq!(actual_bond_reserves, expected.2.into());
                }
                Err(_) => {}
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_present_value() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let current_block_timestamp = rng.gen_range(fixed!(1)..=fixed!(1e4));
            let actual = state.calculate_present_value(current_block_timestamp.into());
            match chain
                .mock_lp_math()
                .calculate_present_value(PresentValueParams {
                    share_reserves: state.info.share_reserves,
                    bond_reserves: state.info.bond_reserves,
                    longs_outstanding: state.info.longs_outstanding,
                    share_adjustment: state.info.share_adjustment,
                    time_stretch: state.config.time_stretch,
                    vault_share_price: state.info.vault_share_price,
                    initial_vault_share_price: state.config.initial_vault_share_price,
                    minimum_share_reserves: state.config.minimum_share_reserves,
                    minimum_transaction_amount: state.config.minimum_transaction_amount,
                    long_average_time_remaining: state
                        .calculate_scaled_normalized_time_remaining(
                            state.long_average_maturity_time(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    short_average_time_remaining: state
                        .calculate_scaled_normalized_time_remaining(
                            state.short_average_maturity_time(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    shorts_outstanding: state.shorts_outstanding().into(),
                })
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), FixedPoint::from(expected));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_net_curve_trade_safe() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let current_block_timestamp = rng.gen_range(fixed!(1)..=fixed!(1e4));
            let long_average_time_remaining = state.calculate_normalized_time_remaining(
                state.long_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let short_average_time_remaining = state.calculate_normalized_time_remaining(
                state.short_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let actual = state.calculate_net_curve_trade(
                long_average_time_remaining,
                short_average_time_remaining,
            );
            match chain
                .mock_lp_math()
                .calculate_net_curve_trade(PresentValueParams {
                    share_reserves: state.info.share_reserves,
                    bond_reserves: state.info.bond_reserves,
                    longs_outstanding: state.info.longs_outstanding,
                    share_adjustment: state.info.share_adjustment,
                    time_stretch: state.config.time_stretch,
                    vault_share_price: state.info.vault_share_price,
                    initial_vault_share_price: state.config.initial_vault_share_price,
                    minimum_share_reserves: state.config.minimum_share_reserves,
                    minimum_transaction_amount: state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: state.shorts_outstanding().into(),
                })
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), expected);
                }
                Err(_) => {
                    assert!(actual.is_err());
                }
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_net_flat_trade() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();

            let current_block_timestamp = rng.gen_range(fixed!(1)..=fixed!(1e4));

            let long_average_time_remaining = state.calculate_normalized_time_remaining(
                state.long_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let short_average_time_remaining = state.calculate_normalized_time_remaining(
                state.short_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let actual = state.calculate_net_flat_trade(
                long_average_time_remaining,
                short_average_time_remaining,
            );
            match chain
                .mock_lp_math()
                .calculate_net_flat_trade(PresentValueParams {
                    share_reserves: state.info.share_reserves,
                    bond_reserves: state.info.bond_reserves,
                    longs_outstanding: state.info.longs_outstanding,
                    share_adjustment: state.info.share_adjustment,
                    time_stretch: state.config.time_stretch,
                    vault_share_price: state.info.vault_share_price,
                    initial_vault_share_price: state.config.initial_vault_share_price,
                    minimum_share_reserves: state.config.minimum_share_reserves,
                    minimum_transaction_amount: state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: state.shorts_outstanding().into(),
                })
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), expected);
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_distribute_excess_idle() -> Result<()> {
        let chain = TestChain::new().await?;
        let alice = chain.alice().await?;
        let mock = chain.mock_lp_math();

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();

        for _ in 0..*FUZZ_RUNS {
            // Generate random states.
            let mut present_state = rng.gen::<State>();

            // Make sure maturity times are in the future.
            let current_block_timestamp = alice.now().await?;
            present_state.info.long_average_maturity_time += current_block_timestamp;
            present_state.info.short_average_maturity_time += current_block_timestamp;

            // active_lp_total_supply and _withdrawal_shares_total_supply are just the token supplies.
            // Neither are supplied in the PoolInfo so we need to make them here.  lp_total_supply is defined as:
            // lp_total_supply = active_lp_total_supply + withdrawal_shares_total_supply - withdrawal_shares_ready_to_withdraw
            // active_lp_total_supply = lp_total_supply - withdrawal_shares_total_supply + withdrawal_shares_ready_to_withdraw
            // We clip withdrawal_shares_total_supply to ensure active_lp_total_supply doesn't underflow.
            let active_lp_total_supply = present_state.lp_total_supply()
                + present_state.withdrawal_shares_ready_to_withdraw();
            let withdrawal_shares_total_supply = rng.gen_range(fixed!(0)..=active_lp_total_supply);
            let active_lp_total_supply = active_lp_total_supply - withdrawal_shares_total_supply;
            // This errors out a lot so we need to catch that here.
            let starting_present_value =
                match present_state.calculate_present_value(U256::from(current_block_timestamp)) {
                    Ok(result) => result,
                    Err(_) => continue,
                };

            // Calculate the result from the Rust implementation.
            let actual = present_state.calculate_distribute_excess_idle(
                current_block_timestamp,
                active_lp_total_supply,
                withdrawal_shares_total_supply,
                SHARE_PROCEEDS_MAX_ITERATIONS,
            );

            // To keep precision of long and short average maturity time (from contract call)
            // we scale the block timestamp and position duration by 1e18 to calculate
            // the normalized time remaining.
            let long_average_time_remaining = present_state
                .calculate_scaled_normalized_time_remaining(
                    present_state.long_average_maturity_time(),
                    current_block_timestamp,
                );
            let short_average_time_remaining = present_state
                .calculate_scaled_normalized_time_remaining(
                    present_state.short_average_maturity_time(),
                    current_block_timestamp,
                );
            let net_curve_trade = present_state.calculate_net_curve_trade(
                long_average_time_remaining,
                short_average_time_remaining,
            )?;

            let params = DistributeExcessIdleParams {
                present_value_params: PresentValueParams {
                    share_reserves: present_state.info.share_reserves,
                    bond_reserves: present_state.info.bond_reserves,
                    longs_outstanding: present_state.info.longs_outstanding,
                    share_adjustment: present_state.info.share_adjustment,
                    time_stretch: present_state.config.time_stretch,
                    vault_share_price: present_state.info.vault_share_price,
                    initial_vault_share_price: present_state.config.initial_vault_share_price,
                    minimum_share_reserves: present_state.config.minimum_share_reserves,
                    minimum_transaction_amount: present_state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: present_state.shorts_outstanding().into(),
                },
                starting_present_value: starting_present_value.into(),
                active_lp_total_supply: active_lp_total_supply.into(),
                withdrawal_shares_total_supply: withdrawal_shares_total_supply.into(),
                idle: present_state.calculate_idle_share_reserves().into(),
                net_curve_trade: net_curve_trade,
                original_share_reserves: present_state.share_reserves().into(),
                original_share_adjustment: present_state.share_adjustment(),
                original_bond_reserves: present_state.bond_reserves().into(),
            };

            // Make the solidity call and compare to the Rust implementation.
            match mock
                .calculate_distribute_excess_idle(params, SHARE_PROCEEDS_MAX_ITERATIONS.into())
                .call()
                .await
            {
                Ok(expected) => {
                    let (sol_withdrawal_shares_redeemed, sol_share_proceeds) = expected;
                    let (rust_withdrawal_shares_redeemed, rust_share_proceeds) = actual?;
                    assert_eq!(
                        sol_withdrawal_shares_redeemed,
                        U256::from(rust_withdrawal_shares_redeemed)
                    );
                    assert_eq!(sol_share_proceeds, U256::from(rust_share_proceeds));
                }
                Err(_) => {
                    assert!(actual.is_err())
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_distribute_excess_idle_withdrawal_shares_redeemed() -> Result<()> {
        let chain = TestChain::new().await?;
        let alice = chain.alice().await?;
        let mock = chain.mock_lp_math();

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();

        for _ in 0..*FAST_FUZZ_RUNS {
            // Generate random states.
            let mut present_state = rng.gen::<State>();

            // Make sure maturity times are in the future.
            let current_block_timestamp = alice.now().await?;
            present_state.info.long_average_maturity_time += current_block_timestamp;
            present_state.info.short_average_maturity_time += current_block_timestamp;

            // active_lp_total_supply and _withdrawal_shares_total_supply are just the token supplies.
            // Neither are supplied in the PoolInfo so we need to make them here.  lp_total_supply is defined as:
            // lp_total_supply = active_lp_total_supply + withdrawal_shares_total_supply - withdrawal_shares_ready_to_withdraw
            // active_lp_total_supply = lp_total_supply - withdrawal_shares_total_supply + withdrawal_shares_ready_to_withdraw
            // We clip withdrawal_shares_total_supply to ensure active_lp_total_supply doesn't underflow.
            let active_lp_total_supply = present_state.lp_total_supply()
                + present_state.withdrawal_shares_ready_to_withdraw();
            let withdrawal_shares_total_supply = rng.gen_range(fixed!(0)..=active_lp_total_supply);
            let active_lp_total_supply = active_lp_total_supply - withdrawal_shares_total_supply;
            // This errors out a lot so we need to catch that here.
            let starting_present_value =
                match present_state.calculate_present_value(U256::from(current_block_timestamp)) {
                    Ok(result) => result,
                    Err(_) => continue,
                };
            let (share_reserves_delta, _) =
                present_state.calculate_max_share_reserves_delta_safe(current_block_timestamp)?;

            // Calculate the result from the Rust implementation.
            let actual = present_state.calculate_distribute_excess_idle_withdrawal_shares_redeemed(
                current_block_timestamp,
                share_reserves_delta,
                active_lp_total_supply,
                withdrawal_shares_total_supply,
            );

            // To keep precision of long and short average maturity time (from contract call)
            // we scale the block timestamp and position duration by 1e18 to calculate
            // the normalized time remaining.
            let long_average_time_remaining = present_state
                .calculate_scaled_normalized_time_remaining(
                    present_state.long_average_maturity_time(),
                    current_block_timestamp,
                );
            let short_average_time_remaining = present_state
                .calculate_scaled_normalized_time_remaining(
                    present_state.short_average_maturity_time(),
                    current_block_timestamp,
                );
            let net_curve_trade = present_state.calculate_net_curve_trade(
                long_average_time_remaining,
                short_average_time_remaining,
            )?;

            let params = DistributeExcessIdleParams {
                present_value_params: PresentValueParams {
                    share_reserves: present_state.info.share_reserves,
                    bond_reserves: present_state.info.bond_reserves,
                    longs_outstanding: present_state.info.longs_outstanding,
                    share_adjustment: present_state.info.share_adjustment,
                    time_stretch: present_state.config.time_stretch,
                    vault_share_price: present_state.info.vault_share_price,
                    initial_vault_share_price: present_state.config.initial_vault_share_price,
                    minimum_share_reserves: present_state.config.minimum_share_reserves,
                    minimum_transaction_amount: present_state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: present_state.shorts_outstanding().into(),
                },
                starting_present_value: starting_present_value.into(),
                active_lp_total_supply: active_lp_total_supply.into(),
                withdrawal_shares_total_supply: withdrawal_shares_total_supply.into(),
                idle: present_state.calculate_idle_share_reserves().into(),
                net_curve_trade: net_curve_trade,
                original_share_reserves: present_state.share_reserves().into(),
                original_share_adjustment: present_state.share_adjustment(),
                original_bond_reserves: present_state.bond_reserves().into(),
            };

            // Make the solidity call and compare to the Rust implementation.
            match mock
                .calculate_distribute_excess_idle_withdrawal_shares_redeemed(
                    params,
                    share_reserves_delta.into(),
                )
                .call()
                .await
            {
                Ok(expected) => {
                    let sol_withdrawal_shares_redeemed = expected;
                    let rust_withdrawal_shares_redeemed = U256::from(actual?);
                    assert_eq!(
                        sol_withdrawal_shares_redeemed,
                        rust_withdrawal_shares_redeemed,
                    );
                }
                Err(_) => {
                    assert!(actual.is_err())
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_distribute_excess_idle_share_proceeds_net_long_edge_case() -> Result<()>
    {
        let chain = TestChain::new().await?;
        let alice = chain.alice().await?;
        let mock = chain.mock_lp_math();

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();

        for _ in 0..*FAST_FUZZ_RUNS {
            // Generate random states.
            let original_state = rng.gen::<State>();
            let mut present_state = rng.gen::<State>();

            // Generate random variables.
            let net_curve_trade = rng.gen_range(fixed_i256!(0)..=fixed!(1e24)).raw(); // 1 million

            // active_lp_total_supply and _withdrawal_shares_total_supply are just the token supplies.
            // Neither are supplied in the PoolInfo so we need to make them here.  lp_total_supply is defined as:
            // lp_total_supply = active_lp_total_supply + withdrawal_shares_total_supply - withdrawal_shares_ready_to_withdraw
            // active_lp_total_supply = lp_total_supply - withdrawal_shares_total_supply + withdrawal_shares_ready_to_withdraw
            // We clip withdrawal_shares_total_supply to ensure active_lp_total_supply doesn't underflow.
            let active_lp_total_supply = present_state.lp_total_supply()
                + present_state.withdrawal_shares_ready_to_withdraw();
            let withdrawal_shares_total_supply = rng.gen_range(fixed!(0)..=active_lp_total_supply);
            let active_lp_total_supply = active_lp_total_supply - withdrawal_shares_total_supply;

            // Make sure maturity times are in the future.
            let current_block_timestamp = alice.now().await?;
            present_state.info.long_average_maturity_time += current_block_timestamp;
            present_state.info.short_average_maturity_time += current_block_timestamp;

            // This errors out a lot so we need to catch that here.
            let starting_present_value =
                match original_state.calculate_present_value(U256::from(current_block_timestamp)) {
                    Ok(result) => result,
                    Err(_) => continue,
                };

            // Calculate idle capital.
            let idle = present_state.calculate_idle_share_reserves();

            // Calculate the result from the Rust implementation.
            let actual = present_state
                .calculate_distribute_excess_idle_share_proceeds_net_long_edge_case_safe(
                    current_block_timestamp,
                    original_state.share_adjustment(),
                    original_state.share_reserves(),
                    starting_present_value,
                    active_lp_total_supply,
                    withdrawal_shares_total_supply,
                );

            let long_average_time_remaining = present_state
                .calculate_scaled_normalized_time_remaining(
                    present_state.long_average_maturity_time(),
                    current_block_timestamp,
                );
            let short_average_time_remaining = present_state
                .calculate_scaled_normalized_time_remaining(
                    present_state.short_average_maturity_time(),
                    current_block_timestamp,
                );
            // Gather the parameters for the solidity call.  There are a lot
            // that aren't actually used, but the solidity call needs them.
            let params = DistributeExcessIdleParams {
                present_value_params: PresentValueParams {
                    share_reserves: present_state.info.share_reserves,
                    bond_reserves: present_state.info.bond_reserves,
                    longs_outstanding: present_state.info.longs_outstanding,
                    share_adjustment: present_state.info.share_adjustment,
                    time_stretch: present_state.config.time_stretch,
                    vault_share_price: present_state.info.vault_share_price,
                    initial_vault_share_price: present_state.config.initial_vault_share_price,
                    minimum_share_reserves: present_state.config.minimum_share_reserves,
                    minimum_transaction_amount: present_state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: present_state.shorts_outstanding().into(),
                },
                starting_present_value: starting_present_value.into(),
                active_lp_total_supply: active_lp_total_supply.into(),
                withdrawal_shares_total_supply: withdrawal_shares_total_supply.into(),
                idle: idle.into(),
                net_curve_trade: net_curve_trade,
                original_share_reserves: original_state.share_reserves().into(),
                original_share_adjustment: original_state.share_adjustment(),
                original_bond_reserves: original_state.bond_reserves().into(),
            };

            // Make the solidity call and compare to the Rust implementation.
            match mock
                .calculate_distribute_excess_idle_share_proceeds_net_long_edge_case_safe(params)
                .call()
                .await
            {
                Ok(expected) => {
                    let (expected_result, expected_success) = expected;
                    let (actual_result, actual_success) = actual?;
                    assert_eq!(actual_result, FixedPoint::from(expected_result));
                    assert_eq!(actual_success, expected_success);
                }
                Err(_) => {
                    assert!(actual.is_err())
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_should_short_circuit_distribute_excess_idle_share_proceeds() -> Result<()> {
        let chain = TestChain::new().await?;
        let mock = chain.mock_lp_math();

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();

        for _ in 0..*FAST_FUZZ_RUNS {
            // Generate random states.
            let present_state = rng.gen::<State>();

            let starting_present_value = rng.gen_range(fixed!(0)..=fixed!(1e24));
            let present_value = rng.gen_range(fixed!(0)..=fixed!(1e24));
            let active_lp_total_supply = rng.gen_range(fixed!(0)..=fixed!(1e24));
            let lp_total_supply = rng.gen_range(fixed!(0)..=fixed!(1e24));

            // Calculate the result from the Rust implementation.
            let actual = present_state.should_short_circuit_distribute_excess_idle_share_proceeds(
                active_lp_total_supply,
                starting_present_value,
                lp_total_supply,
                present_value,
            );

            // Gather the parameters for the solidity call.
            let mut params = DistributeExcessIdleParams::default();
            params.active_lp_total_supply = active_lp_total_supply.into();
            params.starting_present_value = starting_present_value.into();

            // Make the solidity call and compare to the Rust implementation.
            match mock
                .should_short_circuit_distribute_excess_idle_share_proceeds(
                    params,
                    lp_total_supply.into(),
                    present_value.into(),
                )
                .call()
                .await
            {
                Ok(expected) => {
                    if expected == true {
                        assert!(actual? == true);
                    } else {
                        assert!(actual? == false);
                    }
                }
                Err(_) => {
                    assert!(actual.is_err())
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_shares_delta_given_bonds_delta_derivative() -> Result<()> {
        let chain = TestChain::new().await?;
        let mock = chain.mock_lp_math();

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();

        for _ in 0..*FAST_FUZZ_RUNS {
            // Generate random states.
            let original_state = rng.gen::<State>();
            let present_state = rng.gen::<State>();

            // Get the bond amount, which in this case is equal to the net_curve_trade.
            let bond_amount = rng.gen_range(fixed_i256!(0)..=fixed!(1e24)).raw(); // 1 million

            // Maturity time goes from 0 to position duration, so we'll just set
            // this to zero to make the math simpler.
            let current_block_timestamp = fixed!(0);

            // Calcuulate the result from the Rust implementation.
            let actual = present_state.calculate_shares_delta_given_bonds_delta_derivative(
                bond_amount,
                original_state.share_reserves(),
                original_state.bond_reserves(),
                original_state.effective_share_reserves()?,
                original_state.share_adjustment(),
            );

            // This errors out a lot so we need to catch that here.
            let starting_present_value_result =
                original_state.calculate_present_value(U256::from(current_block_timestamp));
            if starting_present_value_result.is_err() {
                continue;
            }
            let starting_present_value = starting_present_value_result?;
            let idle = present_state.calculate_idle_share_reserves();

            // Gather the parameters for the solidity call.  There are a lot
            // that aren't actually used, but the solidity call needs them.
            let params = DistributeExcessIdleParams {
                present_value_params: PresentValueParams {
                    share_reserves: present_state.info.share_reserves,
                    bond_reserves: present_state.info.bond_reserves,
                    longs_outstanding: present_state.info.longs_outstanding,
                    share_adjustment: present_state.info.share_adjustment,
                    time_stretch: present_state.config.time_stretch,
                    vault_share_price: present_state.info.vault_share_price,
                    initial_vault_share_price: present_state.config.initial_vault_share_price,
                    minimum_share_reserves: present_state.config.minimum_share_reserves,
                    minimum_transaction_amount: present_state.config.minimum_transaction_amount,
                    long_average_time_remaining: present_state
                        .calculate_normalized_time_remaining(
                            present_state.long_average_maturity_time().into(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    short_average_time_remaining: present_state
                        .calculate_normalized_time_remaining(
                            present_state.short_average_maturity_time().into(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    shorts_outstanding: present_state.shorts_outstanding().into(),
                },
                starting_present_value: starting_present_value.into(),
                active_lp_total_supply: original_state.lp_total_supply().into(),
                withdrawal_shares_total_supply: uint256!(0),
                idle: idle.into(),
                net_curve_trade: bond_amount,
                original_share_reserves: original_state.share_reserves().into(),
                original_share_adjustment: original_state.share_adjustment(),
                original_bond_reserves: original_state.bond_reserves().into(),
            };

            // Make the solidity call and compare to the Rust implementation.
            match mock
                .calculate_shares_delta_given_bonds_delta_derivative_safe(
                    params,
                    U256::from(original_state.effective_share_reserves()?),
                    bond_amount,
                )
                .call()
                .await
            {
                Ok(expected) => {
                    let (result, success) = expected;
                    if !success && result == uint256!(0) {
                        assert!(actual.is_err());
                    } else if success && result == uint256!(0) {
                        assert_eq!(actual?, fixed!(0));
                    } else {
                        assert_eq!(actual?, FixedPoint::from(expected.0));
                    }
                }
                Err(_) => {
                    assert!(actual.is_err())
                }
            }
        }
        Ok(())
    }
}
