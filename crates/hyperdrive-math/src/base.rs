use ethers::types::U256;
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::State;

impl State {
    /// Calculates the number of share reserves that are not reserved by open
    /// positions.
    // FIXME: This function is redundant with `calculate_solvency`.
    pub fn calculate_idle_share_reserves(&self) -> Result<FixedPoint> {
        self.calculate_solvency()
    }

    /// Calculates the pool's solvency.
    ///
    /// ```math
    /// s = z - \tfrac{\text{exposure}}{c} - z_{\text{min}}
    /// ```
    pub fn calculate_solvency(&self) -> Result<FixedPoint> {
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
    ///
    /// FIXME: Delete this function. Creating shares and base versions of the
    /// same function is redundant and not a precedent we want to set, otherwise
    /// every function becomes a candidate for both a shares and base version.
    #[deprecated(note = "Use `calculate_idle_share_reserves` and `vault_share_price` instead")]
    pub fn calculate_idle_share_reserves_in_base(&self) -> Result<FixedPoint> {
        Ok(self
            // NOTE: Round up to underestimate the pool's idle.
            .calculate_idle_share_reserves()?
            .mul_down(self.vault_share_price())?)
    }

    /// Given a scaled FixedPoint maturity time, calculate the normalized time
    /// remaining with high precision.
    pub fn calculate_scaled_normalized_time_remaining(
        &self,
        scaled_maturity_time: FixedPoint,
        current_time: U256,
    ) -> FixedPoint {
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
