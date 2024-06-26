use eyre::{eyre, Result};
use fixed_point::FixedPoint;

use crate::{calculate_rate_given_fixed_price, State, YieldSpace};

impl State {
    /// Calculates the long amount that will be opened for a given base amount.
    ///
    /// The long amount `$y(x)$` that a trader will receive is given by:
    ///
    /// ```math
    /// y(x) = y_{*}(x) - c(x)
    /// ```
    ///
    /// Where `$y_{*}(x)$` is the amount of long that would be opened if there was
    /// no curve fee and `$c(x)$` is the
    /// [curve fee](State::open_long_curve_fee). `$y_{*}(x)$` is given by:
    ///
    /// ```math
    /// y_{*}(x) = y - \left(
    ///                k - \tfrac{c}{\mu} \cdot \left(
    ///                    \mu \cdot \left( z + \tfrac{x}{c}
    ///                \right) \right)^{1 - t_s}
    ///            \right)^{\tfrac{1}{1 - t_s}}
    /// ```
    pub fn calculate_open_long<F: Into<FixedPoint>>(&self, base_amount: F) -> Result<FixedPoint> {
        let base_amount = base_amount.into();

        if base_amount < self.minimum_transaction_amount() {
            return Err(eyre!("MinimumTransactionAmount: Input amount too low",));
        }

        let bond_amount =
            self.calculate_bonds_out_given_shares_in_down(base_amount / self.vault_share_price())?;

        // Throw an error if opening the long would result in negative interest.
        let ending_spot_price =
            self.calculate_spot_price_after_long(base_amount, bond_amount.into())?;
        let max_spot_price = self.calculate_max_spot_price()?;
        if ending_spot_price > max_spot_price {
            return Err(eyre!("InsufficientLiquidity: Negative Interest",));
        }

        Ok(bond_amount - self.open_long_curve_fee(base_amount)?)
    }

    /// Calculate an updated pool state after opening a long.
    ///
    /// For a given base amount and bond amount, base is converted to
    /// shares and the reserves are updated such that
    /// `state.bond_reserves -= bond_amount` and
    /// `state.share_reserves += base_amount / vault_share_price`.
    pub fn calculate_pool_state_after_open_long(
        &self,
        base_amount: FixedPoint,
        maybe_bond_amount: Option<FixedPoint>,
    ) -> Result<Self> {
        let bond_amount = match maybe_bond_amount {
            Some(bond_amount) => bond_amount,
            None => self.calculate_open_long(base_amount)?,
        };
        let mut state = self.clone();
        state.info.bond_reserves -= bond_amount.into();
        state.info.share_reserves += (base_amount / self.vault_share_price()).into();
        Ok(state)
    }

    /// Calculate the share deltas to be applied to the pool after opening a long.
    pub fn calculate_pool_deltas_after_open_long(
        &self,
        base_amount: FixedPoint,
    ) -> Result<FixedPoint> {
        let bond_amount = self.calculate_open_long(base_amount)?;
        Ok(bond_amount)
    }

    /// Calculates the spot price after opening a Hyperdrive long.
    /// If a bond_amount is not provided, then one is estimated using `calculate_open_long`.
    pub fn calculate_spot_price_after_long(
        &self,
        base_amount: FixedPoint,
        maybe_bond_amount: Option<FixedPoint>,
    ) -> Result<FixedPoint> {
        let bond_amount = match maybe_bond_amount {
            Some(bond_amount) => bond_amount,
            None => self.calculate_open_long(base_amount)?,
        };
        let mut state: State = self.clone();
        state.info.bond_reserves -= bond_amount.into();
        state.info.share_reserves += (base_amount / state.vault_share_price()
            - self.open_long_governance_fee(base_amount, None)? / state.vault_share_price())
        .into();
        state.calculate_spot_price()
    }

