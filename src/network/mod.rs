use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::rpc::RpcClient;
use subxt_signer::sr25519::Keypair;

pub type GlinConfig = PolkadotConfig;
pub type GlinClient = OnlineClient<GlinConfig>;

/// Create a client connection to GLIN network
pub async fn create_client(rpc_url: &str) -> Result<GlinClient> {
    let client = OnlineClient::<GlinConfig>::from_url(rpc_url).await?;
    Ok(client)
}

/// Create legacy RPC methods for direct RPC calls
pub async fn create_rpc_client(rpc_url: &str) -> Result<LegacyRpcMethods<GlinConfig>> {
    let rpc_client = RpcClient::from_url(rpc_url).await?;
    Ok(LegacyRpcMethods::<GlinConfig>::new(rpc_client))
}

/// Get account keypair from name (for development)
pub fn get_dev_account(name: &str) -> Result<Keypair> {
    use subxt_signer::sr25519::dev;

    let keypair = match name.to_lowercase().as_str() {
        "alice" => dev::alice(),
        "bob" => dev::bob(),
        "charlie" => dev::charlie(),
        "dave" => dev::dave(),
        "eve" => dev::eve(),
        "ferdie" => dev::ferdie(),
        _ => anyhow::bail!(
            "Unknown dev account: {}. Use alice, bob, charlie, dave, eve, or ferdie",
            name
        ),
    };

    Ok(keypair)
}

/// Load account from seed phrase or secret URI
pub fn account_from_seed(seed: &str) -> Result<Keypair> {
    use subxt_signer::SecretUri;
    use std::str::FromStr;

    // Try as secret URI first (supports //Alice format)
    if let Ok(uri) = SecretUri::from_str(seed) {
        return Keypair::from_uri(&uri)
            .map_err(|e| anyhow::anyhow!("Failed to create keypair from URI: {:?}", e));
    }

    // Try as mnemonic phrase
    use subxt_signer::bip39::Mnemonic;
    if let Ok(mnemonic) = Mnemonic::parse(seed) {
        return Keypair::from_phrase(&mnemonic, None)
            .map_err(|e| anyhow::anyhow!("Failed to create keypair from phrase: {:?}", e));
    }

    anyhow::bail!("Invalid seed format. Use a secret URI (e.g., //Alice) or mnemonic phrase")
}

/// Get account address from keypair
pub fn get_address(keypair: &Keypair) -> String {
    use subxt::utils::AccountId32;

    let account_id: AccountId32 = keypair.public_key().into();
    format!("{:?}", account_id)
}
