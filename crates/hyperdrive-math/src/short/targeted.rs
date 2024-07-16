use ethers::types::I256;
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::State;

impl State {
    /// Gets a target short that can be opened given a budget to achieve a
    /// desired fixed rate.
    ///
    /// If the short amount to reach the target is greater than the budget,
    /// the budget is returned. If the short amount to reach the target is
    /// invalid (i.e. it would produce an insolvent pool), then an error is
    /// thrown, and the user is advised to use
    /// [calculate_max_short](State::calculate_max_short).
    ///
    pub fn calculate_targeted_short_with_budget<
        F1: Into<FixedPoint>,
        F2: Into<FixedPoint>,
        F3: Into<FixedPoint>,
        F4: Into<FixedPoint>,
        I: Into<I256>,
    >(
        &self,
        budget: F1,
        target_rate: F2,
        open_vault_share_price: F3,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<F4>,
    ) -> Result<FixedPoint> {
        // TODO: use budget
        // We need to do Newton's method again here to iteratively find the bond_amount input to
        // calc_open_short such that the base_amount required is <= budget.
        self.calculate_targeted_short(
            target_rate,
            open_vault_share_price,
            checkpoint_exposure,
            maybe_max_iterations,
            maybe_allowable_error,
        )
    }

    /// Gets a target short that can be opened to achieve a desired fixed rate.
    fn calculate_targeted_short<
        F1: Into<FixedPoint>,
        F2: Into<FixedPoint>,
        F3: Into<FixedPoint>,
        I: Into<I256>,
    >(
        &self,
        target_rate: F1,
        open_vault_share_price: F2,
        checkpoint_exposure: I,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<F3>,
    ) -> Result<FixedPoint> {
        // Check input args.
        let target_rate = target_rate.into();
        let checkpoint_exposure = checkpoint_exposure.into();
        let open_vault_share_price = open_vault_share_price.into();
        let allowable_error = match maybe_allowable_error {
            Some(allowable_error) => allowable_error.into(),
            None => fixed!(1e14),
        };
        let current_rate = self.calculate_spot_rate()?;
        if target_rate <= current_rate {
            return Err(eyre!(
                "target_rate = {} argument must be greater than the current_rate = {} for a targeted short.",
                target_rate, current_rate,
            ));
        }
        Ok(fixed!(0))
    }

    /// Calculate the base & bond deltas for a short trade that moves the
    /// current state to the given desired ending reserve levels.
    ///
    /// Given a target ending pool share reserves, `$z_t$`, and bond reserves,
    /// `$y_t$`, the trade deltas to achieve that state would be:
    ///
    /// ```math
    /// \Delta y = y_t - y \\
    /// \Delta x = c \cdot \left(
    ///   z_{e,0} - z_t - (\Phi_c(\Delta y) - \Phi_g(\Delta y))
    /// \right)
    /// ```
    ///
    /// where `$c$` is the vault share price, `$\Phi_c(\Delta y)$` is the
    /// (open_short_curve_fee)[State::open_short_curve_fee], and
    /// `$\Phi_g(\Delta y)$` is the
    /// (open_short_governance_fee)[State::open_short_governance_fee].
    fn short_trade_deltas_from_reserves(
        &self,
        ending_share_reserves: FixedPoint,
        ending_bond_reserves: FixedPoint,
    ) -> Result<(FixedPoint, FixedPoint)> {
        if ending_bond_reserves < self.bond_reserves() {
            return Err(eyre!(
                "Expected ending_bond_reserves={} >= bond_reserves={} for a short trade.",
                ending_bond_reserves,
                self.bond_reserves()
            ));
        }
        let bond_delta = ending_bond_reserves - self.bond_reserves();
        let curve_fee_base = self.open_short_curve_fee(bond_delta)?;
        let curve_fee_shares = curve_fee_base.div_up(self.vault_share_price());
        let gov_curve_fee_shares = self
            .open_short_governance_fee(bond_delta, Some(curve_fee_base))?
            .div_up(self.vault_share_price());
        let fees = curve_fee_shares - gov_curve_fee_shares;
        let shares_delta = self.effective_share_reserves()? - ending_share_reserves - fees;
        let base_delta = self.vault_share_price() * shares_delta;
        Ok((base_delta, bond_delta))
    }
}

#[cfg(test)]
mod tests {
    use hyperdrive_test_utils::constants::FAST_FUZZ_RUNS;
    use rand::{thread_rng, Rng};

    use super::*;

    #[tokio::test]
    async fn fuzz_short_trade_deltas_from_reserves() -> Result<()> {
        let test_tolerance = fixed!(1e7);
        let mut rng = thread_rng();
        let mut counter = 0;
        for _ in 0..*FAST_FUZZ_RUNS {
            // Get the reserves before the short.
            let old_state = rng.gen::<State>();
            let base_reserves = old_state.vault_share_price() * old_state.share_reserves();
            let bond_reserves = old_state.bond_reserves();
            // Update the state with a random short.
            let max_short = old_state.calculate_absolute_max_short(
                old_state.calculate_spot_price()?,
                I256::from(0),
                None,
            )?;
            let bond_amount = rng.gen_range(old_state.minimum_transaction_amount()..=max_short);
            let new_state = old_state.calculate_pool_state_after_open_short(bond_amount, None)?;
            // Get the reserves after the short.
            let target_share_reserves = new_state.share_reserves();
            let target_bond_reserves = new_state.bond_reserves();
            // Compute new state deltas.
            let base_delta =
                (new_state.vault_share_price() * target_share_reserves) - base_reserves;
            let bond_delta = bond_reserves - target_bond_reserves;
            // Calculate the old state deltas to achieve the short.
            let (target_base_delta, target_bond_delta) = old_state
                .short_trade_deltas_from_reserves(target_share_reserves, target_bond_reserves)?;
            // By what amount does the resulting deltas differ from the target?
            let base_error = if base_delta > target_base_delta {
                base_delta - target_base_delta
            } else {
                target_base_delta - base_delta
            };
            assert!(
                base_error <= test_tolerance,
                "expected abs(base_delta-target_base_delta)={} <= test_tolerance={}",
                base_error,
                test_tolerance
            );
            let bond_error = if bond_delta > target_bond_delta {
                bond_delta - target_bond_delta
            } else {
                target_bond_delta - bond_delta
            };
            assert!(
                bond_error <= test_tolerance,
                "expected abs(bond_delta-target_bond_delta)={} <= test_tolerance={}",
                bond_error,
                test_tolerance
            );
            counter += 1;
        }
        assert!(counter >= 1_000); // this passed at least 1,000 times
        Ok(())
    }
}