    /// Calculate the spot rate after a long has been opened.
    /// If a bond_amount is not provided, then one is estimated using
    /// [calculate_open_long](State::calculate_open_long).
    ///
    /// We calculate the rate for a fixed length of time as:
    ///
    /// ```math
    /// r(\Delta y) = \frac{1 - p(\Delta y)}{p(\Delta y) t}
    /// ```
    ///
    /// where `$p(x)$` is the spot price after a long for `delta_base` `$= x$`
    /// and `$t$` is the normalized position druation.
    ///
    /// In this case, we use the resulting spot price after a hypothetical long
    /// for `base_amount` is opened.
    pub fn calculate_spot_rate_after_long(
        &self,
        base_amount: FixedPoint,
        maybe_bond_amount: Option<FixedPoint>,
    ) -> Result<FixedPoint> {
        Ok(calculate_rate_given_fixed_price(
            self.calculate_spot_price_after_long(base_amount, maybe_bond_amount)?,
            self.position_duration(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use ethers::types::{I256, U256};
    use fixed_point::fixed;
    use hyperdrive_test_utils::{chain::TestChain, constants::FUZZ_RUNS};
    use hyperdrive_wrappers::wrappers::ihyperdrive::Options;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::test_utils::{
        agent::HyperdriveMathAgent, preamble::initialize_pool_with_random_state,
    };

    #[tokio::test]
    async fn fuzz_calculate_spot_price_after_long() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob. Alice
        // is funded with a large amount of capital so that she can initialize
        // the pool. Bob is funded with a small amount of capital so that we
        // can test opening a long and verify that the ending spot price is what
        // we expect.
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

            // Attempt to predict the spot price after opening a long.
            let base_paid = rng.gen_range(fixed!(0.1e18)..=bob.calculate_max_long(None).await?);
            let expected_spot_price = bob
                .get_state()
                .await?
                .calculate_spot_price_after_long(base_paid, None)?;

            // Open the long.
            bob.open_long(base_paid, None, None).await?;

            // Verify that the predicted spot price is equal to the ending spot
            // price. These won't be exactly equal because the vault share price
            // increases between the prediction and opening the long.
            let actual_spot_price = bob.get_state().await?.calculate_spot_price()?;
            let delta = if actual_spot_price > expected_spot_price {
                actual_spot_price - expected_spot_price
            } else {
                expected_spot_price - actual_spot_price
            };
            let tolerance = fixed!(1e9);
            assert!(
                delta < tolerance,
                "expected: delta = {} < {} = tolerance",
                delta,
                tolerance
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_spot_rate_after_long() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob. Alice
        // is funded with a large amount of capital so that she can initialize
        // the pool. Bob is funded with a small amount of capital so that we
        // can test opening a long and verify that the ending spot rate is what
        // we expect.
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

            // Attempt to predict the spot price after opening a long.
            let base_paid = rng.gen_range(
                alice.get_state().await?.minimum_transaction_amount()
                    ..=bob.calculate_max_long(None).await?,
            );
            let expected_spot_rate = bob
                .get_state()
                .await?
                .calculate_spot_rate_after_long(base_paid, None)?;

            // Open the long.
            bob.open_long(base_paid, None, None).await?;

            // Verify that the predicted spot rate is equal to the ending spot
            // rate. These won't be exactly equal because the vault share price
            // increases between the prediction and opening the long.
            let actual_spot_rate = bob.get_state().await?.calculate_spot_rate()?;
            let delta = if actual_spot_rate > expected_spot_rate {
                actual_spot_rate - expected_spot_rate
            } else {
                expected_spot_rate - actual_spot_rate
            };
            let tolerance = fixed!(1e9);
            assert!(
                delta < tolerance,
                "expected: delta = {} < {} = tolerance",
                delta,
                tolerance
            );

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }

    // Tests open long with an amount smaller than the minimum.
    #[tokio::test]
    async fn test_error_open_long_min_txn_amount() -> Result<()> {
        let mut rng = thread_rng();
        let state = rng.gen::<State>();
        let result = state.calculate_open_long(state.config.minimum_transaction_amount - 10);
        assert!(result.is_err());
        Ok(())
    }

    // Tests open long with an amount larger than the maximum.
    #[tokio::test]
    async fn fuzz_error_open_long_max_txn_amount() -> Result<()> {
        // This amount gets added to the max trade to cause a failure.
        // TODO: You should be able to add a small amount (e.g. 1e18) to max to fail.
        // calc_open_long or calc_max_long must be incorrect for the additional
        // amount to have to be so large.
        let max_base_delta = fixed!(1_000_000_000e18);

        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            let state = rng.gen::<State>();
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::try_from(I256::MAX)?);
                if rng.gen() {
                    -I256::try_from(value)?
                } else {
                    I256::try_from(value)?
                }
            };
            let max_iterations = 7;
            // TODO: We should use calculate_absolute_max_long here because that is what we are testing.
            // We need to catch panics because of FixedPoint overflows & underflows.
            let max_trade = panic::catch_unwind(|| {
                state.calculate_max_long(U256::MAX, checkpoint_exposure, Some(max_iterations))
            });
            // Since we're fuzzing it's possible that the max can fail.
            // We're only going to use it in this test if it succeeded.
            match max_trade {
                Ok(max_trade) => match max_trade {
                    Ok(max_trade) => {
                        let base_amount = max_trade + max_base_delta;
                        let bond_amount =
                            panic::catch_unwind(|| state.calculate_open_long(base_amount));
                        match bond_amount {
                            Ok(result) => match result {
                                Ok(_) => {
                                    return Err(eyre!(
                                        format!(
                                            "calculate_open_long for {} base should have failed but succeeded.",
                                            base_amount,
                                        )
                                    ));
                                }
                                Err(_) => continue, // Open threw an Err.
                            },
                            Err(_) => continue, // Open threw a panic, likely due to FixedPoint under/over flow.
                        }
                    }
                    Err(_) => continue, // Max threw an Err.
                },
                Err(_) => continue, // Max thew an panic, likely due to FixedPoint under/over flow.
            }
        }

        Ok(())
    }

    #[tokio::test]
    pub async fn fuzz_sol_calc_open_long() -> Result<()> {
        let tolerance = fixed!(1e3);

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
            let max_long = bob.calculate_max_long(None).await?;
            let base_amount = rng.gen_range(min_txn_amount..=max_long);

            // Fund a little extra to allow for of slippage.
            bob.fund(base_amount + base_amount * fixed!(0.001e18))
                .await?;
            match bob
                .hyperdrive()
                .open_long(
                    base_amount.into(),
                    fixed!(0).into(),
                    fixed!(0).into(),
                    Options {
                        destination: bob.address(),
                        as_base: true,
                        extra_data: [].into(),
                    },
                )
                .call()
                .await
            {
                Ok((_, sol_bonds)) => {
                    // Anvil ticks the block before applying solidity fn; update state with new price.
                    let new_vault_share_price = alice.get_state().await?.vault_share_price();
                    state.info.vault_share_price = new_vault_share_price.into();
                    let rust_bonds = state.calculate_open_long(base_amount);

                    // Compare the Rust open long call output against calculate_open_long.
                    let rust_bonds_unwrapped = rust_bonds.unwrap();
                    let error = if rust_bonds_unwrapped >= sol_bonds.into() {
                        rust_bonds_unwrapped - FixedPoint::from(sol_bonds)
                    } else {
                        FixedPoint::from(sol_bonds) - rust_bonds_unwrapped
                    };
                    assert!(
                        error <= tolerance,
                        "error {} exceeds tolerance of {}",
                        error,
                        tolerance
                    );
                }
                Err(sol_err) => {
                    // Anvil ticks the block before applying solidity fn; update state with new price.
                    let new_vault_share_price = alice.get_state().await?.vault_share_price();
                    state.info.vault_share_price = new_vault_share_price.into();
                    let rust_bonds = state.calculate_open_long(base_amount);
                    assert!(
                        rust_bonds.is_err(),
                        "sol_err={:#?}, but rust_bonds={:#?} did not error",
                        sol_err,
                        rust_bonds
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
