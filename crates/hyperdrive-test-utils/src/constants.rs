use std::env;

pub use test_utils::constants::*;

lazy_static! {
    // The Ethereum URL the tests should connect to. If None, then the tests
    // will spawn an anvil node.
    pub static ref MAYBE_ETHEREUM_URL: Option<String> = env::var("HYPERDRIVE_ETHEREUM_URL").ok().or(None);

    // The fuzz tests take differing amounts of time. While we want to do as many
    // fuzz iterations as possible, we must also consider the developer experience.
    pub static ref SLOW_FUZZ_RUNS: u64 = env::var("HYPERDRIVE_SLOW_FUZZ_RUNS").ok().map(|s| s.parse().unwrap()).unwrap_or(50);
    pub static ref FUZZ_RUNS: u64 = env::var("HYPERDRIVE_FUZZ_RUNS").ok().map(|s| s.parse().unwrap()).unwrap_or(100);
    pub static ref FAST_FUZZ_RUNS: u64 = env::var("HYPERDRIVE_FAST_FUZZ_RUNS").ok().map(|s| s.parse().unwrap()).unwrap_or(10_000);
}
