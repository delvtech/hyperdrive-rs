use std::{cmp::min, collections::btree_map::Entry};

use ethers::{prelude::EthLogDecode, signers::LocalWallet, types::U256};
use eyre::Result;
use fixedpointmath::{fixed, uint256, FixedPoint};
use hyperdrive_test_utils::{
    agent::{Agent, ContractCall, TxOptions},
    chain::ChainClient,
};
use hyperdrive_wrappers::wrappers::ihyperdrive::{Checkpoint, IHyperdriveEvents, Options};
use rand_chacha::ChaCha8Rng;
use tracing::instrument;

use crate::State;

pub trait HyperdriveMathAgent {
    /// Gets the current state of the pool.
    async fn get_state(&self) -> Result<State>;

    /// Gets the latest checkpoint.
    async fn latest_checkpoint(&self) -> Result<U256>;

    /// Calculates the spot price.
    async fn calculate_spot_price(&self) -> Result<FixedPoint<U256>>;

    /// Calculates the long amount that will be opened for a given base amount.
    async fn calculate_open_long(&self, base_amount: FixedPoint<U256>) -> Result<FixedPoint<U256>>;

    /// Calculates the deposit required to short a given amount of bonds with the
    /// current market state.
    async fn calculate_open_short(&self, bond_amount: FixedPoint<U256>)
        -> Result<FixedPoint<U256>>;

