use std::{collections::BTreeMap, fmt, sync::Arc, time::Duration};

use ethers::{
    abi::Detokenize,
    contract::ContractCall as ContractCallEthers,
    prelude::EthLogDecode,
    providers::Middleware,
    signers::LocalWallet,
    types::{Address, BlockId, I256, U256, U64},
};
use eyre::Result;
use fixedpointmath::{uint256, FixedPoint};
use hyperdrive_wrappers::wrappers::{
    erc20_mintable::ERC20Mintable,
    ihyperdrive::{Checkpoint, IHyperdrive, IHyperdriveEvents, Options, PoolConfig},
    mock_erc4626::MockERC4626,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use test_utils::chain::ChainClient;
use tokio::time::sleep;
use tracing::instrument;

use crate::addresses::Addresses;

#[derive(Clone, Default)]
pub struct Wallet {
    pub base: FixedPoint<U256>,
    pub lp_shares: FixedPoint<U256>,
    pub withdrawal_shares: FixedPoint<U256>,
    pub longs: BTreeMap<FixedPoint<U256>, FixedPoint<U256>>,
    pub shorts: BTreeMap<FixedPoint<U256>, FixedPoint<U256>>,
}

// TODO: This struct needs to be cleaned up.
//
/// An agent that interacts with the Hyperdrive protocol and records its
/// balances of longs, shorts, base, and lp shares (both active and withdrawal
/// shares).
pub struct Agent<M, R: Rng + SeedableRng> {
    client: Arc<ChainClient<LocalWallet>>,
    hyperdrive: IHyperdrive<M>,
    vault: MockERC4626<M>,
    base: ERC20Mintable<M>,
    config: PoolConfig,
    pub wallet: Wallet,
    // TODO: It would probably be better to store an Arc<R> here so that all of
    // the agents reference the same Rng.
    rng: R,
    seed: u64,
}

impl<M, R: Rng + SeedableRng> fmt::Debug for Agent<M, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Agent")
            .field("address", &self.client.address())
            .field("seed", &self.seed)
            .field("base", &self.wallet.base)
            .field("lp_shares", &self.wallet.lp_shares)
            .field("withdrawal_shares", &self.wallet.withdrawal_shares)
            .field("longs", &self.wallet.longs)
            .field("shorts", &self.wallet.shorts)
            .finish()
    }
}

#[derive(Clone, Default, Debug)]
pub struct TxOptions {
    from: Option<Address>,
    gas: Option<U256>,
    gas_price: Option<U256>,
    value: Option<U256>,
    block: Option<BlockId>,
    is_legacy: bool,
}

impl TxOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(mut self, from: Address) -> Self {
        self.from = Some(from);
        self
    }

    pub fn gas(mut self, gas: U256) -> Self {
        self.gas = Some(gas);
        self
    }

    pub fn gas_price(mut self, gas_price: U256) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    pub fn value(mut self, value: U256) -> Self {
        self.value = Some(value);
        self
    }

    pub fn block(mut self, block: BlockId) -> Self {
        self.block = Some(block);
        self
    }

    pub fn legacy(mut self) -> Self {
        self.is_legacy = true;
        self
    }
}

/// A helper struct that makes it easy to apply transaction options to a
/// contract call.
pub struct ContractCall<M, D>(pub ContractCallEthers<M, D>);

impl<M, D: Detokenize> ContractCall<M, D> {
    pub fn apply(self, tx_options: TxOptions) -> Self {
        let mut call = self.0;
        if let Some(from) = tx_options.from {
            call = call.from(from);
        }
        if let Some(gas) = tx_options.gas {
            call = call.gas(gas);
        }
        if let Some(gas_price) = tx_options.gas_price {
            call = call.gas_price(gas_price);
        }
        if let Some(value) = tx_options.value {
            call = call.value(value);
        }
        if let Some(block) = tx_options.block {
            call = call.block(block);
        }
        if tx_options.is_legacy {
            call = call.legacy();
        }

        ContractCall(call)
    }
}

// TODO: This should crash gracefully and would ideally dump the replication
// information to a file that can be read by the framework to easily debug what
// happened.
impl Agent<ChainClient<LocalWallet>, ChaCha8Rng> {
    /// Setup ///

    pub async fn new(
        client: Arc<ChainClient<LocalWallet>>,
        addresses: Addresses,
        maybe_seed: Option<u64>,
    ) -> Result<Self> {
        let seed = maybe_seed.unwrap_or(17);
        let base = ERC20Mintable::new(addresses.base_token, client.clone());
        let vault = IHyperdrive::new(addresses.erc4626_hyperdrive, client.clone())
            .vault_shares_token()
            .call()
            .await?;
        let vault = MockERC4626::new(vault, client.clone());
        // TODO: Eventually, the agent should be able to support several
        // different pools simultaneously.
        let hyperdrive = IHyperdrive::new(addresses.erc4626_hyperdrive, client.clone());
        Ok(Self {
            client,
            hyperdrive: hyperdrive.clone(),
            vault,
            base,
            config: hyperdrive.get_pool_config().call().await?,
            wallet: Wallet::default(),
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        })
    }

