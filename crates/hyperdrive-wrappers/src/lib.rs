#[rustfmt::skip]
#[allow(clippy::all)]
pub mod wrappers;
use ethers::{abi::Error, core::abi::Tokenize};
use std::{collections::HashMap, sync::Arc};

use ethers::{
    abi::{Abi, AbiEncode, Address},
    contract::{ContractError, ContractFactory, ContractInstance},
    providers::Middleware,
};

#[macro_use]
extern crate lazy_static;
// Assuming the mappings are known at compile time and are static
lazy_static! {
    static ref LIBRARY_PLACEHOLDERS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("LPMath", "__$2b4fa6f02a36eedfe41c65e8dd342257d3$__");
        // Add other mappings here
        m
    };
}

pub async fn deploy_linked_contract<M, T: Tokenize>(
    abi: Abi,
    bytecode: &str,
    libraries: Vec<(&str, Address)>,
    provider: Arc<M>,
    constructor_args: T,
) -> Result<ContractInstance<Arc<M>, M>, ContractError<M>>
where
    M: Middleware,
{
    // Replace placeholders in bytecode
    let mut linked_bytecode = bytecode.to_string();
    for (lib_name, address) in libraries {
        if let Some(placeholder) = LIBRARY_PLACEHOLDERS.get(lib_name) {
            linked_bytecode = linked_bytecode.replace(placeholder, &address.encode_hex());
        } else {
            return Err(ContractError::AbiError(
                ethers::abi::AbiError::DecodingError(Error::InvalidData),
            ));
        }
    }

    // Create the contract factory with the linked bytecode
    let factory = ContractFactory::new(abi, linked_bytecode.parse().unwrap(), provider.clone());

    // Deploy the contract
    let contract = factory.deploy(constructor_args)?.send().await?;

    Ok(contract)
}
