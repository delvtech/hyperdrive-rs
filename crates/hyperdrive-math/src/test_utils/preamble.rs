use std::panic;

use ethers::{
    signers::LocalWallet,
    types::{I256, U128, U256},
};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, uint256, FixedPoint};
use hyperdrive_test_utils::{agent::Agent, chain::ChainClient};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::{test_utils::agent::HyperdriveMathAgent, State};

/// Executes random trades throughout a Hyperdrive term.
///
/// This fn initializes a Hyperdrive pool and does random trades
/// to force the pool into a random state.
pub async fn initialize_pool_with_random_state(
    rng: &mut ChaCha8Rng,
    alice: &mut Agent<ChainClient<LocalWallet>, ChaCha8Rng>,
    bob: &mut Agent<ChainClient<LocalWallet>, ChaCha8Rng>,
    celine: &mut Agent<ChainClient<LocalWallet>, ChaCha8Rng>,
) -> Result<()> {
    // Get random pool liquidity & fixed rate.
    let pool_initial_contribution = rng.gen_range(fixed!(10_000e18)..=fixed!(1_000_000_000e18));
    let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));

    // Alice initializes the pool.
    alice.fund(pool_initial_contribution).await?;
    alice
        .initialize(fixed_rate, pool_initial_contribution, None)
        .await?;

    // Determine a bounded random amount of trade sequences.
    let mut state = alice.get_state().await?;
    let total_time = fixed!(1.1e18) * state.position_duration();
    let num_steps = rng.gen_range(fixed!(1e18)..=fixed!(3e18));
    let time_delta = total_time / num_steps;

    // Advance the time for over a position term and make trades in some of the
    // checkpoints.
    let min_txn = state.minimum_transaction_amount();
    let mut time_remaining = total_time.change_type::<I256>()?;
    while time_remaining > fixed!(0) {
        state = alice.get_state().await?;
        // Bob opens a long.
        let max_long = get_max_long(&state, None)?;
        let long_amount = rng.gen_range(min_txn..=max_long);
        // Fund extra for slippage.
        bob.fund(long_amount + fixed!(10e18)).await?;
        bob.open_long(long_amount, None, None).await?;

        // Celine opens a short.
        state = alice.get_state().await?;
        let max_short = state.absolute_max_short_guess()?; // Less accurate but faster to compute.
        let short_bond_amount = if max_short == min_txn {
            min_txn
        } else {
            rng.gen_range(min_txn..=max_short)
        };
        let user_deposit_shares =
            state.calculate_open_short(short_bond_amount, state.vault_share_price())?;
        // Fund extra for slippage.
        celine
            .fund(user_deposit_shares * state.vault_share_price() + fixed!(10e18))
            .await?;
        celine.open_short(short_bond_amount, None, None).await?;
        // Advance the time and mint all of the intermediate checkpoints.
        let variable_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
        alice.advance_time(variable_rate, time_delta).await?;
        time_remaining -= time_delta.change_type::<I256>()?;
    }

    // Add some liquidity again to make sure future bots can make trades.
    let state = alice.get_state().await?;
    let min_share_reserves = state
        .calculate_min_share_reserves_given_exposure()?
        .change_type::<U256>()?;
    let positive_share_adjustment =
        FixedPoint::<U256>::try_from(state.share_adjustment().min(0.into()))?;
    let min_base = (min_share_reserves + positive_share_adjustment) * state.vault_share_price()
        + state.long_exposure();
    let liquidity_amount = rng.gen_range(min_base..=min_base + fixed!(100_000e18));
    alice.fund(liquidity_amount).await?;
    alice.add_liquidity(liquidity_amount, None).await?;
    // Advance time logner than a position duration & checkpoint one last time
    // to ensure all positions are closed.
    let variable_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
    alice
        .advance_time(variable_rate, state.position_duration() * fixed!(2e18))
        .await?;
    alice
        .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
        .await?;
    Ok(())
}

