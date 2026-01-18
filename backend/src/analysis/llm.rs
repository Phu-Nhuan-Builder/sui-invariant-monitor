use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{MonitorError, Result};
use super::ModuleMetadata;

/// Suggested invariant from LLM analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedInvariant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub formula: String,
    pub severity: String,  // "critical", "high", "medium", "low"
    pub fields_used: Vec<String>,
}

/// Result of AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub package_id: String,
    pub module_name: String,
    pub suggested_invariants: Vec<SuggestedInvariant>,
    pub analysis_notes: String,
}

/// LLM provider type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    OpenRouter,
    Ollama,
}

/// LLM client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: Option<String>,      // For OpenRouter
    pub model: String,
    pub base_url: Option<String>,     // For Ollama (default: http://localhost:11434)
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: LlmProvider::Ollama,
            api_key: None,
            model: "llama3.2".to_string(),
            base_url: Some("http://localhost:11434".to_string()),
        }
    }
}

/// Abstract LLM client trait
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn analyze_module(&self, metadata: &ModuleMetadata) -> Result<AnalysisResult>;
}

/// OpenRouter LLM client
pub struct OpenRouterClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .http1_only()  // Force HTTP/1.1 to avoid OpenRouter issues
                .timeout(std::time::Duration::from_secs(120))  // 2 minute timeout for LLM analysis
                .build()
                .unwrap(),
            api_key,
            model,
        }
    }

    fn build_prompt(&self, metadata: &ModuleMetadata) -> String {
        let mut struct_info = String::new();
        for s in &metadata.structs {
            struct_info.push_str(&format!("\nstruct {} {{\n", s.name));
            for f in &s.fields {
                struct_info.push_str(&format!("  {}: {},\n", f.name, f.type_));
            }
            struct_info.push_str("}\n");
        }

        format!(
            r#"You are a smart contract security expert analyzing a Sui Move module.

Package: {}
Module: {}

Structs:
{}

Analyze this module and suggest safety invariants to monitor. For each invariant, provide:
1. A unique ID (e.g., INV-001)
2. A descriptive name
3. A clear description of what it checks
4. The formula/condition (using field names from the structs)
5. Severity level (critical/high/medium/low)
6. Which fields are used

Focus on:
- Balance/supply consistency
- Numeric bounds and overflow prevention
- State machine validity
- Access control consistency
- Economic invariants

Respond ONLY with valid JSON in this exact format:
{{
  "suggested_invariants": [
    {{
      "id": "INV-001",
      "name": "Invariant Name",
      "description": "What this invariant checks",
      "formula": "field_a <= field_b",
      "severity": "high",
      "fields_used": ["field_a", "field_b"]
    }}
  ],
  "analysis_notes": "Brief analysis summary"
}}"#,
            metadata.package_id, metadata.module_name, struct_info
        )
    }
}

#[async_trait]
impl LlmClient for OpenRouterClient {
    async fn analyze_module(&self, metadata: &ModuleMetadata) -> Result<AnalysisResult> {
        let prompt = self.build_prompt(metadata);

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "http://localhost")  // Required by OpenRouter
            .header("X-Title", "sui-invariant-monitor")  // Required by OpenRouter
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "temperature": 0.3,
                "max_tokens": 2000
            }))
            .send()
            .await
            .map_err(|e| MonitorError::AlertError(format!("OpenRouter request failed: {}", e)))?;

        // Check status and get body for better error messages
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| MonitorError::ParseError(format!("Failed to read response body: {}", e)))?;

        if !status.is_success() {
            return Err(MonitorError::AlertError(format!(
                "OpenRouter error {}: {}",
                status,
                body
            )));
        }

        let json: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| MonitorError::ParseError(format!("Failed to parse OpenRouter JSON: {}", e)))?;

        // Extract content from response (handle both string and array formats)
        let content = json
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .ok_or_else(|| MonitorError::ParseError("No content in OpenRouter response".to_string()))?;

        let content_str = match content {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Array(arr) => {
                arr.iter()
                    .filter_map(|v| v.get("text").and_then(|t| t.as_str()))
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => return Err(MonitorError::ParseError("Invalid content format".to_string())),
        };

        // Parse the JSON response from LLM
        let analysis: serde_json::Value = serde_json::from_str(&content_str)
            .map_err(|e| MonitorError::ParseError(format!("Failed to parse LLM JSON: {}", e)))?;

        let suggested_invariants: Vec<SuggestedInvariant> = analysis
            .get("suggested_invariants")
            .and_then(|i| serde_json::from_value(i.clone()).ok())
            .unwrap_or_default();

        let analysis_notes = analysis
            .get("analysis_notes")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();

        Ok(AnalysisResult {
            package_id: metadata.package_id.clone(),
            module_name: metadata.module_name.clone(),
            suggested_invariants,
            analysis_notes,
        })
    }
}

