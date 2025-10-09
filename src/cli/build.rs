use clap::Parser;
use colored::Colorize;
use std::process::Command;

#[derive(Parser)]
pub struct BuildArgs {
    /// Path to the contract project
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Build in release mode
    #[arg(long)]
    pub release: bool,

    /// Verify the contract after building
    #[arg(long)]
    pub verify: bool,

    /// Copy build outputs to artifacts directory (Hardhat-style)
    #[arg(long, default_value = "artifacts")]
    pub artifacts_dir: Option<String>,

    /// Skip artifacts copy (only use target/ink)
    #[arg(long)]
    pub no_artifacts: bool,

    /// Build all contracts in the workspace
    #[arg(long)]
    pub all: bool,
}

pub async fn execute(args: BuildArgs) -> anyhow::Result<()> {
    // If --all flag is set, find and build all contracts
    if args.all {
        return build_all_contracts(&args).await;
    }

    build_single_contract(&args)
}

/// Build a single contract
fn build_single_contract(args: &BuildArgs) -> anyhow::Result<()> {
    println!("{}", "Building contract...".cyan().bold());

    // Check if cargo-contract is installed
    let cargo_contract_check = Command::new("cargo")
        .arg("contract")
        .arg("--version")
        .output();

    if cargo_contract_check.is_err() {
        anyhow::bail!(
            "cargo-contract not found. Install it with: {}",
            "cargo install cargo-contract --force".yellow()
        );
    }

    // Build the contract
    let mut cmd = Command::new("cargo");
    cmd.arg("contract").arg("build");

    if args.release {
        cmd.arg("--release");
    }

    cmd.current_dir(&args.path);

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Build failed:\n{}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    println!(
        "\n{} Contract built successfully!",
        "✓".green().bold()
    );

    // Print output paths
    let target_dir = std::path::Path::new(&args.path).join("target/ink");
    if target_dir.exists() {
        println!("\n{}", "Output files:".bold());
        for entry in std::fs::read_dir(&target_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                println!("  {} {}", "→".cyan(), path.display());
            }
        }
    }

    if args.verify {
        println!("\n{}", "Verifying contract...".cyan());
        verify_built_contract(&target_dir)?;
    }

    // Copy to artifacts directory (Hardhat-style)
    if !args.no_artifacts {
        if let Some(artifacts_dir) = &args.artifacts_dir {
            copy_to_artifacts(&args.path, artifacts_dir)?;

            println!("\n{} Artifacts copied to {}/",
                "✓".green().bold(),
                artifacts_dir
            );
        }
    }

    Ok(())
}

/// Verify the built contract artifacts
fn verify_built_contract(target_dir: &std::path::Path) -> anyhow::Result<()> {
    use sp_core_hashing::blake2_256;

    // Find WASM and metadata files
    let mut wasm_path = None;
    let mut metadata_path = None;

    for entry in std::fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            wasm_path = Some(path);
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !file_name.ends_with(".contract") {
                metadata_path = Some(path);
            }
        }
    }

    let wasm_path = wasm_path.ok_or_else(|| anyhow::anyhow!("WASM file not found"))?;
    let metadata_path = metadata_path.ok_or_else(|| anyhow::anyhow!("Metadata file not found"))?;

    // 1. Calculate WASM code hash
    let wasm_bytes = std::fs::read(&wasm_path)?;
    let code_hash = blake2_256(&wasm_bytes);

    println!("  {} 0x{}", "Code hash:".cyan(), hex::encode(code_hash));

    // 2. Validate metadata structure
    let metadata_json = std::fs::read_to_string(&metadata_path)?;
    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)?;
    crate::contract::metadata::validate_metadata(&metadata)?;

    println!("  {} Metadata structure valid", "✓".green());

    // 3. Check WASM size
    let size_kb = wasm_bytes.len() / 1024;
    let size_mb = size_kb as f64 / 1024.0;

    if size_mb >= 1.0 {
        println!("  {} {:.2} MB", "WASM size:".cyan(), size_mb);
    } else {
        println!("  {} {} KB", "WASM size:".cyan(), size_kb);
    }

    // Size warnings
    if size_kb > 100 {
        println!("  {} Large contract size, deployment may be expensive", "⚠".yellow());
    }

    if size_kb > 500 {
        println!("  {} Contract size over 500 KB - consider optimization", "⚠".yellow().bold());
        println!("    {}", "Try building with --release flag".dimmed());
    }

    // 4. Validate contract metadata completeness
    let constructors = crate::contract::metadata::list_constructors(&metadata);
    let messages = crate::contract::metadata::list_messages(&metadata);

    println!("  {} {} constructor(s)", "Constructors:".cyan(), constructors.len());
    println!("  {} {} message(s)", "Messages:".cyan(), messages.len());

    if constructors.is_empty() {
        anyhow::bail!("Contract has no constructors - invalid contract");
    }

    if messages.is_empty() {
        println!("  {} Contract has no messages (read-only contract)", "⚠".yellow());
    }

    println!("\n{} Contract verification passed", "✓".green().bold());

    Ok(())
}

