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
    let pool_initial_contribution = rng.gen_range(fixed!(1_000e18)..=fixed!(1_000_000_000e18));
    let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));

    // Alice initializes the pool.
    alice.fund(pool_initial_contribution).await?;
    alice
        .initialize(fixed_rate, pool_initial_contribution, None)
        .await?;

    // Advance the time for over a term and make trades in some of the checkpoints.
    let mut time_remaining = alice.get_config().position_duration;
    while time_remaining > uint256!(0) {
        let mut state = alice.get_state().await?;
        let min_txn = state.minimum_transaction_amount();

        // Bob opens a long.
        let max_long = get_max_long(state, None)?;
        let long_amount = rng.gen_range(min_txn..=max_long);
        bob.fund(long_amount + fixed!(10e18)).await?; // Fund a little extra for slippage.
        bob.open_long(long_amount, None, None).await?;

        // Celine opens a short.
        state = alice.get_state().await?;

        // Get the current checkpoint exposure.
        let checkpoint_exposure = alice
            .get_checkpoint_exposure(state.to_checkpoint(alice.now().await?))
            .await?;
        let max_short = get_max_short(state, checkpoint_exposure, None)?;
        let short_amount = rng.gen_range(min_txn..=max_short);
        celine.fund(short_amount + fixed!(10e18)).await?; // Fund a little extra for slippage.
        celine.open_short(short_amount, None, None).await?;

        // Advance the time and mint all of the intermediate checkpoints.
        let multiplier = rng.gen_range(fixed!(10e18)..=fixed!(100e18));
        let delta = FixedPoint::from(time_remaining)
            .min(FixedPoint::from(alice.get_config().checkpoint_duration) * multiplier);
        time_remaining -= U256::from(delta);
        let variable_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
        alice.advance_time(variable_rate, delta).await?;
    }

    // Mint a checkpoint to close any matured positions.
    alice
        .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
        .await?;

    // Add some liquidity again to make sure future bots can make trades.
    let liquidity_amount = rng.gen_range(fixed!(1_000e18)..=fixed!(100_000_000e18));
    alice.fund(liquidity_amount).await?;
    alice.add_liquidity(liquidity_amount, None).await?;

    Ok(())
}

/// Conservative and safe estimate of the maximum long.
fn get_max_long(state: State, maybe_max_num_tries: Option<usize>) -> Result<FixedPoint<U256>> {
    let max_num_tries = maybe_max_num_tries.unwrap_or(10);
    let checkpoint_exposure = I256::from(0);

    // We need a guaranteed method of picking a good upper-bound, even if underlying functions aren't working.
    // So we will first attempt to use `calculate_max_long` and then we will double check and reduce
    // the max if necessary.
    let mut max_long = match panic::catch_unwind(|| {
        state.calculate_max_long(U256::from(U128::MAX), checkpoint_exposure, Some(3))
    }) {
        Ok(max_long_no_panic) => match max_long_no_panic {
            Ok(max_long_no_err) => max_long_no_err,
            Err(_) => state.bond_reserves() * state.calculate_spot_price()? * fixed!(10e18),
        },
        Err(_) => state.bond_reserves() * state.calculate_spot_price()? * fixed!(10e18),
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

/// Conservative and safe estimate of the maximum short.
fn get_max_short(
    state: State,
    checkpoint_exposure: I256,
    maybe_max_num_tries: Option<usize>,
) -> Result<FixedPoint<U256>> {
    let max_num_tries = maybe_max_num_tries.unwrap_or(10);

    // We linearly interpolate between the current spot price and the minimum
    // price that the pool can support. This is a conservative estimate of
    // the short's realized price.
    let conservative_price = {
        // We estimate the minimum price that short will pay by a
        // weighted average of the spot price and the minimum possible
        // spot price the pool can quote. We choose the weights so that this
        // is an underestimate of the worst case realized price.
        let spot_price = state.calculate_spot_price()?;
        let min_price = state.calculate_min_spot_price()?;

        // Calculate the linear interpolation.
        let weight = fixed!(1e18).pow(fixed!(1e18) - state.time_stretch())?;
        spot_price * (fixed!(1e18) - weight) + min_price * weight
    };

    // Compute the max short.
    let mut max_short = match panic::catch_unwind(|| {
        state.calculate_max_short(
            U256::from(U128::MAX),
            state.vault_share_price(),
            checkpoint_exposure,
            Some(conservative_price),
            Some(3),
        )
    }) {
        Ok(max_short_no_panic) => match max_short_no_panic {
            Ok(max_short) => max_short,
            Err(_) => state.share_reserves() / state.vault_share_price() * fixed!(10e18),
        },
        Err(_) => state.share_reserves() / state.vault_share_price() * fixed!(10e18),
    };
    let mut num_tries = 0;
    let mut success = false;
    while !success {
        max_short = match panic::catch_unwind(|| {
            state.calculate_open_short(max_short, state.vault_share_price())
        }) {
            Ok(short_result_no_panic) => match short_result_no_panic {
                Ok(_) => {
                    success = true;
                    max_short
                }
                Err(_) => max_short / fixed!(10e18),
            },
            Err(_) => max_short / fixed!(10e18),
        };
        if max_short < state.minimum_transaction_amount() {
            return Err(eyre!(
                "max_short={} was less than minimum_transaction_amount={}.",
                max_short,
                state.minimum_transaction_amount()
            ));
        }
        num_tries += 1;
        if num_tries > max_num_tries {
            return Err(eyre!(
                "Failed to find a max short. Last attempted value was {}",
                max_short,
            ));
        }
    }
    Ok(max_short)
}

#[cfg(test)]
mod tests {
    use fixedpointmath::fixed;
    use hyperdrive_test_utils::{chain::TestChain, constants::FUZZ_RUNS};
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

        for _ in 0..*FUZZ_RUNS {
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

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Run the preamble.
            initialize_pool_with_random_state(&mut rng, &mut alice, &mut bob, &mut celine).await?;

            // Get state and trade details, then open the max short.
            let state = alice.get_state().await?;
            let max_short = bob.calculate_max_short(None).await?;
            assert!(max_short >= state.minimum_transaction_amount());
            bob.fund(max_short + fixed!(10e18)).await?;
            bob.open_short(max_short, None, None).await?;

            // Reset the chain & agents.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
            celine.reset(Default::default()).await?;
        }
        Ok(())
    }
}
