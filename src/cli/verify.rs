use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
pub struct VerifyArgs {
    /// Contract address to verify
    pub address: String,

    /// Path to contract WASM file
    #[arg(short, long)]
    pub wasm: Option<PathBuf>,

    /// Path to contract metadata (ABI) JSON file
    #[arg(short, long)]
    pub metadata: Option<PathBuf>,

    /// Path to source code directory
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// Network where contract is deployed
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Compiler version used
    #[arg(long)]
    pub compiler_version: Option<String>,
}

pub async fn execute(args: VerifyArgs) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Verifying contract: {}", args.address)
            .cyan()
            .bold()
    );

    // Auto-detect files if not provided
    let (wasm_path, metadata_path, source_path) = if args.wasm.is_none() || args.metadata.is_none()
    {
        find_verification_files(".")?
    } else {
        (
            args.wasm.unwrap(),
            args.metadata.unwrap(),
            args.source.unwrap_or_else(|| PathBuf::from(".")),
        )
    };

    println!("\n{}", "Verification files:".bold());
    println!("  {} {}", "WASM:".cyan(), wasm_path.display());
    println!("  {} {}", "Metadata:".cyan(), metadata_path.display());
    println!("  {} {}", "Source:".cyan(), source_path.display());

    // Get network configuration
    let network_config = crate::config::load_network(&args.network)?;

    println!("\n{}", "Verification details:".bold());
    println!("  {} {}", "Contract:".cyan(), args.address);
    println!("  {} {}", "Network:".cyan(), args.network);

    if let Some(compiler) = &args.compiler_version {
        println!("  {} {}", "Compiler:".cyan(), compiler);
    }

    // Read files
    let wasm_bytes = std::fs::read(&wasm_path)?;
    let metadata_json = std::fs::read_to_string(&metadata_path)?;

    println!("\n{}", "Verifying contract code...".cyan());

    // Calculate code hash of the WASM
    use sp_core_hashing::blake2_256;
    let code_hash = blake2_256(&wasm_bytes);
    let code_hash_hex = format!("0x{}", hex::encode(code_hash));

    println!("  {} {}", "Code hash:".cyan(), code_hash_hex);
    println!("  {} {} bytes", "WASM size:".cyan(), wasm_bytes.len());

    // Verify the code hash matches on-chain
    println!("\n{}", "Checking on-chain...".cyan());

    let client = glin_client::create_client(&network_config.rpc).await?;

    // Query the code storage to verify it exists
    let code_storage_query = subxt::dynamic::storage(
        "Contracts",
        "PristineCode",
        vec![subxt::dynamic::Value::from_bytes(code_hash)],
    );

    let code_exists = client
        .storage()
        .at_latest()
        .await?
        .fetch(&code_storage_query)
        .await?;

    if code_exists.is_some() {
        println!("{} Code hash verified on-chain", "✓".green().bold());
    } else {
        println!("{} Warning: Code not found on-chain", "⚠".yellow().bold());
        println!("  {}", "Make sure the contract is uploaded first".dimmed());
    }

    // Upload to explorer (if available)
    if let Some(explorer) = &network_config.explorer {
        println!("\n{}", "Uploading to explorer...".cyan());
        let verification_url = format!("{}/api/verify", explorer);
        println!("  {} {}", "Endpoint:".cyan(), verification_url);

        // Prepare verification payload
        let payload = serde_json::json!({
            "address": args.address,
            "code_hash": code_hash_hex,
            "wasm": hex::encode(&wasm_bytes),
            "metadata": serde_json::from_str::<serde_json::Value>(&metadata_json)?,
            "compiler_version": args.compiler_version.unwrap_or_else(|| "latest".to_string()),
            "network": args.network,
        });

        // Submit verification request
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        match client.post(&verification_url).json(&payload).send().await {
            Ok(response) if response.status().is_success() => {
                println!("\n{} Contract verification submitted!", "✓".green().bold());

                // Try to parse response for verification ID
                if let Ok(resp_json) = response.json::<serde_json::Value>().await {
                    if let Some(verification_id) = resp_json.get("verification_id") {
                        println!("  {} {}", "Verification ID:".cyan(), verification_id);
                    }
                    if let Some(status) = resp_json.get("status") {
                        println!("  {} {}", "Status:".cyan(), status);
                    }
                }

                println!("\n{}", "Verification info:".bold());
                println!(
                    "  {} {}/contract/{}#code",
                    "View on Explorer:".cyan(),
                    explorer,
                    args.address
                );
                println!(
                    "\n{}",
                    "Verification usually completes in 1-2 minutes...".dimmed()
                );
            }
            Ok(response) => {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());

                println!("\n{} Verification submission failed", "✗".red().bold());
                println!("  {} {}", "Status:".cyan(), status);
                println!("  {} {}", "Error:".cyan(), error_text);

                anyhow::bail!("Explorer returned status {}: {}", status, error_text);
            }
            Err(e) => {
                println!("\n{} Failed to connect to explorer", "✗".red().bold());
                println!("  {} {}", "Error:".cyan(), e);

                // Provide helpful fallback instructions
                println!("\n{}", "Manual verification:".bold());
                println!("  1. Visit: {}/verify", explorer);
                println!("  2. Enter contract address: {}", args.address);
                println!("  3. Upload WASM: {}", wasm_path.display());
                println!("  4. Upload metadata: {}", metadata_path.display());

                anyhow::bail!("Could not connect to explorer API: {}", e);
            }
        }
    } else {
        anyhow::bail!("No explorer configured for network '{}'", args.network);
    }

    Ok(())
}

fn find_verification_files(path: &str) -> anyhow::Result<(PathBuf, PathBuf, PathBuf)> {
    let target_dir = PathBuf::from(path).join("target/ink");

    if !target_dir.exists() {
        anyhow::bail!(
            "Contract not built. Run {} first",
            "glin-forge build".yellow()
        );
    }

    let mut wasm_file = None;
    let mut json_file = None;

    for entry in std::fs::read_dir(&target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            wasm_file = Some(path);
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !file_name.ends_with(".contract") {
                json_file = Some(path);
            }
        }
    }

    let source_path = PathBuf::from(path);

    match (wasm_file, json_file) {
        (Some(wasm), Some(json)) => Ok((wasm, json, source_path)),
        _ => anyhow::bail!(
            "Could not find contract artifacts in {}",
            target_dir.display()
        ),
    }
}
