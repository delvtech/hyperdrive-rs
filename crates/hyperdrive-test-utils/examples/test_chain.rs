use eyre::Result;
use fixedpointmath::fixed;
use hyperdrive_test_utils::chain::TestChain;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a new test chain.
    let chain = TestChain::new().await?;

    // Get an agent instance for Alice.
    let mut alice = chain.alice().await?;

    // Initialize the pool. In order for Alice to initialize the pool, she'll
    // need to mint base tokens and approve the Hyperdrive pool. We can
    // accomplish this by calling the `fund` method on the agent.
    let rate = fixed!(0.05e18);
    let contribution = fixed!(500_000_000e18);
    alice.fund(contribution).await?;
    alice.initialize(rate, contribution, None).await?;

    // Log the pool config.
    println!("pool config = {:#?}", alice.get_config());

    Ok(())
}
