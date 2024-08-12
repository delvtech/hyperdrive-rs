use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::{calculate_rate_given_fixed_price, State, YieldSpace};

impl State {
    /// Calculates the amount of base the trader will need to deposit for a
    /// short of a given size.
    ///
    /// For some number of bonds being shorted, `$\Delta y$`, the short deposit,
    /// `$D(\Delta y)$`, is made up of several components:
    ///
    /// - The short principal:
    ///   `$P_{\text{lp}}(\Delta y)$`
    /// - The curve fee:
    ///   `$\Phi_{c,os}(\Delta y) = \phi_{c} \cdot ( 1 - p_{0} ) \cdot \Delta y$`
    /// - The governance-curve fee:
    ///   `$\Phi_{g,os}(\Delta y) = \phi_{g} \Phi_{c,os}(\Delta y)$`
    /// - The flat fee:
    ///   `$\Phi_{f,os}(\Delta y) = \tfrac{1}{c} ( \Delta y \cdot (1 - t) \cdot \phi_{f} )$`
    /// - The total value in shares that underlies the bonds:
    ///   `$\tfrac{c_1}{c_0 \cdot c} \Delta y$`
    ///
    /// The short principal is given by:
    ///
    /// ```math
    /// P_{\text{lp}}(\Delta y) = z - \tfrac{1}{\mu} \cdot (
    ///     \tfrac{\mu}{c} \cdot (k - (y + \Delta y)^{1 - t_s})
    /// )^{\tfrac{1}{1 - t_s}}
    /// ```
    ///
    /// The adjusted value in shares that underlies the bonds is given by:
    ///
    /// ```math
    /// P_\text{adj} = \left( \frac{c_1}{c_0} + \phi_f \right) \cdot \frac{\Delta y}{c}
    /// ```
    ///
    /// And finally the short deposit in base is:
    ///
    /// ```math
    /// D(\Delta y) =
    /// \begin{cases}
    ///     P_\text{adj} - P_{\text{lp}}(\Delta y) + \Phi_{c}(\Delta y),
    ///       & \text{if } P_{\text{adj}} > P_{\text{lp}}(\Delta y) - \Phi_{c}(\Delta y) \\
    ///     0, & \text{otherwise}
    /// \end{cases}
    /// ```
    pub fn calculate_open_short(
        &self,
        bond_amount: FixedPoint<U256>,
        open_vault_share_price: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // Ensure that the bond amount is greater than or equal to the minimum
        // transaction amount.
        if bond_amount < self.minimum_transaction_amount() {
            return Err(eyre!(
                "MinimumTransactionAmount: Input amount too low. bond_amount = {:#?} must be >= {:#?}",
                bond_amount,
                self.minimum_transaction_amount()
            ));
        }

        // If the open share price hasn't been set, we use the current share
        // price, since this is what will be set as the checkpoint share price
        // in this transaction.
        let open_vault_share_price = if open_vault_share_price == fixed!(0) {
            self.vault_share_price()
        } else {
            open_vault_share_price
        };

        // Calculate the effect that opening the short will have on the pool's
        // share reserves.
        let share_reserves_delta = self.calculate_short_principal(bond_amount)?;

        // NOTE: Round up to make the check stricter.
        //
        // If the base proceeds of selling the bonds is greater than the bond
        // amount, then the trade occurred in the negative interest domain.
        // We revert in these pathological cases.
        if share_reserves_delta.mul_up(self.vault_share_price()) > bond_amount {
            return Err(eyre!(
                "InsufficientLiquidity: Negative Interest.
                expected bond_amount={} <= share_reserves_delta_in_shares={}",
                bond_amount,
                share_reserves_delta
            ));
        }

        // NOTE: Round up to overestimate the base deposit.
        //
        // The trader will need to deposit capital to pay for the fixed rate,
        // the fees, and any back-paid interest that will be received back upon
        // closing the trade.
        let curve_fee_shares = self
            .open_short_curve_fee(bond_amount)?
            .div_up(self.vault_share_price());
        if share_reserves_delta < curve_fee_shares {
            return Err(eyre!(format!(
                "The transaction curve fee = {}, computed with coefficient = {},
                is too high. It must be less than share reserves delta = {}",
                curve_fee_shares,
                self.curve_fee(),
                share_reserves_delta
            )));
        }

        // If negative interest has accrued during the current checkpoint, we
        // set the close vault share price to equal the open vault share price.
        // This ensures that shorts don't benefit from negative interest that
        // accrued during the current checkpoint.
        let close_vault_share_price = open_vault_share_price.max(self.vault_share_price());

        // Now we can calculate adjusted proceeds account for the backdated
        // vault price:
        //
        // ```math
        // \text{base_proceeds} = (
        //    \frac{c1 \cdot \Delta y}{c0 \cdot c}
        //    + \frac{\Delta y \cdot \phi_f}{c} - \Delta z
        // ) \cdot c
        // ```
        let base_proceeds = self
            .calculate_short_proceeds_up(
                bond_amount,
                share_reserves_delta - curve_fee_shares,
                open_vault_share_price,
                close_vault_share_price,
            )
            .mul_up(self.vault_share_price());

        Ok(base_proceeds)
    }

