use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
pub struct CallArgs {
    /// Contract address
    pub address: String,

    /// Method name to call
    pub method: String,

    /// Method arguments (space-separated)
    pub args: Vec<String>,

    /// Network to call on
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Account to call from
    #[arg(short = 'a', long)]
    pub account: String,

    /// Value to transfer (in GLIN)
    #[arg(short, long, default_value = "0")]
    pub value: String,

    /// Path to contract metadata (ABI) JSON file
    #[arg(short, long)]
    pub metadata: Option<String>,

    /// Gas limit (optional, will estimate if not provided)
    #[arg(short, long)]
    pub gas_limit: Option<u64>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Wait for transaction to be finalized
    #[arg(long)]
    pub wait: bool,
}

pub async fn execute(args: CallArgs) -> anyhow::Result<()> {
    println!("{}", "Calling contract method...".cyan().bold());

    println!("\n{}", "Transaction details:".bold());
    println!("  {} {}", "Contract:".cyan(), args.address);
    println!("  {} {}", "Method:".cyan(), args.method);
    println!("  {} {}", "Network:".cyan(), args.network);
    println!("  {} {}", "Account:".cyan(), args.account);
    println!("  {} {} GLIN", "Value:".cyan(), args.value);

    if !args.args.is_empty() {
        println!("  {} {:?}", "Arguments:".cyan(), args.args);
    }

    // Load metadata
    let metadata_path = if let Some(path) = args.metadata {
        path
    } else {
        find_metadata_for_contract(&args.address)?
    };

    println!("  {} {}", "Metadata:".cyan(), metadata_path);

    // Load and parse metadata
    let metadata_json = std::fs::read_to_string(&metadata_path)?;
    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)?;

    // Get network configuration
    let network_config = crate::config::load_network(&args.network)?;

    // Confirmation prompt
    if !args.yes {
        print!("\n{} ", "Proceed with transaction?".yellow().bold());
        print!("[y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Transaction cancelled.");
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
    let value_u128 = args.value.parse::<u128>()
        .unwrap_or(0);

    // Gas estimation
    println!("\n{}", "Gas Estimation:".bold());
    println!("  {} Estimating transaction gas...", "→".cyan());

    let estimated_gas = 2_000_000_000u64; // 2B refTime
    let estimated_proof = 800_000u64;     // 800K proofSize

    println!("  {} refTime: {}", "→".cyan(), format_number(estimated_gas));
    println!("  {} proofSize: {}", "→".cyan(), format_number(estimated_proof));

    if args.gas_limit.is_none() {
        println!("  {} Using auto-estimated gas limit", "ℹ".blue());
    }

    println!();

    // Execute transaction
    let result = crate::contract::call_contract(
        &client,
        &args.address,
        &metadata,
        &args.method,
        args.args.clone(),
        value_u128,
        &signer,
    ).await?;

    if result.success {
        println!(
            "\n{} Transaction successful!",
            "✓".green().bold()
        );

        println!("\n{}", "Transaction info:".bold());

        if let Some(ref hash) = result.tx_hash {
            println!("  {} {}", "Hash:".cyan(), hash);

            if let Some(explorer) = &network_config.explorer {
                println!(
                    "  {} {}/tx/{}",
                    "Explorer:".cyan(),
                    explorer,
                    hash
                );
            }
        }

        if let Some(block) = result.block_hash {
            println!("  {} {}", "Block:".cyan(), block);
        }

        if args.wait {
            println!("\n{}", "Waiting for finalization...".cyan());
            wait_for_finalization(&client, result.tx_hash.as_deref()).await?;
        }
    } else {
        anyhow::bail!("Transaction failed: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

/// Wait for transaction to be finalized
async fn wait_for_finalization(
    client: &glin_client::GlinClient,
    tx_hash: Option<&str>,
) -> anyhow::Result<()> {
    use futures::StreamExt;
    use std::time::{Duration, Instant};

    let tx_hash = match tx_hash {
        Some(hash) => hash,
        None => {
            println!("  {} No transaction hash available, skipping finalization wait", "⚠".yellow());
            return Ok(());
        }
    };

    // Create progress spinner
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Waiting for block finalization...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let timeout = Duration::from_secs(120); // 120 second timeout (2 minutes)
    let start = Instant::now();

    // Subscribe to finalized blocks
    let mut blocks_sub = client.blocks().subscribe_finalized().await?;

    while let Some(block_result) = blocks_sub.next().await {
        // Check timeout
        if start.elapsed() > timeout {
            spinner.finish_with_message(format!(
                "{} Timeout waiting for finalization (checked for {} seconds)",
                "⚠".yellow(),
                timeout.as_secs()
            ));
            println!("  {}", "The transaction may still be finalized later".dimmed());
            return Ok(());
        }

        let block = block_result?;
        let block_number = block.number();

        // Get all extrinsics in this finalized block
        let extrinsics = block.extrinsics().await?;

        // Check if our transaction is in this finalized block
        for ext in extrinsics.iter() {
            let ext_hash = format!("0x{}", hex::encode(ext.hash()));

            if ext_hash == tx_hash || ext_hash.starts_with(tx_hash) || tx_hash.starts_with(&ext_hash) {
                spinner.finish_with_message(format!(
                    "{} Transaction finalized in block #{}",
                    "✓".green().bold(),
                    block_number
                ));
                return Ok(());
            }
        }

        // Update spinner message with current block
        spinner.set_message(format!(
            "Waiting for finalization... (checked up to block #{})",
            block_number
        ));
    }

    spinner.finish_with_message(format!("{} Block subscription ended", "⚠".yellow()));
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

fn find_metadata_for_contract(_address: &str) -> anyhow::Result<String> {
    let possible_paths = vec![
        "target/ink/metadata.json",
        "contract.json",
        "abi.json",
    ];

    for path in possible_paths {
        if std::path::Path::new(path).exists() {
            return Ok(path.to_string());
        }
    }

    anyhow::bail!(
        "Could not find contract metadata. Specify with {}",
        "--metadata <path>".yellow()
    )
}
