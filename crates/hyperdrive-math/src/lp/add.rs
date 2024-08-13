use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::FixedPoint;

use crate::State;

impl State {
    /// Calculates the LP shares for a given contribution when adding liquidity.
    pub fn calculate_add_liquidity(
        &self,
        current_block_timestamp: U256,
        contribution: FixedPoint<U256>,
        min_lp_share_price: FixedPoint<U256>,
        min_apr: FixedPoint<U256>,
        max_apr: FixedPoint<U256>,
        as_base: bool,
    ) -> Result<FixedPoint<U256>> {
        // Enforce the slippage guard.
        let apr = self.calculate_spot_rate()?;
        if apr < min_apr || apr > max_apr {
            return Err(eyre!("InvalidApr: Apr is outside the slippage guard."));
        }

        // Get lp_total_supply for the lp_shares calculation.
        let lp_total_supply = self.lp_total_supply();

        // Get the starting_present_value.
        let starting_present_value = self.calculate_present_value(current_block_timestamp)?;

        // Get the ending_present_value.
        let new_state = self.calculate_pool_state_after_add_liquidity(contribution, as_base)?;
        let ending_present_value = new_state.calculate_present_value(current_block_timestamp)?;

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

    pub fn calculate_pool_state_after_add_liquidity(
        &self,
        contribution: FixedPoint<U256>,
        as_base: bool,
    ) -> Result<State> {
        // Ensure that the contribution is greater than or equal to the minimum
        // transaction amount.
        if contribution < self.minimum_transaction_amount() {
            return Err(eyre!(
                "MinimumTransactionAmount: Contribution is smaller than the minimum transaction."
            ));
        }

        let share_contribution = {
            if as_base {
                I256::try_from(contribution.div_down(self.vault_share_price()))?
            } else {
                I256::try_from(contribution)?
            }
        };
        Ok(self.get_state_after_liquidity_update(share_contribution)?)
    }

    pub fn calculate_pool_deltas_after_add_liquidity(
        &self,
        contribution: FixedPoint<U256>,
        as_base: bool,
    ) -> Result<(FixedPoint<U256>, I256, FixedPoint<U256>)> {
        let share_contribution = match as_base {
            true => contribution / self.vault_share_price(),
            false => contribution,
        };
        let (share_reserves, share_adjustment, bond_reserves) = self.calculate_update_liquidity(
            self.share_reserves(),
            self.share_adjustment(),
            self.bond_reserves(),
            self.minimum_share_reserves(),
            I256::from(0),
        )?;
        let (new_share_reserves, new_share_adjustment, new_bond_reserves) = self
            .calculate_update_liquidity(
                self.share_reserves(),
                self.share_adjustment(),
                self.bond_reserves(),
                self.minimum_share_reserves(),
                I256::try_from(share_contribution)?,
            )?;
        Ok((
            new_share_reserves - share_reserves,
            new_share_adjustment - share_adjustment,
            new_bond_reserves - bond_reserves,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use fixedpointmath::{fixed, int256, uint256};
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS},
    };
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::test_utils::agent::HyperdriveMathAgent;

    #[tokio::test]
    async fn fuzz_calculate_add_liquidity_unhappy_with_random_state() -> Result<()> {
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
            // Testing mostly unhappy paths here since random state will mostly fail.
            match panic::catch_unwind(|| {
                state.calculate_add_liquidity(
                    current_block_timestamp,
                    contribution,
                    min_lp_share_price,
                    min_apr,
                    max_apr,
                    true,
                )
            }) {
                Ok(lp_shares) => match lp_shares {
                    Ok(lp_shares) => assert!(lp_shares >= min_lp_share_price),
                    Err(err) => {
                        let message = err.to_string();

                        if message == "MinimumTransactionAmount: Contribution is smaller than the minimum transaction." {
                            assert!(contribution < state.minimum_transaction_amount());
                        }

                        else if message == "InvalidApr: Apr is outside the slippage guard." {
                            let apr = state.calculate_spot_rate()?;
                            assert!(apr < min_apr || apr > max_apr);
                        }

                        else if message == "DecreasedPresentValueWhenAddingLiquidity: Present value decreased after adding liquidity." {
                            let share_contribution =
                                I256::try_from(contribution / state.vault_share_price()).unwrap();
                            let new_state = state.get_state_after_liquidity_update(share_contribution)?;
                            let starting_present_value = state.calculate_present_value(current_block_timestamp)?;
                            let ending_present_value = new_state.calculate_present_value(current_block_timestamp)?;
                            assert!(ending_present_value < starting_present_value);
                        }

                        else if message == "MinimumTransactionAmount: Not enough lp shares minted." {
                            let share_contribution =
                                I256::try_from(contribution / state.vault_share_price()).unwrap();
                            let new_state = state.get_state_after_liquidity_update(share_contribution)?;
                            let starting_present_value = state.calculate_present_value(current_block_timestamp)?;
                            let ending_present_value = new_state.calculate_present_value(current_block_timestamp)?;
                            let lp_shares = (ending_present_value - starting_present_value)
                                .mul_div_down(state.lp_total_supply(), starting_present_value);
                            assert!(lp_shares < state.minimum_transaction_amount());
                        }

                        else if message == "OutputLimit: Not enough lp shares minted." {
                            let share_contribution =
                                I256::try_from(contribution / state.vault_share_price()).unwrap();
                            let new_state = state.get_state_after_liquidity_update(share_contribution)?;
                            let starting_present_value = state.calculate_present_value(current_block_timestamp)?;
                            let ending_present_value = new_state.calculate_present_value(current_block_timestamp)?;
                            let lp_shares = (ending_present_value - starting_present_value)
                                .mul_div_down(state.lp_total_supply(), starting_present_value);
                            assert!(contribution.div_down(lp_shares) < min_lp_share_price);
                        }
                    }
                },
                Err(_) => continue, // FixedPoint<U256> underflow or overflow.
            }
        }

        Ok(())
    }
    #[tokio::test]
    async fn fuzz_calculate_add_liquidity() -> Result<()> {
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
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_pool_state_after_add_liquidity() -> Result<()> {
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
            let rate = fixed!(0);
            alice
                .advance_time(
                    rate,
                    FixedPoint::from(config.checkpoint_duration).mul_down(fixed!(0.5e18)),
                )
                .await?;

            // Get the State from solidity before adding liquidity.
            let state = State {
                config: bob.get_state().await?.config.clone(),
                info: bob.get_state().await?.info.clone(),
            };

            // Bob adds liquidity
            bob.add_liquidity(budget, None).await?;

            // Get the State from solidity after adding liquidity.
            let expected_state = State {
                config: bob.get_state().await?.config.clone(),
                info: bob.get_state().await?.info.clone(),
            };

            // Calculate lp_shares from the rust function.
            let actual_state = state
                .calculate_pool_state_after_add_liquidity(budget, true)
                .unwrap();

            // Ensure the states are equal within a tolerance.
            let share_reserves_equal = expected_state.share_reserves()
                <= actual_state.share_reserves() + fixed!(1e9)
                && expected_state.share_reserves() >= actual_state.share_reserves() - fixed!(1e9);
            assert!(share_reserves_equal, "Share reserves should be equal.");

            let bond_reserves_equal = expected_state.bond_reserves()
                <= actual_state.bond_reserves() + fixed!(1e10)
                && expected_state.bond_reserves() >= actual_state.bond_reserves() - fixed!(1e10);
            assert!(bond_reserves_equal, "Bond reserves should be equal.");

            let share_adjustment_equal = expected_state.share_adjustment()
                <= actual_state.share_adjustment() + int256!(1e10)
                && expected_state.share_adjustment()
                    >= actual_state.share_adjustment() - int256!(1e10);
            assert!(share_adjustment_equal, "Share adjustment should be equal.");

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }
}
