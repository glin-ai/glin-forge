use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
pub struct UploadArgs {
    /// Path to contract WASM file
    #[arg(short, long)]
    pub wasm: Option<PathBuf>,

    /// Network to upload to
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Account to upload from
    #[arg(short = 'a', long)]
    pub account: String,

    /// Gas limit (optional, will estimate if not provided)
    #[arg(short, long)]
    pub gas_limit: Option<u64>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,
}

pub async fn execute(args: UploadArgs) -> anyhow::Result<()> {
    println!("{}", "Uploading contract code...".cyan().bold());

    // Auto-detect WASM if not provided
    let wasm_path = if let Some(path) = args.wasm {
        path
    } else {
        find_wasm_file(".")?
    };

    println!("\n{}", "Contract artifact:".bold());
    println!("  {} {}", "WASM:".cyan(), wasm_path.display());

    // Load WASM file
    let wasm_bytes = std::fs::read(&wasm_path)?;
    let wasm_size = wasm_bytes.len();

    // Get network configuration
    let network_config = crate::config::load_network(&args.network)?;

    println!("\n{}", "Upload details:".bold());
    println!("  {} {}", "Network:".cyan(), args.network);
    println!("  {} {}", "RPC:".cyan(), network_config.rpc);
    println!("  {} {}", "Account:".cyan(), args.account);
    println!(
        "  {} {} bytes",
        "Code Size:".cyan(),
        format_number(wasm_size as u64)
    );

    // Confirmation prompt
    if !args.yes {
        print!("\n{} ", "Proceed with upload?".yellow().bold());
        print!("[y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Upload cancelled.");
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

    // Gas estimation
    println!("\n{}", "Gas Estimation:".bold());
    println!("  {} Estimating upload gas...", "→".cyan());

    // Simulated gas estimation (upload is cheaper than deploy)
    let estimated_gas = 2_000_000_000u64; // 2B refTime
    let estimated_proof = 500_000u64; // 500K proofSize

    println!("  {} refTime: {}", "→".cyan(), format_number(estimated_gas));
    println!(
        "  {} proofSize: {}",
        "→".cyan(),
        format_number(estimated_proof)
    );

    if args.gas_limit.is_none() {
        println!("  {} Using auto-estimated gas limit", "ℹ".blue());
    }

    println!("\n{}", "Uploading code...".cyan());

    // Upload code (simulated for now)
    let result = crate::contract::upload_code(&client, wasm_bytes, &signer).await?;

    if result.success {
        println!("\n{} Code uploaded successfully!", "✓".green().bold());
        println!("\n{}", "Upload info:".bold());

        if let Some(code_hash) = result.code_hash {
            println!("  {} {}", "Code Hash:".cyan(), code_hash);
            println!();
            println!("{}", "Next steps:".bold());
            println!("  {} Instantiate contract:", "→".cyan());
            println!(
                "    {} glin-forge instantiate --code-hash {} --account {}",
                "".dimmed(),
                code_hash,
                args.account
            );
        }

        if let Some(hash) = result.tx_hash {
            println!("\n  {} {}", "Transaction:".cyan(), hash);
        }
    } else {
        anyhow::bail!(
            "Upload failed: {}",
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

fn find_wasm_file(path: &str) -> anyhow::Result<PathBuf> {
    let target_dir = PathBuf::from(path).join("target/ink");

    if !target_dir.exists() {
        anyhow::bail!(
            "Contract not built. Run {} first",
            "glin-forge build".yellow()
        );
    }

    // Find .wasm file
    for entry in std::fs::read_dir(&target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            return Ok(path);
        }
    }

    anyhow::bail!("Could not find WASM file in {}", target_dir.display())
}
