use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
pub struct NetworkArgs {
    #[command(subcommand)]
    command: NetworkCommands,
}

#[derive(Subcommand)]
enum NetworkCommands {
    /// List available networks
    List,

    /// Switch to a different network
    Use {
        /// Network name
        name: String,
    },

    /// Show current network
    Current,
}

pub async fn execute(args: NetworkArgs) -> anyhow::Result<()> {
    match args.command {
        NetworkCommands::List => list_networks().await,
        NetworkCommands::Use { name } => use_network(&name).await,
        NetworkCommands::Current => show_current().await,
    }
}

async fn list_networks() -> anyhow::Result<()> {
    println!("{}", "Available Networks:".cyan().bold());
    println!();

    let networks = vec![
        ("testnet", "wss://testnet.glin.network", "GLIN Testnet", true),
        ("mainnet", "wss://rpc.glin.network", "GLIN Mainnet", false),
        ("local", "ws://localhost:9944", "Local Node", false),
    ];

    for (name, rpc, description, is_default) in networks {
        let marker = if is_default { " (default)".green() } else { "".normal() };

        println!("  {}{}", name.yellow().bold(), marker);
        println!("    {} {}", "Description:".cyan(), description);
        println!("    {} {}", "RPC:".cyan(), rpc);
        println!();
    }

    Ok(())
}

async fn use_network(name: &str) -> anyhow::Result<()> {
    let valid_networks = vec!["testnet", "mainnet", "local"];

    if !valid_networks.contains(&name) {
        anyhow::bail!(
            "Network '{}' not found. Available: {}",
            name,
            valid_networks.join(", ")
        );
    }

    println!("{}", format!("Switching to network: {}", name).cyan().bold());

    let network_config = crate::config::load_network(name)?;

    println!();
    println!("{}", "Network info:".bold());
    println!("  {} {}", "Name:".cyan(), name);
    println!("  {} {}", "RPC:".cyan(), network_config.rpc);

    if let Some(explorer) = network_config.explorer {
        println!("  {} {}", "Explorer:".cyan(), explorer);
    }

    println!();
    println!("{} Switched to {} network", "✓".green().bold(), name.yellow());

    Ok(())
}

async fn show_current() -> anyhow::Result<()> {
    let default_network = "testnet";

    println!("{}", "Current Network:".cyan().bold());
    println!();

    let network_config = crate::config::load_network(default_network)?;

    println!("  {} {}", "Name:".cyan(), default_network.yellow());
    println!("  {} {}", "RPC:".cyan(), network_config.rpc);

    if let Some(explorer) = network_config.explorer {
        println!("  {} {}", "Explorer:".cyan(), explorer);
    }

    Ok(())
}
