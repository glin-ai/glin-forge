use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
pub struct WatchArgs {
    /// Contract address to watch
    pub address: String,

    /// Event name to filter (optional, shows all if not specified)
    pub event: Option<String>,

    /// Network to connect to
    #[arg(short, long, default_value = "testnet")]
    pub network: String,

    /// Follow mode (keep watching for new events)
    #[arg(short, long)]
    pub follow: bool,

    /// Maximum number of events to show
    #[arg(long, default_value = "10")]
    pub limit: usize,

    /// Show events from block number
    #[arg(long)]
    pub from_block: Option<u64>,
}

pub async fn execute(args: WatchArgs) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Watching contract events: {}", args.address)
            .cyan()
            .bold()
    );

    let network_config = crate::config::load_network(&args.network)?;

    println!("\n{}", "Configuration:".bold());
    println!("  {} {}", "Contract:".cyan(), args.address);
    println!("  {} {}", "Network:".cyan(), args.network);

    if let Some(event) = &args.event {
        println!("  {} {}", "Event filter:".cyan(), event);
    } else {
        println!("  {} {}", "Event filter:".cyan(), "All events");
    }

    if args.follow {
        println!("  {} {}", "Mode:".cyan(), "Follow (live)");
    }

    println!("\n{}", "Connecting to network...".cyan());

    // Connect to network
    let client = crate::network::create_client(&network_config.rpc).await?;
    println!("{} Connected to {}", "✓".green(), network_config.rpc);

    println!("\n{}", "Watching for events...".cyan());
    println!("{}", "Press Ctrl+C to stop\n".dimmed());

    let mut event_count = 0;

    if args.follow {
        // Subscribe to finalized blocks and watch for contract events
        let mut blocks_sub = client.blocks().subscribe_finalized().await?;

        while let Some(block_result) = blocks_sub.next().await {
            let block = block_result?;
            let block_number = block.number();
            let events = block.events().await?;

            for event in events.iter() {
                let event = event?;

                // Filter for Contracts pallet events
                if event.pallet_name() == "Contracts" {
                    let variant = event.variant_name();

                    // Filter by event name if specified
                    if let Some(filter) = &args.event {
                        if variant != filter.as_str() {
                            continue;
                        }
                    }

                    // Check if limit reached
                    if event_count >= args.limit {
                        println!("\n{} Reached limit of {} events", "✓".green().bold(), args.limit);
                        return Ok(());
                    }

                    println!("{} Block #{}", "→".cyan(), block_number);
                    println!("  {} {}", variant.yellow().bold(), format_event_data(&event)?);
                    println!();

                    event_count += 1;
                }
            }
        }
    } else {
        // Get historical events from a range of blocks
        let latest_block = client.blocks().at_latest().await?;
        let latest_number = latest_block.number() as u64;

        let start_block = args.from_block.unwrap_or_else(|| {
            latest_number.saturating_sub(100)
        });

        for block_num in start_block..=latest_number {
            if event_count >= args.limit {
                break;
            }

            // Get block hash for this number using RPC
            let rpc = crate::network::create_rpc_client(&network_config.rpc).await?;

            let block_hash_opt: Option<subxt::utils::H256> = rpc
                .chain_get_block_hash(Some(block_num.into()))
                .await?;

            if let Some(block_hash) = block_hash_opt {
                let block = client.blocks().at(block_hash).await?;
                let events = block.events().await?;

                for event in events.iter() {
                    let event = event?;

                    if event.pallet_name() == "Contracts" {
                        let variant = event.variant_name();

                        if let Some(filter) = &args.event {
                            if variant != filter.as_str() {
                                continue;
                            }
                        }

                        if event_count >= args.limit {
                            break;
                        }

                        println!("{} Block #{}", "→".cyan(), block_num);
                        println!("  {} {}", variant.yellow().bold(), format_event_data(&event)?);
                        println!();

                        event_count += 1;
                    }
                }
            }
        }

        println!("\n{} Displayed {} events", "✓".green().bold(), event_count);
        if event_count == 0 {
            println!("{}", "No contract events found in recent blocks".dimmed());
        }
        println!("Use {} to keep watching for new events", "--follow".yellow());
    }

    Ok(())
}

/// Format event data for display
fn format_event_data<T: subxt::Config>(event: &subxt::events::EventDetails<T>) -> anyhow::Result<String> {
    // Get event field values
    let field_values = event.field_values()?;

    // Try to convert to JSON for display
    if let Ok(json) = serde_json::to_value(&field_values) {
        Ok(json.to_string())
    } else {
        Ok(String::from("(no data)"))
    }
}
