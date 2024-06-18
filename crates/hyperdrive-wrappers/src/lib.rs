#[rustfmt::skip]
#[allow(clippy::all)]
pub mod wrappers;


pub mod linked_factory {
    use std::sync::Arc;

    use ethers::{abi::Abi, prelude::*, utils::hex};
    use ethers_solc::utils::library_hash_placeholder;

    /// Creates a contract factory with the given ABI and bytecode, linking the given libraries
    /// with their respective addresses.
    pub fn create<M, I, S>(
        abi: Abi,
        bytecode_str: &str,
        libs: I,
        client: Arc<M>,
    ) -> std::io::Result<ContractFactory<M>>
    where
        M: Middleware,
        I: IntoIterator<Item = (S, Address)>,
        S: AsRef<str>,
    {
        let mut linked_bytecode: String = bytecode_str.to_string();

        for (lib, addr) in libs {
            let hex_addr = hex::encode(addr);
            let place_holder = library_hash_placeholder(lib.as_ref());
            linked_bytecode = linked_bytecode.replace(&format!("__{place_holder}__"), &hex_addr);
        }

        let raw_bytecode: Bytes = hex::decode(&linked_bytecode).unwrap().into();
        let factory = ContractFactory::new(abi, raw_bytecode, client);
        Ok(factory)
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use ethers::{
        abi::Address,
        core::utils::Anvil,
        middleware::SignerMiddleware,
        providers::{Http, Provider},
        signers::{LocalWallet, Signer},
        types::U256,
    };
    use eyre::Result;

    use crate::wrappers::erc4626_target0::{ERC4626Target0, ERC4626Target0Libs, Fees, PoolConfig};

    #[tokio::test]
    async fn test_link_and_deploy() -> Result<()> {
        let anvil = Anvil::new().spawn();
        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let provider =
            Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(anvil.chain_id()),
        ));

        let pool_config: PoolConfig = PoolConfig {
            base_token: Address::zero(),
            vault_shares_token: Address::zero(),
            linker_factory: Address::zero(),
            linker_code_hash: [0u8; 32],
            initial_vault_share_price: U256::from(0),
            minimum_share_reserves: U256::from(0),
            minimum_transaction_amount: U256::from(0),
            position_duration: U256::from(0),
            checkpoint_duration: U256::from(0),
            time_stretch: U256::from(0),
            governance: Address::zero(),
            fee_collector: Address::zero(),
            sweep_collector: Address::zero(),
            checkpoint_rewarder: Address::zero(),
            fees: Fees {
                curve: U256::from(0),
                flat: U256::from(0),
                governance_lp: U256::from(0),
                governance_zombie: U256::from(0),
            },
            circuit_breaker_delta: U256::from(0),
        };

        let erc4626_target0 = ERC4626Target0::link_and_deploy(
            client.clone(),
            (pool_config,),
            ERC4626Target0Libs {
                lp_math: Address::zero(),
            },
        )
        .unwrap()
        .send()
        .await
        .unwrap();

        println!(
            "ERC4626Target0 deployed at: {:?}",
            erc4626_target0.address()
        );

        Ok(())
    }
}
