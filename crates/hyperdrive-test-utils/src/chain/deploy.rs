use std::future::Future;

/// This module contains implementations on the `Chain` struct that make it easy
/// to deploy Hyperdrive pools, factories, and deployer coordinators.
use ethers::{
    core::utils::keccak256,
    prelude::EthLogDecode,
    signers::Signer,
    types::{Address, I256, U256},
};
use eyre::Result;
use fixedpointmath::{fixed, ln, uint256, FixedPoint};
use hyperdrive_wrappers::wrappers::{
    erc20_forwarder_factory::ERC20ForwarderFactory,
    erc20_mintable::ERC20Mintable,
    erc4626_hyperdrive::ERC4626Hyperdrive,
    erc4626_hyperdrive_core_deployer::ERC4626HyperdriveCoreDeployer,
    erc4626_hyperdrive_deployer_coordinator::ERC4626HyperdriveDeployerCoordinator,
    erc4626_target0::{ERC4626Target0, ERC4626Target0Libs},
    erc4626_target0_deployer::{ERC4626Target0Deployer, ERC4626Target0DeployerLibs},
    erc4626_target1::{ERC4626Target1, ERC4626Target1Libs},
    erc4626_target1_deployer::{ERC4626Target1Deployer, ERC4626Target1DeployerLibs},
    erc4626_target2::{ERC4626Target2, ERC4626Target2Libs},
    erc4626_target2_deployer::{ERC4626Target2Deployer, ERC4626Target2DeployerLibs},
    erc4626_target3::{ERC4626Target3, ERC4626Target3Libs},
    erc4626_target3_deployer::{ERC4626Target3Deployer, ERC4626Target3DeployerLibs},
    erc4626_target4::{ERC4626Target4, ERC4626Target4Libs},
    erc4626_target4_deployer::{ERC4626Target4Deployer, ERC4626Target4DeployerLibs},
    hyperdrive_factory::{
        self, FactoryConfig, Fees as FactoryFees, HyperdriveFactory, HyperdriveFactoryEvents,
        Options, PoolDeployConfig,
    },
    hyperdrive_registry::HyperdriveRegistry,
    ihyperdrive::{Fees, PoolConfig},
    lp_math::LPMath,
    mock_erc4626::MockERC4626,
    mock_lido::MockLido,
    steth_hyperdrive_core_deployer::StETHHyperdriveCoreDeployer,
    steth_hyperdrive_deployer_coordinator::StETHHyperdriveDeployerCoordinator,
    steth_target0_deployer::{StETHTarget0Deployer, StETHTarget0DeployerLibs},
    steth_target1_deployer::{StETHTarget1Deployer, StETHTarget1DeployerLibs},
    steth_target2_deployer::{StETHTarget2Deployer, StETHTarget2DeployerLibs},
    steth_target3_deployer::{StETHTarget3Deployer, StETHTarget3DeployerLibs},
    steth_target4_deployer::{StETHTarget4Deployer, StETHTarget4DeployerLibs},
};
use serde::{Deserialize, Deserializer, Serialize};
use test_utils::{chain::Chain, constants::ETH};

use crate::addresses::Addresses;

fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let dec_string: String = Deserialize::deserialize(deserializer)?;
    let u256 = U256::from_dec_str(&dec_string).map_err(serde::de::Error::custom)?;
    Ok(u256)
}

// This is defined in hyperdrive-math, but re-defined here to avoid cyclic dependencies.
pub fn calculate_time_stretch(
    rate: FixedPoint<U256>,
    position_duration: FixedPoint<U256>,
) -> Result<FixedPoint<U256>> {
    let seconds_in_a_year = FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    // Calculate the benchmark time stretch. This time stretch is tuned for
    // a position duration of 1 year.
    let time_stretch = fixed!(5.24592e18)
        / (fixed!(0.04665e18) * FixedPoint::from(U256::from(rate) * uint256!(100)));
    let time_stretch = fixed!(1e18) / time_stretch;

    // We know that the following simultaneous equations hold:
    //
    // (1 + apr) * A ** timeStretch = 1
    //
    // and
    //
    // (1 + apr * (positionDuration / 365 days)) * A ** targetTimeStretch = 1
    //
    // where A is the reserve ratio. We can solve these equations for the
    // target time stretch as follows:
    //
    // targetTimeStretch = (
    //     ln(1 + apr * (positionDuration / 365 days)) /
    //     ln(1 + apr)
    // ) * timeStretch
    //
    // NOTE: Round down so that the output is an underestimate.
    Ok((FixedPoint::try_from(ln(I256::try_from(
        fixed!(1e18) + rate.mul_div_down(position_duration, seconds_in_a_year),
    )?)?)?
        / FixedPoint::try_from(ln(I256::try_from(fixed!(1e18) + rate)?)?)?)
        * time_stretch)
}

