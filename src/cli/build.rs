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
}

pub async fn execute(args: BuildArgs) -> anyhow::Result<()> {
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
