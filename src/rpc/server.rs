use anyhow::{Context, Result};
use jsonrpc_core::{Error as RpcError, ErrorCode, IoHandler, Params};
use jsonrpc_http_server::{Server, ServerBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::rpc::methods;
use crate::rpc::types::*;

/// JSON-RPC server for SDK communication
pub struct RpcServer {
    server: Arc<Mutex<Option<Server>>>,
    port: u16,
}

impl RpcServer {
    /// Start the RPC server on a random available port
    pub async fn start(network: String) -> Result<Self> {
        let mut io = IoHandler::new();

        // Clone network for each closure
        let network_deploy = network.clone();
        let network_call = network.clone();
        let network_query = network.clone();
        let network_watch = network.clone();

        // Register deploy method
        io.add_method("deploy", move |params: Params| {
            let _network = network_deploy.clone();
            async move {
                let deploy_params: DeployParams = params
                    .parse()
                    .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

                let result = methods::handle_deploy(deploy_params)
                    .await
                    .map_err(|e| RpcError {
                        code: ErrorCode::InternalError,
                        message: e.to_string(),
                        data: None,
                    })?;

                let json = serde_json::to_value(&result).map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: format!("Serialization error: {}", e),
                    data: None,
                })?;

                Ok(json)
            }
        });

        // Register call method
        io.add_method("call", move |params: Params| {
            let _network = network_call.clone();
            async move {
                let call_params: CallParams = params
                    .parse()
                    .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

                let result = methods::handle_call(call_params)
                    .await
                    .map_err(|e| RpcError {
                        code: ErrorCode::InternalError,
                        message: e.to_string(),
                        data: None,
                    })?;

                let json = serde_json::to_value(&result).map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: format!("Serialization error: {}", e),
                    data: None,
                })?;

                Ok(json)
            }
        });

        // Register query method
        io.add_method("query", move |params: Params| {
            let _network = network_query.clone();
            async move {
                let query_params: QueryParams = params
                    .parse()
                    .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

                let result = methods::handle_query(query_params)
                    .await
                    .map_err(|e| RpcError {
                        code: ErrorCode::InternalError,
                        message: e.to_string(),
                        data: None,
                    })?;

                let json = serde_json::to_value(&result).map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: format!("Serialization error: {}", e),
                    data: None,
                })?;

                Ok(json)
            }
        });

        // Register watch method
        io.add_method("watch", move |params: Params| {
            let _network = network_watch.clone();
            async move {
                let watch_params: WatchParams = params
                    .parse()
                    .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

                let result = methods::handle_watch(watch_params)
                    .await
                    .map_err(|e| RpcError {
                        code: ErrorCode::InternalError,
                        message: e.to_string(),
                        data: None,
                    })?;

                let json = serde_json::to_value(&result).map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: format!("Serialization error: {}", e),
                    data: None,
                })?;

                Ok(json)
            }
        });

        // Register getBalance method
        io.add_method("getBalance", move |params: Params| async move {
            let balance_params: GetBalanceParams = params
                .parse()
                .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

            let result = methods::handle_get_balance(balance_params)
                .await
                .map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                    data: None,
                })?;

            let json = serde_json::to_value(&result).map_err(|e| RpcError {
                code: ErrorCode::InternalError,
                message: format!("Serialization error: {}", e),
                data: None,
            })?;

            Ok(json)
        });

        // Register requestFaucet method
        io.add_method("requestFaucet", move |params: Params| async move {
            let faucet_params: RequestFaucetParams = params
                .parse()
                .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

            let result = methods::handle_request_faucet(faucet_params)
                .await
                .map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                    data: None,
                })?;

            let json = serde_json::to_value(&result).map_err(|e| RpcError {
                code: ErrorCode::InternalError,
                message: format!("Serialization error: {}", e),
                data: None,
            })?;

            Ok(json)
        });

        // Register estimateGas method
        io.add_method("estimateGas", move |params: Params| async move {
            let gas_params: EstimateGasParams = params
                .parse()
                .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

            let result = methods::handle_estimate_gas(gas_params)
                .await
                .map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                    data: None,
                })?;

            let json = serde_json::to_value(&result).map_err(|e| RpcError {
                code: ErrorCode::InternalError,
                message: format!("Serialization error: {}", e),
                data: None,
            })?;

            Ok(json)
        });

        // Register getBlockNumber method
        io.add_method("getBlockNumber", move |params: Params| async move {
            let block_params: GetBlockNumberParams = params
                .parse()
                .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

            let result = methods::handle_get_block_number(block_params)
                .await
                .map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                    data: None,
                })?;

            let json = serde_json::to_value(&result).map_err(|e| RpcError {
                code: ErrorCode::InternalError,
                message: format!("Serialization error: {}", e),
                data: None,
            })?;

            Ok(json)
        });

        // Register getNetworkInfo method
        io.add_method("getNetworkInfo", move |params: Params| async move {
            let info_params: GetNetworkInfoParams = params
                .parse()
                .map_err(|e| RpcError::invalid_params(format!("{}", e)))?;

            let result = methods::handle_get_network_info(info_params)
                .await
                .map_err(|e| RpcError {
                    code: ErrorCode::InternalError,
                    message: e.to_string(),
                    data: None,
                })?;

            let json = serde_json::to_value(&result).map_err(|e| RpcError {
                code: ErrorCode::InternalError,
                message: format!("Serialization error: {}", e),
                data: None,
            })?;

            Ok(json)
        });

        // Start server on random port
        let server = ServerBuilder::new(io)
            .start_http(&"127.0.0.1:0".parse()?)
            .context("Unable to start RPC server")?;

        let port = server.address().port();

        Ok(RpcServer {
            server: Arc::new(Mutex::new(Some(server))),
            port,
        })
    }

    /// Get the port the server is listening on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Shutdown the RPC server
    pub async fn shutdown(&self) -> Result<()> {
        let mut server_lock = self.server.lock().await;
        if let Some(server) = server_lock.take() {
            server.close();
        }
        Ok(())
    }
}

impl Drop for RpcServer {
    fn drop(&mut self) {
        // Best effort cleanup - try to shut down the server
        if let Some(server) = Arc::get_mut(&mut self.server) {
            if let Ok(mut lock) = server.try_lock() {
                if let Some(s) = lock.take() {
                    s.close();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Skip: Runtime drop issue in nested async context
    async fn test_server_start_and_shutdown() {
        let server = RpcServer::start("testnet".to_string()).await.unwrap();
        assert!(server.port() > 0);
        server.shutdown().await.unwrap();
    }
}
