use std::{
    cmp::Ordering,
    panic::{catch_unwind, AssertUnwindSafe},
};

use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixed_point::FixedPoint;
use fixed_point_macros::{fixed, int256};

use crate::{calculate_effective_share_reserves, State, YieldSpace};

impl State {
    // Calculates the lp_shares for a given contribution when adding liquidity.
    pub fn calculate_add_liquidity(
        &self,
        current_block_timestamp: U256,
        contribution: FixedPoint,
        min_lp_share_price: FixedPoint,
        min_apr: FixedPoint,
        max_apr: FixedPoint,
        as_base: bool,
    ) -> Result<FixedPoint> {
        // Ensure that the contribution is greater than or equal to the minimum
        // transaction amount.
        if contribution < self.minimum_transaction_amount() {
            return Err(eyre!(
                "MinimumTransactionAmount: Contribution is smaller than the minimum transaction."
            ));
        }

        // Enforce the slippage guard.
        let apr = self.calculate_spot_rate();
        if apr < min_apr || apr > max_apr {
            return Err(eyre!("InvalidApr: Apr is outside the slippage guard."));
        }

        // Get lp_total_supply for the lp_shares calculation.
        let lp_total_supply = self.lp_total_supply();

        // Get the starting_present_value.
        let starting_present_value = self.calculate_present_value(current_block_timestamp);

        // Get the ending_present_value.
        let share_contribution = {
            if as_base {
                // Attempt a crude conversion from base to shares.
                I256::try_from(contribution / self.vault_share_price()).unwrap()
            } else {
                I256::try_from(contribution).unwrap()
            }
        };
        let new_state = self.get_state_after_liquidity_update(share_contribution);
        let ending_present_value = new_state.calculate_present_value(current_block_timestamp);

        // Ensure the present value didn't decrease after adding liquidity.
        if ending_present_value < starting_present_value {
            return Err(eyre!("DecreasedPresentValueWhenAddingLiquidity: Present value decreased after adding liquidity."));
        }

        // Calculate the lp_shares.
        let lp_shares = (ending_present_value - starting_present_value)
            .mul_div_down(lp_total_supply, starting_present_value);

        // Ensure that enough lp_shares are minted so that they can be redeemed.
        if lp_shares < self.minimum_transaction_amount() {
            return Err(eyre!(
                "MinimumTransactionAmount: Not enough lp shares minted."
            ));
        }

        // Enforce the minimum LP share price slippage guard.
        if contribution.div_down(lp_shares) < min_lp_share_price {
            return Err(eyre!("OutputLimit: Not enough lp shares minted."));
        }

        Ok(lp_shares)
    }

    // Gets the resulting state when updating liquidity.
    pub fn get_state_after_liquidity_update(&self, share_reserves_delta: I256) -> State {
        let share_reserves = self.share_reserves();
        let share_adjustment = self.share_adjustment();
        let bond_reserves = self.bond_reserves();
        let minimum_share_reserves = self.minimum_share_reserves();

        // Calculate new reserve and adjustment levels.
        let (updated_share_reserves, updated_share_adjustment, updated_bond_reserves) = self
            .calculate_update_liquidity(
                share_reserves,
                share_adjustment,
                bond_reserves,
                minimum_share_reserves,
                share_reserves_delta,
            )
            .unwrap();

        // Update and return the new state.
        let mut new_info = self.info.clone();
        new_info.share_reserves = U256::from(updated_share_reserves);
        new_info.share_adjustment = updated_share_adjustment;
        new_info.bond_reserves = U256::from(updated_bond_reserves);
        State {
            config: self.config.clone(),
            info: new_info,
        }
    }

    // Calculates the resulting share_reserves, share_adjustment, and
    // bond_reserves when updating liquidity with a share_reserves_delta.
    fn calculate_update_liquidity(
        &self,
        share_reserves: FixedPoint,
        share_adjustment: I256,
        bond_reserves: FixedPoint,
        minimum_share_reserves: FixedPoint,
        share_reserves_delta: I256,
    ) -> Result<(FixedPoint, I256, FixedPoint), &'static str> {
        if share_reserves_delta == I256::zero() {
            return Ok((share_reserves, share_adjustment, bond_reserves));
        }

        // Get the updated share reserves.
        let new_share_reserves = if share_reserves_delta > I256::zero() {
            I256::try_from(share_reserves).unwrap() + share_reserves_delta
        } else {
            I256::try_from(share_reserves).unwrap() - share_reserves_delta
        };

