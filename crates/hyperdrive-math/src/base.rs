use ethers::types::U256;
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::State;

impl State {
    /// Calculates the number of share reserves that are not reserved by open
    /// positions.
    pub fn calculate_idle_share_reserves(&self) -> FixedPoint<U256> {
        let long_exposure = self.long_exposure().div_up(self.vault_share_price());
        match self.share_reserves() > long_exposure + self.minimum_share_reserves() {
            true => self.share_reserves() - long_exposure - self.minimum_share_reserves(),
            false => fixed!(0),
        }
    }

    /// Calculates the pool's solvency.
    ///
    /// ```math
    /// s = z - \tfrac{\text{exposure}}{c} - z_{\text{min}}
    /// ```
    pub fn calculate_solvency(&self) -> Result<FixedPoint<U256>> {
        let share_reserves = self.share_reserves();
        let long_exposure_shares = self.long_exposure() / self.vault_share_price();
        let min_share_reserves = self.minimum_share_reserves();
        if share_reserves > long_exposure_shares + min_share_reserves {
            Ok((share_reserves - long_exposure_shares) - min_share_reserves)
        } else {
            return Err(eyre!(
                "State is insolvent. Expected share_reserves={} > long_exposure_shares={} + min_share_reserves={}",
                share_reserves, long_exposure_shares, min_share_reserves
            ));
        }
    }

    /// Calculates the number of base reserves that are not reserved by open
    /// positions.
    pub fn calculate_idle_share_reserves_in_base(&self) -> FixedPoint<U256> {
        // NOTE: Round up to underestimate the pool's idle.
        let long_exposure = self.long_exposure().div_up(self.vault_share_price());

        // Calculate the idle base reserves.
        let mut idle_shares_in_base = fixed!(0);
        if self.share_reserves() > (long_exposure + self.minimum_share_reserves()) {
            idle_shares_in_base =
                (self.share_reserves() - long_exposure - self.minimum_share_reserves())
                    * self.vault_share_price();
        }

        idle_shares_in_base
    }

    /// Given a scaled FixedPoint<U256> maturity time, calculate the normalized time
    /// remaining with high precision.
    pub fn calculate_scaled_normalized_time_remaining(
        &self,
        scaled_maturity_time: FixedPoint<U256>,
        current_time: U256,
    ) -> FixedPoint<U256> {
        let scaled_latest_checkpoint =
            FixedPoint::from(self.to_checkpoint(current_time)) * fixed!(1e36);
        let scaled_position_duration = self.position_duration() * fixed!(1e36);
        if scaled_maturity_time > scaled_latest_checkpoint {
            // NOTE: Round down to underestimate the time remaining.
            (scaled_maturity_time - scaled_latest_checkpoint).div_down(scaled_position_duration)
        } else {
            fixed!(0)
        }
    }
}
