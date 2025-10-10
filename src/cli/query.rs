use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
pub struct QueryArgs {
    /// Contract address
    pub address: String,

    /// Method name to query
    pub method: String,

    /// Method arguments (space-separated)
    pub args: Vec<String>,

    /// Network to query on
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Path to contract metadata (ABI) JSON file
    #[arg(short, long)]
    pub metadata: Option<String>,

    /// Format output as JSON
    #[arg(long)]
    pub json: bool,
}

pub async fn execute(args: QueryArgs) -> anyhow::Result<()> {
    println!("{}", "Querying contract...".cyan().bold());

    println!("\n{}", "Query details:".bold());
    println!("  {} {}", "Contract:".cyan(), args.address);
    println!("  {} {}", "Method:".cyan(), args.method);
    println!("  {} {}", "Network:".cyan(), args.network);

    if !args.args.is_empty() {
        println!("  {} {:?}", "Arguments:".cyan(), args.args);
    }

    // Load metadata
    let metadata_path = if let Some(path) = args.metadata {
        path
    } else {
        // Try to find in current directory
        find_metadata_for_contract(&args.address)?
    };

    println!("  {} {}", "Metadata:".cyan(), metadata_path);

    // Load and parse metadata
    let metadata_json = std::fs::read_to_string(&metadata_path)?;
    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)?;

    // Get network configuration
    let network_config = crate::config::load_network(&args.network)?;

    println!("\n{}", "Connecting to network...".cyan());

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await?;
    println!("{} Connected to {}", "✓".green(), network_config.rpc);

    // Execute query
    let result = crate::contract::query_contract(
        &client,
        &network_config.rpc,
        &args.address,
        &metadata,
        &args.method,
        args.args.clone(),
    )
    .await?;

    if result.success {
        println!("\n{} Query successful!", "✓".green().bold());

        if args.json {
            let json_output = serde_json::json!({
                "success": true,
                "data": result.data,
                "error": null
            });
            println!("\n{}", serde_json::to_string_pretty(&json_output)?);
        } else {
            println!("\n{}", "Result:".bold());
            if let Some(data) = result.data {
                println!("  {}", data.green());
            } else {
                println!("  {}", "No data returned".yellow());
            }
        }
    } else {
        anyhow::bail!(
            "Query failed: {}",
            result.error.unwrap_or_else(|| "Unknown error".to_string())
        );
    }

    Ok(())
}

fn find_metadata_for_contract(_address: &str) -> anyhow::Result<String> {
    // Try to find metadata in common locations
    let possible_paths = vec!["target/ink/metadata.json", "contract.json", "abi.json"];

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
