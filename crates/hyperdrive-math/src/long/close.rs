use ethers::types::U256;
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::{State, YieldSpace};

impl State {
    /// Calculates the amount of shares the trader will receive after fees for closing a long
    pub fn calculate_close_long<F: Into<FixedPoint<U256>>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint<U256>> {
        let bond_amount = bond_amount.into();

        if bond_amount < self.config.minimum_transaction_amount.into() {
            return Err(eyre!("MinimumTransactionAmount: Input amount too low"));
        }

        // Calculate the long proceeds and fees
        let long_proceeds =
            self.calculate_close_long_flat_plus_curve(bond_amount, maturity_time, current_time)?;
        let long_curve_fee = self.close_long_curve_fee(bond_amount, maturity_time, current_time)?;
        let long_flat_fee = self.close_long_flat_fee(bond_amount, maturity_time, current_time);

        // Catch the underflow caused by the fees exceeding the proceeds
        if long_proceeds < (long_curve_fee + long_flat_fee) {
            return Err(eyre!(
                "Closing the long results in fees exceeding the long proceeds."
            ));
        }

        // Subtract the fees from the trade
        Ok(long_proceeds - long_curve_fee - long_flat_fee)
    }

    /// Calculate the amount of shares returned when selling bonds without considering fees.
    fn calculate_close_long_flat_plus_curve<F: Into<FixedPoint<U256>>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint<U256>> {
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

    /// Calculates the amount of shares the trader will receive after fees for closing a long
    /// assuming no slippage, market impact, or liquidity constraints. This is the spot valuation.
    ///
    /// To get this value, we use the same calculations as `calculate_close_long`, except
    /// for the curve part of the trade, where we replace `calculate_shares_out_given_bonds_in`
    /// for the following:
    ///
    /// `$\text{curve} = \tfrac{\Delta y}{c} \cdot p \cdot t$`
    ///
    /// `$\Delta y = \text{bond_amount}$`
    /// `$c = \text{close_vault_share_price (current if non-matured)}$`
    pub fn calculate_market_value_long<F: Into<FixedPoint<U256>>>(
        &self,
        bond_amount: F,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint<U256>> {
        let bond_amount = bond_amount.into();

        let spot_price = self.calculate_spot_price()?;
        if spot_price > fixed!(1e18) {
            return Err(eyre!("Negative fixed interest!"));
        }

        // get the time remaining
        let time_remaining = self.calculate_normalized_time_remaining(maturity_time, current_time);

        // let flat_value = bond_amount * (fixed!(1e18) - time_remaining);
        let flat_value =
            bond_amount.mul_div_down(fixed!(1e18) - time_remaining, self.vault_share_price());
        let curve_bonds = bond_amount * time_remaining;
        let curve_value = curve_bonds * spot_price / self.vault_share_price();

        let trading_proceeds = flat_value + curve_value;
        let flat_fees_paid = self.close_long_flat_fee(bond_amount, maturity_time, current_time);
        let curve_fees_paid =
            self.close_long_curve_fee(bond_amount, maturity_time, current_time)?;
        let fees_paid = flat_fees_paid + curve_fees_paid;

        if fees_paid > trading_proceeds {
            Ok(fixed!(0))
        } else {
            Ok(trading_proceeds - flat_fees_paid - curve_fees_paid)
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers::types::I256;
    use fixedpointmath::int256;
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

    // Tests market valuation against hyperdrive valuation when closing a long.
    // This function aims to give an estimated position value without considering
    // slippage, market impact, or any other liquidity constraints.
    #[tokio::test]
    async fn test_calculate_market_value_long() -> Result<()> {
        let tolerance = int256!(1e12); // 0.000001

        // Fuzz the spot valuation and hyperdrive valuation against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let mut scaled_tolerance = tolerance;

            let state = rng.gen::<State>();
            let bond_amount = state.minimum_transaction_amount();
            let maturity_time = U256::try_from(state.position_duration())?;
            let current_time = rng.gen_range(fixed!(0)..=FixedPoint::from(maturity_time));

            // Ensure curve_fee is smaller than spot_price to avoid overflows
            // on the hyperdrive valuation, as that'd mean having to pay a larger
            // amount of fees than the current value of the long.
            let spot_price = state.calculate_spot_price()?;
            if state.curve_fee() * (fixed!(1e18) - spot_price) > spot_price {
                continue;
            }

            // When the reserves ratio is too small, the market impact makes the error between
            // the valuations larger, so we scale the test's tolerance up to make up for it,
            // since this is meant to be an estimate that ignores liquidity constraints.
            let reserves_ratio = state.effective_share_reserves()? / state.bond_reserves();
            if reserves_ratio < fixed!(1e12) {
                scaled_tolerance *= int256!(100);
            } else if reserves_ratio < fixed!(1e14) {
                scaled_tolerance *= int256!(10);
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

            let diff = spot_valuation
                .abs_diff(hyperdrive_valuation)
                .change_type::<I256>()?
                .raw();

            assert!(
                diff < scaled_tolerance,
                r#"
    hyperdrive_valuation: {hyperdrive_valuation}
          spot_valuation: {spot_valuation}
                    diff: {diff}
               tolerance: {scaled_tolerance}
"#,
            );
        }

        Ok(())
    }
}
