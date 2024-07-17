#[cfg(test)]
mod tests {

    use ethers::types::U256;
    use eyre::Result;
    use fixedpointmath::fixed;
    use hyperdrive_test_utils::{chain::TestChain, constants::FUZZ_RUNS};
    use hyperdrive_wrappers::wrappers::ihyperdrive::Checkpoint;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use crate::test_utils::{
        agent::HyperdriveMathAgent, preamble::initialize_pool_with_random_state,
    };

    // TODO: Unignore after we add the logic to apply checkpoints prior to computing
    // the max long.
    #[ignore]
    #[tokio::test]
    pub async fn test_integration_calculate_max_short() -> Result<()> {
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

            // Celine opens a max short. Despite the trading that happened before this,
            // we expect Celine to open the max short on the pool or consume almost all
            // of her budget.
            let state = alice.get_state().await?;
            let Checkpoint {
                vault_share_price: open_vault_share_price,
                weighted_spot_price: _,
                last_weighted_spot_price_update_time: _,
            } = alice
                .get_checkpoint(state.to_checkpoint(alice.now().await?))
                .await?;
            let checkpoint_exposure = alice
                .get_checkpoint_exposure(state.to_checkpoint(alice.now().await?))
                .await?;
            let global_max_short = state.calculate_max_short(
                U256::MAX,
                open_vault_share_price,
                checkpoint_exposure,
                None,
                None,
            )?;
            let budget = bob.base();
            let slippage_tolerance = fixed!(0.001e18);
            let max_short = bob.calculate_max_short(Some(slippage_tolerance)).await?;
            bob.open_short(max_short, None, None).await?;

            if max_short != global_max_short {
                // We currently allow up to a tolerance of 3%, which means
                // that the max short is always consuming at least 97% of
                // the budget.
                let error_tolerance = fixed!(0.03e18);
                assert!(
                    bob.base() < budget * (fixed!(1e18) - slippage_tolerance) * error_tolerance,
                    "expected (base={}) < (budget={}) * {} = {}",
                    bob.base(),
                    budget,
                    error_tolerance,
                    budget * error_tolerance
                );
            }

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }

        Ok(())
    }

    // TODO: Unignore after we add the logic to apply checkpoints prior to computing
    // the max long.
    #[ignore]
    #[tokio::test]
    pub async fn test_integration_calculate_max_long() -> Result<()> {
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

            // One of three things should be true after opening the long:
            //
            // 1. The pool's spot price reached the max spot price prior to
            //    considering fees.
            // 2. The pool's solvency is close to zero.
            // 3. Bob's budget is consumed.
            let max_spot_price = bob.get_state().await?.calculate_max_spot_price()?;
            let max_long = bob.calculate_max_long(None).await?;
            let spot_price_after_long = bob
                .get_state()
                .await?
                .calculate_spot_price_after_long(max_long, None)?;
            bob.open_long(max_long, None, None).await?;
            let is_max_price = max_spot_price - spot_price_after_long < fixed!(1e15);
            let is_solvency_consumed = {
                let state = bob.get_state().await?;
                let error_tolerance =
                    fixed!(1_000e18).mul_div_down(state.calculate_spot_rate()?, fixed!(0.1e18));
                state.calculate_solvency()? < error_tolerance
            };
            let is_budget_consumed = {
                let error_tolerance = fixed!(1e18);
                bob.base() < error_tolerance
            };
            assert!(
                is_max_price || is_solvency_consumed || is_budget_consumed,
                "Invalid max long."
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }

        Ok(())
    }
}
