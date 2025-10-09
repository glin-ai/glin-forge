use anyhow::{Context, Result};
use futures::StreamExt;
use crate::rpc::types::{
    DeployParams, DeployResult,
    CallParams, CallResult,
    QueryParams, QueryResult,
    WatchParams, WatchResult, ContractEvent,
    GetBalanceParams, GetBalanceResult,
    RequestFaucetParams, RequestFaucetResult,
    EstimateGasParams, EstimateGasResult,
    GetBlockNumberParams, GetBlockNumberResult,
    GetNetworkInfoParams, GetNetworkInfoResult,
};

/// Handle deploy RPC method
pub async fn handle_deploy(params: DeployParams) -> Result<DeployResult> {
    // Load WASM and metadata
    let wasm_bytes = std::fs::read(&params.wasm)
        .context(format!("Failed to read WASM file: {}", params.wasm))?;

    let metadata_json = std::fs::read_to_string(&params.metadata)
        .context(format!("Failed to read metadata file: {}", params.metadata))?;

    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)
        .context("Failed to parse metadata")?;

    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Get signer account
    let signer = glin_client::get_dev_account(&params.account)
        .context(format!("Failed to get account: {}", params.account))?;

    // Deploy contract using existing logic
    let result = crate::contract::deploy_contract(
        &client,
        wasm_bytes,
        &metadata,
        params.args,
        None,
        params.value,
        &signer,
    ).await
        .context("Failed to deploy contract")?;

    // Convert to RPC result type
    Ok(DeployResult {
        success: result.success,
        address: result.contract_address,
        code_hash: result.code_hash,
        tx_hash: result.tx_hash,
        block_hash: result.block_hash,
        error: result.error,
    })
}

/// Handle call RPC method
pub async fn handle_call(params: CallParams) -> Result<CallResult> {
    // Load metadata
    let metadata_json = std::fs::read_to_string(&params.metadata)
        .context(format!("Failed to read metadata file: {}", params.metadata))?;

    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)
        .context("Failed to parse metadata")?;

    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Get signer account
    let signer = glin_client::get_dev_account(&params.account)
        .context(format!("Failed to get account: {}", params.account))?;

    // Call contract using existing logic
    let result = crate::contract::call_contract(
        &client,
        &params.address,
        &metadata,
        &params.method,
        params.args,
        params.value,
        &signer,
    ).await
        .context("Failed to call contract")?;

    // Convert to RPC result type
    Ok(CallResult {
        success: result.success,
        tx_hash: result.tx_hash,
        block_hash: result.block_hash,
        error: result.error,
    })
}

/// Handle query RPC method
pub async fn handle_query(params: QueryParams) -> Result<QueryResult> {
    // Load metadata
    let metadata_json = std::fs::read_to_string(&params.metadata)
        .context(format!("Failed to read metadata file: {}", params.metadata))?;

    let metadata = crate::contract::metadata::parse_metadata(&metadata_json)
        .context("Failed to parse metadata")?;

    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network for query
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Query contract using existing logic
    let result = crate::contract::query_contract(
        &client,
        &network_config.rpc,
        &params.address,
        &metadata,
        &params.method,
        params.args,
    ).await
        .context("Failed to query contract")?;

    // Convert to RPC result type
    Ok(QueryResult {
        success: result.success,
        data: result.data.and_then(|d| serde_json::from_str(&d).ok()),
        error: result.error,
    })
}

/// Handle watch RPC method
pub async fn handle_watch(params: WatchParams) -> Result<WatchResult> {
    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    let mut events = Vec::new();
    let limit = params.limit.unwrap_or(10);

    if params.follow {
        // Follow mode: subscribe to new blocks (limited to avoid blocking)
        let mut blocks_sub = client.blocks().subscribe_finalized().await
            .context("Failed to subscribe to blocks")?;

        while let Some(block_result) = blocks_sub.next().await {
            if events.len() >= limit {
                break;
            }

            let block = block_result.context("Failed to get block")?;
            let block_number = block.number() as u64;
            let block_events = block.events().await.context("Failed to get events")?;

            for event in block_events.iter() {
                let event = event.context("Failed to decode event")?;

                // Filter for Contracts pallet events
                if event.pallet_name() == "Contracts" {
                    let variant = event.variant_name();

                    // Filter by event name if specified
                    if let Some(filter) = &params.event {
                        if variant != filter.as_str() {
                            continue;
                        }
                    }

                    if events.len() >= limit {
                        break;
                    }

                    // Extract event data
                    let field_values = event.field_values().context("Failed to get field values")?;
                    let data = serde_json::to_value(&field_values)
                        .unwrap_or(serde_json::Value::Null);

                    events.push(ContractEvent {
                        block_number,
                        event_name: variant.to_string(),
                        data,
                    });
                }
            }
        }
    } else {
        // Historical mode: get events from recent blocks
        let latest_block = client.blocks().at_latest().await
            .context("Failed to get latest block")?;
        let latest_number = latest_block.number() as u64;

        let start_block = params.from_block.unwrap_or_else(|| {
            latest_number.saturating_sub(100)
        });

        for block_num in start_block..=latest_number {
            if events.len() >= limit {
                break;
            }

            // Get block hash for this number using RPC
            let rpc = glin_client::create_rpc_client(&network_config.rpc).await
                .context("Failed to create RPC client")?;

            let block_hash_opt: Option<subxt::utils::H256> = rpc
                .chain_get_block_hash(Some(block_num.into()))
                .await
                .context("Failed to get block hash")?;

            if let Some(block_hash) = block_hash_opt {
                let block = client.blocks().at(block_hash).await
                    .context("Failed to get block")?;
                let block_events = block.events().await
                    .context("Failed to get block events")?;

                for event in block_events.iter() {
                    let event = event.context("Failed to decode event")?;

                    if event.pallet_name() == "Contracts" {
                        let variant = event.variant_name();

                        if let Some(filter) = &params.event {
                            if variant != filter.as_str() {
                                continue;
                            }
                        }

                        if events.len() >= limit {
                            break;
                        }

                        // Extract event data
                        let field_values = event.field_values().context("Failed to get field values")?;
                        let data = serde_json::to_value(&field_values)
                            .unwrap_or(serde_json::Value::Null);

                        events.push(ContractEvent {
                            block_number: block_num,
                            event_name: variant.to_string(),
                            data,
                        });
                    }
                }
            }
        }
    }

    Ok(WatchResult {
        success: true,
        events,
        error: None,
    })
}

