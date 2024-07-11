use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::{State, YieldSpace};

impl State {
    fn calculate_close_short_flat<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> FixedPoint {
        // NOTE: We overestimate the trader's share payment to avoid sandwiches.
        let bond_amount = bond_amount.into();
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);
        bond_amount.mul_div_up(
            fixed!(1e18) - normalized_time_remaining,
            self.vault_share_price(),
        )
    }

    fn calculate_close_short_curve<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);
        if normalized_time_remaining > fixed!(0) {
            // NOTE: Round the `shareCurveDelta` up to overestimate the share
            // payment.
            //
            let curve_bonds_in = bond_amount.mul_up(normalized_time_remaining);
            Ok(self.calculate_shares_in_given_bonds_out_up_safe(curve_bonds_in)?)
        } else {
            Ok(fixed!(0))
        }
    }

    fn calculate_close_short_flat_plus_curve<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();
        // Calculate the flat part of the trade
        let flat = self.calculate_close_short_flat(bond_amount, maturity_time, current_time);
        // Calculate the curve part of the trade
        let curve = self.calculate_close_short_curve(bond_amount, maturity_time, current_time)?;

        Ok(flat + curve)
    }

    /// Calculates the proceeds in shares of closing a short position. This
    /// takes into account the trading profits, the interest that was
    /// earned by the short, the flat fee the short pays, and the amount of
    /// margin that was released by closing the short. The adjusted value in
    /// shares that underlies the bonds is given by:
    ///
    /// ```math
    /// P_{\text{adj}} = \left( \frac{c1}{c_0 \cdot c} + \phi_f \right)
    /// \cdot \frac{\Delta y}{c}
    /// ```
    ///
    /// and the short proceeds are given by:
    ///
    /// ```math
    /// \text{proceeds} =
    /// \begin{cases}
    ///     P_\text{adj} - dz,
    ///       & \text{if } P_{\text{adj}} > dz \\
    ///     0,              & \text{otherwise}
    /// \end{cases}
    /// ```
    ///
    /// where `$dz$` is the pool share adjustment. In the event that the
    /// interest is negative and outweighs the trading profits and margin
    /// released, the short's proceeds are marked to zero.
    pub fn calculate_short_proceeds_up(
        &self,
        bond_amount: FixedPoint,
        share_amount: FixedPoint,
        open_vault_share_price: FixedPoint,
        close_vault_share_price: FixedPoint,
    ) -> FixedPoint {
        // NOTE: Round up to overestimate the short proceeds.
        //
        // The total value is the amount of shares that underlies the bonds that
        // were shorted. The bonds start by being backed 1:1 with base, and the
        // total value takes into account all of the interest that has accrued
        // since the short was opened.
        //
        // total_value = (c1 / (c0 * c)) * dy
        let mut total_value = bond_amount
            .mul_div_up(close_vault_share_price, open_vault_share_price)
            .div_up(self.vault_share_price());

        // NOTE: Round up to overestimate the short proceeds.
        //
        // We increase the total value by the flat fee amount, because it is
        // included in the total amount of capital underlying the short.
        total_value += bond_amount.mul_div_up(self.flat_fee(), self.vault_share_price());

        // If the interest is more negative than the trading profits and margin
        // released, then the short proceeds are marked to zero. Otherwise, we
        // calculate the proceeds as the sum of the trading proceeds, the
        // interest proceeds, and the margin released.
        if total_value > share_amount {
            // proceeds = (c1 / c0 * c) * dy - dz
            total_value - share_amount
        } else {
            fixed!(0)
        }
    }

    /// Calculates the proceeds in shares of closing a short position. This
    /// takes into account the trading profits, the interest that was
    /// earned by the short, the flat fee the short pays, and the amount of
    /// margin that was released by closing the short. The adjusted value in
    /// shares that underlies the bonds is given by:
    ///
    /// ```math
    /// P_{\text{adj}} = \left( \frac{c1}{c_0 \cdot c} + \phi_f \right)
    /// \cdot \frac{\Delta y}{c}
    /// ```
    ///
    /// and the short proceeds are given by:
    ///
    /// ```math
    /// \text{proceeds} =
    /// \begin{cases}
    ///     P_\text{adj} - dz
    ///       & \text{if } P_{\text{adj}} > dz \\
    ///     0,              & \text{otherwise}
    /// \end{cases}
    /// ```
    ///
    /// where `$dz$` is the pool share adjustment. In the event that the
    /// interest is negative and outweighs the trading profits and margin
    /// released, the short's proceeds are marked to zero.
    fn calculate_short_proceeds_down(
        &self,
        bond_amount: FixedPoint,
        share_amount: FixedPoint,
        open_vault_share_price: FixedPoint,
        close_vault_share_price: FixedPoint,
    ) -> FixedPoint {
        // NOTE: Round down to underestimate the short proceeds.
        //
        // The total value is the amount of shares that underlies the bonds that
        // were shorted. The bonds start by being backed 1:1 with base, and the
        // total value takes into account all of the interest that has accrued
        // since the short was opened.
        //
        // total_value = (c1 / (c0 * c)) * dy
        let mut total_value = bond_amount
            .mul_div_down(close_vault_share_price, open_vault_share_price)
            .div_down(self.vault_share_price());

        // NOTE: Round down to underestimate the short proceeds.
        //
        // We increase the total value by the flat fee amount, because it is
        // included in the total amount of capital underlying the short.
        total_value += bond_amount.mul_div_down(self.flat_fee(), self.vault_share_price());

        // If the interest is more negative than the trading profits and margin
        // released, then the short proceeds are marked to zero. Otherwise, we
        // calculate the proceeds as the sum of the trading proceeds, the
        // interest proceeds, and the margin released.
        if total_value > share_amount {
            // proceeds = (c1 / c0 * c) * dy - dz
            total_value - share_amount
        } else {
            fixed!(0)
        }
    }

    /// Since traders pay a curve fee when they close shorts on Hyperdrive,
    /// it is possible for traders to receive a negative interest rate even
    /// if curve's spot price is less than or equal to 1.
    //
    /// Given the curve fee `$\phi_c$` and the starting spot price `$p_0$`, the
    /// maximum spot price is given by:
    ///
    /// ```math
    /// p_{\text{max}} = 1 - \phi_c \cdot (1 - p_0)
    /// ```
    fn calculate_close_short_max_spot_price(&self) -> Result<FixedPoint> {
        Ok(fixed!(1e18)
            - self
                .curve_fee()
                .mul_up(fixed!(1e18) - self.calculate_spot_price()?))
    }

    /// Calculates the amount of shares the trader will receive after fees for closing a short
    pub fn calculate_close_short<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        open_vault_share_price: F,
        close_vault_share_price: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();
        let open_vault_share_price = open_vault_share_price.into();
        let close_vault_share_price = close_vault_share_price.into();

        if bond_amount < self.config.minimum_transaction_amount.into() {
            return Err(eyre!("MinimumTransactionAmount: Input amount too low"));
        }

        // Ensure that the trader didn't purchase bonds at a negative interest
        // rate after accounting for fees.
        let share_curve_delta =
            self.calculate_close_short_curve(bond_amount, maturity_time, current_time)?;
        let bond_reserves_delta = bond_amount
            .mul_up(self.calculate_normalized_time_remaining(maturity_time, current_time));
        let short_curve_spot_price = {
            let mut state: State = self.clone();
            state.info.bond_reserves -= bond_reserves_delta.into();
            state.info.share_reserves += share_curve_delta.into();
            state.calculate_spot_price()?
        };
        let max_spot_price = self.calculate_close_short_max_spot_price()?;
        if short_curve_spot_price > max_spot_price {
            return Err(eyre!("InsufficientLiquidity: Negative Interest"));
        }

        // Ensure ending spot price is less than one
        let curve_fee = self.close_short_curve_fee(bond_amount, maturity_time, current_time)?;
        let share_curve_delta_with_fees = share_curve_delta + curve_fee
            - self.close_short_governance_fee(
                bond_amount,
                maturity_time,
                current_time,
                Some(curve_fee),
            )?;
        let share_curve_delta_with_fees_spot_price = {
            let mut state: State = self.clone();
            state.info.bond_reserves -= bond_reserves_delta.into();
            state.info.share_reserves += share_curve_delta_with_fees.into();
            state.calculate_spot_price()?
        };
        if share_curve_delta_with_fees_spot_price > fixed!(1e18) {
            return Err(eyre!("InsufficientLiquidity: Negative Interest"));
        }

        // Now calculate short proceeds
        // TODO we've already calculated a couple of internal variables needed by this function,
        // rework to avoid recalculating the curve and bond reserves
        // https://github.com/delvtech/hyperdrive/issues/943
        let share_reserves_delta =
            self.calculate_close_short_flat_plus_curve(bond_amount, maturity_time, current_time)?;
        // Calculate flat + curve and subtract the fees from the trade.
        let share_reserves_delta_with_fees = share_reserves_delta
            + self.close_short_curve_fee(bond_amount, maturity_time, current_time)?
            + self.close_short_flat_fee(bond_amount, maturity_time, current_time);

        // Calculate the share proceeds owed to the short.
        Ok(self.calculate_short_proceeds_down(
            bond_amount,
            share_reserves_delta_with_fees,
            open_vault_share_price,
            close_vault_share_price,
        ))
    }

    /// Calculates the market value of a short position.
    ///
    /// market_value = yield_accrued + trading_proceeds - curve_fees_paid + flat_fees_returned
    /// ```math
    /// \begin{aligned}
    /// \text{yield_accrued} &= \text{closing_bond_value} - \Delta y \\
    /// \text{closing_bond_value} = \Delta y \cdot \dfrac{c}{c_0} \\
    /// \text{trading_proceeds} &= \Delta y \cdot (1 - p) \cdot t \\
    /// \text{curve_fees_paid} &= \text{trading_proceeds} \cdot \phi_c \\
    /// \text{flat_fees_returned} &= \Delta y \cdot t \cdot \phi_f \\
    /// ```
    ///
    /// `$\Delta y = \text{bond_amount}$`
    /// `$c = \text{close_vault_share_price (current if non-matured)}$`
    /// `$c_0 = \text{open_vault_share_price}$`
    /// `$p = \text{spot_price}$`
    /// `$t = \text{time_remaining}$`
    pub fn calculate_market_value_short<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        open_vault_share_price: F,
        close_vault_share_price: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();
        let open_vault_share_price = open_vault_share_price.into();
        let close_vault_share_price = close_vault_share_price.into();

        let spot_price = self.calculate_spot_price()?;
        if spot_price > fixed!(1e18) {
            return Err(eyre!("Negative fixed interest!"));
        }

        // get the time remaining
        let time_remaining = self.calculate_normalized_time_remaining(maturity_time, current_time);
        // yield_accrued = closing_bond_value - bond_amount
        // closing_bond_value = dy * c1/c0
        let closing_bond_value = bond_amount * close_vault_share_price / open_vault_share_price;
        // trading_proceeds = dy * (1 - p) * t
        let trading_proceeds = bond_amount * (fixed!(1e18) - spot_price) * (time_remaining);
        // curve_fees_paid = trading_proceeds * curve_fee
        let curve_fees_paid = trading_proceeds * self.config.fees.curve.into();
        // flat_fees_returned = dy * t * flat_fee
        let flat_fees_returned = bond_amount * time_remaining * self.config.fees.flat.into();

        let total_value = closing_bond_value + trading_proceeds + flat_fees_returned;
        if total_value >= (bond_amount + curve_fees_paid) {
            // market_value = total_value - bond_amount - curve_fees_paid
            Ok(total_value - bond_amount - curve_fees_paid)
        } else {
            // If the interest is more negative than the trading proceeds and
            // the margin released, we mark short's value to 0
            Ok(fixed!(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use fixedpointmath::int256;
    use hyperdrive_test_utils::{chain::TestChain, constants::FAST_FUZZ_RUNS};
    use rand::{thread_rng, Rng};

    use super::*;

    #[tokio::test]
    async fn fuzz_sol_calculate_short_proceeds_up() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let bond_amount = rng.gen_range(fixed!(0)..=state.bond_reserves());
            let share_amount = rng.gen_range(fixed!(0)..=bond_amount);
            let open_vault_share_price = rng.gen_range(fixed!(0)..=state.vault_share_price());
            let actual = panic::catch_unwind(|| {
                state.calculate_short_proceeds_up(
                    bond_amount,
                    share_amount,
                    open_vault_share_price,
                    state.vault_share_price(),
                )
            });
            match chain
                .mock_hyperdrive_math()
                .calculate_short_proceeds_up(
                    bond_amount.into(),
                    share_amount.into(),
                    open_vault_share_price.into(),
                    state.vault_share_price().into(),
                    state.vault_share_price().into(),
                    state.flat_fee().into(),
                )
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_sol_calculate_short_proceeds_down() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let bond_amount = rng.gen_range(fixed!(0)..=state.bond_reserves());
            let share_amount = rng.gen_range(fixed!(0)..=bond_amount);
            let open_vault_share_price = rng.gen_range(fixed!(0)..=state.vault_share_price());
            let actual = panic::catch_unwind(|| {
                state.calculate_short_proceeds_down(
                    bond_amount,
                    share_amount,
                    open_vault_share_price,
                    state.vault_share_price(),
                )
            });
            match chain
                .mock_hyperdrive_math()
                .calculate_short_proceeds_down(
                    bond_amount.into(),
                    share_amount.into(),
                    open_vault_share_price.into(),
                    state.vault_share_price().into(),
                    state.vault_share_price().into(),
                    state.flat_fee().into(),
                )
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_sol_calculate_close_short_flat_plus_curve() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let in_ = rng.gen_range(fixed!(0)..=state.bond_reserves());
            let maturity_time = state.position_duration();
            let current_time = rng.gen_range(fixed!(0)..=maturity_time);
            let actual = panic::catch_unwind(|| {
                state.calculate_close_short_flat_plus_curve(
                    in_,
                    maturity_time.into(),
                    current_time.into(),
                )
            });

            let normalized_time_remaining = state
                .calculate_normalized_time_remaining(maturity_time.into(), current_time.into());
            match chain
                .mock_hyperdrive_math()
                .calculate_close_short(
                    state.effective_share_reserves()?.into(),
                    state.bond_reserves().into(),
                    in_.into(),
                    normalized_time_remaining.into(),
                    state.t().into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap().unwrap(), FixedPoint::from(expected.2)),
                Err(_) => assert!(actual.is_err() || actual.unwrap().is_err()),
            }
        }

        Ok(())
    }

    // Tests close short with an amount smaller than the minimum.
    #[tokio::test]
    async fn test_close_short_min_txn_amount() -> Result<()> {
        let mut rng = thread_rng();
        let state = rng.gen::<State>();
        let result = state.calculate_close_short(
            (state.config.minimum_transaction_amount - 10).into(),
            state.calculate_spot_price()?,
            state.vault_share_price(),
            0.into(),
            0.into(),
        );
        assert!(result.is_err());
        Ok(())
    }

    // Tests market valuation against yield space valuation when closing a short
    // with the minimum transaction amount.
    #[tokio::test]
    async fn test_calculate_market_value_short() -> Result<()> {
        let tolerance_rel = int256!(1e14); // 0.01%
        let tolerance_abs = int256!(1e12); // 0.0000
        let mut tolerance = tolerance_rel;

        // Fuzz the spot valuation and yield space valuation against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let bond_amount = state.minimum_transaction_amount();
            let open_vault_share_price = rng.gen_range(fixed!(0.5e18)..=fixed!(2.5e18));
            let maturity_time = state.position_duration();
            let current_time = rng.gen_range(fixed!(0)..=maturity_time);

            let hyperdrive_valuation = state.calculate_close_short(
                bond_amount,
                open_vault_share_price,
                state.vault_share_price(),
                maturity_time.into(),
                current_time.into(),
            )?;

            let spot_valuation = state.calculate_market_value_short(
                bond_amount,
                open_vault_share_price,
                state.vault_share_price(),
                maturity_time.into(),
                current_time.into(),
            )? / state.vault_share_price();

            let error = if spot_valuation > fixed!(0) && hyperdrive_valuation > fixed!(0) {
                tolerance = tolerance_rel;
                if spot_valuation > hyperdrive_valuation {
                    I256::try_from(spot_valuation / hyperdrive_valuation - fixed!(1e18))?
                } else {
                    -I256::try_from(fixed!(1e18) - spot_valuation / hyperdrive_valuation)?
                }
            } else {
                // at least one of them is 0, so we can't divide
                tolerance = tolerance_abs;
                I256::try_from(spot_valuation + hyperdrive_valuation)?
            };

            assert!(
                error < tolerance,
                "error {:?} exceeds tolerance of {}",
                error,
                tolerance
            );
        }

        Ok(())
    }
}
