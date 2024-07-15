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
}
