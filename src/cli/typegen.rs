use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
pub struct TypegenArgs {
    /// Path to contract metadata (ABI) JSON file
    #[arg(short, long)]
    pub abi: Option<PathBuf>,

    /// Contract address to fetch ABI from
    #[arg(short, long)]
    pub contract: Option<String>,

    /// Output directory for generated types
    #[arg(short, long, default_value = "./types")]
    pub output: PathBuf,

    /// Network to fetch ABI from (when using --contract)
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Generate React hooks alongside types
    #[arg(long)]
    pub hooks: bool,
}

pub async fn execute(args: TypegenArgs) -> anyhow::Result<()> {
    println!("{}", "Generating TypeScript types...".cyan().bold());

    // Load ABI
    let abi_json = if let Some(abi_path) = &args.abi {
        std::fs::read_to_string(abi_path)?
    } else if let Some(contract_addr) = &args.contract {
        println!("{} Fetching metadata from network...", "→".cyan());

        // Get network configuration
        let network_config = crate::config::load_network(&args.network)?;

        // Create client
        let client = glin_client::create_client(&network_config.rpc).await?;

        // Prepare fetcher options
        let cache_dir = crate::contract::metadata_fetcher::get_default_cache_dir()?;
        let options = crate::contract::metadata_fetcher::MetadataFetchOptions {
            local_path: None,
            explorer_url: network_config.explorer.clone(),
            cache_dir: Some(cache_dir),
        };

        // Fetch metadata using multi-strategy approach
        let metadata =
            crate::contract::metadata_fetcher::fetch_contract_metadata(&client, contract_addr, options)
                .await?;

        // Convert InkProject back to JSON string for compatibility
        serde_json::to_string(&metadata)?
    } else {
        // Try to find in current directory
        let default_path = PathBuf::from("target/ink").join("metadata.json");
        if default_path.exists() {
            std::fs::read_to_string(&default_path)?
        } else {
            anyhow::bail!("No ABI specified. Use --abi <path> or --contract <address>");
        }
    };

    let abi: serde_json::Value = serde_json::from_str(&abi_json)?;

    // Parse contract metadata using codegen module
    let contract_name = crate::codegen::extract_contract_name(&abi)?;
    let messages = crate::codegen::extract_messages(&abi)?;

    println!("\n{}", "Contract info:".bold());
    println!("  {} {}", "Name:".cyan(), contract_name);
    println!("  {} {}", "Messages:".cyan(), messages.len());

    // Generate TypeScript types using codegen module
    let ts_content = crate::codegen::generate_typescript_types(&contract_name, &abi)?;

    // Create output directory
    std::fs::create_dir_all(&args.output)?;

    // Write types file
    let types_file = args.output.join(format!("{}.ts", contract_name));
    std::fs::write(&types_file, ts_content)?;

    println!(
        "\n{} TypeScript types generated!",
        "✓".green().bold()
    );
    println!("  {} {}", "Output:".cyan(), types_file.display());

    // Generate React hooks if requested
    if args.hooks {
        let hooks_content = crate::codegen::generate_react_hooks(&contract_name, &abi)?;
        let hooks_file = args.output.join(format!("use{}.ts", contract_name));
        std::fs::write(&hooks_file, hooks_content)?;

        println!("  {} {}", "Hooks:".cyan(), hooks_file.display());
    }

    println!("\n{}", "Usage example:".bold());
    println!("  import {{ {}Contract }} from './{}'", contract_name, types_file.display());

    Ok(())
}
