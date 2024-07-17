use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, int256, FixedPoint};

use crate::{State, YieldSpace};

impl State {
    /// Calculates the amount of shares the trader will receive after fees for closing a long
    pub fn calculate_close_long<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();

        if bond_amount < self.config.minimum_transaction_amount.into() {
            return Err(eyre!("MinimumTransactionAmount: Input amount too low"));
        }

        // Subtract the fees from the trade
        Ok(
            self.calculate_close_long_flat_plus_curve(bond_amount, maturity_time, current_time)?
                - self.close_long_curve_fee(bond_amount, maturity_time, current_time)?
                - self.close_long_flat_fee(bond_amount, maturity_time, current_time),
        )
    }

    /// Calculate the amount of shares returned when selling bonds without considering fees.
    fn calculate_close_long_flat_plus_curve<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);

        // Calculate the flat part of the trade
        let flat = bond_amount.mul_div_down(
            fixed!(1e18) - normalized_time_remaining,
            self.vault_share_price(),
        );

        // Calculate the curve part of the trade
        let curve = if normalized_time_remaining > fixed!(0) {
            let curve_bonds_in = bond_amount * normalized_time_remaining;
            self.calculate_shares_out_given_bonds_in_down(curve_bonds_in)?
        } else {
            fixed!(0)
        };

        Ok(flat + curve)
    }

    /// Calculates the market value in shares of a long position.
    ///
    /// market_estimate = trading_proceeds - flat_fees_paid - curve_fees_paid
    /// trading_proceeds = flat_value + curve_value
    /// ```math
    /// \begin{aligned}
    /// \text{trading_proceeds} &= \Delta y \cdot (1 - t) + \Delta y \cdot t \cdot p \\
    ///                  &= \Delta y \cdot ((1 - t) + (t \cdot p)) \\
    /// \text{flat_value} &= \Delta y \cdot (1 - t) \\
    /// \text{curve_bonds} &= \Delta y \cdot t \\
    /// \text{curve_value} &= \text{curve_bonds} \cdot p \\
    /// &= \Delta y \cdot t \cdot p \\
    /// \text{flat_fees_paid} &= \text{flat_value} \cdot \phi_f \\
    /// &= \Delta y \cdot (1 - t) \cdot \text{flat_fee} \\
    /// \text{curve_fees_paid} &= (\text{curve_bonds} - \text{curve_value}) \cdot \phi_c \\
    /// &= \Delta y \cdot t \cdot (1 - p) \cdot \phi_c \\
    /// \end{aligned}
    /// ```
    ///
    /// `$\Delta y = \text{bond_amount}$`
    /// `$p  = \text{spot_price}$`
    /// `$t  = \text{time_remaining}$`
    pub fn calculate_market_value_long<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();

        let spot_price = self.calculate_spot_price()?;
        if spot_price > fixed!(1e18) {
            return Err(eyre!("Negative fixed interest!"));
        }

        // get the time remaining
        let time_remaining = self.calculate_normalized_time_remaining(maturity_time, current_time);

        let flat_value = bond_amount * (fixed!(1e18) - time_remaining);
        let curve_bonds = bond_amount * time_remaining;
        let curve_value = curve_bonds * spot_price;

        let trading_proceeds = flat_value + curve_value;
        let flat_fees_paid = flat_value * self.config.fees.flat.into();
        // curve_fees_paid would only underflow if spot_price > 1, which is checked earlier
        let curve_fees_paid = (curve_bonds - curve_value) * self.config.fees.curve.into();
        let fees_paid = flat_fees_paid + curve_fees_paid;

        if fees_paid > trading_proceeds {
            Ok(fixed!(0))
        } else {
            Ok((trading_proceeds - flat_fees_paid - curve_fees_paid) / self.vault_share_price())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use hyperdrive_test_utils::{chain::TestChain, constants::FAST_FUZZ_RUNS};
    use rand::{thread_rng, Rng};

    use super::*;

    #[tokio::test]
    async fn fuzz_calculate_close_long_flat_plus_curve() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let in_ = rng.gen_range(fixed!(0)..=state.effective_share_reserves()?);
            let maturity_time = state.checkpoint_duration();
            let current_time = rng.gen_range(fixed!(0)..=maturity_time);
            let normalized_time_remaining = state
                .calculate_normalized_time_remaining(maturity_time.into(), current_time.into());
            let actual = state.calculate_close_long_flat_plus_curve(
                in_,
                maturity_time.into(),
                current_time.into(),
            );
            match chain
                .mock_hyperdrive_math()
                .calculate_close_long(
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
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected.2)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    // Tests close long with an amount smaller than the minimum.
    #[tokio::test]
    async fn test_close_long_min_txn_amount() -> Result<()> {
        let mut rng = thread_rng();
        let state = rng.gen::<State>();
        let result = state.calculate_close_long(
            state.config.minimum_transaction_amount - 10,
            0.into(),
            0.into(),
        );
        assert!(result.is_err());
        Ok(())
    }

    // Tests market valuation against hyperdrive valuation when closing a long
    // with the minimum transaction amount.
    #[tokio::test]
    async fn test_calculate_market_value_long() -> Result<()> {
        let tolerance = int256!(1e14);

        // Fuzz the spot valuation and hyperdrive valuation against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng
                .gen::<State>()
                .calculate_pool_state_after_add_liquidity(fixed!(1e27), true)?;
            let bond_amount = state.minimum_transaction_amount();
            let maturity_time = state.position_duration();
            let current_time = rng.gen_range(fixed!(0)..=maturity_time);

            // Ensure curve_fee is smaller than spot_price to avoid overflows
            // on the hyperdrive valuation, as that'd mean having to pay a larger
            // amount of fees than the current value of the long
            if state.curve_fee() > state.calculate_spot_price()? {
                continue;
            }

            let hyperdrive_valuation = state.calculate_close_long(
                bond_amount,
                maturity_time.into(),
                current_time.into(),
            )?;

            let spot_valuation = state.calculate_market_value_long(
                bond_amount,
                maturity_time.into(),
                current_time.into(),
            )?;

            let error = if spot_valuation > hyperdrive_valuation {
                I256::try_from(spot_valuation / hyperdrive_valuation - fixed!(1e18))?
            } else {
                -I256::try_from(fixed!(1e18) - spot_valuation / hyperdrive_valuation)?
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
