use std::ops::{Deref, DerefMut};

/// This module implements a wrapper around the `Chain` struct that is used for
/// testing. This chain abstraction will connect to a remote chain or spin up an
/// anvil instance. In addition to that, it deploys a set of test contracts to
/// the chain.
use ethers::signers::{LocalWallet, Signer};
use eyre::Result;
use fixedpointmath::uint256;
use hyperdrive_wrappers::wrappers::{
    lp_math::LPMath,
    mock_fixed_point_math::MockFixedPointMath,
    mock_hyperdrive_math::MockHyperdriveMath,
    mock_lp_math::{MockLPMath, MockLPMathLibs},
    mock_yield_space_math::MockYieldSpaceMath,
};
use rand_chacha::ChaCha8Rng;
use test_utils::{
    chain::{Chain, ChainClient},
    constants::{ALICE, BOB, CELINE, DEPLOYER},
};

use crate::{addresses::Addresses, agent::Agent, chain::TestnetDeploy};

pub struct TestChain {
    pub base: Chain,
    pub addresses: Addresses,
    pub mock_fixed_point_math: MockFixedPointMath<ChainClient<LocalWallet>>,
    pub mock_hyperdrive_math: MockHyperdriveMath<ChainClient<LocalWallet>>,
    pub mock_lp_math: MockLPMath<ChainClient<LocalWallet>>,
    pub mock_yield_space_math: MockYieldSpaceMath<ChainClient<LocalWallet>>,
}

impl Deref for TestChain {
    type Target = Chain;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for TestChain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TestChain {
    /// Constructs a new test chain. This will attempt to read the RPC URL from
    /// the `HYPERDRIVE_ETHEREUM_URL` environment variable. If the environment
    /// variable is not set, a local anvil node is spun up.
    ///
    /// Once the chain is spun up, the test chain will pre-fund the test accounts
    /// Alice, Bob, Celine, and a deployer with a large amount of ether. Then,
    /// using the deployer account, the test chain will deploy a test deployment
    /// of Hyperdrive as well as a full set of mock contracts. The test
    /// deployment is exposed through three pre-initialized agents, Alice, Bob,
    /// and Celine.
    pub async fn new() -> Result<Self> {
        // Spin up the chain.
        let chain = Chain::connect(std::env::var("HYPERDRIVE_ETHEREUM_URL").ok(), None)
            .await
            .unwrap();

        // Pre-fund alice, bob, celine, and deployer with a large amount of ether.
        chain.deal(ALICE.address(), uint256!(100_000e18)).await?;
        chain.deal(BOB.address(), uint256!(100_000e18)).await?;
        chain.deal(CELINE.address(), uint256!(100_000e18)).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;

        // Deploy the mock contracts.
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client.clone(), ())?
            .send()
            .await?;
        let mock_hyperdrive_math = MockHyperdriveMath::deploy(client.clone(), ())?
            .send()
            .await?;
        let lp_math = LPMath::deploy(client.clone(), ())?.send().await?;
        let mock_lp_math = MockLPMath::link_and_deploy(
            client.clone(),
            (),
            MockLPMathLibs {
                lp_math: lp_math.address(),
            },
        )?
        .send()
        .await?;
        let mock_yield_space_math = MockYieldSpaceMath::deploy(client.clone(), ())?
            .send()
            .await?;

        // Deploy the test contracts.
        let addresses = chain.test_deploy(DEPLOYER.clone()).await?;

        Ok(Self {
            base: chain,
            addresses,
            mock_fixed_point_math,
            mock_hyperdrive_math,
            mock_lp_math,
            mock_yield_space_math,
        })
    }

    /// Gets the addresses of the test contracts.
    pub fn addresses(&self) -> Addresses {
        self.addresses.clone()
    }

    /// Gets alice.
    pub async fn alice(&self) -> Result<Agent<ChainClient<LocalWallet>, ChaCha8Rng>> {
        Agent::new(
            self.client(ALICE.clone()).await?,
            self.addresses.clone(),
            None,
        )
        .await
    }

    /// Gets bob.
    pub async fn bob(&self) -> Result<Agent<ChainClient<LocalWallet>, ChaCha8Rng>> {
        Agent::new(
            self.client(BOB.clone()).await?,
            self.addresses.clone(),
            None,
        )
        .await
    }

    /// Gets celine.
    pub async fn celine(&self) -> Result<Agent<ChainClient<LocalWallet>, ChaCha8Rng>> {
        Agent::new(
            self.client(CELINE.clone()).await?,
            self.addresses.clone(),
            None,
        )
        .await
    }

    /// Gets the mock fixed point math contract.
    pub fn mock_fixed_point_math(&self) -> &MockFixedPointMath<ChainClient<LocalWallet>> {
        &self.mock_fixed_point_math
    }

    /// Gets the mock hyperdrive math contract.
    pub fn mock_hyperdrive_math(&self) -> &MockHyperdriveMath<ChainClient<LocalWallet>> {
        &self.mock_hyperdrive_math
    }

    /// Gets the mock LP math contract.
    pub fn mock_lp_math(&self) -> &MockLPMath<ChainClient<LocalWallet>> {
        &self.mock_lp_math
    }

    /// Gets the mock yield space math contract.
    pub fn mock_yield_space_math(&self) -> &MockYieldSpaceMath<ChainClient<LocalWallet>> {
        &self.mock_yield_space_math
    }
}
