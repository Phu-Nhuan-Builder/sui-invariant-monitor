use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{MonitorError, Result};

/// Metadata about a Move module fetched from Sui RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub package_id: String,
    pub module_name: String,
    pub structs: Vec<StructMetadata>,
    pub functions: Vec<FunctionMetadata>,
}

/// Metadata about a Move struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructMetadata {
    pub name: String,
    pub abilities: Vec<String>,
    pub fields: Vec<FieldMetadata>,
}

/// Metadata about a struct field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub name: String,
    pub type_: String,
}

/// Metadata about a function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetadata {
    pub name: String,
    pub visibility: String,
    pub is_entry: bool,
    pub parameters: Vec<String>,
    pub return_types: Vec<String>,
}

/// Fetches module metadata from Sui RPC
pub struct MetadataFetcher {
    client: reqwest::Client,
    rpc_url: String,
}

impl MetadataFetcher {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url: rpc_url.to_string(),
        }
    }

    /// Fetch normalized module metadata for a package
    pub async fn fetch_module_metadata(
        &self,
        package_id: &str,
        module_name: &str,
    ) -> Result<ModuleMetadata> {
        let response = self.client
            .post(&self.rpc_url)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "sui_getNormalizedMoveModule",
                "params": [package_id, module_name]
            }))
            .send()
            .await
            .map_err(|e| MonitorError::RpcError(e.to_string()))?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| MonitorError::ParseError(e.to_string()))?;

        if let Some(error) = json.get("error") {
            return Err(MonitorError::RpcError(error.to_string()));
        }

        let result = json.get("result")
            .ok_or_else(|| MonitorError::ParseError("No result in response".to_string()))?;

        self.parse_module_metadata(package_id, module_name, result)
    }

    /// Get list of modules in a package
    pub async fn fetch_package_modules(&self, package_id: &str) -> Result<Vec<String>> {
        let response = self.client
            .post(&self.rpc_url)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "sui_getNormalizedMoveModulesByPackage",
                "params": [package_id]
            }))
            .send()
            .await
            .map_err(|e| MonitorError::RpcError(e.to_string()))?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| MonitorError::ParseError(e.to_string()))?;

        if let Some(error) = json.get("error") {
            return Err(MonitorError::RpcError(error.to_string()));
        }

        let result = json.get("result")
            .ok_or_else(|| MonitorError::ParseError("No result in response".to_string()))?;

        let modules: Vec<String> = result.as_object()
            .map(|obj| obj.keys().cloned().collect())
            .unwrap_or_default();

        Ok(modules)
    }

    fn parse_module_metadata(
        &self,
        package_id: &str,
        module_name: &str,
        data: &Value,
    ) -> Result<ModuleMetadata> {
        let mut structs = Vec::new();
        let mut functions = Vec::new();

        // Parse structs
        if let Some(struct_map) = data.get("structs").and_then(|s| s.as_object()) {
            for (name, struct_data) in struct_map {
                let abilities = struct_data.get("abilities")
                    .and_then(|a| a.get("abilities"))
                    .and_then(|a| a.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();

                let fields = struct_data.get("fields")
                    .and_then(|f| f.as_array())
                    .map(|arr| {
                        arr.iter().filter_map(|field| {
                            let name = field.get("name")?.as_str()?.to_string();
                            let type_ = self.parse_type_tag(field.get("type")?);
                            Some(FieldMetadata { name, type_ })
                        }).collect()
                    })
                    .unwrap_or_default();

                structs.push(StructMetadata {
                    name: name.clone(),
                    abilities,
                    fields,
                });
            }
        }

        // Parse functions
        if let Some(func_map) = data.get("exposedFunctions").and_then(|f| f.as_object()) {
            for (name, func_data) in func_map {
                let visibility = func_data.get("visibility")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Private")
                    .to_string();

                let is_entry = func_data.get("isEntry")
                    .and_then(|e| e.as_bool())
                    .unwrap_or(false);

                let parameters = func_data.get("parameters")
                    .and_then(|p| p.as_array())
                    .map(|arr| arr.iter().map(|t| self.parse_type_tag(t)).collect())
                    .unwrap_or_default();

                let return_types = func_data.get("return")
                    .and_then(|r| r.as_array())
                    .map(|arr| arr.iter().map(|t| self.parse_type_tag(t)).collect())
                    .unwrap_or_default();

                functions.push(FunctionMetadata {
                    name: name.clone(),
                    visibility,
                    is_entry,
                    parameters,
                    return_types,
                });
            }
        }

        Ok(ModuleMetadata {
            package_id: package_id.to_string(),
            module_name: module_name.to_string(),
            structs,
            functions,
        })
    }

    fn parse_type_tag(&self, type_data: &Value) -> String {
        // Handle different type formats from Sui RPC
        if let Some(s) = type_data.as_str() {
            return s.to_string();
        }

        if let Some(obj) = type_data.as_object() {
            // Handle struct types
            if let Some(struct_info) = obj.get("Struct") {
                let address = struct_info.get("address").and_then(|a| a.as_str()).unwrap_or("");
                let module = struct_info.get("module").and_then(|m| m.as_str()).unwrap_or("");
                let name = struct_info.get("name").and_then(|n| n.as_str()).unwrap_or("");
                return format!("{}::{}::{}", address, module, name);
            }

            // Handle vector types
            if let Some(inner) = obj.get("Vector") {
                return format!("vector<{}>", self.parse_type_tag(inner));
            }

            // Handle reference types
            if let Some(inner) = obj.get("Reference") {
                return format!("&{}", self.parse_type_tag(inner));
            }
            if let Some(inner) = obj.get("MutableReference") {
                return format!("&mut {}", self.parse_type_tag(inner));
            }

            // Handle type parameters
            if let Some(idx) = obj.get("TypeParameter") {
                return format!("T{}", idx);
            }
        }

        format!("{:?}", type_data)
    }
}