    pub fn client(&self) -> Arc<ChainClient<LocalWallet>> {
        self.client.clone()
    }

    pub fn hyperdrive(&self) -> &IHyperdrive<ChainClient<LocalWallet>> {
        &self.hyperdrive
    }

    pub fn vault(&self) -> &MockERC4626<ChainClient<LocalWallet>> {
        &self.vault
    }

    /// LPs ///

    #[instrument(skip(self))]
    pub async fn initialize(
        &mut self,
        rate: FixedPoint<U256>,
        contribution: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()> {
        // Ensure that the agent has a sufficient base balance to initialize the pool.
        if self.wallet.base < contribution {
            return Err(eyre::eyre!(
                "insufficient base balance to initialize: {:?} < {:?}",
                self.wallet.base,
                contribution
            ));
        }
        self.wallet.base -= contribution;

        // Initialize the pool and record the LP shares that were received in the wallet.
        let log = {
            let tx = ContractCall(self.hyperdrive.initialize(
                contribution.into(),
                rate.into(),
                Options {
                    destination: self.client.address(),
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
                        if let Ok(IHyperdriveEvents::InitializeFilter(log)) =
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
        self.wallet.lp_shares = log.lp_amount.into();

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn add_liquidity(
        &mut self,
        contribution: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()> {
        // Ensure that the agent has a sufficient base balance to add liquidity.
        if self.wallet.base < contribution {
            return Err(eyre::eyre!(
                "insufficient base balance to add liquidity: {:?} < {:?}",
                self.wallet.base,
                contribution
            ));
        }
        self.wallet.base -= contribution;

        // Add liquidity and record the LP shares that were received in the wallet.
        let log = {
            let tx = ContractCall(self.hyperdrive.add_liquidity(
                contribution.into(),
                uint256!(0),
                uint256!(0),
                U256::MAX,
                Options {
                    destination: self.client.address(),
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
                        if let Ok(IHyperdriveEvents::AddLiquidityFilter(log)) =
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
        self.wallet.lp_shares += log.lp_amount.into();

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn remove_liquidity(
        &mut self,
        shares: FixedPoint<U256>,
        maybe_options: Option<Options>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<(U256, U256)> {
        // Ensure that the agent has a sufficient balance of LP shares.
        if self.wallet.lp_shares < shares {
            return Err(eyre::eyre!(
                "insufficient LP share balance to remove liquidity: {:?} < {:?}",
                self.wallet.lp_shares,
                shares
            ));
        }

        let options = match maybe_options {
            Some(options) => options,
            None => Options {
                destination: self.client.address(),
                as_base: true,
                extra_data: [].into(),
            },
        };

        // Decrease the wallet's LP share balance.
        self.wallet.lp_shares -= shares;

        // Remove liquidity and record the base and withdrawal shares that were
        // received.
        let log = {
            let tx = ContractCall(self.hyperdrive.remove_liquidity(
                shares.into(),
                uint256!(0),
                options,
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
                        if let Ok(IHyperdriveEvents::RemoveLiquidityFilter(log)) =
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
        self.wallet.base += log.amount.into();
        self.wallet.withdrawal_shares += log.withdrawal_share_amount.into();

        Ok((log.amount, log.withdrawal_share_amount))
    }

    #[instrument(skip(self))]
    pub async fn redeem_withdrawal_shares(
        &mut self,
        shares: FixedPoint<U256>,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()> {
        // Ensure that the agent has a sufficient balance of withdrawal shares.
        if self.wallet.withdrawal_shares < shares {
            return Err(eyre::eyre!(
                "insufficient withdrawal share balance to redeem withdrawal shares: {:?} < {:?}",
                self.wallet.withdrawal_shares,
                shares
            ));
        }

        // Redeem the withdrawal shares and record the base and withdrawal
        // shares that were redeemed.
        let log = {
            let tx = ContractCall(self.hyperdrive.redeem_withdrawal_shares(
                shares.into(),
                uint256!(0),
                Options {
                    destination: self.client.address(),
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
                        if let Ok(IHyperdriveEvents::RedeemWithdrawalSharesFilter(log)) =
                            IHyperdriveEvents::decode_log(&log.into())
                        {
                            Some(log)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            logs.first().cloned()
        };
        if let Some(log) = log {
            // We ensure trades here are executed as base
            // Panic here since we pass as_base=True in the call.
            if !log.as_base {
                panic!("Trades are expected to be executed as base.")
            }
            self.wallet.base += log.amount.into();
            self.wallet.withdrawal_shares -= log.withdrawal_share_amount.into();
        }

        Ok(())
    }

    /// Checkpoint ///

    #[instrument(skip(self))]
    pub async fn checkpoint(
        &self,
        checkpoint: U256,
        max_iterations: U256,
        maybe_tx_options: Option<TxOptions>,
    ) -> Result<()> {
        let tx = ContractCall(self.hyperdrive.checkpoint(checkpoint, max_iterations))
            .apply(self.pre_process_options(maybe_tx_options));
        tx.0.send().await?.await?;
        Ok(())
    }

    /// Test Utils ///

    /// Funds the wallet with some base tokens and sets the approval on the
    /// Hyperdrive contract.
    pub async fn fund(&mut self, amount: FixedPoint<U256>) -> Result<()> {
        // Mint some base tokens.
        self.base
            .mint(amount.into())
            .from(self.client.address())
            .send()
            .await?;

        // HACK: Sleep for a few ms to give anvil some time to catch up. We
        // shouldn't need this, but anvil gets stuck in timeout loops when
        // these calls are made in quick succession with retries.
        sleep(Duration::from_millis(50)).await;

        // Approve hyperdrive to spend the base tokens.
        self.base
            .approve(self.hyperdrive.address(), amount.into())
            .from(self.client.address())
            .send()
            .await?;

        // HACK: Sleep for a few ms to give anvil some time to catch up. We
        // shouldn't need this, but anvil gets stuck in timeout loops when
        // these calls are made in quick succession with retries.
        sleep(Duration::from_millis(50)).await;

        // Increase the base balance in the wallet.
        self.wallet.base += amount;

        Ok(())
    }

    /// Advances the chain's time and changes the pool's variable rate so that
    /// interest accrues.
    pub async fn advance_time(
        &self,
        rate: FixedPoint<U256>,
        duration: FixedPoint<U256>,
    ) -> Result<()> {
        // Set the new variable rate.
        self.vault.set_rate(rate.into()).send().await?;

        // Advance the chain's time and mine a block. Mining a block is
        // important because client's check the current block time by looking
        // at the latest block's timestamp.
        self.client
            .provider()
            .request::<[u128; 1], i128>("anvil_increaseTime", [duration.try_into()?])
            .await?;
        self.client
            .provider()
            .request::<[u128; 1], ()>("anvil_mine", [1])
            .await?;

        Ok(())
    }

    /// Resets the agent's wallet.
    ///
    /// This is useful for testing because it makes it easy to use the agent
    /// across multiple snapshots.
    pub async fn reset(&mut self, wallet: Wallet) -> Result<()> {
        // Reset the nonce mananger.
        self.client.reset_nonce(None).await?;

        // Reset the wallet.
        self.wallet = wallet;

        Ok(())
    }

    /// Getters ///

    pub fn address(&self) -> Address {
        self.client.address()
    }

    pub fn base(&self) -> FixedPoint<U256> {
        self.wallet.base
    }

    pub fn lp_shares(&self) -> FixedPoint<U256> {
        self.wallet.lp_shares
    }

    pub fn withdrawal_shares(&self) -> FixedPoint<U256> {
        self.wallet.withdrawal_shares
    }

    pub fn longs(&self) -> &BTreeMap<FixedPoint<U256>, FixedPoint<U256>> {
        &self.wallet.longs
    }

    pub fn shorts(&self) -> &BTreeMap<FixedPoint<U256>, FixedPoint<U256>> {
        &self.wallet.shorts
    }

    /// Gets the current timestamp.
    pub async fn now(&self) -> Result<U256> {
        Ok(self
            .client
            .get_block(self.client.get_block_number().await?)
            .await?
            .unwrap()
            .timestamp)
    }

    /// Gets the current block number.
    pub async fn current_block_number(&self) -> Result<U64> {
        Ok(self
            .client
            .get_block(self.client.get_block_number().await?)
            .await?
            .unwrap()
            .number
            .unwrap())
    }

    /// Gets the pool config.
    pub fn get_config(&self) -> &PoolConfig {
        &self.config
    }

    /// Gets a checkpoint.
    pub async fn get_checkpoint(&self, id: U256) -> Result<Checkpoint> {
        Ok(self.hyperdrive.get_checkpoint(id).await?)
    }

    /// Gets the checkpoint exposure.
    pub async fn get_checkpoint_exposure(&self, id: U256) -> Result<I256> {
        Ok(self.hyperdrive.get_checkpoint_exposure(id).await?)
    }

    // TODO: We'll need to implement helpers that give us the maximum trade
    // for an older checkpoint. We'll need to use these when closig trades.

    /// Helpers ///

    pub fn pre_process_options(&self, maybe_tx_options: Option<TxOptions>) -> TxOptions {
        maybe_tx_options
            .map(|mut tx_options| {
                if tx_options.from.is_none() {
                    tx_options.from = Some(self.client.address());
                }
                tx_options
            })
            .unwrap_or_default()
    }
}
