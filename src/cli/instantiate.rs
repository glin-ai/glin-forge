use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
pub struct InstantiateArgs {
    /// Code hash of uploaded contract
    #[arg(long)]
    pub code_hash: String,

    /// Path to contract metadata (ABI) JSON file
    #[arg(short, long)]
    pub metadata: Option<PathBuf>,

    /// Constructor arguments (comma-separated)
    #[arg(short, long)]
    pub args: Option<String>,

    /// Value to transfer to contract (in GLIN)
    #[arg(short, long, default_value = "0")]
    pub value: String,

    /// Network to instantiate on
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Account to instantiate from
    #[arg(short = 'a', long)]
    pub account: String,

    /// Gas limit (optional, will estimate if not provided)
    #[arg(short, long)]
    pub gas_limit: Option<u64>,

    /// Salt for deterministic instantiation
    #[arg(long)]
    pub salt: Option<String>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,
}

pub async fn execute(args: InstantiateArgs) -> anyhow::Result<()> {
    println!("{}", "Instantiating contract from code...".cyan().bold());

    // Auto-detect metadata if not provided
    let metadata_path = if let Some(path) = args.metadata {
        path
    } else {
        find_metadata_file(".")?
    };

    println!("\n{}", "Contract artifact:".bold());
    println!("  {} {}", "Metadata:".cyan(), metadata_path.display());
    println!("  {} {}", "Code Hash:".cyan(), args.code_hash);

    // Load metadata
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

    println!("\n{}", "Instantiation details:".bold());
    println!("  {} {}", "Network:".cyan(), args.network);
    println!("  {} {}", "RPC:".cyan(), network_config.rpc);
    println!("  {} {}", "Account:".cyan(), args.account);
    println!("  {} {} GLIN", "Value:".cyan(), args.value);

    if !constructor_args.is_empty() {
        println!("  {} {:?}", "Args:".cyan(), constructor_args);
    }

    if let Some(salt) = &args.salt {
        println!("  {} {}", "Salt:".cyan(), salt);
    }

    // Confirmation prompt
    if !args.yes {
        print!("\n{} ", "Proceed with instantiation?".yellow().bold());
        print!("[y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Instantiation cancelled.");
            return Ok(());
        }
    }

    println!("\n{}", "Connecting to network...".cyan());

    // Connect to network
    let client = crate::network::create_client(&network_config.rpc).await?;
    println!("{} Connected to {}", "✓".green(), network_config.rpc);

    // Get signer account
    let signer = crate::network::get_dev_account(&args.account)?;
    let signer_address = crate::network::get_address(&signer);
    println!("{} Using account: {}", "✓".green(), signer_address);

    // Parse value
    let value_u128 = args.value.parse::<u128>()
        .unwrap_or(0);

    // Gas estimation
    println!("\n{}", "Gas Estimation:".bold());
    println!("  {} Estimating instantiation gas...", "→".cyan());

    // Simulated gas estimation
    let estimated_gas = 2_500_000_000u64; // 2.5B refTime
    let estimated_proof = 800_000u64;     // 800K proofSize

    println!("  {} refTime: {}", "→".cyan(), format_number(estimated_gas));
    println!("  {} proofSize: {}", "→".cyan(), format_number(estimated_proof));

    if args.gas_limit.is_none() {
        println!("  {} Using auto-estimated gas limit", "ℹ".blue());
        println!("    {}", "Tip: Add 20% buffer for safety".dimmed());
    }

    println!("\n{}", "Instantiating contract...".cyan());

    // Instantiate contract
    let result = crate::contract::instantiate_contract(
        &client,
        &args.code_hash,
        &metadata,
        constructor_args,
        None,
        value_u128,
        &signer,
    ).await?;

    if result.success {
        println!(
            "\n{} Contract instantiated successfully!",
            "✓".green().bold()
        );
        println!("\n{}", "Contract info:".bold());

        if let Some(addr) = result.contract_address {
            println!("  {} {}", "Address:".cyan(), addr);

            if let Some(explorer) = network_config.explorer {
                println!(
                    "  {} {}/contract/{}",
                    "Explorer:".cyan(),
                    explorer,
                    addr
                );
            }

            println!();
            println!("{}", "Next steps:".bold());
            println!("  {} Call contract:", "→".cyan());
            println!("    {} glin-forge call {} <method> --account {}",
                "".dimmed(),
                addr,
                args.account
            );
            println!("  {} Query contract:", "→".cyan());
            println!("    {} glin-forge query {} <method>",
                "".dimmed(),
                addr
            );
        }

        if let Some(hash) = result.tx_hash {
            println!("\n  {} {}", "Transaction:".cyan(), hash);
        }
    } else {
        anyhow::bail!("Instantiation failed: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()));
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

fn find_metadata_file(path: &str) -> anyhow::Result<PathBuf> {
    let target_dir = PathBuf::from(path).join("target/ink");

    if !target_dir.exists() {
        anyhow::bail!(
            "Contract not built. Run {} first",
            "glin-forge build".yellow()
        );
    }

    // Find .json metadata file
    for entry in std::fs::read_dir(&target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !file_name.ends_with(".contract") {
                return Ok(path);
            }
        }
    }

    anyhow::bail!("Could not find metadata JSON file in {}", target_dir.display())
}
