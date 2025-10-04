use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub rpc: String,
    pub explorer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub networks: HashMap<String, NetworkConfig>,
    pub default_network: String,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        let mut networks = HashMap::new();

        networks.insert(
            "testnet".to_string(),
            NetworkConfig {
                rpc: "wss://testnet.glin.network".to_string(),
                explorer: Some("https://explorer-testnet.glin.network".to_string()),
            },
        );

        networks.insert(
            "mainnet".to_string(),
            NetworkConfig {
                rpc: "wss://rpc.glin.network".to_string(),
                explorer: Some("https://explorer.glin.network".to_string()),
            },
        );

        networks.insert(
            "local".to_string(),
            NetworkConfig {
                rpc: "ws://localhost:9944".to_string(),
                explorer: None,
            },
        );

        Self {
            networks,
            default_network: "testnet".to_string(),
        }
    }
}

pub fn load_network(network_name: &str) -> anyhow::Result<NetworkConfig> {
    let config = ForgeConfig::default();

    config
        .networks
        .get(network_name)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Network '{}' not found in configuration", network_name))
}
