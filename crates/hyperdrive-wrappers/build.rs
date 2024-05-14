use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};

use dotenv::dotenv;
use ethers::prelude::Abigen;
use eyre::Result;
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

const HYPERDRIVE_URL: &str = "https://github.com/delvtech/hyperdrive.git";

// The list of contracts we want to generate wrappers for.
const TARGETS: &[&str] = &[
    // Interfaces
    "IERC20",
    "IStETHHyperdrive",
    "IHyperdrive",
    "IHyperdriveFactory",
    // Libraries
    "LPMath",
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
    let git_ref = read_to_string(version_file)?.trim().to_string();

    // Clone the hyperdrive repository if it doesn't exist
    let hyperdrive_dir = root.join("hyperdrive");

    if !local_development {
        if hyperdrive_dir.exists() {
            checkout_branch(&git_ref, &hyperdrive_dir)?;
        } else {
            clone_repo(HYPERDRIVE_URL, &git_ref, &hyperdrive_dir)?;
        }
    }

    // Compile the contracts.
    Command::new("forge")
        .current_dir(&hyperdrive_dir)
        .arg("install")
        .output()?;
    Command::new("forge")
        .current_dir(&hyperdrive_dir)
        .args(["build", "--force"])
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
    artifacts.sort_by(|a, b| a.name.cmp(&b.name));
    artifacts.dedup_by(|a, b| a.name.eq(&b.name));
    for artifact in artifacts {
        let target = artifact
            .name
            // Ensure that `StETH` is converted to `steth` in snake case.
            .replace("StETH", "STETH")
            // Ensure that `IHyperdrive` is converted to `ihyperdrive` in snake case.
            .replace("IHyperdrive", "IHYPERDRIVE")
            .to_snake_case();

        // Write the generated contract wrapper.
        let target_file = generated.join(format!("{}.rs", target));
        Abigen::new(&artifact.name, artifact.path)?
            .add_derive("serde::Serialize")?
            .add_derive("serde::Deserialize")?
            .generate()?
            .write_to_file(&target_file)?;

        // Check if there are any external libraries that the contract depends
        // on. If there are, append a `link_and_deploy` function to the
        // generated contract wrapper that can be used to deploy the contract.
        if !artifact.libs.is_empty() {
            let mut generated_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(target_file)?;

            writeln!(
                generated_file,
                "{}",
                format_args!(
                    r#"
pub struct {name}Libs {{
    {libs}
}}

impl<M: ::ethers::providers::Middleware> {name}<M> {{
    pub fn link_and_deploy<T: ::ethers::core::abi::Tokenize>(
        client: ::std::sync::Arc<M>,
        constructor_args: T,
        libs: {name}Libs,
    ) -> ::core::result::Result<
        ::ethers::contract::builders::ContractDeployer<M, Self>,
        ::ethers::contract::ContractError<M>,
    > {{
        let factory = crate::linked_factory::create(
            {all_caps_name}_ABI.clone(),
            "{bytecode}",
            [
                {lib_entries}
            ],
            client.clone(),
        ).unwrap();
        let deployer = factory.deploy(constructor_args)?;
        let deployer = ::ethers::contract::ContractDeployer::new(deployer);
        Ok(deployer)
    }}
}}
"#,
                    name = artifact.name,
                    all_caps_name = artifact.name.to_uppercase(),
                    bytecode = artifact.bytecode,
                    libs = artifact
                        .libs
                        .iter()
                        .map(|lib| {
                            format!(
                                "pub {}: ::ethers::types::Address,",
                                lib.lib_name.to_snake_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n    "),
                    lib_entries = artifact
                        .libs
                        .iter()
                        .map(|lib| {
                            format!(
                                r#"(
                    "{path}:{name}",
                    libs.{key},
                )"#,
                                path = lib.file_path,
                                name = lib.lib_name,
                                key = lib.lib_name.to_snake_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(",\n            "),
                )
            )?;
        }

        // Append the generated contract wrapper to the mod file.
        writeln!(mod_file, "pub mod {};", target)?;
    }

    Ok(())
}

struct ArtifactLibs {
    file_path: String,
    lib_name: String,
}

struct Artifact {
    path: String,
    name: String,
    bytecode: String,
    libs: Vec<ArtifactLibs>,
}

fn get_artifacts(artifacts_path: &Path) -> Result<Vec<Artifact>> {
    if !artifacts_path.exists() {
        create_dir_all(artifacts_path)?;
    }
    let mut artifacts = Vec::new();
    for entry in std::fs::read_dir(artifacts_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let artifact_path = path.clone().into_os_string().into_string().unwrap();
            let name = String::from(path.file_stem().unwrap().to_str().unwrap());
            // If the artifact is one of our targets, add it to the list.
            if TARGETS.contains(&name.as_str()) {
                // Parse the artifact to get the bytecode and the the external
                // libraries it depends on.
                let (bytecode, libs) = parse_bytecode(&artifact_path)?;
                artifacts.push(Artifact {
                    path: artifact_path,
                    name,
                    bytecode,
                    libs,
                });
            }
        } else {
            artifacts.extend(get_artifacts(&path)?);
        }
    }
    Ok(artifacts)
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ArtifactByteCode {
    object: String,
    linkReferences: HashMap<
        String, // file_path
        HashMap<
            String, // lib_name
            serde_json::Value,
        >,
    >,
}

#[derive(Serialize, Deserialize)]
struct ArtifactWithBytecode {
    bytecode: ArtifactByteCode,
}

fn parse_bytecode(artifact_path: &str) -> eyre::Result<(String, Vec<ArtifactLibs>)> {
    let source_code = read_to_string(artifact_path)?;
    let artifact = serde_json::from_str::<ArtifactWithBytecode>(&source_code)?;
    let libs = artifact
        .bytecode
        .linkReferences
        .iter()
        .flat_map(|(file_path, libs)| {
            libs.iter()
                .map(|(lib_name, _)| ArtifactLibs {
                    file_path: file_path.clone(),
                    lib_name: lib_name.clone(),
                })
                .collect::<Vec<ArtifactLibs>>()
        })
        .collect::<Vec<ArtifactLibs>>();
    Ok((artifact.bytecode.object, libs))
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
        .args(["fetch"])
        .status()?;

    if !status.success() {
        eyre::bail!("Failed to fetch in repository at {:?}", repo_path);
    }

    status = Command::new("git")
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
        .current_dir(repo_path)
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