/// Copy build artifacts to artifacts directory (Hardhat-style)
fn copy_to_artifacts(project_path: &str, artifacts_dir: &str) -> anyhow::Result<()> {
    // Find contract name from Cargo.toml
    let cargo_toml_path = std::path::Path::new(project_path).join("Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path)?;
    let toml_value: toml::Value = toml::from_str(&cargo_toml_content)?;
    let contract_name = toml_value
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| anyhow::anyhow!("Contract name not found in Cargo.toml"))?;

    // Resolve to absolute path
    let base_path = std::env::current_dir()?.join(project_path);

    // Check for workspace - look for target/ink in current dir and parent dirs
    let mut target_dir = base_path.join("target/ink");
    let mut artifacts_base = base_path.clone();

    if !target_dir.exists() {
        // Try parent directory (workspace case)
        if let Some(parent_path) = base_path.parent() {
            let workspace_target = parent_path.join("target/ink");
            if workspace_target.exists() {
                target_dir = workspace_target;
                artifacts_base = parent_path.to_path_buf();
            }
        }
    }

    // Source directory with contract files
    let source_dir = target_dir.join(contract_name);

    if !source_dir.exists() {
        anyhow::bail!(
            "Build artifacts not found at {}. Run build first.",
            source_dir.display()
        );
    }

    // Create artifacts/<contract_name>/ directory (in workspace root if workspace)
    let artifacts_path = artifacts_base
        .join(artifacts_dir)
        .join(contract_name);
    std::fs::create_dir_all(&artifacts_path)?;

    // Copy all 3 files: .json, .wasm, .contract
    let mut files_copied = 0;
    for entry in std::fs::read_dir(&source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_str().unwrap_or("");
                if ext_str == "json" || ext_str == "wasm" || ext_str == "contract" {
                    let file_name = path.file_name().unwrap();
                    let dest = artifacts_path.join(file_name);
                    std::fs::copy(&path, &dest)?;
                    files_copied += 1;
                }
            }
        }
    }

    if files_copied == 0 {
        anyhow::bail!("No artifacts found to copy from {}", source_dir.display());
    }

    Ok(())
}

/// Build all contracts in a workspace
async fn build_all_contracts(args: &BuildArgs) -> anyhow::Result<()> {
    use std::path::Path;

    println!("{}", "Building all contracts in workspace...".cyan().bold());
    println!();

    let base_path = Path::new(&args.path);
    let contracts_dir = base_path.join("contracts");

    if !contracts_dir.exists() {
        anyhow::bail!(
            "No contracts directory found. Expected at: {}",
            contracts_dir.display()
        );
    }

    // Find all contract directories (directories with Cargo.toml containing [package])
    let mut contract_paths = Vec::new();

    for entry in std::fs::read_dir(&contracts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let cargo_toml = path.join("Cargo.toml");
            if cargo_toml.exists() {
                // Verify it's a contract project
                let content = std::fs::read_to_string(&cargo_toml)?;
                if content.contains("[package]") {
                    contract_paths.push(path);
                }
            }
        }
    }

    if contract_paths.is_empty() {
        println!("{} No contracts found in {}/", "⚠".yellow(), contracts_dir.display());
        return Ok(());
    }

    println!("Found {} contract(s) to build:", contract_paths.len());
    for path in &contract_paths {
        println!("  {} {}", "→".cyan(), path.file_name().unwrap().to_string_lossy());
    }
    println!();

    let mut built_count = 0;
    let mut failed = Vec::new();

    for contract_path in &contract_paths {
        let contract_name = contract_path.file_name().unwrap().to_string_lossy();
        println!("{} Building {}...", "▸".cyan().bold(), contract_name.bold());

        // Build this contract
        let build_args = BuildArgs {
            path: contract_path.to_string_lossy().to_string(),
            release: args.release,
            verify: args.verify,
            artifacts_dir: args.artifacts_dir.clone(),
            no_artifacts: args.no_artifacts,
            all: false,
        };

        match build_single_contract(&build_args) {
            Ok(_) => {
                built_count += 1;
                println!();
            }
            Err(e) => {
                failed.push((contract_name.to_string(), e.to_string()));
                println!("{} Failed to build {}: {}\n", "✗".red().bold(), contract_name, e);
            }
        }
    }

    println!();
    println!("{}", "=== Build Summary ===".bold());
    println!("  {} {}/{} contracts built successfully",
        "✓".green(),
        built_count,
        contract_paths.len()
    );

    if !failed.is_empty() {
        println!("  {} {} failed:", "✗".red(), failed.len());
        for (name, error) in &failed {
            println!("    • {}: {}", name, error);
        }
        anyhow::bail!("Some contracts failed to build");
    }

    Ok(())
}
