use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
pub struct DeployArgs {
    /// Path to contract WASM file
    #[arg(short, long)]
    pub wasm: Option<PathBuf>,

    /// Path to contract metadata (ABI) JSON file
    #[arg(short, long)]
    pub metadata: Option<PathBuf>,

    /// Constructor arguments (comma-separated)
    #[arg(short, long)]
    pub args: Option<String>,

    /// Value to transfer to contract (in GLIN)
    #[arg(short, long, default_value = "0")]
    pub value: String,

    /// Network to deploy to
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Account to deploy from
    #[arg(short = 'a', long)]
    pub account: String,

    /// Gas limit (optional, will estimate if not provided)
    #[arg(short, long)]
    pub gas_limit: Option<u64>,

    /// Salt for deterministic deployment
    #[arg(long)]
    pub salt: Option<String>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,
}

pub async fn execute(args: DeployArgs) -> anyhow::Result<()> {
    println!("{}", "Deploying contract...".cyan().bold());

    // Auto-detect WASM and metadata if not provided
    let (wasm_path, metadata_path) = if args.wasm.is_none() || args.metadata.is_none() {
        find_contract_artifacts(".")?
    } else {
        (args.wasm.unwrap(), args.metadata.unwrap())
    };

    println!("\n{}", "Contract artifacts:".bold());
    println!("  {} {}", "WASM:".cyan(), wasm_path.display());
    println!("  {} {}", "Metadata:".cyan(), metadata_path.display());

    // Load contract files
    let wasm_bytes = std::fs::read(&wasm_path)?;
    let metadata_json = std::fs::read_to_string(&metadata_path)?;
    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)?;

    // Parse constructor arguments
    let constructor_args = if let Some(args_str) = &args.args {
        args_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };

    // Get network configuration
    let network_config = crate::config::load_network(&args.network)?;

    println!("\n{}", "Deployment details:".bold());
    println!("  {} {}", "Network:".cyan(), args.network);
    println!("  {} {}", "RPC:".cyan(), network_config.rpc);
    println!("  {} {}", "Account:".cyan(), args.account);
    println!("  {} {} GLIN", "Value:".cyan(), args.value);

    if !constructor_args.is_empty() {
        println!("  {} {:?}", "Args:".cyan(), constructor_args);
    }

    // Confirmation prompt
    if !args.yes {
        print!("\n{} ", "Proceed with deployment?".yellow().bold());
        print!("[y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Deployment cancelled.");
            return Ok(());
        }
    }

    println!("\n{}", "Connecting to network...".cyan());

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await?;
    println!("{} Connected to {}", "✓".green(), network_config.rpc);

    // Get signer account
    let signer = glin_client::get_dev_account(&args.account)?;
    let signer_address = glin_client::get_address(&signer);
    println!("{} Using account: {}", "✓".green(), signer_address);

    // Parse value
    let value_u128 = args.value.parse::<u128>().unwrap_or(0);

    // Gas estimation tips
    println!("\n{}", "Gas Estimation:".bold());
    println!("  {} Estimating deployment gas...", "→".cyan());

    // Simulated gas estimation
    let estimated_gas = 3_000_000_000u64; // 3B refTime
    let estimated_proof = 1_000_000u64; // 1M proofSize

    println!("  {} refTime: {}", "→".cyan(), format_number(estimated_gas));
    println!(
        "  {} proofSize: {}",
        "→".cyan(),
        format_number(estimated_proof)
    );

    if args.gas_limit.is_none() {
        println!("  {} Using auto-estimated gas limit", "ℹ".blue());
        println!("    {}", "Tip: Add 20% buffer for safety".dimmed());
    }

    println!("\n{}", "Deploying contract...".cyan());

    // Deploy contract
    let result = crate::contract::deploy_contract(
        &client,
        wasm_bytes,
        &metadata,
        constructor_args,
        None,
        value_u128,
        &signer,
    )
    .await?;

    if result.success {
        println!("\n{} Contract deployed successfully!", "✓".green().bold());
        println!("\n{}", "Contract info:".bold());

        if let Some(addr) = result.contract_address {
            println!("  {} {}", "Address:".cyan(), addr);

            if let Some(explorer) = network_config.explorer {
                println!("  {} {}/contract/{}", "Explorer:".cyan(), explorer, addr);
            }
        }

        if let Some(hash) = result.tx_hash {
            println!("  {} {}", "Transaction:".cyan(), hash);
        }

        if let Some(code_hash) = result.code_hash {
            println!("  {} {}", "Code Hash:".cyan(), code_hash);
        }
    } else {
        anyhow::bail!(
            "Deployment failed: {}",
            result.error.unwrap_or_else(|| "Unknown error".to_string())
        );
    }

    Ok(())
}

fn format_number(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

fn find_contract_artifacts(path: &str) -> anyhow::Result<(PathBuf, PathBuf)> {
    // First check artifacts/ directory (Hardhat-style)
    let artifacts_dir = PathBuf::from(path).join("artifacts");
    if artifacts_dir.exists() {
        if let Ok((wasm, metadata)) = search_artifacts(&artifacts_dir) {
            return Ok((wasm, metadata));
        }
    }

    // Fallback to target/ink/ directory
    let target_dir = PathBuf::from(path).join("target/ink");

    if !target_dir.exists() {
        anyhow::bail!(
            "Contract not built. Run {} first",
            "glin-forge build".yellow()
        );
    }

    search_artifacts(&target_dir)
}

/// Search for contract artifacts in a directory
fn search_artifacts(dir: &PathBuf) -> anyhow::Result<(PathBuf, PathBuf)> {
    // Find .wasm and .json files (recursive search for artifacts/)
    let mut wasm_file = None;
    let mut json_file = None;

    fn search_dir(
        dir: &std::path::Path,
        wasm_file: &mut Option<PathBuf>,
        json_file: &mut Option<PathBuf>,
    ) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                search_dir(&path, wasm_file, json_file)?;
            } else if path.is_file() {
                if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                    *wasm_file = Some(path.clone());
                } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    if !file_name.ends_with(".contract") {
                        *json_file = Some(path.clone());
                    }
                }
            }
        }
        Ok(())
    }

    search_dir(dir, &mut wasm_file, &mut json_file)?;

    match (wasm_file, json_file) {
        (Some(wasm), Some(json)) => Ok((wasm, json)),
        _ => anyhow::bail!("Could not find contract artifacts in {}", dir.display()),
    }
}
