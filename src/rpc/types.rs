use serde::{Deserialize, Serialize};

/// Parameters for deploying a contract
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeployParams {
    /// Path to WASM file
    pub wasm: String,

    /// Path to metadata JSON file
    pub metadata: String,

    /// Constructor arguments
    #[serde(default)]
    pub args: Vec<String>,

    /// Value to send with deployment (in GLIN)
    #[serde(default)]
    pub value: u128,

    /// Network to deploy to (testnet, mainnet, local)
    pub network: String,

    /// Account to deploy from (alice, bob, or custom)
    pub account: String,

    /// Optional gas limit override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,

    /// Optional salt for deterministic deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

/// Result of deploying a contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployResult {
    pub success: bool,
    pub address: Option<String>,
    pub code_hash: Option<String>,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

/// Parameters for calling a contract method
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CallParams {
    /// Contract address
    pub address: String,

    /// Path to metadata JSON file
    pub metadata: String,

    /// Method name to call
    pub method: String,

    /// Method arguments
    #[serde(default)]
    pub args: Vec<String>,

    /// Value to send (in GLIN)
    #[serde(default)]
    pub value: u128,

    /// Network
    pub network: String,

    /// Calling account
    pub account: String,

    /// Optional gas limit override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,
}

/// Result of calling a contract method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResult {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

/// Parameters for querying a contract
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryParams {
    /// Contract address
    pub address: String,

    /// Path to metadata JSON file
    pub metadata: String,

    /// Method name to query
    pub method: String,

    /// Method arguments
    #[serde(default)]
    pub args: Vec<String>,

    /// Network
    pub network: String,
}

/// Result of querying a contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Parameters for watching contract events
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WatchParams {
    /// Contract address to watch
    pub address: String,

    /// Event name filter (optional, shows all if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    /// Network
    pub network: String,

    /// Follow mode (keep watching for new events)
    #[serde(default)]
    pub follow: bool,

    /// Maximum number of events to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    /// Show events from block number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<u64>,
}

/// Result of watching contract events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchResult {
    pub success: bool,
    pub events: Vec<ContractEvent>,
    pub error: Option<String>,
}

/// A contract event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    pub block_number: u64,
    pub event_name: String,
    pub data: serde_json::Value,
}

/// Parameters for getting balance
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBalanceParams {
    pub address: String,
    pub network: String,
}

/// Result of getting balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceResult {
    pub success: bool,
    pub balance: Option<String>,
    pub error: Option<String>,
}

/// Parameters for requesting faucet
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestFaucetParams {
    pub address: String,
    pub network: String,
}

/// Result of requesting faucet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestFaucetResult {
    pub success: bool,
    pub amount: Option<String>,
    pub tx_hash: Option<String>,
    pub error: Option<String>,
}

/// Parameters for estimating gas
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateGasParams {
    pub address: String,
    pub method: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub value: u128,
    pub from: String,
    pub network: String,
}

/// Result of estimating gas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateGasResult {
    pub success: bool,
    pub gas_limit: Option<u64>,
    pub estimated_cost: Option<String>,
    pub error: Option<String>,
}

/// Parameters for getting block number
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockNumberParams {
    pub network: String,
}

/// Result of getting block number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockNumberResult {
    pub success: bool,
    pub block_number: Option<u64>,
    pub error: Option<String>,
}

/// Parameters for getting network info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetNetworkInfoParams {
    pub network: String,
}

/// Result of getting network info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNetworkInfoResult {
    pub success: bool,
    pub name: Option<String>,
    pub rpc: Option<String>,
    pub block_number: Option<u64>,
    pub error: Option<String>,
}
