#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::wildcard_in_or_patterns)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::print_literal)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::format_in_format_args)]

use clap::{Parser, Subcommand};
use colored::Colorize;

mod cli;
mod codegen;
mod config;
mod contract;
mod rpc;

#[derive(Parser)]
#[command(name = "glin-forge")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new contract project
    Init(cli::init::InitArgs),

    /// Create a new contract from template
    New(cli::new::NewArgs),

    /// Build the contract
    Build(cli::build::BuildArgs),

    /// Run contract tests
    Test(cli::test::TestArgs),

    /// Deploy contract to network
    Deploy(cli::deploy::DeployArgs),

    /// Call a contract method (transaction)
    Call(cli::call::CallArgs),

    /// Query contract state (read-only)
    Query(cli::query::QueryArgs),

    /// Upload contract code without instantiation
    Upload(cli::upload::UploadArgs),

    /// Instantiate contract from code hash
    Instantiate(cli::instantiate::InstantiateArgs),

    /// Generate TypeScript types from ABI
    Typegen(cli::typegen::TypegenArgs),

    /// Watch contract events
    Watch(cli::watch::WatchArgs),

    /// Verify contract on explorer
    Verify(cli::verify::VerifyArgs),

    /// Manage configuration
    Config(cli::config::ConfigArgs),

    /// Manage accounts
    Account(cli::account::AccountArgs),

    /// Check account balance
    Balance(cli::balance::BalanceArgs),

    /// Manage networks
    Network(cli::network::NetworkArgs),

    /// Run a TypeScript deployment script
    Run(cli::run::RunArgs),

    /// Analyze contract code for security and optimization
    Analyze(cli::analyze::AnalyzeArgs),

    /// Start an interactive console (REPL)
    Console(cli::console::ConsoleArgs),

    /// Clean build artifacts
    Clean(cli::clean::CleanArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init(args) => cli::init::execute(args).await,
        Commands::New(args) => cli::new::execute(args).await,
        Commands::Build(args) => cli::build::execute(args).await,
        Commands::Test(args) => cli::test::execute(args).await,
        Commands::Deploy(args) => cli::deploy::execute(args).await,
        Commands::Call(args) => cli::call::execute(args).await,
        Commands::Query(args) => cli::query::execute(args).await,
        Commands::Upload(args) => cli::upload::execute(args).await,
        Commands::Instantiate(args) => cli::instantiate::execute(args).await,
        Commands::Typegen(args) => cli::typegen::execute(args).await,
        Commands::Watch(args) => cli::watch::execute(args).await,
        Commands::Verify(args) => cli::verify::execute(args).await,
        Commands::Config(args) => cli::config::execute(args).await,
        Commands::Account(args) => cli::account::execute(args).await,
        Commands::Balance(args) => cli::balance::execute(args).await,
        Commands::Network(args) => cli::network::execute(args).await,
        Commands::Run(args) => cli::run::execute(args).await,
        Commands::Analyze(args) => {
            cli::analyze::run(args)?;
            Ok(())
        }
        Commands::Console(args) => cli::console::execute(args).await,
        Commands::Clean(args) => cli::clean::execute(args).await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}
