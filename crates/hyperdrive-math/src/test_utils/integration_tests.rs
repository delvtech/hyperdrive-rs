#[cfg(test)]
mod tests {

    use ethers::{signers::LocalWallet, types::U256};
    use eyre::Result;
    use fixed_point::FixedPoint;
    use fixed_point_macros::{fixed, uint256};
    use hyperdrive_wrappers::wrappers::ihyperdrive::Checkpoint;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    use test_utils::{
        agent::Agent,
        chain::{ChainClient, TestChain},
        constants::FUZZ_RUNS,
    };

    use crate::test_utils::agent::HyperdriveMathAgent;

    /// Executes random trades throughout a Hyperdrive term.
    async fn preamble(
        rng: &mut ChaCha8Rng,
        alice: &mut Agent<ChainClient<LocalWallet>, ChaCha8Rng>,
        bob: &mut Agent<ChainClient<LocalWallet>, ChaCha8Rng>,
        celine: &mut Agent<ChainClient<LocalWallet>, ChaCha8Rng>,
        fixed_rate: FixedPoint,
    ) -> Result<()> {
        // Fund the agent accounts and initialize the pool.
        alice
            .fund(rng.gen_range(fixed!(1_000e18)..=fixed!(500_000_000e18)))
            .await?;
        bob.fund(rng.gen_range(fixed!(1_000e18)..=fixed!(500_000_000e18)))
            .await?;
        celine
            .fund(rng.gen_range(fixed!(1_000e18)..=fixed!(500_000_000e18)))
            .await?;

        // Alice initializes the pool.
        alice.initialize(fixed_rate, alice.base(), None).await?;

        // Advance the time for over a term and make trades in some of the checkpoints.
        let mut time_remaining = alice.get_config().position_duration;
        while time_remaining > uint256!(0) {
            // Bob opens a long.
            let discount = rng.gen_range(fixed!(0.1e18)..=fixed!(0.5e18));
            let long_amount =
                rng.gen_range(fixed!(1e12)..=bob.calculate_max_long(None).await? * discount);
            bob.open_long(long_amount, None, None).await?;

            // Celine opens a short.
            let discount = rng.gen_range(fixed!(0.1e18)..=fixed!(0.5e18));
            let bond_amount =
                rng.gen_range(fixed!(1e12)..=celine.calculate_max_short(None).await? * discount);
            celine.open_short(bond_amount, None, None).await?;

            // Advance the time and mint all of the intermediate checkpoints.
            let multiplier = rng.gen_range(fixed!(5e18)..=fixed!(50e18));
            let delta = FixedPoint::from(time_remaining)
                .min(FixedPoint::from(alice.get_config().checkpoint_duration) * multiplier);
            time_remaining -= U256::from(delta);
            alice
                .advance_time(
                    fixed!(0), // TODO: Use a real rate.
                    delta,
                )
                .await?;
        }

        // Mint a checkpoint to close any matured positions from the first checkpoint
        // of trading.
        alice
            .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
            .await?;

        Ok(())
    }

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
            let fixed_rate = fixed!(0.05e18);
            preamble(&mut rng, &mut alice, &mut bob, &mut celine, fixed_rate).await?;

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
            );
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
            alice.reset(Default::default());
            bob.reset(Default::default());
            celine.reset(Default::default());
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
            let fixed_rate = fixed!(0.05e18);
            preamble(&mut rng, &mut alice, &mut bob, &mut celine, fixed_rate).await?;

            // One of three things should be true after opening the long:
            //
            // 1. The pool's spot price reached the max spot price prior to
            //    considering fees.
            // 2. The pool's solvency is close to zero.
            // 3. Bob's budget is consumed.
            let max_spot_price = bob.get_state().await?.calculate_max_spot_price();
            let max_long = bob.calculate_max_long(None).await?;
            let spot_price_after_long = bob
                .get_state()
                .await?
                .calculate_spot_price_after_long(max_long, None)?;
            bob.open_long(max_long, None, None).await?;
            let is_max_price = max_spot_price - spot_price_after_long < fixed!(1e15);
            let is_solvency_consumed = {
                let state = bob.get_state().await?;
                let error_tolerance = fixed!(1_000e18).mul_div_down(fixed_rate, fixed!(0.1e18));
                state.calculate_solvency() < error_tolerance
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
            alice.reset(Default::default());
            bob.reset(Default::default());
            celine.reset(Default::default());
        }

        Ok(())
    }
}