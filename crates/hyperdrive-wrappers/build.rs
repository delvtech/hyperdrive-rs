use std::{env, fs::read_to_string, io::Write, path::Path, process::Command};

use dotenv::dotenv;
use ethers::{abi::RawAbi, prelude::Abigen};
use eyre::Result;
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use serde_json;

const HYPERDRIVE_URL: &str = "https://github.com/delvtech/hyperdrive.git";

// The list of contracts we want to generate wrappers for.
const TARGETS: &[&str] = &[
    // Interfaces
    "IERC20",
    "IStETHHyperdrive",
    "IHyperdrive",
    "IHyperdriveFactory",
    // Tokens
    "ERC20Mintable",
    "ERC20ForwarderFactory",
    // Libraries
    "LPMath",
    // Hyperdrive Factory
    "HyperdriveFactory",
    // Hyperdrive Registry
    "HyperdriveRegistry",
    // ERC4626 Hyperdrive
    "ERC4626Hyperdrive",
    "ERC4626HyperdriveCoreDeployer",
    "ERC4626HyperdriveDeployerCoordinator",
    "ERC4626Target0",
    "ERC4626Target1",
    "ERC4626Target2",
    "ERC4626Target3",
    "ERC4626Target0Deployer",
    "ERC4626Target1Deployer",
    "ERC4626Target2Deployer",
    "ERC4626Target3Deployer",
    // stETH Hyperdrive
    "StETHHyperdrive",
    "StETHHyperdriveDeployerCoordinator",
    "StETHHyperdriveCoreDeployer",
    "StETHTarget0",
    "StETHTarget1",
    "StETHTarget2",
    "StETHTarget3",
    "StETHTarget0Deployer",
    "StETHTarget1Deployer",
    "StETHTarget2Deployer",
    "StETHTarget3Deployer",
    // Test Contracts
    "ERC20Mintable",
    "EtchingVault",
    "MockERC4626",
    "MockFixedPointMath",
    "MockHyperdriveMath",
    "MockLido",
    "MockLPMath",
    "MockYieldSpaceMath",
];