/// Handle getBalance RPC method
pub async fn handle_get_balance(params: GetBalanceParams) -> Result<GetBalanceResult> {
    use subxt::utils::AccountId32;
    use std::str::FromStr;

    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Parse account ID
    let account_id = AccountId32::from_str(&params.address)
        .context("Failed to parse address")?;

    // Query account info using dynamic storage
    let account_query = subxt::dynamic::storage(
        "System",
        "Account",
        vec![subxt::dynamic::Value::from_bytes(&account_id.0)],
    );

    let account_info = client
        .storage()
        .at_latest()
        .await?
        .fetch(&account_query)
        .await?;

    if let Some(info) = account_info {
        let value = info.to_value()?;

        if let Ok(json) = serde_json::to_value(&value) {
            let free = json
                .get("data")
                .and_then(|d| d.get("free"))
                .and_then(|f| f.as_str())
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or(0);

            return Ok(GetBalanceResult {
                success: true,
                balance: Some(free.to_string()),
                error: None,
            });
        }
    }

    Ok(GetBalanceResult {
        success: true,
        balance: Some("0".to_string()),
        error: None,
    })
}

/// Handle requestFaucet RPC method
pub async fn handle_request_faucet(params: RequestFaucetParams) -> Result<RequestFaucetResult> {
    use subxt::utils::AccountId32;
    use std::str::FromStr;

    // Only allow faucet on testnet/local
    if params.network != "testnet" && params.network != "local" {
        return Ok(RequestFaucetResult {
            success: false,
            amount: None,
            tx_hash: None,
            error: Some("Faucet only available on testnet and local networks".to_string()),
        });
    }

    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Use Alice as faucet account
    let faucet_signer = glin_client::get_dev_account("alice")
        .context("Failed to get faucet account")?;

    // Send tokens (100 GLIN)
    let amount = 100_000_000_000_000_000_000u128; // 100 GLIN with 18 decimals

    // Parse recipient address
    let dest = AccountId32::from_str(&params.address)
        .context("Failed to parse address")?;

    // Create transfer extrinsic
    let transfer_tx = subxt::dynamic::tx(
        "Balances",
        "transfer_keep_alive",
        vec![
            subxt::dynamic::Value::from_bytes(&dest.0),
            subxt::dynamic::Value::u128(amount),
        ],
    );

    // Submit and wait for transaction
    let events = client
        .tx()
        .sign_and_submit_then_watch_default(&transfer_tx, &faucet_signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let tx_hash = format!("0x{}", hex::encode(events.extrinsic_hash()));

    Ok(RequestFaucetResult {
        success: true,
        amount: Some(amount.to_string()),
        tx_hash: Some(tx_hash),
        error: None,
    })
}

/// Handle estimateGas RPC method
pub async fn handle_estimate_gas(_params: EstimateGasParams) -> Result<EstimateGasResult> {
    // Estimate gas (simplified - returns a default estimate)
    // TODO: Implement proper gas estimation using contract metadata and dry-run
    let gas_limit = 100_000_000_000u64; // Default 100 billion gas units
    let gas_price = 1u128; // 1 unit per gas
    let estimated_cost = (gas_limit as u128 * gas_price).to_string();

    Ok(EstimateGasResult {
        success: true,
        gas_limit: Some(gas_limit),
        estimated_cost: Some(estimated_cost),
        error: None,
    })
}

/// Handle getBlockNumber RPC method
pub async fn handle_get_block_number(params: GetBlockNumberParams) -> Result<GetBlockNumberResult> {
    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Get latest block number
    let latest_block = client.blocks().at_latest().await
        .context("Failed to get latest block")?;
    let block_number = latest_block.number() as u64;

    Ok(GetBlockNumberResult {
        success: true,
        block_number: Some(block_number),
        error: None,
    })
}

/// Handle getNetworkInfo RPC method
pub async fn handle_get_network_info(params: GetNetworkInfoParams) -> Result<GetNetworkInfoResult> {
    // Get network configuration
    let network_config = crate::config::load_network(&params.network)
        .context(format!("Failed to load network config for: {}", params.network))?;

    // Connect to network
    let client = glin_client::create_client(&network_config.rpc).await
        .context(format!("Failed to connect to network: {}", network_config.rpc))?;

    // Get latest block number
    let latest_block = client.blocks().at_latest().await
        .context("Failed to get latest block")?;
    let block_number = latest_block.number() as u64;

    Ok(GetNetworkInfoResult {
        success: true,
        name: Some(params.network.clone()),
        rpc: Some(network_config.rpc.clone()),
        block_number: Some(block_number),
        error: None,
    })
}
