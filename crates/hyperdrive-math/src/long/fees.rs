use ethers::types::U256;
use fixed_point::FixedPoint;
use fixed_point_macros::fixed;

use crate::State;

impl State {
    /// Calculates the curve fee paid when opening longs with a given base amount.
    ///
    /// The open long curve fee, $\Phi_{c,ol}(\Delta x)$, is paid in bonds and
    /// is given by:
    ///
    /// $$
    /// \Phi_{c,ol}(\Delta x) = \phi_c \cdot \left( \tfrac{1}{p} - 1 \right) \cdot \Delta x
    /// $$
    pub fn open_long_curve_fee(&self, base_amount: FixedPoint) -> FixedPoint {
        self.curve_fee()
            * ((fixed!(1e18) / self.calculate_spot_price()) - fixed!(1e18))
            * base_amount
    }

    /// Calculates the governance fee paid when opening longs with a given base amount.
    ///
    /// The open long governance fee, $\Phi_{g,ol}(\Delta x)$, is paid in base and
    /// is given by:
    ///
    /// $$
    /// \Phi_{g,ol}(\Delta x) = \phi_g \cdot p \cdot \Phi_{c,ol}(\Delta x)
    /// $$
    pub fn open_long_governance_fee(&self, base_amount: FixedPoint) -> FixedPoint {
        self.governance_lp_fee()
            * self.calculate_spot_price()
            * self.open_long_curve_fee(base_amount)
    }

    /// Calculates the curve fee paid when closing longs for a given bond amount.
    ///
    /// The the close long curve fee, $\Phi_{c,cl}(\Delta y)$, is paid in shares and
    /// is given by:
    ///
    /// $$
    /// \Phi_{c,cl}(\Delta y) = \frac{\phi_c \cdot (1 - p) \cdot \Delta y \cdot t}{c}
    /// $$
    ///
    /// where $t$ is the normalized time remaining until bond maturity.
    pub fn close_long_curve_fee(
        &self,
        bond_amount: FixedPoint,
        maturity_time: U256,
        current_time: U256,
    ) -> FixedPoint {
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);
        self.curve_fee()
            * (fixed!(1e18) - self.calculate_spot_price())
            * bond_amount.mul_div_down(normalized_time_remaining, self.vault_share_price())
    }

    /// Calculates the flat fee paid when closing longs for a given bond amount.
    ///
    /// The close long flat fee, $\Phi_{f,cl}(\Delta y)$, is paid in shares and
    /// is given by:
    ///
    /// $$
    /// \Phi_{f,cl}(\Delta y) = \frac{\Delta y \cdot (1 - t) \cdot \phi_f)}{c}
    /// $$
    ///
    /// where $t$ is the normalized time remaining until bond maturity.
    pub fn close_long_flat_fee(
        &self,
        bond_amount: FixedPoint,
        maturity_time: U256,
        current_time: U256,
    ) -> FixedPoint {
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);
        bond_amount.mul_div_down(
            fixed!(1e18) - normalized_time_remaining,
            self.vault_share_price(),
        ) * self.flat_fee()
    }
}
