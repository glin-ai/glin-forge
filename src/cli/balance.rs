use clap::Parser;
use colored::Colorize;
use subxt::utils::AccountId32;

#[derive(Parser)]
pub struct BalanceArgs {
    /// Account address or name
    pub account: String,

    /// Network to query
    #[arg(short, long, default_value = "testnet")]
    pub network: String,
}

pub async fn execute(args: BalanceArgs) -> anyhow::Result<()> {
    println!("{}", "Checking balance...".cyan().bold());

    // Get network configuration
    let network_config = crate::config::load_network(&args.network)?;

    println!("  {} {}", "Network:".cyan(), args.network);

    // Determine if input is address or account name
    let address = if args.account.starts_with('5') {
        args.account.clone()
    } else {
        let keypair = glin_client::get_dev_account(&args.account)?;
        glin_client::get_address(&keypair)
    };

    println!("  {} {}", "Address:".cyan(), address);

    println!("\n{}", "Connecting to network...".cyan());

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await?;
    println!("{} Connected", "✓".green());

    // Parse account ID
    let account_id = parse_account_id(&address)?;

    // Query account info using dynamic storage
    let account_query = subxt::dynamic::storage(
        "System",
        "Account",
        vec![subxt::dynamic::Value::from_bytes(account_id.0)],
    );

    let account_info = client
        .storage()
        .at_latest()
        .await?
        .fetch(&account_query)
        .await?;

    println!("\n{}", "Balance:".bold());

    if let Some(info) = account_info {
        // Decode the account info
        let value = info.to_value()?;

        // Try to extract balance fields from the value
        // AccountInfo structure: { nonce, consumers, providers, sufficients, data: { free, reserved, frozen, flags } }
        if let Ok(json) = serde_json::to_value(&value) {
            let free = json
                .get("data")
                .and_then(|d| d.get("free"))
                .and_then(|f| f.as_str())
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or(0);

            let reserved = json
                .get("data")
                .and_then(|d| d.get("reserved"))
                .and_then(|r| r.as_str())
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or(0);

            let frozen = json
                .get("data")
                .and_then(|d| d.get("frozen"))
                .and_then(|f| f.as_str())
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or(0);

            let free_glin = format_balance(free);
            let reserved_glin = format_balance(reserved);
            let frozen_glin = format_balance(frozen);
            let total_glin = format_balance(free + reserved);

            println!("  {} {} GLIN", "Free:".cyan(), free_glin);
            println!("  {} {} GLIN", "Reserved:".cyan(), reserved_glin);
            println!("  {} {} GLIN", "Frozen:".cyan(), frozen_glin);

            println!();
            println!("{}", format!("Total: {} GLIN", total_glin).green().bold());
        } else {
            println!("  {}", "No balance data found".dimmed());
        }
    } else {
        println!("  {}", "Account not found (zero balance)".dimmed());
        println!("  {} 0.0000 GLIN", "Free:".cyan());
    }

    Ok(())
}

/// Parse account ID from SS58 address
fn parse_account_id(address: &str) -> anyhow::Result<AccountId32> {
    use std::str::FromStr;

    if let Ok(account_id) = AccountId32::from_str(address) {
        return Ok(account_id);
    }

    // Try hex format
    if address.starts_with("0x") {
        let bytes = hex::decode(address.trim_start_matches("0x"))?;
        let array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("Address must be 32 bytes"))?;
        return Ok(AccountId32(array));
    }

    anyhow::bail!("Invalid address format: {}", address)
}

/// Format balance from smallest unit to GLIN with decimals
fn format_balance(amount: u128) -> String {
    const DECIMALS: u32 = 18;
    let divisor = 10u128.pow(DECIMALS);

    let whole = amount / divisor;
    let fraction = amount % divisor;

    // Format with 4 decimal places
    let fraction_str = format!("{:018}", fraction);
    let fraction_4dp = &fraction_str[0..4];

    format!("{}.{}", format_with_commas(whole), fraction_4dp)
}

/// Add commas to large numbers
fn format_with_commas(n: u128) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}
