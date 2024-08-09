use ethers::types::{I256, U256};
use eyre::Result;
use fixedpointmath::fixed;

use crate::State;

impl State {
    // Helper method that calculates the average time remaining for the longs
    // and shorts and the net curve trade.
    pub fn calculate_net_curve_trade_from_timestamp(
        &self,
        current_block_timestamp: U256,
    ) -> Result<I256> {
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

        self.calculate_net_curve_trade(long_average_time_remaining, short_average_time_remaining)
    }

    /// Calculates the result of closing the net flat position.
    pub fn calculate_net_flat_trade_from_timestamp(
        &self,
        current_block_timestamp: U256,
    ) -> Result<I256> {
        let long_average_time_remaining = self.calculate_scaled_normalized_time_remaining(
            self.long_average_maturity_time(),
            current_block_timestamp,
        );
        let short_average_time_remaining = self.calculate_scaled_normalized_time_remaining(
            self.short_average_maturity_time(),
            current_block_timestamp,
        );

        self.calculate_net_flat_trade(long_average_time_remaining, short_average_time_remaining)
    }

    /// Gets the resulting state when updating liquidity.
    pub fn get_state_after_liquidity_update(&self, share_reserves_delta: I256) -> Result<State> {
        let share_reserves = self.share_reserves();
        let share_adjustment = self.share_adjustment();
        let bond_reserves = self.bond_reserves();
        let minimum_share_reserves = self.minimum_share_reserves();

        // Calculate new reserve and adjustment levels.
        let (updated_share_reserves, updated_share_adjustment, updated_bond_reserves) = match self
            .calculate_update_liquidity(
                share_reserves,
                share_adjustment,
                bond_reserves,
                minimum_share_reserves,
                share_reserves_delta,
            ) {
            Ok(result) => result,
            Err(_) => (fixed!(0), I256::from(0), fixed!(0)),
        };

        // Update and return the new state.
        let mut new_info = self.info.clone();
        new_info.share_reserves = U256::from(updated_share_reserves);
        new_info.share_adjustment = updated_share_adjustment;
        new_info.bond_reserves = U256::from(updated_bond_reserves);
        Ok(State {
            config: self.config.clone(),
            info: new_info,
        })
    }
}
