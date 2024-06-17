use std::panic;

use ethers::{signers::LocalWallet, types::U256};
use eyre::Result;
use fixed_point::{fixed, uint256, FixedPoint};
use hyperdrive_test_utils::{agent::Agent, chain::ChainClient};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::test_utils::agent::HyperdriveMathAgent;

/// Executes random trades throughout a Hyperdrive term.
///
/// This fn initializes a Hyperdrive pool and does random trades
/// to force the pool into a random state.
pub async fn preamble(
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
        let mut state = alice.get_state().await?;
        let min_txn = FixedPoint::from(state.config.minimum_transaction_amount);

        // Bob opens a long.
        let mut max_long = fixed!(1e29); // 100B max
        let mut success = false;
        let mut long_amount = rng.gen_range(min_txn..=max_long);
        while !success {
            match panic::catch_unwind(|| state.calculate_open_long(long_amount)) {
                Ok(long_result_no_panic) => match long_result_no_panic {
                    Ok(_) => success = true,
                    Err(_) => max_long /= fixed!(10e18),
                },
                Err(_) => max_long /= fixed!(10e18),
            };
            long_amount = rng.gen_range(min_txn..=max_long);
        }
        bob.open_long(long_amount, None, None).await?;
        state = alice.get_state().await?;

        // Celine opens a short.
        let mut max_short = fixed!(1e29); // 100B max
        let mut success = false;
        let mut short_amount = rng.gen_range(min_txn..=max_short);
        while !success {
            match panic::catch_unwind(|| {
                state.calculate_open_short(short_amount, state.vault_share_price())
            }) {
                Ok(short_result_no_panic) => match short_result_no_panic {
                    Ok(_) => success = true,
                    Err(_) => max_short /= fixed!(10e18),
                },
                Err(_) => max_short /= fixed!(10e18),
            };
            short_amount = rng.gen_range(min_txn..=max_short);
        }
        celine.open_short(short_amount, None, None).await?;

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