    /// Calculates the derivative of the short deposit function with respect to
    /// the short amount.
    ///
    /// This derivative allows us to use Newton's method to approximate the
    /// maximum short that a trader can open. The share adjustment derivative is
    /// a constant:
    ///
    /// ```math
    /// P^{\prime}_{\text{adj}}(\Delta y)
    /// = \tfrac{c_{1}}{c_{0} \cdot c} + \tfrac{\phi_{f}}{c}
    /// ```
    ///
    /// The curve fee dervative is given by:
    ///
    /// ```math
    /// \Phi^{\prime}_{\text{c}}(\Delta y) = \phi_{c} \cdot (1 - p_0),
    /// ```
    ///
    /// where `$p_0$` is the opening (or initial) spot price. Using these and the
    /// short principal derivative, we can calculate the open short derivative:
    ///
    /// ```math
    /// D^{\prime}(\Delta y) =
    ///\begin{cases}
    ///    c \cdot \left(
    ///      P^{\prime}_{\text{adj}}(\Delta y)
    ///      - P^{\prime}_{\text{lp}}(\Delta y)
    ///      + \Phi^{\prime}_{c,os}(\Delta y)
    ///    \right),
    ///    & \text{if }
    ///      P_{\text{adj}} > P_{\text{lp}}(\Delta y) - \Phi_{c,os}(\Delta y) \\
    ///    0, & \text{otherwise}
    ///\end{cases}
    /// ```
    pub fn calculate_open_short_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
        open_vault_share_price: FixedPoint<U256>,
        maybe_initial_spot_price: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        // We're avoiding negative interest, so we will cap the close share
        // price to be greater than or equal to the open share price. This
        // will assume the max loss for the trader, some of which may be
        // reimbursed upon closing.
        let close_vault_share_price = open_vault_share_price.max(self.vault_share_price());

        // Short circuit the derivative if the function returns 0.
        if self.calculate_short_proceeds_up(
            bond_amount,
            self.calculate_short_principal(bond_amount)?
                - self
                    .open_short_curve_fee(bond_amount)?
                    .div_up(self.vault_share_price()),
            open_vault_share_price,
            close_vault_share_price,
        ) == fixed!(0)
        {
            return Ok(fixed!(0));
        }

        let spot_price = match maybe_initial_spot_price {
            Some(spot_price) => spot_price,
            None => self.calculate_spot_price()?,
        };

        // All of these are in base.
        let share_adjustment_derivative =
            close_vault_share_price.div_up(open_vault_share_price) + self.flat_fee();
        let short_principal_derivative = self
            .calculate_short_principal_derivative(bond_amount)?
            .mul_up(self.vault_share_price());
        let curve_fee_derivative = self.curve_fee().mul_up((fixed!(1e18) - spot_price));