fn main() -> Result<()> {
    // Re-run this script whenever the build script itself, the version file, or a contract changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hyperdrive.version");
    println!("cargo:rerun-if-changed=hyperdrive/contracts/");

    // load dotenv
    dotenv().ok();

    let local_development = match env::var("LOCAL_DEVELOPMENT") {
        Ok(local_development) => local_development == "true",
        _ => false,
    };

    // Get the root directory of the project
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Load the hyperdrive version to use from the hyperdrive.version file
    let version_file = root.join("hyperdrive.version");
    let git_ref = read_to_string(&version_file)?.trim().to_string();

    // Clone the hyperdrive repository if it doesn't exist
    let hyperdrive_dir = root.join("hyperdrive");

    if !local_development {
        if hyperdrive_dir.exists() {
            checkout_branch(&git_ref, &hyperdrive_dir)?;
        } else {
            clone_repo(&HYPERDRIVE_URL, &git_ref, &hyperdrive_dir)?;
        }
    }

    // Compile the contracts.
    Command::new("forge")
        .current_dir(&hyperdrive_dir)
        .args(["build"])
        .output()?;

    // If there is an existing `wrappers` module, remove it. Then prepare to
    // re-write these files.
    let generated = root.join("src/wrappers");
    if generated.exists() {
        std::fs::remove_dir_all(&generated)?;
    }
    std::fs::create_dir_all(&generated)?;
    let mod_file = generated.join("mod.rs");
    let mut mod_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(mod_file)?;

    // Generate the relevant contract wrappers from Foundry's artifacts.
    let artifacts = hyperdrive_dir.join("out");
    let mut artifacts = get_artifacts(&artifacts)?;
    artifacts.sort_by(|a, b| a.1.cmp(&b.1));
    artifacts.dedup_by(|a, b| a.1.eq(&b.1));

    // For each artifact, generate a wrapper.
    for (source, name, bytecode) in artifacts {
        let target = name
            // Ensure that `StETH` is converted to `steth` in snake case.
            .replace("StETH", "STETH")
            // Ensure that `IHyperdrive` is converted to `ihyperdrive` in snake case.
            .replace("IHyperdrive", "IHYPERDRIVE")
            .to_snake_case();

        // Make the generated contract wrapper file.
        let target_file = generated.join(format!("{}.rs", target));

        // Generate the wrapper with Abigen.
        let mut output = Abigen::new(&name, source)?
            .add_derive("serde::Serialize")?
            .add_derive("serde::Deserialize")?
            .generate()?
            .to_string();

        // Check if bytecode contains placeholders, indicating it is "invalid"
        match bytecode.contains("__$") {
            true => {
                // Find the last '}' in the file to insert before it
                if let Some(pos) = output.rfind('}') {
                    let (start, end) = output.split_at(pos);
                    output = format!(
                        r#"
{}
    impl<M: ::ethers::providers::Middleware> {}<M> {{
        pub async fn deploy_linked_contract<T: ::ethers::abi::Tokenize>(
            libraries: Vec<(&str, ::ethers::abi::Address)>,
            provider: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<::ethers::contract::ContractInstance<::std::sync::Arc<M>, M>, ::ethers::contract::ContractError<M>>
        where
            M: ::ethers::providers::Middleware,
        {{
            // Replace placeholders in bytecode
            let bytecode = {}_UNLINKED_BYTECODE;
            let mut linked_bytecode = bytecode.to_string();
            for (lib_name, address) in libraries {{
                if let Some(placeholder) = crate::LIBRARY_PLACEHOLDERS.get(lib_name) {{
                    use ethers::abi::AbiEncode;
                    linked_bytecode = linked_bytecode.replace(placeholder, &address.encode_hex().split_at(26).1);
                }} else {{
                    return Err(::ethers::contract::ContractError::AbiError(
                        ethers::abi::AbiError::DecodingError(::ethers::abi::Error::InvalidData),
                    ));
                }}
            }}

            // Create the contract factory with the linked bytecode
            let factory = ::ethers::contract::ContractFactory::new(__abi(), linked_bytecode.parse().unwrap(), provider.clone());

            // Deploy the contract
            let contract = factory.deploy(constructor_args)?.send().await?;

            Ok(contract)
        }}
    }}

    pub const {}_UNLINKED_BYTECODE: &str = "{}";
{}"#,
                        start,
                        name,
                        name.to_uppercase(),
                        name.to_uppercase(),
                        bytecode,
                        end
                    );
                }
            }
            false => (),
        }

        // Write the modified output to the target file.
        std::fs::write(&target_file, output)?;

        // Append the module declaration to the mod.rs file.
        writeln!(mod_file, "pub mod {};", target)?;
    }
    Ok(())
}

fn get_artifacts(artifacts_path: &Path) -> Result<Vec<(String, String, String)>> {
    if !artifacts_path.exists() {
        std::fs::create_dir_all(artifacts_path)?;
    }
    let mut artifacts = Vec::new();
    for entry in std::fs::read_dir(artifacts_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let source = path.clone().into_os_string().into_string().unwrap();
            let name = String::from(path.file_stem().unwrap().to_str().unwrap());
            // If the artifact is one of our targets, add it to the list.
            if TARGETS.contains(&name.as_str()) {
                let contents = std::fs::read_to_string(&path)?;
                let bytecode = contract_bytecode(&contents)?;
                artifacts.push((source, name, bytecode));
            }
        } else {
            artifacts.extend(get_artifacts(&path)?);
        }
    }
    Ok(artifacts)
}

#[derive(Serialize, Deserialize)]
pub struct BytesObject {
    object: String,
}

#[derive(Serialize, Deserialize)]
pub struct AbiObj {
    pub abi: RawAbi,
    pub bytecode: BytesObject,
}

fn contract_bytecode(abi_json: &str) -> std::io::Result<String> {
    let artifact = serde_json::from_str::<AbiObj>(&abi_json)?;
    let bytecode = artifact.bytecode.object;
    Ok(bytecode)
}

// Git helpers
fn clone_repo(url: &str, branch: &str, path: &Path) -> Result<()> {
    let clone_status = Command::new("git")
        .args([
            "clone",
            "--recurse-submodules",
            "--branch",
            branch,
            url,
            path.to_str().unwrap(),
        ])
        .status()?;

    if !clone_status.success() {
        eyre::bail!("Failed to clone hyperdrive repository");
    }

    Ok(())
}

fn checkout_branch(git_ref: &str, repo_path: &Path) -> Result<()> {
    let mut status = Command::new("git")
        .current_dir(repo_path)
        .args(["checkout", git_ref])
        .status()?;

    if !status.success() {
        eyre::bail!(
            "Failed to checkout ref '{}' in repository at {:?}",
            git_ref,
            repo_path
        );
    }

    status = Command::new("git")
        .current_dir(&repo_path)
        .args(["pull", "origin", &git_ref])
        .status()?;

    if !status.success() {
        eyre::bail!(
            "Failed to pull latest changes from ref '{}' in repository at {:?}",
            git_ref,
            repo_path
        );
    }

    Ok(())
}