/// A configuration for a test chain that specifies the factory parameters,
/// the base token parameters, the lido parameters, and the parameters for an
/// ERC4626 and a stETH hyperdrive pool.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct TestChainConfig {
    // admin configuration
    admin: Address,
    is_competition_mode: bool,
    // base token configuration
    base_token_name: String,
    base_token_symbol: String,
    base_token_decimals: u8,
    // vault configuration
    vault_name: String,
    vault_symbol: String,
    #[serde(deserialize_with = "deserialize_u256")]
    vault_starting_rate: U256,
    // lido configuration
    #[serde(deserialize_with = "deserialize_u256")]
    lido_starting_rate: U256,
    // factory configuration
    #[serde(deserialize_with = "deserialize_u256")]
    factory_checkpoint_duration_resolution: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_checkpoint_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_checkpoint_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_position_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_position_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_circuit_breaker_delta: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_circuit_breaker_delta: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_fixed_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_fixed_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_time_stretch_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_time_stretch_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_curve_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_flat_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_governance_lp_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_min_governance_zombie_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_curve_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_flat_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_governance_lp_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    factory_max_governance_zombie_fee: U256,
    // erc4626 hyperdrive configuration
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_contribution: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_fixed_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_time_stretch_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_minimum_share_reserves: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_minimum_transaction_amount: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_circuit_breaker_delta: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_position_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_checkpoint_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_curve_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_flat_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_governance_lp_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    erc4626_hyperdrive_governance_zombie_fee: U256,
    // steth hyperdrive configuration
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_contribution: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_fixed_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_time_stretch_apr: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_minimum_share_reserves: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_minimum_transaction_amount: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_circuit_breaker_delta: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_position_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_checkpoint_duration: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_curve_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_flat_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_governance_lp_fee: U256,
    #[serde(deserialize_with = "deserialize_u256")]
    steth_hyperdrive_governance_zombie_fee: U256,
}

impl Default for TestChainConfig {
    fn default() -> Self {
        Self {
            // admin configuration
            admin: Address::zero(),
            is_competition_mode: false,
            // base token configuration
            base_token_name: "Base".to_string(),
            base_token_symbol: "BASE".to_string(),
            base_token_decimals: 18,
            // vault configuration
            vault_name: "Delvnet Yield Source".to_string(),
            vault_symbol: "DELV".to_string(),
            vault_starting_rate: uint256!(0.05e18),
            // lido configuration
            lido_starting_rate: uint256!(0.035e18),
            // factory configuration
            factory_checkpoint_duration_resolution: U256::from(60 * 60), // 1 hour
            factory_min_checkpoint_duration: U256::from(60 * 60),        // 1 hour
            factory_max_checkpoint_duration: U256::from(60 * 60 * 24),   // 1 day
            factory_min_position_duration: U256::from(60 * 60 * 24 * 7), // 7 days
            factory_max_position_duration: U256::from(60 * 60 * 24 * 365 * 10), // 10 years
            factory_min_circuit_breaker_delta: uint256!(0.15e18),
            factory_max_circuit_breaker_delta: uint256!(2e18),
            factory_min_fixed_apr: uint256!(0.01e18),
            factory_max_fixed_apr: uint256!(0.5e18),
            factory_min_time_stretch_apr: uint256!(0.01e18),
            factory_max_time_stretch_apr: uint256!(0.5e18),
            factory_min_curve_fee: uint256!(0.0001e18),
            factory_min_flat_fee: uint256!(0.0001e18),
            factory_min_governance_lp_fee: uint256!(0.15e18),
            factory_min_governance_zombie_fee: uint256!(0.03e18),
            factory_max_curve_fee: uint256!(0.1e18),
            factory_max_flat_fee: uint256!(0.001e18),
            factory_max_governance_lp_fee: uint256!(0.15e18),
            factory_max_governance_zombie_fee: uint256!(0.03e18),
            // erc4626 hyperdrive configuration
            erc4626_hyperdrive_contribution: uint256!(100_000_000e18),
            erc4626_hyperdrive_fixed_apr: uint256!(0.05e18),
            erc4626_hyperdrive_time_stretch_apr: uint256!(0.05e18),
            erc4626_hyperdrive_minimum_share_reserves: uint256!(10e18),
            erc4626_hyperdrive_minimum_transaction_amount: uint256!(0.001e18),
            erc4626_hyperdrive_circuit_breaker_delta: uint256!(2e18),
            erc4626_hyperdrive_position_duration: U256::from(60 * 60 * 24 * 7), // 7 days
            erc4626_hyperdrive_checkpoint_duration: U256::from(60 * 60),        // 1 hour
            erc4626_hyperdrive_curve_fee: uint256!(0.01e18),
            erc4626_hyperdrive_flat_fee: uint256!(0.0005e18) / uint256!(52), // 0.05% APR
            erc4626_hyperdrive_governance_lp_fee: uint256!(0.15e18),
            erc4626_hyperdrive_governance_zombie_fee: uint256!(0.03e18),
            // steth hyperdrive configuration
            steth_hyperdrive_contribution: uint256!(50_000e18),
            steth_hyperdrive_fixed_apr: uint256!(0.035e18),
            steth_hyperdrive_time_stretch_apr: uint256!(0.035e18),
            steth_hyperdrive_minimum_share_reserves: uint256!(1e15),
            steth_hyperdrive_minimum_transaction_amount: uint256!(1e15),
            steth_hyperdrive_circuit_breaker_delta: uint256!(2e18),
            steth_hyperdrive_position_duration: U256::from(60 * 60 * 24 * 7), // 7 days
            steth_hyperdrive_checkpoint_duration: U256::from(60 * 60),        // 1 hour
            steth_hyperdrive_curve_fee: uint256!(0.01e18),
            steth_hyperdrive_flat_fee: uint256!(0.0005e18) / uint256!(52), // 0.05% APR
            steth_hyperdrive_governance_lp_fee: uint256!(0.15e18),
            steth_hyperdrive_governance_zombie_fee: uint256!(0.03e18),
        }
    }
}

