use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::{MonitorError, Result};

pub struct SuiFetcher {
    client: Client,
    rpc_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiObjectResponse {
    pub data: Option<SuiObjectData>,
    pub error: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiObjectData {
    pub object_id: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub content: Option<SuiParsedData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiParsedData {
    pub data_type: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub fields: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    message: String,
}

impl SuiFetcher {
    pub async fn new(rpc_url: &str) -> Result<Self> {
        let client = Client::new();
        
        // Verify connection with a simple request
        let response = client
            .post(rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "sui_getLatestCheckpointSequenceNumber",
                "params": []
            }))
            .send()
            .await
            .map_err(|e| MonitorError::RpcError(format!("Connection failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(MonitorError::RpcError(format!(
                "RPC returned status {}",
                response.status()
            )));
        }

        Ok(Self {
            client,
            rpc_url: rpc_url.to_string(),
        })
    }

    /// Fetch a single object by ID with full content
    pub async fn fetch_object(&self, object_id: &str) -> Result<SuiObjectResponse> {
        let response = self.client
            .post(&self.rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "sui_getObject",
                "params": [
                    object_id,
                    {
                        "showType": true,
                        "showContent": true,
                        "showOwner": true
                    }
                ]
            }))
            .send()
            .await
            .map_err(|e| MonitorError::RpcError(e.to_string()))?;

        let rpc_response: RpcResponse<SuiObjectResponse> = response
            .json()
            .await
            .map_err(|e| MonitorError::ParseError(e.to_string()))?;

        if let Some(error) = rpc_response.error {
            return Err(MonitorError::RpcError(error.message));
        }

        rpc_response.result.ok_or_else(|| {
            MonitorError::ObjectNotFound(object_id.to_string())
        })
    }

    /// Fetch multiple objects by IDs
    pub async fn fetch_objects(&self, object_ids: &[String]) -> Result<Vec<SuiObjectResponse>> {
        let mut results = Vec::with_capacity(object_ids.len());
        
        for id in object_ids {
            match self.fetch_object(id).await {
                Ok(obj) => results.push(obj),
                Err(e) => {
                    tracing::warn!("Failed to fetch object {}: {}", id, e);
                    // Continue with other objects
                }
            }
        }

        Ok(results)
    }

    /// Fetch balance of an address for a specific coin type
    pub async fn fetch_balance(&self, address: &str, coin_type: Option<&str>) -> Result<u128> {
        let coin_type = coin_type.unwrap_or("0x2::sui::SUI");
        
        let response = self.client
            .post(&self.rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "suix_getBalance",
                "params": [address, coin_type]
            }))
            .send()
            .await
            .map_err(|e| MonitorError::RpcError(e.to_string()))?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BalanceResponse {
            total_balance: String,
        }

        let rpc_response: RpcResponse<BalanceResponse> = response
            .json()
            .await
            .map_err(|e| MonitorError::ParseError(e.to_string()))?;

        if let Some(error) = rpc_response.error {
            return Err(MonitorError::RpcError(error.message));
        }

        let balance = rpc_response.result
            .ok_or_else(|| MonitorError::ParseError("No balance result".to_string()))?;

        balance.total_balance
            .parse()
            .map_err(|e| MonitorError::ParseError(format!("Invalid balance: {}", e)))
    }
}
