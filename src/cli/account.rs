use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
pub struct AccountArgs {
    #[command(subcommand)]
    command: AccountCommands,
}

#[derive(Subcommand)]
enum AccountCommands {
    /// List available accounts
    List,

    /// Generate new account
    Generate {
        /// Account name
        name: String,
    },

    /// Import account from seed
    Import {
        /// Account name
        name: String,

        /// Seed phrase or private key
        #[arg(short, long)]
        seed: String,
    },

    /// Show account details
    Show {
        /// Account name
        name: String,
    },
}

pub async fn execute(args: AccountArgs) -> anyhow::Result<()> {
    match args.command {
        AccountCommands::List => list_accounts().await,
        AccountCommands::Generate { name } => generate_account(&name).await,
        AccountCommands::Import { name, seed } => import_account(&name, &seed).await,
        AccountCommands::Show { name } => show_account(&name).await,
    }
}

async fn list_accounts() -> anyhow::Result<()> {
    println!("{}", "Available Accounts:".cyan().bold());
    println!();

    // Development accounts
    println!("{}", "Development Accounts:".bold());
    let dev_accounts = vec!["alice", "bob", "charlie", "dave", "eve", "ferdie"];

    for (idx, account) in dev_accounts.iter().enumerate() {
        let pair = glin_client::get_dev_account(account)?;
        let address = glin_client::get_address(&pair);

        println!(
            "  {}. {} {}",
            idx + 1,
            account.yellow(),
            format!("({})", &address[..10]).dimmed()
        );
    }

    println!();
    println!("{}", "Custom Accounts:".bold());
    println!("  {}", "No custom accounts configured".dimmed());
    println!();
    println!(
        "{}",
        "Use 'glin-forge account generate <name>' to create a new account".dimmed()
    );

    Ok(())
}

async fn generate_account(name: &str) -> anyhow::Result<()> {
    println!("{}", format!("Generating new account: {}", name).cyan().bold());

    // Generate random mnemonic phrase
    use subxt_signer::bip39::Mnemonic;
    use rand::Rng;

    // Generate random entropy for 12-word mnemonic (128 bits = 16 bytes)
    let mut entropy = [0u8; 16];
    rand::thread_rng().fill(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    let phrase = mnemonic.to_string();

    // Generate keypair from mnemonic
    use subxt_signer::sr25519::Keypair;
    let keypair = Keypair::from_phrase(&mnemonic, None)?;
    let address = glin_client::get_address(&keypair);

    println!("\n{} Account generated!", "✓".green().bold());
    println!();
    println!("{}", "Account Details:".bold());
    println!("  {} {}", "Name:".cyan(), name);
    println!("  {} {}", "Address:".cyan(), address);
    println!();
    println!("{}", "Seed Phrase (KEEP SAFE!):".yellow().bold());
    println!("  {}", phrase);
    println!();
    println!("{}", "⚠️  Store this seed phrase securely!".yellow());
    println!("{}", "   Anyone with this phrase can access your funds.".dimmed());

    Ok(())
}

async fn import_account(name: &str, seed: &str) -> anyhow::Result<()> {
    println!("{}", format!("Importing account: {}", name).cyan().bold());

    let pair = glin_client::account_from_seed(seed)?;
    let address = glin_client::get_address(&pair);

    println!("\n{} Account imported!", "✓".green().bold());
    println!();
    println!("{}", "Account Details:".bold());
    println!("  {} {}", "Name:".cyan(), name);
    println!("  {} {}", "Address:".cyan(), address);

    Ok(())
}

async fn show_account(name: &str) -> anyhow::Result<()> {
    println!("{}", format!("Account: {}", name).cyan().bold());

    // Try development accounts first
    match glin_client::get_dev_account(name) {
        Ok(pair) => {
            let address = glin_client::get_address(&pair);

            println!();
            println!("{}", "Account Details:".bold());
            println!("  {} {}", "Name:".cyan(), name);
            println!("  {} {}", "Type:".cyan(), "Development");
            println!("  {} {}", "Address:".cyan(), address);
        }
        Err(_) => {
            anyhow::bail!("Account '{}' not found", name);
        }
    }

    Ok(())
}