    /// Calculates the max long that can be opened in the current checkpoint.
    async fn calculate_max_long(
        &self,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>>;

    /// Gets the long that moves the fixed rate to a target value.
    async fn calculate_targeted_long(
        &self,
        target_rate: FixedPoint<U256>,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>>;

    /// Calculates the max short that can be opened in the current checkpoint.
    ///
    /// Since interest can accrue between the time the calculation is made and
    /// the transaction is submitted, it's convenient to have a slippage
    /// tolerance to lower the revert rate.
    async fn calculate_max_short(
        &self,
        maybe_slippage_tolerance: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>>;

    async fn open_long(
        &mut self,
        base_paid: FixedPoint<U256>,
        maybe_slippage_tolerance: Option<FixedPoint<U256>>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()>;

    async fn close_long(
        &mut self,
        maturity_time: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()>;

    async fn open_short(
        &mut self,
        bond_amount: FixedPoint<U256>,
        maybe_slippage_tolerance: Option<FixedPoint<U256>>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<(FixedPoint<U256>, FixedPoint<U256>)>;

    async fn close_short(
        &mut self,
        maturity_time: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<FixedPoint<U256>>;
}

impl HyperdriveMathAgent for Agent<ChainClient<LocalWallet>, ChaCha8Rng> {
    /// Gets the current state of the pool.
    async fn get_state(&self) -> Result<State> {
        let pool_config = self.get_config().clone();
        let pool_info = self.hyperdrive().get_pool_info().await?;
        Ok(State::new(pool_config, pool_info))
    }

    /// Gets the latest checkpoint.
    async fn latest_checkpoint(&self) -> Result<U256> {
        Ok(self.get_state().await?.to_checkpoint(self.now().await?))
    }
    /// Calculates the spot price.
    async fn calculate_spot_price(&self) -> Result<FixedPoint<U256>> {
        self.get_state().await?.calculate_spot_price()
    }

    /// Calculates the long amount that will be opened for a given base amount.
    async fn calculate_open_long(&self, base_amount: FixedPoint<U256>) -> Result<FixedPoint<U256>> {
        let state = self.get_state().await?;
        state.calculate_open_long(base_amount)
    }
    /// Calculates the deposit required to short a given amount of bonds with the
    /// current market state.
    async fn calculate_open_short(
        &self,
        bond_amount: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        let state = self.get_state().await?;
        let Checkpoint {
            vault_share_price: open_vault_share_price,
            ..
        } = self.get_checkpoint(self.latest_checkpoint().await?).await?;
        state.calculate_open_short(bond_amount, open_vault_share_price.into())
    }

    /// Calculates the max long that can be opened in the current checkpoint.
    async fn calculate_max_long(
        &self,
        maybe_max_iterations: Option<usize>,
    ) -> Result<FixedPoint<U256>> {
        let state = self.get_state().await?;
        let checkpoint_exposure = self
            .hyperdrive()
            .get_checkpoint_exposure(state.to_checkpoint(self.now().await?))
            .await?;
        Ok(
            state.calculate_max_long(
                self.wallet.base,
                checkpoint_exposure,
                maybe_max_iterations,
            )?,
        )
    }

    /// Gets the long that moves the fixed rate to a target value.
    async fn calculate_targeted_long(
        &self,
        target_rate: FixedPoint<U256>,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let state = self.get_state().await?;
        let checkpoint_exposure = self
            .hyperdrive()
            .get_checkpoint_exposure(state.to_checkpoint(self.now().await?))
            .await?;
        Ok(state
            .calculate_targeted_long_with_budget(
                self.wallet.base,
                target_rate,
                checkpoint_exposure,
                maybe_max_iterations,
                maybe_allowable_error,
            )
            .unwrap())
    }

    /// Calculates the max short that can be opened in the current checkpoint.
    ///
    /// Since interest can accrue between the time the calculation is made and
    /// the transaction is submitted, it's convenient to have a slippage
    /// tolerance to lower the revert rate.
    async fn calculate_max_short(
        &self,
        maybe_slippage_tolerance: Option<FixedPoint<U256>>,
    ) -> Result<FixedPoint<U256>> {
        let budget =
            self.wallet.base * (fixed!(1e18) - maybe_slippage_tolerance.unwrap_or(fixed!(0.01e18)));

        let latest_checkpoint = self.latest_checkpoint().await?;
        let Checkpoint {
            vault_share_price: open_vault_share_price,
            ..
        } = self.get_checkpoint(latest_checkpoint).await?;
        let checkpoint_exposure = self.get_checkpoint_exposure(latest_checkpoint).await?;
        let state = self.get_state().await?;

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
            let base_reserves = FixedPoint::from(state.info.vault_share_price)
                * (FixedPoint::from(state.info.share_reserves));
            let weight = (min(self.wallet.base, base_reserves) / base_reserves)
                .pow(fixed!(1e18) - FixedPoint::from(self.get_config().time_stretch))?;
            spot_price * (fixed!(1e18) - weight) + min_price * weight
        };

        state.calculate_max_short(
            budget,
            open_vault_share_price,
            checkpoint_exposure,
            Some(conservative_price),
            None,
        )
    }

    #[instrument(skip(self))]
    async fn open_long(
        &mut self,
        base_paid: FixedPoint<U256>,
        maybe_slippage_tolerance: Option<FixedPoint<U256>>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()> {
        // Ensure that the agent has a sufficient base balance to open the long.
        if self.wallet.base < base_paid {
            return Err(eyre::eyre!(
                "insufficient base balance to open long: {:?} < {:?}",
                self.wallet.base,
                base_paid
            ));
        }

        // Decrease the wallet's base balance.
        self.wallet.base -= base_paid;

        // Open the long and record the trade in the wallet.
        let log = {
            let min_output = {
                let slippage_tolerance = maybe_slippage_tolerance.unwrap_or(fixed!(0.01e18));
                self.calculate_open_long(base_paid).await? * (fixed!(1e18) - slippage_tolerance)
            };
            let tx = ContractCall(self.hyperdrive().open_long(
                base_paid.into(),
                min_output.into(),
                fixed!(0).into(), // TODO: This is fine for testing, but not prod.
                Options {
                    destination: self.client().address(),
                    as_base: true,
                    extra_data: [].into(),
                },
            ))
            .apply(self.pre_process_options(maybe_tx_options));
            let logs =
                tx.0.send()
                    .await?
                    .await?
                    .unwrap()
                    .logs
                    .into_iter()
                    .filter_map(|log| {
                        if let Ok(IHyperdriveEvents::OpenLongFilter(log)) =
                            IHyperdriveEvents::decode_log(&log.into())
                        {
                            Some(log)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            logs[0].clone()
        };
        *self
            .wallet
            .longs
            .entry(log.maturity_time.into())
            .or_default() += log.bond_amount.into();

        Ok(())
    }

    #[instrument(skip(self))]
    async fn close_long(
        &mut self,
        maturity_time: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()> {
        // TODO: It would probably be better for this part of the agent to just
        // be a dumb wrapper around Hyperdrive. It's going to be useful to test
        // with inputs that we'd consider invalid.
        //
        // If the wallet has a sufficient balance of longs, update the long
        // balance. Otherwise, return an error.
        match self.wallet.longs.entry(maturity_time) {
            Entry::Occupied(mut entry) => {
                let long_balance = entry.get();
                if *long_balance > bond_amount {
                    entry.insert(*long_balance - bond_amount);
                } else if *long_balance == bond_amount {
                    entry.remove();
                } else {
                    return Err(eyre::eyre!(
                        "insufficient long balance to close long: {:?} < {:?}",
                        long_balance,
                        bond_amount
                    ));
                }
            }
            Entry::Vacant(_) => {
                return Err(eyre::eyre!("no longs to close"));
            }
        }

        // Close the long and increase the wallet's base balance.
        let log = {
            let tx = ContractCall(self.hyperdrive().close_long(
                maturity_time.into(),
                bond_amount.into(),
                uint256!(0),
                Options {
                    destination: self.client().address(),
                    as_base: true,
                    extra_data: [].into(),
                },
            ))
            .apply(self.pre_process_options(maybe_tx_options));
            let logs =
                tx.0.send()
                    .await?
                    .await?
                    .unwrap()
                    .logs
                    .into_iter()
                    .filter_map(|log| {
                        if let Ok(IHyperdriveEvents::CloseLongFilter(log)) =
                            IHyperdriveEvents::decode_log(&log.into())
                        {
                            Some(log)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            logs[0].clone()
        };
        // We ensure trades here are executed as base
        // Panic here since we pass as_base=True in the call.
        if !log.as_base {
            panic!("Trades are expected to be executed as base.")
        }
        self.wallet.base += log.amount.into();

        Ok(())
    }

    /// Shorts ///

    #[instrument(skip(self))]
    async fn open_short(
        &mut self,
        bond_amount: FixedPoint<U256>,
        maybe_slippage_tolerance: Option<FixedPoint<U256>>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<(FixedPoint<U256>, FixedPoint<U256>)> {
        // Open the short and record the trade in the wallet.
        let log = {
            let max_deposit = {
                let slippage_tolerance = maybe_slippage_tolerance.unwrap_or(fixed!(0.01e18));
                self.calculate_open_short(bond_amount).await? * (fixed!(1e18) + slippage_tolerance)
            };
            let tx = ContractCall(self.hyperdrive().open_short(
                bond_amount.into(),
                max_deposit.into(),
                fixed!(0).into(), // TODO: This is fine for testing, but not prod.
                Options {
                    destination: self.client().address(),
                    as_base: true,
                    extra_data: [].into(),
                },
            ))
            .apply(self.pre_process_options(maybe_tx_options));
            let logs =
                tx.0.send()
                    .await?
                    .await?
                    .unwrap()
                    .logs
                    .into_iter()
                    .filter_map(|log| {
                        if let Ok(IHyperdriveEvents::OpenShortFilter(log)) =
                            IHyperdriveEvents::decode_log(&log.into())
                        {
                            Some(log)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            logs[0].clone()
        };
        *self
            .wallet
            .shorts
            .entry(log.maturity_time.into())
            .or_default() += log.bond_amount.into();

        // Decrease the wallet's base balance.
        // We ensure trades here are executed as base
        // Panic here since we pass as_base=True in the call.
        if !log.as_base {
            panic!("Trades are expected to be executed as base.")
        }
        self.wallet.base -= log.amount.into();

        Ok((log.maturity_time.into(), log.amount.into()))
    }

    #[instrument(skip(self))]
    async fn close_short(
        &mut self,
        maturity_time: FixedPoint<U256>,
        bond_amount: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<FixedPoint<U256>> {
        // If the wallet has a sufficient balance of shorts, update the short
        // balance. Otherwise, return an error.
        match self.wallet.shorts.entry(maturity_time) {
            Entry::Occupied(mut entry) => {
                let short_balance = entry.get();
                if *short_balance > bond_amount {
                    entry.insert(*short_balance - bond_amount);
                } else if *short_balance == bond_amount {
                    entry.remove();
                } else {
                    return Err(eyre::eyre!(
                        "insufficient short balance to close short: {:?} < {:?}",
                        short_balance,
                        bond_amount
                    ));
                }
            }
            Entry::Vacant(_) => {
                return Err(eyre::eyre!("no shorts to close"));
            }
        }

        // Close the long and increase the wallet's base balance.
        let log = {
            let tx = ContractCall(self.hyperdrive().close_short(
                maturity_time.into(),
                bond_amount.into(),
                uint256!(0),
                Options {
                    destination: self.client().address(),
                    as_base: true,
                    extra_data: [].into(),
                },
            ))
            .apply(self.pre_process_options(maybe_tx_options));
            let logs =
                tx.0.send()
                    .await?
                    .await?
                    .unwrap()
                    .logs
                    .into_iter()
                    .filter_map(|log| {
                        if let Ok(IHyperdriveEvents::CloseShortFilter(log)) =
                            IHyperdriveEvents::decode_log(&log.into())
                        {
                            Some(log)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            logs[0].clone()
        };
        // We ensure trades here are executed as base
        // Panic here since we pass as_base=True in the call.
        if !log.as_base {
            panic!("Trades are expected to be executed as base.")
        }
        self.wallet.base += log.amount.into();

        Ok(log.amount.into())
    }
}
