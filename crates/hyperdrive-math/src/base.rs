use ethers::types::U256;
use fixedpointmath::{fixed, FixedPoint};

use crate::State;

impl State {
    /// Calculates the number of share reserves that are not reserved by open
    /// positions.
    pub fn calculate_idle_share_reserves(&self) -> FixedPoint {
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
    pub fn calculate_solvency(&self) -> FixedPoint {
        self.share_reserves()
            - self.long_exposure() / self.vault_share_price()
            - self.minimum_share_reserves()
    }

    /// Calculates the number of base reserves that are not reserved by open
    /// positions.
    pub fn calculate_idle_share_reserves_in_base(&self) -> FixedPoint {
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
