use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser)]
pub struct RunArgs {
    /// Path to TypeScript/JavaScript deployment script
    pub script: PathBuf,

    /// Network to run on (testnet, mainnet, local)
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Watch mode - rerun on file changes
    #[arg(short, long)]
    pub watch: bool,
}

pub async fn execute(args: RunArgs) -> anyhow::Result<()> {
    println!("{}", "Starting glin-forge SDK runtime...".cyan().bold());

    // Validate script path exists
    if !args.script.exists() {
        anyhow::bail!("Script not found: {}", args.script.display());
    }

    // Check if script is TypeScript or JavaScript
    let extension = args.script
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if extension != "ts" && extension != "js" {
        anyhow::bail!(
            "Script must be a TypeScript (.ts) or JavaScript (.js) file, got: .{}",
            extension
        );
    }

    println!("\n{}", "Script details:".bold());
    println!("  {} {}", "Path:".cyan(), args.script.display());
    println!("  {} {}", "Network:".cyan(), args.network);

    // Start JSON-RPC server
    println!("\n{}", "Starting RPC server...".cyan());
    let rpc_server = crate::rpc::RpcServer::start(args.network.clone()).await?;
    let port = rpc_server.port();
    println!("{} RPC server listening on port {}", "✓".green(), port);

    // Set environment variables for SDK
    std::env::set_var("GLIN_FORGE_RPC_PORT", port.to_string());
    std::env::set_var("GLIN_FORGE_NETWORK", &args.network);

    println!("\n{}", "Executing script...".cyan());
    println!("{}", "─".repeat(60));

    // Execute script
    let result = execute_script(&args.script, extension).await;

    println!("{}", "─".repeat(60));

    // Shutdown RPC server
    println!("\n{}", "Shutting down RPC server...".cyan());
    rpc_server.shutdown().await?;
    println!("{} RPC server stopped", "✓".green());

    // Handle script result
    match result {
        Ok(_) => {
            println!("\n{} Script completed successfully!", "✓".green().bold());
            Ok(())
        }
        Err(e) => {
            println!("\n{} Script failed: {}", "✗".red().bold(), e);
            Err(e)
        }
    }
}

/// Execute a TypeScript or JavaScript script
async fn execute_script(script: &PathBuf, extension: &str) -> anyhow::Result<()> {
    let script_path = script.canonicalize()?;

    // Determine runtime command based on file extension
    let (command, args) = if extension == "ts" {
        // Try to use tsx (faster) or ts-node (fallback)
        if which::which("tsx").is_ok() {
            ("tsx", vec![script_path.to_string_lossy().to_string()])
        } else if which::which("ts-node").is_ok() {
            ("ts-node", vec![script_path.to_string_lossy().to_string()])
        } else {
            anyhow::bail!(
                "TypeScript runtime not found. Please install tsx or ts-node:\n  npm install -g tsx\n  # or\n  npm install -g ts-node"
            );
        }
    } else {
        // JavaScript - use node
        ("node", vec![script_path.to_string_lossy().to_string()])
    };

    // Execute the script
    let mut child = Command::new(command)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", command, e))?;

    // Wait for completion
    let status = child.wait()?;

    if !status.success() {
        anyhow::bail!(
            "Script exited with code: {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}
