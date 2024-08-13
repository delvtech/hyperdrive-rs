use ethers::types::U256;
use eyre::Result;
use fixedpointmath::{fixed, FixedPoint};

use crate::State;

impl State {
    /// Calculates the curve fee paid when opening longs with a given base amount.
    ///
    /// The open long curve fee, `$\Phi_{c,ol}(\Delta x)$`, is paid in bonds and
    /// is given by:
    ///
    /// ```math
    /// \Phi_{c,ol}(\Delta x) = \phi_c
    /// \cdot \left( \tfrac{1}{p} - 1 \right) \cdot \Delta x
    /// ```
    pub fn open_long_curve_fee(&self, base_amount: FixedPoint<U256>) -> Result<FixedPoint<U256>> {
        // NOTE: Round up to overestimate the curve fee.
        Ok(self
            .curve_fee()
            .mul_up(fixed!(1e18).div_up(self.calculate_spot_price()?) - fixed!(1e18))
            .mul_up(base_amount))
    }

    /// Calculates the governance fee paid when opening longs with a given base
    /// amount.
    ///
    /// The open long governance fee, `$\Phi_{g,ol}(\Delta x)$`, is paid in base
    /// and is given by:
    ///
    /// ```math
    /// \Phi_{g,ol}(\Delta x) = \phi_g \cdot p \cdot \Phi_{c,ol}(\Delta x)
    /// ```
    pub fn open_long_governance_fee(
        &self,
        base_amount: FixedPoint<U256>,
        maybe_curve_fee: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let curve_fee = match maybe_curve_fee {
            Some(maybe_curve_fee) => maybe_curve_fee,
            None => self.open_long_curve_fee(base_amount)?,
        };
        // NOTE: Round down to underestimate the governance curve fee.
        Ok(curve_fee
            .mul_down(self.governance_lp_fee())
            .mul_down(self.calculate_spot_price()?))
    }

    /// Calculates the curve fee paid when closing longs for a given bond
    /// amount.
    ///
    /// The the close long curve fee, `$\Phi_{c,cl}(\Delta y)$`, is paid in
    /// shares and is given by:
    ///
    /// ```math
    /// \Phi_{c,cl}(\Delta y) =
    /// \frac{\phi_c \cdot (1 - p) \cdot \Delta y \cdot t}{c}
    /// ```
    ///
    /// where `$t$` is the normalized time remaining until bond maturity.
    pub fn close_long_curve_fee(
        &self,
        bond_amount: FixedPoint<U256>,
        maturity_time: U256,
        current_time: U256,
    ) -> Result<FixedPoint<U256>> {
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);
        // NOTE: Round up to overestimate the curve fee.
        Ok(self
            .curve_fee()
            .mul_up(fixed!(1e18) - self.calculate_spot_price()?)
            .mul_up(bond_amount)
            .mul_div_up(normalized_time_remaining, self.vault_share_price()))
    }

    /// Calculates the flat fee paid when closing longs for a given bond amount.
    ///
    /// The close long flat fee, `$\Phi_{f,cl}(\Delta y)$`, is paid in shares
    /// and is given by:
    ///
    /// ```math
    /// \Phi_{f,cl}(\Delta y) = \frac{\Delta y \cdot (1 - t) \cdot \phi_f)}{c}
    /// ```
    ///
    /// where `$t$` is the normalized time remaining until bond maturity.
    pub fn close_long_flat_fee(
        &self,
        bond_amount: FixedPoint<U256>,
        maturity_time: U256,
        current_time: U256,
    ) -> FixedPoint<U256> {
        let normalized_time_remaining =
            self.calculate_normalized_time_remaining(maturity_time, current_time);
        // NOTE: Round up to overestimate the flat fee.
        bond_amount
            .mul_div_up(
                fixed!(1e18) - normalized_time_remaining,
                self.vault_share_price(),
            )
            .mul_up(self.flat_fee())
    }
}