/// Conservative and safe estimate of the maximum long.
fn get_max_long(state: &State, maybe_max_num_tries: Option<usize>) -> Result<FixedPoint<U256>> {
    let max_num_tries = maybe_max_num_tries.unwrap_or(10);
    let checkpoint_exposure = I256::from(0);

    // We need a guaranteed method of picking a good upper-bound, even if underlying functions aren't working.
    // So we will first attempt to use `calculate_max_long` and then we will double check and reduce
    // the max if necessary.
    let mut max_long = match panic::catch_unwind(|| {
        state.calculate_max_long(U256::from(U128::MAX), checkpoint_exposure, Some(5))
    }) {
        Ok(max_long_no_panic) => match max_long_no_panic {
            Ok(max_long_no_err) => max_long_no_err,
            Err(_) => state.bond_reserves() * state.calculate_spot_price_down()? * fixed!(10e18),
        },
        Err(_) => state.bond_reserves() * state.calculate_spot_price_down()? * fixed!(10e18),
    };

    let mut num_tries = 0;
    let mut success = false;
    while !success {
        max_long = match panic::catch_unwind(|| state.calculate_open_long(max_long)) {
            Ok(long_result_no_panic) => match long_result_no_panic {
                Ok(_) => {
                    success = true;
                    max_long
                }
                Err(_) => max_long / fixed!(10e18),
            },
            Err(_) => max_long / fixed!(10e18),
        };
        if max_long < state.minimum_transaction_amount() {
            return Err(eyre!(
                "max_long={} was less than minimum_transaction_amount={}",
                max_long,
                state.minimum_transaction_amount()
            ));
        }
        num_tries += 1;
        if num_tries > max_num_tries {
            return Err(eyre!(
                "Failed to find a max long. Last attempted value was {}",
                max_long,
            ));
        }
    }
    Ok(max_long)
}

#[cfg(test)]
mod tests {
    use fixedpointmath::fixed;
    use hyperdrive_test_utils::{chain::TestChain, constants::SLOW_FUZZ_RUNS};
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::test_utils::agent::HyperdriveMathAgent;

    #[tokio::test]
    async fn fuzz_max_long_after_preamble() -> Result<()> {
        // Set up a random number generator. We use ChaCha8Rng with a randomly
        // generated seed, which makes it easy to reproduce test failures given
        // the seed.
        let mut rng = {
            let mut rng = thread_rng();
            let seed = rng.gen();
            ChaCha8Rng::seed_from_u64(seed)
        };

        // Initialize the test chain & agents.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let mut celine = chain.celine().await?;

        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;

            // Get state and trade details, then open the max long.
            let state = alice.get_state().await?;
            let max_long = bob.calculate_max_long(None).await?;
            assert!(max_long >= state.minimum_transaction_amount());
            bob.fund(max_long + fixed!(10e18)).await?;
            bob.open_long(max_long, None, None).await?;

            // Reset the chain & agents.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn fuzz_max_short_after_preamble() -> Result<()> {
        // Set up a random number generator. We use ChaCha8Rng with a randomly
        // generated seed, which makes it easy to reproduce test failures given
        // the seed.
        let mut rng = {
            let mut rng = thread_rng();
            let seed = rng.gen();
            ChaCha8Rng::seed_from_u64(seed)
        };
        // Initialize the test chain & agents.
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let mut celine = chain.celine().await?;
        for _ in 0..*SLOW_FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;
            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;
            // Get state.
            let state = alice.get_state().await?;
            // Check that a short is possible.
            assert!(state
                .solvency_after_short(state.minimum_transaction_amount())
                .is_ok());
            // Open the max short.
            let max_short = celine.calculate_max_short(None).await?;
            assert!(max_short >= state.minimum_transaction_amount());
            celine.fund(max_short + fixed!(10e18)).await?;
            celine.open_short(max_short, None, None).await?;
            // Reset the chain & agents.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }
        Ok(())
    }
}