        // Ensure the minimum share reserve level.
        if new_share_reserves < I256::try_from(minimum_share_reserves).unwrap() {
            return Err("Update would result in share reserves below minimum.");
        }

        // Convert to Fixedpoint to allow the math below.
        let new_share_reserves = FixedPoint::from(new_share_reserves);

        // Get the updated share adjustment.
        let new_share_adjustment = if share_adjustment >= I256::zero() {
            let share_adjustment_fp = FixedPoint::from(share_adjustment);
            I256::try_from(new_share_reserves.mul_div_down(share_adjustment_fp, share_reserves))
                .unwrap()
        } else {
            let share_adjustment_fp = FixedPoint::from(-share_adjustment);
            -I256::try_from(new_share_reserves.mul_div_up(share_adjustment_fp, share_reserves))
                .unwrap()
        };

        // Get the updated bond reserves.
        let old_effective_share_reserves = calculate_effective_share_reserves(
            self.effective_share_reserves(),
            self.share_adjustment(),
        );
        let new_effective_share_reserves =
            calculate_effective_share_reserves(new_share_reserves, new_share_adjustment);
        let new_bond_reserves =
            bond_reserves.mul_div_down(new_effective_share_reserves, old_effective_share_reserves);

        Ok((new_share_reserves, new_share_adjustment, new_bond_reserves))
    }

    /// Calculates the number of base that are not reserved by open positions.
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

    /// Function that takes in a scaled FixedPoint maturity time and calculates
    /// normalized time remaining with higher precision.
    fn calculate_scaled_normalized_time_remaining(
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

    /// Calculates the present value of LPs capital in the pool.
    pub fn calculate_present_value(&self, current_block_timestamp: U256) -> FixedPoint {
        // Calculate the average time remaining for the longs and shorts.

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

        let present_value: I256 = I256::try_from(self.share_reserves()).unwrap()
            + self.calculate_net_curve_trade(
                long_average_time_remaining,
                short_average_time_remaining,
            )
            + self.calculate_net_flat_trade(
                long_average_time_remaining,
                short_average_time_remaining,
            )
            - I256::try_from(self.minimum_share_reserves()).unwrap();

        if present_value < int256!(0) {
            panic!("Negative present value!");
        }
        present_value.into()
    }

    // NOTE: This version will not panic.
    //
    // Calculates the present value of LPs capital in the pool.
    pub fn calculate_present_value_safe(
        &self,
        current_block_timestamp: U256,
    ) -> Result<FixedPoint> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            // Assume the following methods return Results and can be handled with `?`
            let long_average_time_remaining = self.calculate_normalized_time_remaining(
                self.long_average_maturity_time().into(),
                current_block_timestamp,
            );

            let short_average_time_remaining = self.calculate_normalized_time_remaining(
                self.short_average_maturity_time().into(),
                current_block_timestamp,
            );

            // Conversion and calculation
            let share_reserves = I256::try_from(self.share_reserves())
                .map_err(|_| eyre!("Failed to convert share_reserves"))?;

            let minimum_share_reserves = I256::try_from(self.minimum_share_reserves())
                .map_err(|_| eyre!("Failed to convert minimum_share_reserves"))?;

            let present_value = share_reserves
                + self.calculate_net_curve_trade(
                    long_average_time_remaining,
                    short_average_time_remaining,
                )
                + self.calculate_net_flat_trade(
                    long_average_time_remaining,
                    short_average_time_remaining,
                )
                - minimum_share_reserves;

            // Check for non-positive present value
            if present_value < I256::from(0) {
                Err(eyre!("Negative present value!"))
            } else {
                Ok(present_value.into()) // Converting I256 to FixedPoint, assuming it's possible
            }
        }));

        // Handling the Result of `catch_unwind`
        match result {
            Ok(Ok(present_value)) => Ok(present_value), // Unwrap successful result
            Ok(Err(e)) => Err(e),                       // Propagate the inner error
            Err(_) => Err(eyre!("Panic occurred during calculation")),
        }
    }

    pub fn calculate_net_curve_trade(
        &self,
        long_average_time_remaining: FixedPoint,
        short_average_time_remaining: FixedPoint,
    ) -> I256 {
        // NOTE: To underestimate the impact of closing the net curve position,
        // we round up the long side of the net curve position (since this
        // results in a larger value removed from the share reserves) and round
        // down the short side of the net curve position (since this results in
        // a smaller value added to the share reserves).
        //
        // The net curve position is the net of the longs and shorts that are
        // currently tradeable on the curve. Given the amount of outstanding
        // longs `y_l` and shorts `y_s` as well as the average time remaining
        // of outstanding longs `t_l` and shorts `t_s`, we can
        // compute the net curve position as:
        //
        // netCurveTrade = y_l * t_l - y_s * t_s.
        let net_curve_position: I256 =
            I256::try_from(self.longs_outstanding().mul_up(long_average_time_remaining)).unwrap()
                - I256::try_from(
                    self.shorts_outstanding()
                        .mul_down(short_average_time_remaining),
                )
                .unwrap();

        // If the net curve position is positive, then the pool is net long.
        // Closing the net curve position results in the longs being paid out
        // from the share reserves, so we negate the result.
        match net_curve_position.cmp(&int256!(0)) {
            Ordering::Greater => {
                let net_curve_position: FixedPoint = FixedPoint::from(net_curve_position);
                let max_curve_trade = self
                    .calculate_max_sell_bonds_in_safe(self.minimum_share_reserves())
                    .unwrap();
                if max_curve_trade >= net_curve_position {
                    match self.calculate_shares_out_given_bonds_in_down_safe(net_curve_position) {
                        Ok(net_curve_trade) => -I256::try_from(net_curve_trade).unwrap(),
                        Err(err) => {
                            // If the net curve position is smaller than the
                            // minimum transaction amount and the trade fails,
                            // we mark it to 0. This prevents liveness problems
                            // when the net curve position is very small.
                            if net_curve_position < self.minimum_transaction_amount() {
                                I256::zero()
                            } else {
                                panic!("net_curve_trade failure: {}", err);
                            }
                        }
                    }
                } else {
                    // If the share adjustment is greater than or equal to zero,
                    // then the effective share reserves are less than or equal to
                    // the share reserves. In this case, the maximum amount of
                    // shares that can be removed from the share reserves is
                    // `effectiveShareReserves - minimumShareReserves`.
                    if self.share_adjustment() >= I256::from(0) {
                        -I256::try_from(
                            self.effective_share_reserves() - self.minimum_share_reserves(),
                        )
                        .unwrap()

                    // Otherwise, the effective share reserves are greater than the
                    // share reserves. In this case, the maximum amount of shares
                    // that can be removed from the share reserves is
                    // `shareReserves - minimumShareReserves`.
                    } else {
                        -I256::try_from(self.share_reserves() - self.minimum_share_reserves())
                            .unwrap()
                    }
                }
            }
            Ordering::Less => {
                let net_curve_position: FixedPoint = FixedPoint::from(-net_curve_position);
                let max_curve_trade = self.calculate_max_buy_bonds_out_safe().unwrap();
                if max_curve_trade >= net_curve_position {
                    match self.calculate_shares_in_given_bonds_out_up_safe(net_curve_position) {
                        Ok(net_curve_trade) => I256::try_from(net_curve_trade).unwrap(),
                        Err(err) => {
                            // If the net curve position is smaller than the
                            // minimum transaction amount and the trade fails,
                            // we mark it to 0. This prevents liveness problems
                            // when the net curve position is very small.
                            if net_curve_position < self.minimum_transaction_amount() {
                                I256::zero()
                            } else {
                                panic!("net_curve_trade failure: {}", err);
                            }
                        }
                    }
                } else {
                    let max_share_payment = self.calculate_max_buy_shares_in_safe().unwrap();

                    // NOTE: We round the difference down to underestimate the
                    // impact of closing the net curve position.
                    I256::try_from(
                        max_share_payment
                            + (net_curve_position - max_curve_trade)
                                .div_down(self.vault_share_price()),
                    )
                    .unwrap()
                }
            }
            Ordering::Equal => int256!(0),
        }
    }

    pub fn calculate_net_flat_trade(
        &self,
        long_average_time_remaining: FixedPoint,
        short_average_time_remaining: FixedPoint,
    ) -> I256 {
        // NOTE: In order to underestimate the impact of closing all of the
        // flat trades, we round the impact of closing the shorts down and round
        // the impact of closing the longs up.
        //
        // Compute the net of the longs and shorts that will be traded flat and
        // apply this net to the reserves.
        I256::try_from(self.shorts_outstanding().mul_div_down(
            fixed!(1e18) - short_average_time_remaining,
            self.vault_share_price(),
        ))
        .unwrap()
            - I256::try_from(self.longs_outstanding().mul_div_up(
                fixed!(1e18) - long_average_time_remaining,
                self.vault_share_price(),
            ))
            .unwrap()
    }

    // Calculates the number of share reserves that are not reserved by open
    // positions.
    fn calculate_idle_share_reserves(&self) -> FixedPoint {
        let long_exposure = self.long_exposure().div_up(self.vault_share_price());
        let idle_shares = {
            if self.share_reserves() > long_exposure + self.minimum_share_reserves() {
                return self.share_reserves() - long_exposure - self.minimum_share_reserves();
            }
            fixed!(0)
        };

        idle_shares
    }

    // Given a signed bond amount, this function calculates the negation of the
    // derivative of `calculateSharesOutGivenBondsIn` when the bond amount is
    // positive or the derivative of `calculateSharesInGivenBondsOut` when the
    // bond amount is negative.
    // Note that the state is the present state of the pool and original values
    // passed in as parameters.  Present sate variables are not expressly
    // paased in because so that downstream function like kUp() can still be
    // used.
    fn calculate_shares_delta_given_bonds_delta_derivative(
        &self,
        bond_amount: I256,
        original_share_reserves: FixedPoint,
        original_bond_reserves: FixedPoint,
        original_effective_share_reserves: FixedPoint,
        original_share_adjustment: I256,
    ) -> Result<FixedPoint> {
        // Calculate the bond reserves after the bond amount is applied.
        let bond_reserves_after = if bond_amount >= I256::zero() {
            self.bond_reserves() + bond_amount.into()
        } else {
            let bond_amount = FixedPoint::from(U256::try_from(-bond_amount)?);
            if bond_amount < self.bond_reserves() {
                self.bond_reserves() - bond_amount
            } else {
                return Err(eyre!("Calculating the bond reserves underflows"));
            }
        };

        // NOTE: Round up since this is on the rhs of the final subtraction.
        //
        // derivative = c * (mu * z_e(x)) ** -t_s +
        //              (y / z_e) * (y(x)) ** -t_s -
        //              (y / z_e) * (y(x) + dy) ** -t_s
        let effective_share_reserves = self.effective_share_reserves();
        let derivative = self.vault_share_price().div_up(
            self.initial_vault_share_price()
                .mul_down(effective_share_reserves)
                .pow(self.time_stretch()),
        ) + original_bond_reserves.div_up(
            original_effective_share_reserves
                .mul_down(self.bond_reserves().pow(self.time_stretch())),
        );

        // NOTE: Rounding this down rounds the subtraction up.
        let right_hand_side = original_bond_reserves.div_down(
            original_effective_share_reserves.mul_up(bond_reserves_after.pow(self.time_stretch())),
        );
        if derivative < right_hand_side {
            return Err(eyre!("Derivative is less than right hand side"));
        }
        let derivative = derivative - right_hand_side;

        // NOTE: Round up since this is on the rhs of the final subtraction.
        //
        // inner = (
        //             (mu / c) * (k(x) - (y(x) + dy) ** (1 - t_s))
        //         ) ** (t_s / (1 - t_s))
        let k = self.k_up();
        let y_plus_dy_pow_one_minus_t = bond_reserves_after.pow(fixed!(1e18) - self.time_stretch());
        if k < y_plus_dy_pow_one_minus_t {
            return Err(eyre!("k is less than inner"));
        }
        let inner = k - y_plus_dy_pow_one_minus_t;
        let inner = inner.mul_div_up(self.initial_vault_share_price(), self.vault_share_price());
        let inner = if inner >= fixed!(1e18) {
            // NOTE: Round the exponent up since this rounds the result up.
            inner.pow(
                self.time_stretch()
                    .div_up(fixed!(1e18) - self.time_stretch()),
            )
        } else {
            // NOTE: Round the exponent down since this rounds the result up.
            inner.pow(
                self.time_stretch()
                    .div_down(fixed!(1e18) - self.time_stretch()),
            )
        };
        let derivative = derivative.mul_div_up(inner, self.vault_share_price());
        let derivative = if fixed!(1e18) > derivative {
            fixed!(1e18) - derivative
        } else {
            return Ok(fixed!(0));
        };
        let derivative = if original_share_adjustment >= I256::zero() {
            let right_hand_side =
                FixedPoint::try_from(original_share_adjustment)?.div_up(original_share_reserves);
            if right_hand_side > fixed!(1e18) {
                return Err(eyre!("Right hand side is greater than 1e18"));
            }
            let right_hand_side = fixed!(1e18) - right_hand_side;
            derivative.mul_down(right_hand_side)
        } else {
            derivative.mul_down(
                fixed!(1e18)
                    + FixedPoint::try_from(-original_share_adjustment)?
                        .div_down(original_share_reserves),
            )
        };

        Ok(derivative)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        panic,
        panic::{catch_unwind, AssertUnwindSafe},
    };

    use fixed_point_macros::uint256;
    use hyperdrive_wrappers::wrappers::mock_lp_math::{
        DistributeExcessIdleParams, PresentValueParams,
    };
    use rand::{thread_rng, Rng};
    use test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS},
    };

    use super::*;

    #[tokio::test]
    async fn fuzz_test_calculate_add_liquidity_unhappy_with_random_state() -> Result<()> {
        // Get the State from solidity before adding liquidity.
        let mut rng = thread_rng();

        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let contribution = rng.gen_range(fixed!(0)..=state.bond_reserves());
            let current_block_timestamp = U256::from(rng.gen_range(0..=60 * 60 * 24 * 365));
            let min_lp_share_price = rng.gen_range(fixed!(0)..=fixed!(10e18));
            let min_apr = rng.gen_range(fixed!(0)..fixed!(1e18));
            let max_apr = rng.gen_range(fixed!(5e17)..fixed!(1e18));

            // Calculate lp_shares from the rust function.
            let result = catch_unwind(AssertUnwindSafe(|| {
                state.calculate_add_liquidity(
                    current_block_timestamp,
                    contribution,
                    min_lp_share_price,
                    min_apr,
                    max_apr,
                    true,
                )
            }));

            // Testing mostly unhappy paths here since random state will mostly fail.
            match result {
                Ok(result) => match result {
                    Ok(lp_shares) => {
                        assert!(lp_shares >= min_lp_share_price);
                    }
                    Err(err) => {
                        let message = err.to_string();

                        if message == "MinimumTransactionAmount: Contribution is smaller than the minimum transaction." {
                          assert!(contribution < state.minimum_transaction_amount());
                        }

                        else if message == "InvalidApr: Apr is outside the slippage guard." {
                            let apr = state.calculate_spot_rate();
                            assert!(apr < min_apr || apr > max_apr);
                        }

                        else if message == "DecreasedPresentValueWhenAddingLiquidity: Present value decreased after adding liquidity." {
                            let share_contribution =
                                I256::try_from(contribution / state.vault_share_price()).unwrap();
                            let new_state = state.get_state_after_liquidity_update(share_contribution);
                            let starting_present_value = state.calculate_present_value(current_block_timestamp);
                            let ending_present_value = new_state.calculate_present_value(current_block_timestamp);
                            assert!(ending_present_value < starting_present_value);
                        }

                        else if message == "MinimumTransactionAmount: Not enough lp shares minted." {
                            let share_contribution =
                                I256::try_from(contribution / state.vault_share_price()).unwrap();
                            let new_state = state.get_state_after_liquidity_update(share_contribution);
                            let starting_present_value = state.calculate_present_value(current_block_timestamp);
                            let ending_present_value = new_state.calculate_present_value(current_block_timestamp);
                            let lp_shares = (ending_present_value - starting_present_value)
                                .mul_div_down(state.lp_total_supply(), starting_present_value);
                            assert!(lp_shares < state.minimum_transaction_amount());
                        }

                        else if message == "OutputLimit: Not enough lp shares minted." {
                            let share_contribution =
                                I256::try_from(contribution / state.vault_share_price()).unwrap();
                            let new_state = state.get_state_after_liquidity_update(share_contribution);
                            let starting_present_value = state.calculate_present_value(current_block_timestamp);
                            let ending_present_value = new_state.calculate_present_value(current_block_timestamp);
                            let lp_shares = (ending_present_value - starting_present_value)
                                .mul_div_down(state.lp_total_supply(), starting_present_value);
                            assert!(contribution.div_down(lp_shares) < min_lp_share_price);
                        }
                    }
                },
                // ignore inner panics
                Err(_) => {}
            }
        }

        Ok(())
    }
    #[tokio::test]
    async fn fuzz_test_calculate_add_liquidity() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob.
        let mut rng = thread_rng();
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let config = bob.get_config().clone();

        // Test happy paths.
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

            // Some of the checkpoint passes and variable interest accrues.
            alice
                .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
                .await?;
            let rate = rng.gen_range(fixed!(0)..=fixed!(0.5e18));
            alice
                .advance_time(
                    rate,
                    FixedPoint::from(config.checkpoint_duration) * fixed!(0.5e18),
                )
                .await?;

            // Get the State from solidity before adding liquidity.
            let hd_state = bob.get_state().await?;
            let state = State {
                config: hd_state.config.clone(),
                info: hd_state.info.clone(),
            };

            // Bob adds liquidity
            bob.add_liquidity(budget, None).await?;
            let lp_shares_mock = bob.lp_shares();

            // Calculate lp_shares from the rust function.
            let lp_shares = state
                .calculate_add_liquidity(
                    bob.now().await?,
                    budget,
                    fixed!(0),
                    fixed!(0),
                    FixedPoint::from(U256::MAX),
                    true,
                )
                .unwrap();

            // Rust can't account for slippage.
            assert!(lp_shares >= lp_shares_mock, "Should over estimate.");
            // Answer should still be mostly the same.
            assert!(
                fixed!(1e18) - lp_shares_mock / lp_shares < fixed!(1e11),
                "Difference should be less than 0.0000001."
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default());
            bob.reset(Default::default());
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_present_value() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let current_block_timestamp = rng.gen_range(fixed!(1)..=fixed!(1e4));
            let actual = panic::catch_unwind(|| {
                state.calculate_present_value(current_block_timestamp.into())
            });
            match chain
                .mock_lp_math()
                .calculate_present_value(PresentValueParams {
                    share_reserves: state.info.share_reserves,
                    bond_reserves: state.info.bond_reserves,
                    longs_outstanding: state.info.longs_outstanding,
                    share_adjustment: state.info.share_adjustment,
                    time_stretch: state.config.time_stretch,
                    vault_share_price: state.info.vault_share_price,
                    initial_vault_share_price: state.config.initial_vault_share_price,
                    minimum_share_reserves: state.config.minimum_share_reserves,
                    minimum_transaction_amount: state.config.minimum_transaction_amount,
                    long_average_time_remaining: state
                        .calculate_scaled_normalized_time_remaining(
                            state.long_average_maturity_time(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    short_average_time_remaining: state
                        .calculate_scaled_normalized_time_remaining(
                            state.short_average_maturity_time(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    shorts_outstanding: state.shorts_outstanding().into(),
                })
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), FixedPoint::from(expected));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_net_curve_trade() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let current_block_timestamp = rng.gen_range(fixed!(1)..=fixed!(1e4));
            let long_average_time_remaining = state.calculate_normalized_time_remaining(
                state.long_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let short_average_time_remaining = state.calculate_normalized_time_remaining(
                state.short_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let actual = panic::catch_unwind(|| {
                state.calculate_net_curve_trade(
                    long_average_time_remaining,
                    short_average_time_remaining,
                )
            });
            match chain
                .mock_lp_math()
                .calculate_net_curve_trade(PresentValueParams {
                    share_reserves: state.info.share_reserves,
                    bond_reserves: state.info.bond_reserves,
                    longs_outstanding: state.info.longs_outstanding,
                    share_adjustment: state.info.share_adjustment,
                    time_stretch: state.config.time_stretch,
                    vault_share_price: state.info.vault_share_price,
                    initial_vault_share_price: state.config.initial_vault_share_price,
                    minimum_share_reserves: state.config.minimum_share_reserves,
                    minimum_transaction_amount: state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: state.shorts_outstanding().into(),
                })
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), I256::from(expected));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_net_flat_trade() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let current_block_timestamp = rng.gen_range(fixed!(1)..=fixed!(1e4));
            let long_average_time_remaining = state.calculate_normalized_time_remaining(
                state.long_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let short_average_time_remaining = state.calculate_normalized_time_remaining(
                state.short_average_maturity_time().into(),
                current_block_timestamp.into(),
            );
            let actual = panic::catch_unwind(|| {
                state.calculate_net_flat_trade(
                    long_average_time_remaining,
                    short_average_time_remaining,
                )
            });
            match chain
                .mock_lp_math()
                .calculate_net_flat_trade(PresentValueParams {
                    share_reserves: state.info.share_reserves,
                    bond_reserves: state.info.bond_reserves,
                    longs_outstanding: state.info.longs_outstanding,
                    share_adjustment: state.info.share_adjustment,
                    time_stretch: state.config.time_stretch,
                    vault_share_price: state.info.vault_share_price,
                    initial_vault_share_price: state.config.initial_vault_share_price,
                    minimum_share_reserves: state.config.minimum_share_reserves,
                    minimum_transaction_amount: state.config.minimum_transaction_amount,
                    long_average_time_remaining: long_average_time_remaining.into(),
                    short_average_time_remaining: short_average_time_remaining.into(),
                    shorts_outstanding: state.shorts_outstanding().into(),
                })
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), I256::from(expected));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_test_calculate_shares_delta_given_bonds_delta_derivative() -> Result<()> {
        let chain = TestChain::new().await?;
        let mock = chain.mock_lp_math();

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();

        for _ in 0..*FAST_FUZZ_RUNS {
            // Generate random states.
            let original_state = rng.gen::<State>();
            let present_state = rng.gen::<State>();

            // Get the bond amount, which in this case is equal to the net_curve_trade.
            let bond_amount = I256::try_from(rng.gen_range(fixed!(0)..=fixed!(1e24)))?; // 1 million

            // Maturity time goes from 0 to position duration, so we'll just set
            // this to zero to make the math simpler.
            let current_block_timestamp = fixed!(0);

            // Calcuulate the result from the Rust implementation.
            let actual = present_state.calculate_shares_delta_given_bonds_delta_derivative(
                bond_amount,
                original_state.share_reserves(),
                original_state.bond_reserves(),
                original_state.effective_share_reserves(),
                original_state.share_adjustment(),
            );

            // This errors out a lot so we need to catch that here.
            let starting_present_value_result =
                original_state.calculate_present_value_safe(U256::from(current_block_timestamp));
            if starting_present_value_result.is_err() {
                continue;
            }
            let starting_present_value = starting_present_value_result?;
            let idle = present_state.calculate_idle_share_reserves();

            // Gather the parameters for the solidity call.  There are a lot
            // that aren't actually used, but the solidity call needs them.
            let params = DistributeExcessIdleParams {
                present_value_params: PresentValueParams {
                    share_reserves: present_state.info.share_reserves,
                    bond_reserves: present_state.info.bond_reserves,
                    longs_outstanding: present_state.info.longs_outstanding,
                    share_adjustment: present_state.info.share_adjustment,
                    time_stretch: present_state.config.time_stretch,
                    vault_share_price: present_state.info.vault_share_price,
                    initial_vault_share_price: present_state.config.initial_vault_share_price,
                    minimum_share_reserves: present_state.config.minimum_share_reserves,
                    minimum_transaction_amount: present_state.config.minimum_transaction_amount,
                    long_average_time_remaining: present_state
                        .calculate_normalized_time_remaining(
                            present_state.long_average_maturity_time().into(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    short_average_time_remaining: present_state
                        .calculate_normalized_time_remaining(
                            present_state.short_average_maturity_time().into(),
                            current_block_timestamp.into(),
                        )
                        .into(),
                    shorts_outstanding: present_state.shorts_outstanding().into(),
                },
                starting_present_value: starting_present_value.into(),
                active_lp_total_supply: original_state.lp_total_supply().into(),
                withdrawal_shares_total_supply: uint256!(0),
                idle: idle.into(),
                net_curve_trade: bond_amount,
                original_share_reserves: original_state.share_reserves().into(),
                original_share_adjustment: original_state.share_adjustment(),
                original_bond_reserves: original_state.bond_reserves().into(),
            };

            match mock
                .calculate_shares_delta_given_bonds_delta_derivative_safe(
                    params,
                    U256::from(original_state.effective_share_reserves()),
                    bond_amount,
                )
                .call()
                .await
            {
                Ok(expected) => {
                    let (result, success) = expected;
                    if !success && result == uint256!(0) {
                        assert!(actual.is_err());
                    } else if success && result == uint256!(0) {
                        assert_eq!(actual?, fixed!(0));
                    } else {
                        assert_eq!(actual?, FixedPoint::from(expected.0));
                    }
                }
                Err(_) => {
                    assert!(actual.is_err())
                }
            }
        }
        Ok(())
    }
}