/// Ollama LLM client (local)
pub struct OllamaClient {
    client: reqwest::Client,
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(base_url: Option<String>, model: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))  // 2 minute timeout for LLM analysis
                .build()
                .unwrap(),
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            model,
        }
    }

    fn build_prompt(&self, metadata: &ModuleMetadata) -> String {
        // Same prompt as OpenRouter
        let mut struct_info = String::new();
        for s in &metadata.structs {
            struct_info.push_str(&format!("\nstruct {} {{\n", s.name));
            for f in &s.fields {
                struct_info.push_str(&format!("  {}: {},\n", f.name, f.type_));
            }
            struct_info.push_str("}\n");
        }

        format!(
            r#"You are a smart contract security expert analyzing a Sui Move module.

Package: {}
Module: {}

Structs:
{}

Analyze this module and suggest safety invariants to monitor. For each invariant, provide:
1. A unique ID (e.g., INV-001)
2. A descriptive name
3. A clear description of what it checks
4. The formula/condition (using field names from the structs)
5. Severity level (critical/high/medium/low)
6. Which fields are used

Focus on:
- Balance/supply consistency
- Numeric bounds and overflow prevention
- State machine validity
- Access control consistency
- Economic invariants

Respond ONLY with valid JSON in this exact format:
{{
  "suggested_invariants": [
    {{
      "id": "INV-001",
      "name": "Invariant Name",
      "description": "What this invariant checks",
      "formula": "field_a <= field_b",
      "severity": "high",
      "fields_used": ["field_a", "field_b"]
    }}
  ],
  "analysis_notes": "Brief analysis summary"
}}"#,
            metadata.package_id, metadata.module_name, struct_info
        )
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn analyze_module(&self, metadata: &ModuleMetadata) -> Result<AnalysisResult> {
        let prompt = self.build_prompt(metadata);

        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
                "format": "json"
            }))
            .send()
            .await
            .map_err(|e| MonitorError::AlertError(format!("Ollama request failed: {}", e)))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| MonitorError::ParseError(format!("Failed to parse Ollama response: {}", e)))?;

        // Extract response content
        let content = json
            .get("response")
            .and_then(|r| r.as_str())
            .ok_or_else(|| MonitorError::ParseError("No response in Ollama output".to_string()))?;

        // Parse the JSON response from LLM
        let analysis: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| MonitorError::ParseError(format!("Failed to parse LLM JSON: {}", e)))?;

        let suggested_invariants: Vec<SuggestedInvariant> = analysis
            .get("suggested_invariants")
            .and_then(|i| serde_json::from_value(i.clone()).ok())
            .unwrap_or_default();

        let analysis_notes = analysis
            .get("analysis_notes")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();

        Ok(AnalysisResult {
            package_id: metadata.package_id.clone(),
            module_name: metadata.module_name.clone(),
            suggested_invariants,
            analysis_notes,
        })
    }
}

/// Create LLM client based on config
pub fn create_llm_client(config: &LlmConfig) -> Result<Box<dyn LlmClient>> {
    match config.provider {
        LlmProvider::OpenRouter => {
            let api_key = config.api_key.clone()
                .ok_or_else(|| MonitorError::ConfigError("OpenRouter API key required".to_string()))?;
            Ok(Box::new(OpenRouterClient::new(api_key, config.model.clone())))
        }
        LlmProvider::Ollama => {
            Ok(Box::new(OllamaClient::new(config.base_url.clone(), config.model.clone())))
        }
    }
}
