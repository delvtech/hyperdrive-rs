use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixed_point::{fixed, int256, FixedPoint};

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

    /// Calculates the market value in shares of a long position using the equation:
    /// market_estimate = trading_proceeds - flat_fees_paid - curve_fees_paid
    ///
    /// trading_proceeds = flat_value + curve_value
    ///                  = dy * (1 - t) + dy * t * p
    ///                  = dy * ((1 - t) + (t * p))
    /// flat_value       = dy * (1 - t)
    /// curve_bonds      = dy * t
    /// curve_value      = curve_bonds * p
    ///                  = dy * t * p
    /// flat_fees_paid   = flat_value * flat_fee
    ///                  = dy * (1 - t) * flat_fee
    /// curve_fees_paid  = (curve_bonds - curve_value) * curve_fee
    ///                  = dy * t * (1 - p) * curve_fee
    ///
    /// dy = bond_amount
    /// p  = spot_price
    /// t  = time_remaining
    pub fn calculate_value_long<F: Into<FixedPoint>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint> {
        let bond_amount = bond_amount.into();

        let spot_price = self.calculate_spot_price()?;

        // get the time remaining
        let time_remaining = self.calculate_normalized_time_remaining(maturity_time, current_time);

        let flat_value = bond_amount * (fixed!(1e18) - time_remaining);
        let curve_bonds = bond_amount * time_remaining;
        let curve_value = curve_bonds * spot_price;

        let trading_proceeds = flat_value + curve_value;
        let flat_fees_paid = flat_value * self.config.fees.flat.into();
        let curve_fees_paid = (curve_bonds - curve_value) * self.config.fees.curve.into();

        Ok(trading_proceeds - flat_fees_paid - curve_fees_paid)
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

    // Tests market valuation against yield space valuation when closing a long
    // with the minimum transaction amount.
    #[tokio::test]
    async fn test_calculate_value_long() -> Result<()> {
        let tolerance = int256!(1e15);

        // Fuzz the spot valuation and yield space valuation against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let bond_amount = state.minimum_transaction_amount();
            let maturity_time = state.position_duration();
            let current_time = rng.gen_range(fixed!(0)..=maturity_time);

            // Ensure curve_fee is smaller than spot_price to avoid overflows
            // on the yield space valuation
            if state.curve_fee() > state.calculate_spot_price()? {
                continue;
            }

            let yield_space_valuation = panic::catch_unwind(|| {
                state.calculate_close_long(bond_amount, maturity_time.into(), current_time.into())
            })
            .unwrap()
            .unwrap();

            let spot_valuation = state
                .calculate_value_long(bond_amount, maturity_time.into(), current_time.into())
                .unwrap()
                / state.vault_share_price();

            let error = if spot_valuation > yield_space_valuation {
                I256::try_from(spot_valuation / yield_space_valuation - fixed!(1e18))?
            } else {
                -I256::try_from(fixed!(1e18) - spot_valuation / yield_space_valuation)?
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
