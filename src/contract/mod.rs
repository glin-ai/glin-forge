use anyhow::{Result, Context};
use subxt_signer::sr25519::Keypair;
use crate::network::GlinClient;
use subxt::utils::AccountId32;
use scale::Encode;
use ink_metadata::InkProject;

pub mod chain_info;
pub mod encoding;
pub mod metadata;
pub mod metadata_fetcher;

#[derive(Debug)]
pub struct DeployResult {
    pub success: bool,
    pub contract_address: Option<String>,
    pub code_hash: Option<String>,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct TxResult {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct QueryResult {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct UploadResult {
    pub success: bool,
    pub code_hash: Option<String>,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

/// Deploy a contract (upload + instantiate)
pub async fn deploy_contract(
    client: &GlinClient,
    wasm_code: Vec<u8>,
    metadata: &InkProject,
    constructor_args: Vec<String>,
    constructor_name: Option<&str>,
    value: u128,
    signer: &Keypair,
) -> Result<DeployResult> {
    println!("Deploying contract with {} bytes of WASM code", wasm_code.len());
    println!("Constructor args: {:?}", constructor_args);
    println!("Value: {}", value);

    // Encode constructor selector and args
    let data = encode_constructor_call(&constructor_args, metadata, constructor_name)?;

    // Build dynamic transaction for instantiate_with_code
    let gas_limit_value = subxt::dynamic::Value::unnamed_composite(vec![
        subxt::dynamic::Value::u128(5_000_000_000),  // ref_time
        subxt::dynamic::Value::u128(2_000_000),      // proof_size
    ]);

    let tx = subxt::dynamic::tx(
        "Contracts",
        "instantiate_with_code",
        vec![
            subxt::dynamic::Value::u128(value),
            gas_limit_value,
            subxt::dynamic::Value::unnamed_variant("None", vec![]), // storage_deposit_limit
            subxt::dynamic::Value::from_bytes(&wasm_code),
            subxt::dynamic::Value::from_bytes(&data),
            subxt::dynamic::Value::from_bytes(&vec![0u8; 32]), // salt
        ],
    );

    // Submit and watch transaction
    let events = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await
        .context("Failed to submit transaction")?
        .wait_for_finalized_success()
        .await
        .context("Transaction failed")?;

    let tx_hash = format!("0x{}", hex::encode(events.extrinsic_hash()));
    // Note: Block hash not directly available in ExtrinsicEvents in subxt 0.44
    let block_hash = String::from("N/A");

    // Find events
    let mut contract_address = None;
    let mut code_hash = None;

    for event in events.iter() {
        let event = event.context("Failed to decode event")?;

        if event.pallet_name() == "Contracts" {
            match event.variant_name() {
                "Instantiated" => {
                    // Try to extract contract address from event fields
                    let field_values = event.field_values()?;
                    if let Ok(json) = serde_json::to_value(&field_values) {
                        if let Some(contract) = json.get("contract") {
                            contract_address = Some(contract.to_string().trim_matches('"').to_string());
                        }
                    }
                }
                "CodeStored" => {
                    let field_values = event.field_values()?;
                    if let Ok(json) = serde_json::to_value(&field_values) {
                        if let Some(hash) = json.get("code_hash") {
                            code_hash = Some(hash.to_string().trim_matches('"').to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(DeployResult {
        success: true,
        contract_address,
        code_hash,
        tx_hash: Some(tx_hash),
        block_hash: Some(block_hash),
        error: None,
    })
}

/// Upload contract code without instantiation
pub async fn upload_code(
    client: &GlinClient,
    wasm_code: Vec<u8>,
    signer: &Keypair,
) -> Result<UploadResult> {
    println!("Uploading {} bytes of WASM code", wasm_code.len());

    let tx = subxt::dynamic::tx(
        "Contracts",
        "upload_code",
        vec![
            subxt::dynamic::Value::from_bytes(&wasm_code),
            subxt::dynamic::Value::unnamed_variant("None", vec![]), // storage_deposit_limit
            subxt::dynamic::Value::unnamed_variant("Enforced", vec![]), // Determinism::Enforced
        ],
    );

    let events = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await
        .context("Failed to submit upload transaction")?
        .wait_for_finalized_success()
        .await
        .context("Upload transaction failed")?;

    let tx_hash = format!("0x{}", hex::encode(events.extrinsic_hash()));
    let block_hash = String::from("N/A"); // Block hash not in ExtrinsicEvents

    // Find CodeStored event
    let mut code_hash = None;

    for event in events.iter() {
        let event = event.context("Failed to decode event")?;

        if event.pallet_name() == "Contracts" && event.variant_name() == "CodeStored" {
            let field_values = event.field_values()?;
            if let Ok(json) = serde_json::to_value(&field_values) {
                if let Some(hash) = json.get("code_hash") {
                    code_hash = Some(hash.to_string().trim_matches('"').to_string());
                    break;
                }
            }
        }
    }

    Ok(UploadResult {
        success: true,
        code_hash,
        tx_hash: Some(tx_hash),
        block_hash: Some(block_hash),
        error: None,
    })
}

/// Instantiate contract from uploaded code hash
pub async fn instantiate_contract(
    client: &GlinClient,
    code_hash: &str,
    metadata: &InkProject,
    constructor_args: Vec<String>,
    constructor_name: Option<&str>,
    value: u128,
    signer: &Keypair,
) -> Result<DeployResult> {
    println!("Instantiating contract from code hash: {}", code_hash);
    println!("Constructor args: {:?}", constructor_args);
    println!("Value: {}", value);

    // Encode constructor data
    let data = encode_constructor_call(&constructor_args, metadata, constructor_name)?;

    // Decode code hash
    let code_hash_bytes = hex::decode(code_hash.trim_start_matches("0x"))
        .context("Invalid code hash format")?;
    let code_hash_array: [u8; 32] = code_hash_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Code hash must be 32 bytes"))?;

    let gas_limit_value = subxt::dynamic::Value::unnamed_composite(vec![
        subxt::dynamic::Value::u128(5_000_000_000),
        subxt::dynamic::Value::u128(2_000_000),
    ]);

    let tx = subxt::dynamic::tx(
        "Contracts",
        "instantiate",
        vec![
            subxt::dynamic::Value::u128(value),
            gas_limit_value,
            subxt::dynamic::Value::unnamed_variant("None", vec![]),
            subxt::dynamic::Value::from_bytes(&code_hash_array),
            subxt::dynamic::Value::from_bytes(&data),
            subxt::dynamic::Value::from_bytes(&vec![0u8; 32]), // salt
        ],
    );

    let events = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await
        .context("Failed to submit instantiate transaction")?
        .wait_for_finalized_success()
        .await
        .context("Instantiate transaction failed")?;

    let tx_hash = format!("0x{}", hex::encode(events.extrinsic_hash()));
    let block_hash = String::from("N/A"); // Block hash not in ExtrinsicEvents

    // Find Instantiated event
    let mut contract_address = None;

    for event in events.iter() {
        let event = event.context("Failed to decode event")?;

        if event.pallet_name() == "Contracts" && event.variant_name() == "Instantiated" {
            let field_values = event.field_values()?;
            if let Ok(json) = serde_json::to_value(&field_values) {
                if let Some(contract) = json.get("contract") {
                    contract_address = Some(contract.to_string().trim_matches('"').to_string());
                    break;
                }
            }
        }
    }

    Ok(DeployResult {
        success: true,
        contract_address,
        code_hash: Some(code_hash.to_string()),
        tx_hash: Some(tx_hash),
        block_hash: Some(block_hash),
        error: None,
    })
}

/// Call a contract method (transaction)
pub async fn call_contract(
    client: &GlinClient,
    contract_address: &str,
    metadata: &InkProject,
    method: &str,
    args: Vec<String>,
    value: u128,
    signer: &Keypair,
) -> Result<TxResult> {
    println!("Calling contract {} method {}", contract_address, method);
    println!("Args: {:?}", args);
    println!("Value: {}", value);

    // Encode method call
    let data = encode_method_call(method, &args, metadata)?;

    // Parse contract address
    let dest = parse_account_id(contract_address)?;

    let gas_limit_value = subxt::dynamic::Value::unnamed_composite(vec![
        subxt::dynamic::Value::u128(3_000_000_000),
        subxt::dynamic::Value::u128(1_000_000),
    ]);

    let tx = subxt::dynamic::tx(
        "Contracts",
        "call",
        vec![
            subxt::dynamic::Value::unnamed_composite(vec![
                subxt::dynamic::Value::from_bytes(&dest.0),
            ]),
            subxt::dynamic::Value::u128(value),
            gas_limit_value,
            subxt::dynamic::Value::unnamed_variant("None", vec![]),
            subxt::dynamic::Value::from_bytes(&data),
        ],
    );

    let events = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await
        .context("Failed to submit call transaction")?
        .wait_for_finalized_success()
        .await
        .context("Call transaction failed")?;

    let tx_hash = format!("0x{}", hex::encode(events.extrinsic_hash()));
    let block_hash = String::from("N/A"); // Block hash not in ExtrinsicEvents

    Ok(TxResult {
        success: true,
        tx_hash: Some(tx_hash),
        block_hash: Some(block_hash),
        error: None,
    })
}

/// Query contract state (read-only)
pub async fn query_contract(
    _client: &GlinClient,
    rpc_url: &str,
    contract_address: &str,
    metadata: &InkProject,
    method: &str,
    args: Vec<String>,
) -> Result<QueryResult> {
    println!("Querying contract {} method {}", contract_address, method);
    println!("Args: {:?}", args);

    // Encode method call
    let data = encode_method_call(method, &args, metadata)?;

    // Parse contract address
    let dest = parse_account_id(contract_address)?;

    // Create origin (dummy account for dry-run)
    let origin = AccountId32([0u8; 32]);

    // Prepare RPC call parameters
    let call_params = (
        origin.0.to_vec(),
        dest.0.to_vec(),
        0u128, // value
        None::<u64>, // gas_limit (None = estimate)
        None::<u128>, // storage_deposit_limit
        data,
    );

    // Create RPC client for state_call
    let rpc = crate::network::create_rpc_client(rpc_url).await?;

    // Encode call parameters
    let encoded = call_params.encode();

    // Make state_call to query contract (read-only)
    let result_bytes = rpc
        .state_call("ContractsApi_call", Some(&encoded), None)
        .await
        .context("Contract query RPC call failed")?;

    // Decode ContractExecResult
    // The result contains a ContractExecResult structure
    // We need to decode it to extract the actual return data
    let exec_result = decode_contract_exec_result(&result_bytes)?;

    // Get the message spec to find return type
    let message = self::metadata::get_message_spec(metadata, method)?;
    let return_type_spec = self::metadata::get_message_return_type(message);

    // Decode the return data
    let decoded_data = if let Some(data) = exec_result.data {
        self::encoding::decode_result(&data, Some(return_type_spec), metadata)?
    } else {
        serde_json::Value::Null
    };

    Ok(QueryResult {
        success: exec_result.success,
        data: Some(serde_json::to_string(&decoded_data)?),
        error: exec_result.error,
    })
}

/// Simplified ContractExecResult for decoding RPC response
struct ContractExecResultDecoded {
    success: bool,
    data: Option<Vec<u8>>,
    error: Option<String>,
}

/// Decode ContractExecResult from RPC response
fn decode_contract_exec_result(bytes: &[u8]) -> Result<ContractExecResultDecoded> {
    use scale::Decode;

    // ContractExecResult structure (simplified):
    // - gas_consumed: Weight
    // - gas_required: Weight
    // - storage_deposit: StorageDeposit
    // - debug_message: Vec<u8>
    // - result: Result<ExecReturnValue, DispatchError>

    let mut input = &bytes[..];

    // Skip gas_consumed (WeightV2 - 2x u64)
    let _ref_time = u64::decode(&mut input)?;
    let _proof_size = u64::decode(&mut input)?;

    // Skip gas_required (WeightV2 - 2x u64)
    let _ref_time_required = u64::decode(&mut input)?;
    let _proof_size_required = u64::decode(&mut input)?;

    // Skip storage_deposit (enum variant index + optional value)
    let _storage_deposit_variant = u8::decode(&mut input)?;
    // Storage deposit value if variant != 0
    if _storage_deposit_variant != 0 {
        let _deposit_value = u128::decode(&mut input)?;
    }

    // Skip debug_message
    let _debug_msg = Vec::<u8>::decode(&mut input)?;

    // Decode result: Result<ExecReturnValue, DispatchError>
    let result_variant = u8::decode(&mut input)?;

    if result_variant == 0 {
        // Ok variant - contains ExecReturnValue
        // ExecReturnValue { flags: u32, data: Vec<u8> }
        let _flags = u32::decode(&mut input)?;
        let data = Vec::<u8>::decode(&mut input)?;

        Ok(ContractExecResultDecoded {
            success: true,
            data: Some(data),
            error: None,
        })
    } else {
        // Err variant - contains DispatchError
        Ok(ContractExecResultDecoded {
            success: false,
            data: None,
            error: Some("Contract execution failed".to_string()),
        })
    }
}

/// Encode constructor call with selector and arguments
fn encode_constructor_call(
    args: &[String],
    metadata: &InkProject,
    constructor_name: Option<&str>,
) -> Result<Vec<u8>> {
    // Get constructor spec (default to "new" if not specified)
    let constructor = if let Some(name) = constructor_name {
        self::metadata::get_constructor_spec(metadata, name)?
    } else {
        self::metadata::get_default_constructor(metadata)?
    };

    // Get selector
    let selector = self::metadata::get_constructor_selector(constructor);

    // Encode arguments
    let param_specs = constructor.args();
    let encoded_args = self::encoding::encode_args(args, param_specs, metadata)?;

    // Combine selector + encoded args
    let mut result = selector.to_bytes().to_vec();
    result.extend_from_slice(&encoded_args);

    Ok(result)
}

/// Encode method call with selector and arguments
fn encode_method_call(
    method: &str,
    args: &[String],
    metadata: &InkProject,
) -> Result<Vec<u8>> {
    // Get message spec
    let message = self::metadata::get_message_spec(metadata, method)?;

    // Get selector
    let selector = self::metadata::get_message_selector(message);

    // Encode arguments
    let param_specs = message.args();
    let encoded_args = self::encoding::encode_args(args, param_specs, metadata)?;

    // Combine selector + encoded args
    let mut result = selector.to_bytes().to_vec();
    result.extend_from_slice(&encoded_args);

    Ok(result)
}

/// Parse account ID from various formats
fn parse_account_id(address: &str) -> Result<AccountId32> {
    use std::str::FromStr;

    // If it's hex, decode it
    if address.starts_with("0x") {
        let bytes = hex::decode(address.trim_start_matches("0x"))
            .context("Invalid hex address")?;
        let array: [u8; 32] = bytes.try_into()
            .map_err(|_| anyhow::anyhow!("Address must be 32 bytes"))?;
        return Ok(AccountId32(array));
    }

    // Try to parse as AccountId32 directly
    if let Ok(account_id) = AccountId32::from_str(address) {
        return Ok(account_id);
    }

    anyhow::bail!("Invalid address format: {}", address)
}
