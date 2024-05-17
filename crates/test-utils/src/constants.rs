use ethers::{signers::LocalWallet, types::Address, utils::keccak256};

lazy_static! {
    // ETH address
    pub static ref ETH: Address = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".parse::<Address>().unwrap();

    // A set of test accounts.
    pub static ref DEPLOYER: LocalWallet = LocalWallet::from_bytes(&keccak256("deployer")).unwrap();
    pub static ref ALICE: LocalWallet = LocalWallet::from_bytes(&keccak256("alice")).unwrap();
    pub static ref BOB: LocalWallet = LocalWallet::from_bytes(&keccak256("bob")).unwrap();
    pub static ref CELINE: LocalWallet = LocalWallet::from_bytes(&keccak256("celine")).unwrap();
}
