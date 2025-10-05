use clap::{Parser, Subcommand};
use colored::Colorize;

mod cli;
mod config;
mod codegen;
mod contract;

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
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}