pub trait TestnetDeploy {
    /// Deploys a fresh instance of Hyperdrive.
    fn test_deploy<S: Signer + 'static>(
        &self,
        signer: S,
    ) -> impl Future<Output = Result<Addresses>>;

    /// Deploys the full Hyperdrive system equipped with a Hyperdrive Factory,
    /// an ERC4626Hyperdrive instance, and a StETHHyperdrive instance.
    fn full_deploy<S: Signer + 'static>(
        &self,
        signer: S,
        config: TestChainConfig,
    ) -> impl Future<Output = Result<Addresses>>;
}

// TODO: Ultimately, we'll want to spruce this up to. Keeping these functions
// as-is is a temporary measure.
impl TestnetDeploy for Chain {
    async fn test_deploy<S: Signer + 'static>(&self, signer: S) -> Result<Addresses> {
        // Create a client using the signer.
        let client = self.client(signer).await?;

        // Deploy the base token and vault.
        let base = ERC20Mintable::deploy(
            client.clone(),
            (
                "Base".to_string(),
                "BASE".to_string(),
                18_u8,
                Address::zero(),
                false,
                U256::MAX,
            ),
        )?
        .send()
        .await?;
        let vault = MockERC4626::deploy(
            client.clone(),
            (
                base.address(),
                "Mock ERC4626 Vault".to_string(),
                "MOCK".to_string(),
                uint256!(0.05e18),
                Address::zero(),
                false,
                U256::MAX,
            ),
        )?
        .send()
        .await?;

        // Deploy the Hyperdrive instance.
        let config = PoolConfig {
            base_token: base.address(),
            vault_shares_token: vault.address(),
            linker_factory: Address::from_low_u64_be(1),
            linker_code_hash: [1; 32],
            initial_vault_share_price: uint256!(1e18),
            minimum_share_reserves: uint256!(10e18),
            minimum_transaction_amount: uint256!(0.001e18),
            circuit_breaker_delta: uint256!(2e18),
            position_duration: U256::from(60 * 60 * 24 * 365), // 1 year
            checkpoint_duration: U256::from(60 * 60 * 24),     // 1 day
            time_stretch: calculate_time_stretch(
                fixed!(0.05e18),
                U256::from(60 * 60 * 24 * 365).into(),
            )?
            .into(), // time stretch for 5% rate
            governance: client.address(),
            fee_collector: client.address(),
            sweep_collector: client.address(),
            checkpoint_rewarder: client.address(),
            fees: Fees {
                curve: uint256!(0.05e18),
                flat: uint256!(0.0005e18),
                governance_lp: uint256!(0.15e18),
                governance_zombie: uint256!(0.15e18),
            },
        };
        let lp_math = LPMath::deploy(client.clone(), ())?.send().await?;
        let target0 = ERC4626Target0::link_and_deploy(
            client.clone(),
            (config.clone(),),
            ERC4626Target0Libs {
                lp_math: lp_math.address(),
            },
        )?
        .send()
        .await?;
        let target1 = ERC4626Target1::link_and_deploy(
            client.clone(),
            (config.clone(),),
            ERC4626Target1Libs {
                lp_math: lp_math.address(),
            },
        )?
        .send()
        .await?;
        let target2 = ERC4626Target2::link_and_deploy(
            client.clone(),
            (config.clone(),),
            ERC4626Target2Libs {
                lp_math: lp_math.address(),
            },
        )?
        .send()
        .await?;
        let target3 = ERC4626Target3::link_and_deploy(
            client.clone(),
            (config.clone(),),
            ERC4626Target3Libs {
                lp_math: lp_math.address(),
            },
        )?
        .send()
        .await?;
        let target4 = ERC4626Target4::link_and_deploy(
            client.clone(),
            (config.clone(),),
            ERC4626Target4Libs {
                lp_math: lp_math.address(),
            },
        )?
        .send()
        .await?;
        let erc4626_hyperdrive = ERC4626Hyperdrive::deploy(
            client.clone(),
            (
                "ERC4626Hyperdrive".to_string(),
                config,
                target0.address(),
                target1.address(),
                target2.address(),
                target3.address(),
                target4.address(),
            ),
        )?
        .send()
        .await?;

