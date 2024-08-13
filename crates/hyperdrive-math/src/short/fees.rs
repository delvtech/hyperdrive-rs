use ethers::types::U256;
use eyre::Result;
use fixedpointmath::{fixed, FixedPoint};

use crate::State;

impl State {
    /// Calculates the curve fee paid when opening shorts with a given bond amount.
    ///
    /// The open short curve fee, `$\Phi_{c,os}(\Delta y)$`, is paid in base and
    /// is given by:
    ///
    /// ```math
    /// \Phi_{c,os}(\Delta y) = \phi_c \cdot (1 - p) \cdot \Delta y
    /// ```
    pub fn open_short_curve_fee(&self, bond_amount: FixedPoint<U256>) -> Result<FixedPoint<U256>> {
        // NOTE: Round up to overestimate the curve fee.
        Ok(self
            .curve_fee()
            .mul_up(fixed!(1e18) - self.calculate_spot_price()?)
            .mul_up(bond_amount))
    }

    /// Calculates the governance fee paid when opening shorts with a given bond
    /// amount.
    ///
    /// The open short governance fee, `$\Phi_{g,os}(\Delta y)$`, is paid in
    /// base and is given by:
    ///
    /// ```math
    /// \Phi_{g,os}(\Delta y) = \phi_g \cdot \Phi_{c,os}(\Delta y)
    /// ```
    pub fn open_short_governance_fee(
        &self,
        bond_amount: FixedPoint<U256>,
        maybe_curve_fee: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let curve_fee = match maybe_curve_fee {
            Some(maybe_curve_fee) => maybe_curve_fee,
            None => self.open_short_curve_fee(bond_amount)?,
        };
        // NOTE: Round down to underestimate the governance fee.
        Ok(curve_fee.mul_down(self.governance_lp_fee()))
    }

    /// Calculates the curve fee paid when opening shorts with a given bond
    /// amount.
    ///
    /// The close short curve fee, `$\Phi_{c,cs}(\Delta y)$`, is paid in shares
    /// and is given by:
    ///
    /// ```math
    /// \Phi_{c,cs}(\Delta y) = \frac{\phi_c \cdot (1-p) \cdot \Delta y \cdot t}{c}
    /// ```
    ///
    /// where $t$ is the normalized time remaining until bond maturity.
    pub fn close_short_curve_fee(
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

    /// Calculate the governance fee paid when closing shorts with a given bond
    /// amount.
    ///
    /// The close short governance fee, `$\Phi_{g,cs}(\Delta y)$`, is paid in
    /// shares and is given by:
    ///
    /// ```math
    /// \Phi_{g,cs}(\Delta y) = \Phi_{c,cs}(\Delta y) * \phi_g
    /// ```
    ///
    /// NOTE: Round down to underestimate the governance curve fee
    pub fn close_short_governance_fee(
        &self,
        bond_amount: FixedPoint<U256>,
        maturity_time: U256,
        current_time: U256,
        maybe_curve_fee: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let curve_fee = match maybe_curve_fee {
            Some(maybe_curve_fee) => maybe_curve_fee,
            None => self.close_short_curve_fee(bond_amount, maturity_time, current_time)?,
        };
        // NOTE: Round down to underestimate the governance fee.
        Ok(curve_fee.mul_down(self.governance_lp_fee()))
    }

    /// Calculate the flat fee paid when closing shorts with a given bond
    /// amount.
    ///
    /// The close short flat fee, `$\Phi_{f,cs}(\Delta y)$`, is paid in shares
    /// and is given by:
    ///
    /// ```math
    /// \Phi_{f,cs}(\Delta y) = \frac{\Delta y \cdot (1 - t) \cdot \phi_f}{c}
    /// ```
    ///
    /// where `$t$` is the normalized time remaining until bond maturity.
    pub fn close_short_flat_fee(
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
