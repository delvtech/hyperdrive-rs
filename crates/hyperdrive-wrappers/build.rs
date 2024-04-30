use std::{env, fs::create_dir_all, io::Write, path::Path, process::Command};

use dotenv::dotenv;
use ethers::prelude::Abigen;
use eyre::Result;
use heck::ToSnakeCase;

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
    "ERC4626Target4",
    "ERC4626Target0Deployer",
    "ERC4626Target1Deployer",
    "ERC4626Target2Deployer",
    "ERC4626Target3Deployer",
    "ERC4626Target4Deployer",
    // stETH Hyperdrive
    "StETHHyperdrive",
    "StETHHyperdriveDeployerCoordinator",
    "StETHHyperdriveCoreDeployer",
    "StETHTarget0",
    "StETHTarget1",
    "StETHTarget2",
    "StETHTarget3",
    "StETHTarget4",
    "StETHTarget0Deployer",
    "StETHTarget1Deployer",
    "StETHTarget2Deployer",
    "StETHTarget3Deployer",
    "StETHTarget4Deployer",
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
    // Re-run this script whenever root .env, the build script itself, or a contract changes.
    println!("cargo:rerun-if-changed=../../.env");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hyperdrive/contracts/");

    // Fetch the environment variables
    dotenv().ok();
    let git_url = env::var("HYPERDRIVE_REPO_URL").expect("HYPERDRIVE_REPO_URL must be set");
    let git_ref = env::var("HYPERDRIVE_REF").expect("HYPERDRIVE_REF must be set");

    // Get the root directory of the project
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Clone the hyperdrive repository if it doesn't exist
    let hyperdrive_dir = root.join("hyperdrive");
    if !hyperdrive_dir.exists() {
        clone_repo(&git_url, &hyperdrive_dir, &git_ref)?;
    } else {
        // If the repository exists, checkout the specified ref and pull the latest changes.
        checkout_branch(&hyperdrive_dir, &git_ref)?;
        Command::new("git")
            .current_dir(&hyperdrive_dir)
            .args(["pull", "origin", &git_ref])
            .output()?;
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
    for (source, name) in artifacts {
        let target = name
            // Ensure that `StETH` is converted to `steth` in snake case.
            .replace("StETH", "STETH")
            // Ensure that `IHyperdrive` is converted to `ihyperdrive` in snake case.
            .replace("IHyperdrive", "IHYPERDRIVE")
            .to_snake_case();

        // Write the generated contract wrapper.
        let target_file = generated.join(format!("{}.rs", target));
        Abigen::new(name, source)?
            .add_derive("serde::Serialize")?
            .add_derive("serde::Deserialize")?
            // Alias the `IHyperdriveDeployerCoordinator.deploy()` to
            // `deploy_hyperdrive()` to avoid conflicts with the builtin
            // `deploy()` in the wrapper used to call the constructor.
            .add_method_alias("deploy(bytes32,(address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,address,address,address,(uint256,uint256,uint256,uint256)),bytes,bytes32)", "deploy_hyperdrive")
            // Alias the `IHyperdriveCoreDeployer.deploy()` to
            // `deploy_hyperdrive()` to avoid conflicts with the builtin
            // `deploy()` in the wrapper used to call the constructor.
            .add_method_alias("deploy((address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,(uint256,uint256,uint256,uint256)),bytes,address,address,address,address,address,bytes32)", "deploy_hyperdrive")
            // Alias the `IHyperdriveTarget.deploy()` to `deploy_target()`
            // to avoid conflicts with the builtin `deploy()` in the wrapper
            // used to call the constructor.
            .add_method_alias("deploy((address,address,address,bytes32,uint256,uint256,uint256,uint256,uint256,uint256,address,address,address,(uint256,uint256,uint256,uint256)),bytes,bytes32)", "deploy_target")
            .generate()?
            .write_to_file(target_file)?;

        // Append the generated contract wrapper to the mod file.
        writeln!(mod_file, "pub mod {};", target)?;
    }

    Ok(())
}

fn get_artifacts(artifacts_path: &Path) -> Result<Vec<(String, String)>> {
    if !artifacts_path.exists() {
        create_dir_all(artifacts_path)?;
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
                artifacts.push((source, name));
            }
        } else {
            artifacts.extend(get_artifacts(&path)?);
        }
    }
    Ok(artifacts)
}

// Git helpers

fn clone_repo(url: &str, path: &Path, branch: &str) -> Result<()> {
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

fn checkout_branch(repo_path: &Path, git_ref: &str) -> Result<()> {
    let status = Command::new("git")
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

    Ok(())
}