        Ok(Addresses {
            base_token: base.address(),
            erc4626_hyperdrive: erc4626_hyperdrive.address(),
            steth_hyperdrive: Address::zero(),
            factory: Address::zero(),
            hyperdrive_registry: Address::zero(),
        })
    }

    async fn full_deploy<S: Signer + 'static>(
        &self,
        signer: S,
        config: TestChainConfig,
    ) -> Result<Addresses> {
        // Set up a client.
        let address = signer.address();
        let client = self.client(signer).await?;

        // Deploy the base token and vault.
        let base = ERC20Mintable::deploy(
            client.clone(),
            (
                config.base_token_name,
                config.base_token_symbol,
                config.base_token_decimals,
                address,
                config.is_competition_mode,
                U256::MAX,
            ),
        )?
        .send()
        .await?;
        let vault = MockERC4626::deploy(
            client.clone(),
            (
                base.address(),
                config.vault_name,
                config.vault_symbol,
                config.vault_starting_rate,
                address,
                config.is_competition_mode,
                U256::MAX,
            ),
        )?
        .send()
        .await?;
        if config.is_competition_mode {
            base.set_user_role(vault.address(), 1, true).send().await?;
            base.set_role_capability(
                1,
                keccak256("mint(uint256)".as_bytes())[0..4].try_into()?,
                true,
            )
            .send()
            .await?;
            base.set_role_capability(
                1,
                keccak256("burn(uint256)".as_bytes())[0..4].try_into()?,
                true,
            )
            .send()
            .await?;
        }

        // Deploy the HyperdriveRegistry contract to track familiar instances.
        let hyperdrive_registry =
            HyperdriveRegistry::deploy(client.clone(), ("HyperdriveRegistry".to_string(),))?
                .send()
                .await?;

        // Deploy the mock Lido system. We fund Lido with 1 eth to start to
        // avoid reverts when we initialize the pool.
        let lido = {
            let lido = MockLido::deploy(
                client.clone(),
                (
                    config.lido_starting_rate,
                    address,
                    config.is_competition_mode,
                    U256::MAX,
                ),
            )?
            .send()
            .await?;
            self.deal(address, uint256!(1e18)).await?;
            lido.submit(Address::zero())
                .value(uint256!(1e18))
                .send()
                .await?;
            lido
        };

        // Deployer the ERC20 forwarder factory.
        let erc20_forwarder_factory =
            ERC20ForwarderFactory::deploy(client.clone(), ("ForwarderFactory".to_string(),))?
                .send()
                .await?;

        // Deploy the Hyperdrive factory.
        let factory = {
            HyperdriveFactory::deploy(
                client.clone(),
                (
                    FactoryConfig {
                        governance: address,
                        deployer_coordinator_manager: config.admin,
                        hyperdrive_governance: config.admin,
                        default_pausers: vec![config.admin],
                        fee_collector: config.admin,
                        sweep_collector: config.admin,
                        checkpoint_rewarder: config.admin,
                        checkpoint_duration_resolution: config
                            .factory_checkpoint_duration_resolution,
                        min_checkpoint_duration: config.factory_min_checkpoint_duration,
                        max_checkpoint_duration: config.factory_max_checkpoint_duration,
                        min_position_duration: config.factory_min_position_duration,
                        max_position_duration: config.factory_max_position_duration,
                        min_circuit_breaker_delta: config.factory_min_circuit_breaker_delta,
                        max_circuit_breaker_delta: config.factory_max_circuit_breaker_delta,
                        min_fixed_apr: config.factory_min_fixed_apr,
                        max_fixed_apr: config.factory_max_fixed_apr,
                        min_time_stretch_apr: config.factory_min_time_stretch_apr,
                        max_time_stretch_apr: config.factory_max_time_stretch_apr,
                        min_fees: hyperdrive_factory::Fees {
                            curve: config.factory_min_curve_fee,
                            flat: config.factory_min_flat_fee,
                            governance_lp: config.factory_min_governance_lp_fee,
                            governance_zombie: config.factory_min_governance_zombie_fee,
                        },
                        max_fees: hyperdrive_factory::Fees {
                            curve: config.factory_max_curve_fee,
                            flat: config.factory_max_flat_fee,
                            governance_lp: config.factory_max_governance_lp_fee,
                            governance_zombie: config.factory_max_governance_zombie_fee,
                        },
                        linker_factory: erc20_forwarder_factory.address(),
                        linker_code_hash: erc20_forwarder_factory.erc20link_hash().await?,
                    },
                    "HyperdriveFactory".to_string(),
                ),
            )?
            .send()
            .await?
        };

        // Deploy the ERC4626Hyperdrive deployers and add them to the factory.
        let lp_math = LPMath::deploy(client.clone(), ())?.send().await?;
        let erc4626_deployer_coordinator = {
            let core_deployer = ERC4626HyperdriveCoreDeployer::deploy(client.clone(), ())?
                .send()
                .await?;
            let target0 = ERC4626Target0Deployer::link_and_deploy(
                client.clone(),
                (),
                ERC4626Target0DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target1 = ERC4626Target1Deployer::link_and_deploy(
                client.clone(),
                (),
                ERC4626Target1DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target2 = ERC4626Target2Deployer::link_and_deploy(
                client.clone(),
                (),
                ERC4626Target2DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target3 = ERC4626Target3Deployer::link_and_deploy(
                client.clone(),
                (),
                ERC4626Target3DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target4 = ERC4626Target4Deployer::link_and_deploy(
                client.clone(),
                (),
                ERC4626Target4DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            ERC4626HyperdriveDeployerCoordinator::deploy(
                client.clone(),
                (
                    "ERC4626HyperdriveDeployerCoordinator".to_string(),
                    factory.address(),
                    core_deployer.address(),
                    target0.address(),
                    target1.address(),
                    target2.address(),
                    target3.address(),
                    target4.address(),
                ),
            )?
            .send()
            .await?
        };
        factory
            .add_deployer_coordinator(erc4626_deployer_coordinator.address())
            .send()
            .await?;

        // Deploy and initialize an initial ERC4626Hyperdrive instance.
        let erc4626_hyperdrive = {
            base.mint_with_destination(address, config.erc4626_hyperdrive_contribution)
                .send()
                .await?;
            base.approve(
                erc4626_deployer_coordinator.address(),
                config.erc4626_hyperdrive_contribution,
            )
            .send()
            .await?;
            let pool_config = PoolDeployConfig {
                fee_collector: factory.fee_collector().call().await?,
                sweep_collector: factory.sweep_collector().call().await?,
                checkpoint_rewarder: factory.checkpoint_rewarder().call().await?,
                governance: factory.hyperdrive_governance().call().await?,
                linker_factory: factory.linker_factory().call().await?,
                linker_code_hash: factory.linker_code_hash().call().await?,
                time_stretch: uint256!(0),
                base_token: base.address(),
                vault_shares_token: vault.address(),
                minimum_share_reserves: config.erc4626_hyperdrive_minimum_share_reserves,
                minimum_transaction_amount: config.erc4626_hyperdrive_minimum_transaction_amount,
                circuit_breaker_delta: config.erc4626_hyperdrive_circuit_breaker_delta,
                position_duration: config.erc4626_hyperdrive_position_duration,
                checkpoint_duration: config.erc4626_hyperdrive_checkpoint_duration,
                fees: FactoryFees {
                    curve: config.erc4626_hyperdrive_curve_fee,
                    flat: config.erc4626_hyperdrive_flat_fee,
                    governance_lp: config.erc4626_hyperdrive_governance_lp_fee,
                    governance_zombie: config.erc4626_hyperdrive_governance_zombie_fee,
                },
            };
            factory
                .deploy_target(
                    [0x01; 32],
                    erc4626_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.erc4626_hyperdrive_fixed_apr,
                    config.erc4626_hyperdrive_time_stretch_apr,
                    U256::from(0),
                    [0x01; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x01; 32],
                    erc4626_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.erc4626_hyperdrive_fixed_apr,
                    config.erc4626_hyperdrive_time_stretch_apr,
                    U256::from(1),
                    [0x01; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x01; 32],
                    erc4626_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.erc4626_hyperdrive_fixed_apr,
                    config.erc4626_hyperdrive_time_stretch_apr,
                    U256::from(2),
                    [0x01; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x01; 32],
                    erc4626_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.erc4626_hyperdrive_fixed_apr,
                    config.erc4626_hyperdrive_time_stretch_apr,
                    U256::from(3),
                    [0x01; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x01; 32],
                    erc4626_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.erc4626_hyperdrive_fixed_apr,
                    config.erc4626_hyperdrive_time_stretch_apr,
                    U256::from(4),
                    [0x01; 32],
                )
                .send()
                .await?;
            let tx = factory
                .deploy_and_initialize(
                    [0x01; 32],
                    erc4626_deployer_coordinator.address(),
                    "Hyperdrive".to_string(),
                    pool_config,
                    Vec::new().into(),
                    config.erc4626_hyperdrive_contribution,
                    config.erc4626_hyperdrive_fixed_apr,
                    config.erc4626_hyperdrive_time_stretch_apr,
                    Options {
                        as_base: true,
                        destination: address,
                        extra_data: Vec::new().into(),
                    },
                    [0x01; 32],
                )
                .send()
                .await?
                .await?
                .unwrap();
            let logs = tx
                .logs
                .into_iter()
                .filter_map(|log| {
                    if let Ok(HyperdriveFactoryEvents::DeployedFilter(log)) =
                        HyperdriveFactoryEvents::decode_log(&log.into())
                    {
                        Some(log)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            logs[0].clone().hyperdrive
        };

        // Deploy the StETHHyperdrive deployers and add them to the factory.
        let steth_deployer_coordinator = {
            let core_deployer = StETHHyperdriveCoreDeployer::deploy(client.clone(), ())?
                .send()
                .await?;
            let target0 = StETHTarget0Deployer::link_and_deploy(
                client.clone(),
                (),
                StETHTarget0DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target1 = StETHTarget1Deployer::link_and_deploy(
                client.clone(),
                (),
                StETHTarget1DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target2 = StETHTarget2Deployer::link_and_deploy(
                client.clone(),
                (),
                StETHTarget2DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target3 = StETHTarget3Deployer::link_and_deploy(
                client.clone(),
                (),
                StETHTarget3DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            let target4 = StETHTarget4Deployer::link_and_deploy(
                client.clone(),
                (),
                StETHTarget4DeployerLibs {
                    lp_math: lp_math.address(),
                },
            )?
            .send()
            .await?;
            StETHHyperdriveDeployerCoordinator::deploy(
                client.clone(),
                (
                    "StETHHyperdriveDeployerCoordinator".to_string(),
                    factory.address(),
                    core_deployer.address(),
                    target0.address(),
                    target1.address(),
                    target2.address(),
                    target3.address(),
                    target4.address(),
                    lido.address(),
                ),
            )?
            .send()
            .await?
        };
        factory
            .add_deployer_coordinator(steth_deployer_coordinator.address())
            .send()
            .await?;

        // Deploy and initialize an initial StETHHyperdrive instance.
        let steth_hyperdrive = {
            self.deal(address, config.steth_hyperdrive_contribution)
                .await?;
            let pool_config = PoolDeployConfig {
                fee_collector: factory.fee_collector().call().await?,
                sweep_collector: factory.sweep_collector().call().await?,
                checkpoint_rewarder: factory.checkpoint_rewarder().call().await?,
                governance: factory.hyperdrive_governance().call().await?,
                linker_factory: factory.linker_factory().call().await?,
                linker_code_hash: factory.linker_code_hash().call().await?,
                time_stretch: uint256!(0),
                base_token: *ETH,
                vault_shares_token: lido.address(),
                minimum_share_reserves: config.steth_hyperdrive_minimum_share_reserves,
                minimum_transaction_amount: config.steth_hyperdrive_minimum_transaction_amount,
                circuit_breaker_delta: config.steth_hyperdrive_circuit_breaker_delta,
                position_duration: config.steth_hyperdrive_position_duration,
                checkpoint_duration: config.steth_hyperdrive_checkpoint_duration,
                fees: FactoryFees {
                    curve: config.steth_hyperdrive_curve_fee,
                    flat: config.steth_hyperdrive_flat_fee,
                    governance_lp: config.steth_hyperdrive_governance_lp_fee,
                    governance_zombie: config.steth_hyperdrive_governance_zombie_fee,
                },
            };
            factory
                .deploy_target(
                    [0x02; 32],
                    steth_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.steth_hyperdrive_fixed_apr,
                    config.steth_hyperdrive_time_stretch_apr,
                    U256::from(0),
                    [0x02; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x02; 32],
                    steth_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.steth_hyperdrive_fixed_apr,
                    config.steth_hyperdrive_time_stretch_apr,
                    U256::from(1),
                    [0x02; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x02; 32],
                    steth_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.steth_hyperdrive_fixed_apr,
                    config.steth_hyperdrive_time_stretch_apr,
                    U256::from(2),
                    [0x02; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x02; 32],
                    steth_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.steth_hyperdrive_fixed_apr,
                    config.steth_hyperdrive_time_stretch_apr,
                    U256::from(3),
                    [0x02; 32],
                )
                .send()
                .await?;
            factory
                .deploy_target(
                    [0x02; 32],
                    steth_deployer_coordinator.address(),
                    pool_config.clone(),
                    Vec::new().into(),
                    config.steth_hyperdrive_fixed_apr,
                    config.steth_hyperdrive_time_stretch_apr,
                    U256::from(4),
                    [0x02; 32],
                )
                .send()
                .await?;
            let tx = factory
                .deploy_and_initialize(
                    [0x02; 32],
                    steth_deployer_coordinator.address(),
                    "Hyperdrive".to_string(),
                    pool_config,
                    Vec::new().into(),
                    config.steth_hyperdrive_contribution,
                    config.steth_hyperdrive_fixed_apr,
                    config.steth_hyperdrive_time_stretch_apr,
                    Options {
                        as_base: true,
                        destination: address,
                        extra_data: Vec::new().into(),
                    },
                    [0x02; 32],
                )
                .value(config.steth_hyperdrive_contribution)
                .send()
                .await?
                .await?
                .unwrap();
            let logs = tx
                .logs
                .into_iter()
                .filter_map(|log| {
                    if let Ok(HyperdriveFactoryEvents::DeployedFilter(log)) =
                        HyperdriveFactoryEvents::decode_log(&log.into())
                    {
                        Some(log)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            logs[0].clone().hyperdrive
        };

        // Add the Hyperdrive factory instance to the registry contract.
        hyperdrive_registry
            .set_factory_info(vec![factory.address()], vec![1])
            .send()
            .await?;
        hyperdrive_registry
            .set_instance_info(
                vec![erc4626_hyperdrive, steth_hyperdrive],
                vec![1, 1],
                vec![factory.address(), factory.address()],
            )
            .send()
            .await?;
        hyperdrive_registry
            .update_admin(config.admin)
            .send()
            .await?;

        // Transfer ownership of the base token, factory, vault, and lido to the
        // admin address now that we're done minting tokens and updating the
        // configuration.
        base.transfer_ownership(config.admin).send().await?;
        vault.transfer_ownership(config.admin).send().await?;
        lido.transfer_ownership(config.admin).send().await?;
        factory.update_governance(config.admin).send().await?;

        Ok(Addresses {
            base_token: base.address(),
            factory: factory.address(),
            erc4626_hyperdrive,
            steth_hyperdrive,
            hyperdrive_registry: hyperdrive_registry.address(),
        })
    }
}

#[cfg(test)]
mod tests {
    use hyperdrive_wrappers::wrappers::ihyperdrive::IHyperdrive;
    use test_utils::constants::ALICE;

    use super::*;
    use crate::chain::TestChain;

    #[tokio::test]
    async fn test_deploy_devnet() -> Result<()> {
        // Connect to a local anvil chain.
        let chain = TestChain::new().await?;
        chain.deal(ALICE.address(), uint256!(100_000e18)).await?;
        let client = chain.client(ALICE.clone()).await?;

        // Deploy the factory and pools.
        let test_chain_config = TestChainConfig::default();
        let addresses = chain
            .full_deploy(ALICE.clone(), test_chain_config.clone())
            .await?;

        // Verify that the addresses are non-zero.
        assert_ne!(addresses, Addresses::default());

        // Verify that the erc4626 pool config is correct.
        let hyperdrive = IHyperdrive::new(addresses.erc4626_hyperdrive, client.clone());
        let config = hyperdrive.get_pool_config().call().await?;
        assert_eq!(config.base_token, addresses.base_token);
        assert_eq!(
            config.minimum_share_reserves,
            test_chain_config.erc4626_hyperdrive_minimum_share_reserves
        );
        assert_eq!(
            config.position_duration,
            test_chain_config.erc4626_hyperdrive_position_duration
        );
        assert_eq!(
            config.checkpoint_duration,
            test_chain_config.erc4626_hyperdrive_checkpoint_duration
        );
        assert_eq!(
            config.time_stretch,
            calculate_time_stretch(
                test_chain_config.erc4626_hyperdrive_time_stretch_apr.into(),
                test_chain_config
                    .erc4626_hyperdrive_position_duration
                    .into(),
            )?
            .into()
        );
        assert_eq!(config.governance, test_chain_config.admin);
        assert_eq!(config.fee_collector, test_chain_config.admin);
        assert_eq!(config.sweep_collector, test_chain_config.admin);
        assert_eq!(config.checkpoint_rewarder, test_chain_config.admin);
        assert_eq!(
            config.fees,
            Fees {
                curve: test_chain_config.erc4626_hyperdrive_curve_fee,
                flat: test_chain_config.erc4626_hyperdrive_flat_fee,
                governance_lp: test_chain_config.erc4626_hyperdrive_governance_lp_fee,
                governance_zombie: test_chain_config.erc4626_hyperdrive_governance_zombie_fee,
            }
        );

        // Verify that the steth pool config is correct.
        let hyperdrive = IHyperdrive::new(addresses.steth_hyperdrive, client.clone());
        let config = hyperdrive.get_pool_config().call().await?;
        assert_eq!(config.base_token, *ETH);
        assert_eq!(
            config.minimum_share_reserves,
            test_chain_config.steth_hyperdrive_minimum_share_reserves
        );
        assert_eq!(
            config.position_duration,
            test_chain_config.steth_hyperdrive_position_duration
        );
        assert_eq!(
            config.checkpoint_duration,
            test_chain_config.steth_hyperdrive_checkpoint_duration
        );
        assert_eq!(
            config.time_stretch,
            calculate_time_stretch(
                test_chain_config.steth_hyperdrive_time_stretch_apr.into(),
                test_chain_config.steth_hyperdrive_position_duration.into(),
            )?
            .into()
        );
        assert_eq!(config.governance, test_chain_config.admin);
        assert_eq!(config.fee_collector, test_chain_config.admin);
        assert_eq!(
            config.fees,
            Fees {
                curve: test_chain_config.steth_hyperdrive_curve_fee,
                flat: test_chain_config.steth_hyperdrive_flat_fee,
                governance_lp: test_chain_config.steth_hyperdrive_governance_lp_fee,
                governance_zombie: test_chain_config.steth_hyperdrive_governance_zombie_fee,
            }
        );

        // Verify that the registry data has been set for each Hyperdrive contract.
        let registry = HyperdriveRegistry::new(addresses.hyperdrive_registry, client.clone());
        let registry_data_4626 = registry
            .get_instance_info(addresses.erc4626_hyperdrive)
            .call()
            .await?;
        assert_ne!(registry_data_4626.data, uint256!(0));
        let registry_data_steth = registry
            .get_instance_info(addresses.steth_hyperdrive)
            .call()
            .await?;
        assert_ne!(registry_data_steth.data, uint256!(0));

        Ok(())
    }

    #[tokio::test]
    async fn test_deploy_testnet() -> Result<()> {
        // Get a config that matches the one used for testnet.
        let mut test_chain_config = TestChainConfig::default();
        test_chain_config.erc4626_hyperdrive_position_duration = U256::from(60 * 60 * 24 * 365);
        test_chain_config.erc4626_hyperdrive_flat_fee = uint256!(0.0005e18);
        test_chain_config.steth_hyperdrive_position_duration = U256::from(60 * 60 * 24 * 365);
        test_chain_config.steth_hyperdrive_flat_fee = uint256!(0.0005e18);

        // Connect to a local anvil chain.
        let chain = Chain::connect(None, Some(1)).await?;
        chain.deal(ALICE.address(), uint256!(100_000e18)).await?;
        let client = chain.client(ALICE.clone()).await?;

        // Deploy the factory and pools.
        let addresses = chain
            .full_deploy(ALICE.clone(), test_chain_config.clone())
            .await?;

        // Verify that the addresses are non-zero.
        assert_ne!(addresses, Addresses::default());

        // Verify that the erc4626 pool config is correct.
        let hyperdrive = IHyperdrive::new(addresses.erc4626_hyperdrive, client.clone());
        let config = hyperdrive.get_pool_config().call().await?;
        assert_eq!(config.base_token, addresses.base_token);
        assert_eq!(
            config.minimum_share_reserves,
            test_chain_config.erc4626_hyperdrive_minimum_share_reserves
        );
        assert_eq!(
            config.position_duration,
            test_chain_config.erc4626_hyperdrive_position_duration
        );
        assert_eq!(
            config.checkpoint_duration,
            test_chain_config.erc4626_hyperdrive_checkpoint_duration
        );
        assert_eq!(
            config.time_stretch,
            calculate_time_stretch(
                test_chain_config.erc4626_hyperdrive_time_stretch_apr.into(),
                test_chain_config
                    .erc4626_hyperdrive_position_duration
                    .into(),
            )?
            .into()
        );
        assert_eq!(config.governance, test_chain_config.admin);
        assert_eq!(config.fee_collector, test_chain_config.admin);
        assert_eq!(config.sweep_collector, test_chain_config.admin);
        assert_eq!(config.checkpoint_rewarder, test_chain_config.admin);
        assert_eq!(
            config.fees,
            Fees {
                curve: test_chain_config.erc4626_hyperdrive_curve_fee,
                flat: test_chain_config.erc4626_hyperdrive_flat_fee,
                governance_lp: test_chain_config.erc4626_hyperdrive_governance_lp_fee,
                governance_zombie: test_chain_config.erc4626_hyperdrive_governance_zombie_fee,
            }
        );

        // Verify that the steth pool config is correct.
        let hyperdrive = IHyperdrive::new(addresses.steth_hyperdrive, client.clone());
        let config = hyperdrive.get_pool_config().call().await?;
        assert_eq!(config.base_token, *ETH);
        assert_eq!(
            config.minimum_share_reserves,
            test_chain_config.steth_hyperdrive_minimum_share_reserves
        );
        assert_eq!(
            config.position_duration,
            test_chain_config.steth_hyperdrive_position_duration
        );
        assert_eq!(
            config.checkpoint_duration,
            test_chain_config.steth_hyperdrive_checkpoint_duration
        );
        assert_eq!(
            config.time_stretch,
            calculate_time_stretch(
                test_chain_config.steth_hyperdrive_time_stretch_apr.into(),
                test_chain_config.steth_hyperdrive_position_duration.into(),
            )?
            .into()
        );
        assert_eq!(config.governance, test_chain_config.admin);
        assert_eq!(config.fee_collector, test_chain_config.admin);
        assert_eq!(
            config.fees,
            Fees {
                curve: test_chain_config.steth_hyperdrive_curve_fee,
                flat: test_chain_config.steth_hyperdrive_flat_fee,
                governance_lp: test_chain_config.steth_hyperdrive_governance_lp_fee,
                governance_zombie: test_chain_config.steth_hyperdrive_governance_zombie_fee,
            }
        );

        // Verify that the registry data has been set for each Hyperdrive contract.
        let registry = HyperdriveRegistry::new(addresses.hyperdrive_registry, client.clone());
        let registry_data_4626 = registry
            .get_instance_info(addresses.erc4626_hyperdrive)
            .call()
            .await?;
        assert_ne!(registry_data_4626.data, uint256!(0));
        let registry_data_steth = registry
            .get_instance_infos(vec![addresses.steth_hyperdrive]) // use vec to make sure that still works.
            .call()
            .await?;
        assert_ne!(registry_data_steth[0].data, uint256!(0));

        Ok(())
    }
}