        // Multiply by the share price to return base.
        Ok(share_adjustment_derivative - short_principal_derivative + curve_fee_derivative)
    }

    /// Calculates the amount of short principal that the LPs need to pay to
    /// back a short before fees are taken into consideration,
    /// `$P_\text{lp}(\Delta y)$`.
    ///
    /// Let the LP principal that backs $\Delta y$ shorts be given by
    /// `$P_{\text{lp}}(\Delta y)$`. We can solve for this in terms of
    /// `$\Delta y$` using the YieldSpace invariant:
    ///
    /// ```math
    /// k = \tfrac{c}{\mu} \cdot (\mu \cdot (z - P(\Delta y)))^{1 - t_s} + (y + \Delta y)^{1 - t_s} \\
    /// \implies \\
    /// P_{\text{lp}}(\Delta y) = z - \tfrac{1}{\mu}
    /// \cdot \left(
    ///   \tfrac{\mu}{c} \cdot (k - (y + \Delta y)^{1 - t_s})
    /// \right)^{\tfrac{1}{1 - t_s}}
    /// ```
    pub fn calculate_short_principal(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        self.calculate_shares_out_given_bonds_in_down(bond_amount)
    }

    /// Calculates the derivative of the short principal w.r.t. the amount of
    /// bonds that are shorted.
    ///
    /// The derivative is:
    ///
    /// ```math
    /// P^{\prime}_{\text{lp}}(\Delta y) = \tfrac{1}{c} \cdot (y + \Delta y)^{-t_s}
    /// \cdot \left(
    ///     \tfrac{\mu}{c} \cdot (k - (y + \Delta y)^{1 - t_s})
    /// \right)^{\tfrac{t_s}{1 - t_s}}
    /// ```
    pub fn calculate_short_principal_derivative(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // Avoid negative exponent by putting the term in the denominator.
        let lhs = fixed!(1e18).div_up(
            self.vault_share_price()
                .mul_up((self.bond_reserves() + bond_amount).pow(self.time_stretch())?),
        );
        let rhs = (self
            .initial_vault_share_price()
            .div_up(self.vault_share_price())
            .mul_up(
                self.k_up()?
                    - (self.bond_reserves() + bond_amount)
                        .pow(fixed!(1e18) - self.time_stretch())?,
            ))
        .pow(
            self.time_stretch()
                .div_up(fixed!(1e18) - self.time_stretch()),
        )?;
        Ok(lhs.mul_up(rhs))
    }

    /// Calculate an updated pool state after opening a short.
    ///
    /// For a given bond amount and share amount,
    /// the reserves are updated such that
    /// `state.bond_reserves += bond_amount` and
    /// `state.share_reserves -= share_amount`.
    pub fn calculate_pool_state_after_open_short(
        &self,
        bond_amount: FixedPoint<U256>,
        maybe_share_amount: Option<FixedPoint<U256>>,
    ) -> Result<Self> {
        let share_amount = match maybe_share_amount {
            Some(share_amount) => share_amount,
            None => self.calculate_pool_share_delta_after_open_short(bond_amount)?,
        };
        let mut state = self.clone();
        state.info.bond_reserves += bond_amount.into();
        state.info.share_reserves -= share_amount.into();
        Ok(state)
    }

    /// Calculate the share delta to be applied to the pool after opening a short.
    pub fn calculate_pool_share_delta_after_open_short(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        let curve_fee_base = self.open_short_curve_fee(bond_amount)?;
        let curve_fee_shares = curve_fee_base.div_up(self.vault_share_price());
        let gov_curve_fee_shares = self
            .open_short_governance_fee(bond_amount, Some(curve_fee_base))?
            .div_up(self.vault_share_price());
        let short_principal = self.calculate_short_principal(bond_amount)?;
        if short_principal.mul_up(self.vault_share_price()) > bond_amount {
            return Err(eyre!("InsufficientLiquidity: Negative Interest"));
        }
        if short_principal < (curve_fee_shares - gov_curve_fee_shares) {
            return Err(eyre!(
                "short_principal={:#?} is too low to account for fees={:#?}",
                short_principal,
                curve_fee_shares - gov_curve_fee_shares
            ));
        }
        Ok(short_principal - (curve_fee_shares - gov_curve_fee_shares))
    }

    /// Calculates the spot price after opening a short.
    /// Arguments are deltas that would be applied to the pool.
    pub fn calculate_spot_price_after_short(
        &self,
        bond_amount: FixedPoint<U256>,
        maybe_base_amount: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let share_amount = match maybe_base_amount {
            Some(base_amount) => base_amount / self.vault_share_price(),
            None => self.calculate_pool_share_delta_after_open_short(bond_amount)?,
        };
        let updated_state =
            self.calculate_pool_state_after_open_short(bond_amount, Some(share_amount))?;
        updated_state.calculate_spot_price()
    }

    /// Calculate the spot rate after a short has been opened.
    /// If a base_amount is not provided, then one is estimated
    /// using [calculate_pool_share_delta_after_open_short](State::calculate_pool_share_delta_after_open_short).
    ///
    /// We calculate the rate for a fixed length of time as:
    ///
    /// ```math
    /// r(\Delta y) = \frac{1 - p(\Delta y)}{p(\Delta y) \cdot t}
    /// ```
    ///
    /// where `$p(\Delta y)$` is the spot price after a short for
    /// `delta_bonds` `$= \Delta y$` and `$t$` is the normalized position
    /// druation.
    pub fn calculate_spot_rate_after_short(
        &self,
        bond_amount: FixedPoint<U256>,
        maybe_base_amount: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let price = self.calculate_spot_price_after_short(bond_amount, maybe_base_amount)?;
        Ok(calculate_rate_given_fixed_price(
            price,
            self.position_duration(),
        ))
    }

    /// Calculate the implied rate of opening a short at a given size. This rate
    /// is calculated as an APY.
    ///
    /// Given the effective fixed rate the short will pay
    /// `$r_{\text{effective}}$` and the variable rate the short will receive
    /// `$r_{\text{variable}}$`, the short's implied APY,
    /// `$r_{\text{implied}}$` will be:
    ///
    /// ```math
    /// r_{\text{implied}} = \frac{r_{\text{variable}}
    /// - r_{\text{effective}}}{r_{\text{effective}}}
    /// ```
    ///
    /// We can short-cut this calculation using the amount of base the short
    /// will pay and comparing this to the amount of base the short will receive
    /// if the variable rate stays the same. The implied rate is just the ROI
    /// if the variable rate stays the same.
    ///
    /// To do this, we must figure out the term-adjusted yield `$TPY$` according
    /// to the position duration `$t$`. Since we start off from a compounded APY
    /// and also output a compounded TPY, the compounding frequency `$f$` is
    /// simplified away. Thus, the adjusted yield will be:
    ///
    /// ```math
    /// \text{APR} = f \cdot (( 1 + \text{APY})^{\tfrac{1}{f}}  - 1)
    /// ```
    ///
    /// Therefore,
    ///
    /// ```math
    /// \begin{aligned}
    /// TPY &= (1 + \frac{APR}{f})^{d \cdot f} \\
    /// &= (1 + APY)^{d} - 1
    /// \end{aligned}
    /// ```
    ///
    /// We use the TPY to figure out the base proceeds, and calculate the rate
    /// of return based on the short's opening cost. Since shorts must backpay
    /// the variable interest accrued since the last checkpoint, we subtract
    /// that from the opening cost, as they get it back upon closing the short.
    pub fn calculate_implied_rate(
        &self,
        bond_amount: FixedPoint<U256>,
        open_vault_share_price: FixedPoint<U256>,
        variable_apy: FixedPoint<U256>,
    ) -> Result<I256> {
        let full_base_paid = self.calculate_open_short(bond_amount, open_vault_share_price)?;
        let backpaid_interest = bond_amount
            .mul_div_down(self.vault_share_price(), open_vault_share_price)
            - bond_amount;
        let base_paid = full_base_paid - backpaid_interest;
        let tpy =
            (fixed!(1e18) + variable_apy).pow(self.annualized_position_duration())? - fixed!(1e18);
        let base_proceeds = bond_amount * tpy;
        if base_proceeds > base_paid {
            Ok(I256::try_from(
                (base_proceeds - base_paid) / (base_paid * self.annualized_position_duration()),
            )?)
        } else {
            Ok(-I256::try_from(
                (base_paid - base_proceeds) / (base_paid * self.annualized_position_duration()),
            )?)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use ethers::types::U256;
    use fixedpointmath::{fixed, fixed_u256, int256, FixedPointValue};
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS, SLOW_FUZZ_RUNS},
    };
    use hyperdrive_wrappers::wrappers::ihyperdrive::{Checkpoint, Options};
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::test_utils::{
        agent::HyperdriveMathAgent, preamble::initialize_pool_with_random_state,
    };

    #[tokio::test]
    async fn fuzz_calculate_pool_state_after_open_short() -> Result<()> {
        // TODO: There must be a rounding error; we should not need a tolerance.
        let share_adjustment_test_tolerance = fixed_u256!(0);
        let bond_reserves_test_tolerance = fixed!(0);
        let share_reserves_test_tolerance = fixed!(1);
        // Initialize a test chain and agents.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let mut celine = chain.celine().await?;
        // Set up a random number generator. We use ChaCha8Rng with a randomly
        // generated seed, which makes it easy to reproduce test failures given
        // the seed.
        let mut rng = {
            let mut rng = thread_rng();
            let seed = rng.gen();
            ChaCha8Rng::seed_from_u64(seed)
        };
        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain & run the preamble.
            let id = chain.snapshot().await?;
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;
            // Reset the variable rate to zero; get the state.
            alice.advance_time(fixed!(0), fixed!(0)).await?;
            let original_state = alice.get_state().await?;
            // Get a random short amount.
            let checkpoint_exposure = alice
                .get_checkpoint_exposure(original_state.to_checkpoint(alice.now().await?))
                .await?;
            let max_short_amount = original_state.calculate_max_short(
                U256::MAX,
                original_state.vault_share_price(),
                checkpoint_exposure,
                None,
                None,
            )?;
            let bond_amount =
                rng.gen_range(original_state.minimum_transaction_amount()..=max_short_amount);
            // Mock the trade using Rust.
            let rust_state =
                original_state.calculate_pool_state_after_open_short(bond_amount, None)?;
            // Execute the trade on the contracts.
            bob.fund(bond_amount * fixed!(1.5e18)).await?;
            bob.open_short(bond_amount, None, None).await?;
            let sol_state = alice.get_state().await?;
            // Check that the results are the same.
            let rust_share_adjustment = rust_state.share_adjustment();
            let sol_share_adjustment = sol_state.share_adjustment();
            let share_adjustment_error = if rust_share_adjustment < sol_share_adjustment {
                FixedPoint::try_from(sol_share_adjustment - rust_share_adjustment)?
            } else {
                FixedPoint::try_from(rust_share_adjustment - sol_share_adjustment)?
            };
            assert!(
                share_adjustment_error <= share_adjustment_test_tolerance,
                "expected abs(rust_share_adjustment={}-sol_share_adjustment={}) <= test_tolerance={}",
                rust_share_adjustment, sol_share_adjustment, share_adjustment_test_tolerance
            );
            let rust_bond_reserves = rust_state.bond_reserves();
            let sol_bond_reserves = sol_state.bond_reserves();
            let bond_reserves_error = if rust_bond_reserves < sol_bond_reserves {
                sol_bond_reserves - rust_bond_reserves
            } else {
                rust_bond_reserves - sol_bond_reserves
            };
            assert!(
                bond_reserves_error <= bond_reserves_test_tolerance,
                "expected abs(rust_bond_reserves={}-sol_bond_reserves={}) <= test_tolerance={}",
                rust_bond_reserves,
                sol_bond_reserves,
                bond_reserves_test_tolerance
            );
            let rust_share_reserves = rust_state.share_reserves();
            let sol_share_reserves = sol_state.share_reserves();
            let share_reserves_error = if rust_share_reserves < sol_share_reserves {
                sol_share_reserves - rust_share_reserves
            } else {
                rust_share_reserves - sol_share_reserves
            };
            assert!(
                share_reserves_error <= share_reserves_test_tolerance,
                "expected abs(rust_share_reserves={}-sol_share_reserves={}) <= test_tolerance={}",
                rust_share_reserves,
                sol_share_reserves,
                share_reserves_test_tolerance
            );
            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_sol_calculate_pool_share_delta_after_open_short() -> Result<()> {
        let test_tolerance = fixed!(10);

        let chain = TestChain::new().await?;
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed_u256!(0)..=fixed!(10_000_000e18));
                if rng.gen() {
                    -I256::try_from(value).unwrap()
                } else {
                    I256::try_from(value).unwrap()
                }
            };
            // We need to catch panics because of overflows.
            let max_bond_amount = match panic::catch_unwind(|| {
                state.calculate_absolute_max_short(
                    state.calculate_spot_price()?,
                    checkpoint_exposure,
                    None,
                )
            }) {
                Ok(max_bond_amount) => match max_bond_amount {
                    Ok(max_bond_amount) => max_bond_amount,
                    Err(_) => continue, // Max threw an Err.
                },
                Err(_) => continue, // Max threw a panic.
            };
            if max_bond_amount < state.minimum_transaction_amount() + fixed!(1) {
                continue;
            }
            let bond_amount = rng.gen_range(state.minimum_transaction_amount()..=max_bond_amount);
            let rust_pool_delta = state.calculate_pool_share_delta_after_open_short(bond_amount);
            let curve_fee_base = state.open_short_curve_fee(bond_amount)?;
            let gov_fee_base =
                state.open_short_governance_fee(bond_amount, Some(curve_fee_base))?;
            let fees = curve_fee_base.div_up(state.vault_share_price())
                - gov_fee_base.div_up(state.vault_share_price());
            match chain
                .mock_hyperdrive_math()
                .calculate_open_short(
                    state.effective_share_reserves()?.into(),
                    state.bond_reserves().into(),
                    bond_amount.into(),
                    state.time_stretch().into(),
                    state.vault_share_price().into(),
                    state.initial_vault_share_price().into(),
                )
                .call()
                .await
            {
                Ok(sol_pool_delta) => {
                    let sol_pool_delta_with_fees = FixedPoint::from(sol_pool_delta) - fees;
                    let rust_pool_delta_unwrapped = rust_pool_delta.unwrap();
                    let result_equal = sol_pool_delta_with_fees
                        <= rust_pool_delta_unwrapped + test_tolerance
                        && sol_pool_delta_with_fees >= rust_pool_delta_unwrapped - test_tolerance;
                    assert!(result_equal, "Should be equal.");
                }
                Err(_) => {
                    assert!(rust_pool_delta.is_err())
                }
            };
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_sol_calculate_short_principal() -> Result<()> {
        // This test is the same as the yield_space.rs `fuzz_calculate_max_buy_shares_in_safe`,
        // but is worth having around in case we ever change how we compute short principal.
        let chain = TestChain::new().await?;
        let mut rng = thread_rng();
        let state = rng.gen::<State>();
        let bond_amount = rng.gen_range(fixed!(10e18)..=fixed!(10_000_000e18));
        let actual = state.calculate_short_principal(bond_amount);
        match chain
            .mock_yield_space_math()
            .calculate_shares_out_given_bonds_in_down_safe(
                state.effective_share_reserves()?.into(),
                state.bond_reserves().into(),
                bond_amount.into(),
                (fixed!(1e18) - state.time_stretch()).into(),
                state.vault_share_price().into(),
                state.initial_vault_share_price().into(),
            )
            .call()
            .await
        {
            Ok((expected, expected_status)) => {
                assert_eq!(actual.is_ok(), expected_status);
                assert_eq!(actual.unwrap_or(fixed!(0)), expected.into());
            }
            Err(_) => assert!(actual.is_err()),
        }
        Ok(())
    }

    /// This test empirically tests `calculate_short_principal_derivative` by calling
    /// `calculate_short_principal` at two points and comparing the empirical result
    /// with the output of `calculate_short_principal_derivative`.
    #[tokio::test]
    async fn fuzz_calculate_short_principal_derivative() -> Result<()> {
        // We use a relatively large epsilon here due to the underlying fixed point pow
        // function not being monotonically increasing.
        let empirical_derivative_epsilon = fixed!(1e14);
        let test_tolerance = fixed!(1e14);

        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();

            // Min trade amount should be at least 1,000x the derivative epsilon.
            let bond_amount = rng.gen_range(fixed!(1e18)..=fixed!(10_000_000e18));

            // Calculate the function output at the bond amount and a small perturbation away.
            let f_x = match panic::catch_unwind(|| state.calculate_short_principal(bond_amount)) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };
            let f_x_plus_delta = match panic::catch_unwind(|| {
                state.calculate_short_principal(bond_amount + empirical_derivative_epsilon)
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount resulted in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };

            // Sanity check
            assert!(f_x_plus_delta > f_x);

            // Compute the empirical and analytical derivatives.
            let empirical_derivative = (f_x_plus_delta - f_x) / empirical_derivative_epsilon;
            let short_principal_derivative =
                state.calculate_short_principal_derivative(bond_amount)?;

            // Ensure that the empirical and analytical derivatives match.
            let derivative_diff = if short_principal_derivative >= empirical_derivative {
                short_principal_derivative - empirical_derivative
            } else {
                empirical_derivative - short_principal_derivative
            };
            assert!(
                derivative_diff < test_tolerance,
                "expected abs(derivative_diff={}) < test_tolerance={};
                calculated_derivative={}, emperical_derivative={}",
                derivative_diff,
                test_tolerance,
                short_principal_derivative,
                empirical_derivative
            );
        }

        Ok(())
    }

    /// This test empirically tests `calculate_open_short_derivative` by calling
    /// `calculate_open_short` at two points and comparing the empirical result
    /// with the output of `calculate_open_short_derivative`.
    #[tokio::test]
    async fn fuzz_calculate_open_short_derivative() -> Result<()> {
        // We use a relatively large epsilon here due to the underlying fixed point pow
        // function not being monotonically increasing.
        let empirical_derivative_epsilon = fixed!(1e14);
        let test_tolerance = fixed!(1e14);

        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            // Min trade amount should be at least 1,000x the derivative epsilon.
            let bond_amount = rng.gen_range(fixed!(1e18)..=fixed!(10_000_000e18));

            // Calculate the function output at the bond amount and a small perturbation away.
            let f_x = match panic::catch_unwind(|| {
                state.calculate_open_short(bond_amount, state.vault_share_price())
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount results in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };
            let f_x_plus_delta = match panic::catch_unwind(|| {
                state.calculate_open_short(
                    bond_amount + empirical_derivative_epsilon,
                    state.vault_share_price(),
                )
            }) {
                Ok(result) => match result {
                    Ok(result) => result,
                    Err(_) => continue, // The amount results in the pool being insolvent.
                },
                Err(_) => continue, // Overflow or underflow error from FixedPoint<U256>.
            };

            // Sanity check
            assert!(f_x_plus_delta > f_x);

            // Compute the empirical and analytical derivatives.
            // Setting open, close, and current vault share price to be equal assumes 0% variable yield.
            let empirical_derivative = (f_x_plus_delta - f_x) / empirical_derivative_epsilon;
            let short_deposit_derivative = state.calculate_open_short_derivative(
                bond_amount,
                state.vault_share_price(),
                Some(state.calculate_spot_price()?),
            )?;

            // Ensure that the empirical and analytical derivatives match.
            let derivative_diff = if short_deposit_derivative >= empirical_derivative {
                short_deposit_derivative - empirical_derivative
            } else {
                empirical_derivative - short_deposit_derivative
            };
            assert!(
                derivative_diff < test_tolerance,
                "expected abs(derivative_diff={}) < test_tolerance={};
                calculated_derivative={}, emperical_derivative={}",
                derivative_diff,
                test_tolerance,
                short_deposit_derivative,
                empirical_derivative
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_sol_calculate_spot_price_after_short() -> Result<()> {
        let test_tolerance = fixed!(1e3);

        // Spawn a test chain and create two agents -- Alice and Bob. Alice is
        // funded with a large amount of capital so that she can initialize the
        // pool. Bob is funded with a small amount of capital so that we can
        // test opening a short and verify that the ending spot price is what we
        // expect.
        let mut rng = thread_rng();
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Fund Alice and Bob.
            let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
            let contribution = rng.gen_range(fixed!(10_000e18)..=fixed!(500_000_000e18));
            let budget = rng.gen_range(fixed!(10e18)..=fixed!(500_000_000e18));
            alice.fund(contribution).await?;
            bob.fund(budget).await?;

            // Alice initializes the pool.
            alice.initialize(fixed_rate, contribution, None).await?;

            // Attempt to predict the spot price after opening a short.
            let mut state = alice.get_state().await?;
            let bond_amount = rng.gen_range(
                state.minimum_transaction_amount()..=bob.calculate_max_short(None).await?,
            );

            // Open the short.
            bob.open_short(bond_amount, None, None).await?;

            // Calling any Solidity Hyperdrive transaction causes the
            // mock yield source to accrue some interest. We want to use
            // the state before the Solidity OpenShort, but with the
            // vault share price after the block tick.
            let new_state = alice.get_state().await?;
            let new_vault_share_price = new_state.vault_share_price();
            state.info.vault_share_price = new_vault_share_price.into();

            // Verify that the predicted spot price is equal to the ending spot
            // price.
            let expected_spot_price = state.calculate_spot_price_after_short(bond_amount, None)?;
            let actual_spot_price = new_state.calculate_spot_price()?;
            let abs_spot_price_diff = if actual_spot_price >= expected_spot_price {
                actual_spot_price - expected_spot_price
            } else {
                expected_spot_price - actual_spot_price
            };
            assert!(
                abs_spot_price_diff <= test_tolerance,
                "expected abs(spot_price_diff={}) <= test_tolerance={};
                calculated_spot_price={}, actual_spot_price={}",
                abs_spot_price_diff,
                test_tolerance,
                expected_spot_price,
                actual_spot_price,
            );
            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_defaults_calculate_spot_price_after_short() -> Result<()> {
        let mut rng = thread_rng();
        let mut num_checks = 0;
        // We don't need a lot of tests for this because each component is
        // tested elsewhere.
        for _ in 0..*SLOW_FUZZ_RUNS {
            // We use a random state but we will ignore any case where a call
            // fails because we want to test the default behavior when the state
            // allows all actions.
            let state = rng.gen::<State>();
            let checkpoint_exposure = rng
                .gen_range(fixed!(0)..=FixedPoint::<I256>::MAX)
                .raw()
                .flip_sign_if(rng.gen());
            // We need to catch panics because of overflows.
            let max_bond_amount = match panic::catch_unwind(|| {
                state.calculate_absolute_max_short(
                    state.calculate_spot_price()?,
                    checkpoint_exposure,
                    Some(3),
                )
            }) {
                Ok(max_bond_amount) => match max_bond_amount {
                    Ok(max_bond_amount) => max_bond_amount,
                    Err(_) => continue, // Err; max short insolvent
                },
                Err(_) => continue, // panic; likely in FixedPoint<U256>
            };
            if max_bond_amount == fixed!(0) {
                continue;
            }
            // Using the default behavior
            let bond_amount = rng.gen_range(state.minimum_transaction_amount()..=max_bond_amount);
            let price_with_default = state.calculate_spot_price_after_short(bond_amount, None)?;

            // Using a pre-calculated base amount
            let base_amount = match state.calculate_pool_share_delta_after_open_short(bond_amount) {
                Ok(share_amount) => Some(share_amount * state.vault_share_price()),
                Err(_) => continue,
            };
            let price_with_base_amount =
                state.calculate_spot_price_after_short(bond_amount, base_amount)?;

            // Test equality
            assert_eq!(
                price_with_default, price_with_base_amount,
                "`calculate_spot_price_after_short` is not handling default base_amount correctly."
            );
            num_checks += 1
        }
        // We want to make sure we didn't `continue` through all possible fuzz states
        assert!(num_checks > 0);
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_implied_rate() -> Result<()> {
        let tolerance = int256!(1e12);

        // Spawn a test chain with two agents.
        let mut rng = thread_rng();
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Fund Alice and Bob.
            let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
            let contribution = rng.gen_range(fixed!(100_000e18)..=fixed!(100_000_000e18));
            let budget = fixed!(100_000_000e18);
            alice.fund(contribution).await?;
            bob.fund(budget).await?;

            // Alice initializes the pool.
            // TODO: We'd like to set a random position duration & checkpoint duration.
            alice.initialize(fixed_rate, contribution, None).await?;

            // Set a random variable rate.
            let variable_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(1e18));
            alice.advance_time(variable_rate, 12.into()).await?;

            // Bob opens a short with a random bond amount. Before opening the
            // short, we calculate the implied rate.
            let bond_amount = rng.gen_range(
                FixedPoint::from(bob.get_config().minimum_transaction_amount)
                    ..=bob.calculate_max_short(None).await? * fixed!(0.9e18),
            );
            let implied_rate = bob.get_state().await?.calculate_implied_rate(
                bond_amount,
                bob.get_state().await?.vault_share_price(),
                variable_rate,
            )?;
            let (maturity_time, base_paid) = bob.open_short(bond_amount, None, None).await?;

            // The term passes and interest accrues.
            chain
                .increase_time(bob.get_config().position_duration.low_u128())
                .await?;

            // Bob closes his short.
            let base_proceeds = bob.close_short(maturity_time, bond_amount, None).await?;
            let annualized_position_duration =
                bob.get_state().await?.annualized_position_duration();

            // Ensure that the implied rate matches the realized rate from
            // holding the short to maturity.
            let realized_rate = if base_proceeds > base_paid {
                I256::try_from(
                    (base_proceeds - base_paid) / (base_paid * annualized_position_duration),
                )?
            } else {
                -I256::try_from(
                    (base_paid - base_proceeds) / (base_paid * annualized_position_duration),
                )?
            };
            let error = (implied_rate - realized_rate).abs();
            let scaled_tolerance = if implied_rate > int256!(1e18) {
                I256::from(tolerance * implied_rate)
            } else {
                tolerance
            };
            assert!(
                error < scaled_tolerance,
                "error {:?} exceeds tolerance of {} (scaled to {})",
                error,
                tolerance,
                scaled_tolerance
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }

    // Tests open short with an amount smaller than the minimum.
    #[tokio::test]
    async fn test_error_open_short_min_txn_amount() -> Result<()> {
        let min_bond_delta = fixed!(1);

        let mut rng = thread_rng();
        let state = rng.gen::<State>();
        let result = state.calculate_open_short(
            state.minimum_transaction_amount() - min_bond_delta,
            state.vault_share_price(),
        );
        assert!(result.is_err());
        Ok(())
    }

    // Tests open short with an amount larger than the maximum.
    #[tokio::test]
    async fn fuzz_error_open_short_max_txn_amount() -> Result<()> {
        // This amount gets added to the max trade to cause a failure.
        // TODO: You should be able to add a small amount (e.g. 1e18) to max to fail.
        // calc_open_short or calc_max_short must be incorrect for the additional
        // amount to have to be so large.
        let max_bond_delta = fixed!(1_000_000_000e18);

        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = rng
                .gen_range(fixed!(0)..=FixedPoint::<I256>::MAX)
                .raw()
                .flip_sign_if(rng.gen());
            let max_iterations = 7;
            let open_vault_share_price = rng.gen_range(fixed!(0)..=state.vault_share_price());
            // We need to catch panics because of FixedPoint<U256> overflows & underflows.
            let max_trade = panic::catch_unwind(|| {
                state.calculate_absolute_max_short(
                    state.calculate_spot_price()?,
                    checkpoint_exposure,
                    Some(max_iterations),
                )
            });
            // Since we're fuzzing it's possible that the max can fail.
            // We're only going to use it in this test if it succeeded.
            match max_trade {
                Ok(max_trade) => match max_trade {
                    Ok(max_trade) => {
                        let bond_amount = max_trade + max_bond_delta;
                        let base_amount = panic::catch_unwind(|| {
                            state.calculate_open_short(bond_amount, open_vault_share_price)
                        });
                        match base_amount {
                            Ok(result) => match result {
                                Ok(base_amount) => {
                                    return Err(eyre!(format!(
                                        "calculate_open_short on bond_amount={:#?} > max_bond_amount={:#?} \
                                        returned base_amount={:#?}, but should have failed.",
                                        bond_amount,
                                        max_trade,
                                        base_amount,
                                    )));
                                }
                                Err(_) => continue, // Open threw an Err.
                            },
                            Err(_) => continue, // Open threw a panic, likely due to FixedPoint<U256> under/over flow.
                        }
                    }
                    Err(_) => continue, // Max threw an Err.
                },
                Err(_) => continue, // Max thew an panic, likely due to FixedPoint<U256> under/over flow.
            }
        }

        Ok(())
    }

    #[tokio::test]
    pub async fn fuzz_sol_calculate_open_short() -> Result<()> {
        // Set up a random number generator. We use ChaCha8Rng with a randomly
        // generated seed, which makes it easy to reproduce test failures given
        // the seed.
        let mut rng = {
            let mut rng = thread_rng();
            let seed = rng.gen();
            ChaCha8Rng::seed_from_u64(seed)
        };

        // Initialize the test chain.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let mut celine = chain.celine().await?;

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;

            // Get state and trade details.
            let mut state = alice.get_state().await?;
            let min_txn_amount = state.minimum_transaction_amount();
            let max_short = celine.calculate_max_short(None).await?;
            let bond_amount = rng.gen_range(min_txn_amount..=max_short);

            // The base required should always be less than the short amount.
            celine.fund(bond_amount).await?;

            // Compare the open short call output against calculate_open_short.
            match celine
                .hyperdrive()
                .open_short(
                    bond_amount.into(),
                    FixedPoint::from(U256::MAX).into(),
                    fixed!(0).into(),
                    Options {
                        destination: celine.address(),
                        as_base: true,
                        extra_data: [].into(),
                    },
                )
                .call()
                .await
            {
                Ok((_, sol_base)) => {
                    // Calling any Solidity Hyperdrive transaction causes the
                    // mock yield source to accrue some interest. We want to use
                    // the state before the Solidity OpenShort, but with the
                    // vault share price after the block tick.

                    // Get the current vault share price & update state.
                    let vault_share_price = alice.get_state().await?.vault_share_price();
                    state.info.vault_share_price = vault_share_price.into();

                    // Get the open vault share price.
                    let Checkpoint {
                        weighted_spot_price: _,
                        last_weighted_spot_price_update_time: _,
                        vault_share_price: open_vault_share_price,
                    } = alice
                        .get_checkpoint(state.to_checkpoint(alice.now().await?))
                        .await?;

                    // Run the Rust function.
                    let rust_base =
                        state.calculate_open_short(bond_amount, open_vault_share_price.into());

                    // Compare the results.
                    let rust_base_unwrapped = rust_base.unwrap();
                    let sol_base_fp = FixedPoint::from(sol_base);
                    assert_eq!(
                        rust_base_unwrapped, sol_base_fp,
                        "expected rust_base={:#?} == sol_base={:#?}",
                        rust_base_unwrapped, sol_base_fp
                    );
                }
                Err(sol_err) => {
                    // Get the current vault share price & update state.
                    let vault_share_price = alice.get_state().await?.vault_share_price();
                    state.info.vault_share_price = vault_share_price.into();

                    // Get the open vault share price.
                    let Checkpoint {
                        weighted_spot_price: _,
                        last_weighted_spot_price_update_time: _,
                        vault_share_price: open_vault_share_price,
                    } = alice
                        .get_checkpoint(state.to_checkpoint(alice.now().await?))
                        .await?;

                    // Run the Rust function.
                    let rust_base =
                        state.calculate_open_short(bond_amount, open_vault_share_price.into());

                    // Make sure Rust failed.
                    assert!(
                        rust_base.is_err(),
                        "sol_err={:#?}, but rust_base={:#?} did not error",
                        sol_err,
                        rust_base,
                    );
                }
            }

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }

        Ok(())
    }
}
