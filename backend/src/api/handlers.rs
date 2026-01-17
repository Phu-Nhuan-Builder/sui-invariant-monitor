use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::invariants::InvariantResult;
use crate::analysis::{ModuleMetadata, LlmProvider, SuggestedInvariant, AnalysisResult};
use crate::analysis::metadata::MetadataFetcher;
use crate::analysis::llm::{LlmConfig, create_llm_client, LlmClient};
use crate::MonitorState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    uptime_secs: u64,
}

#[derive(Serialize)]
pub struct StatusResponse {
    last_check: Option<String>,
    violations: usize,
    total_invariants: usize,
    all_ok: bool,
    monitored_objects: Vec<String>,
}

#[derive(Deserialize)]
pub struct MonitorRequest {
    pub object_id: String,
    #[serde(default)]
    pub network: Option<String>,
}

#[derive(Serialize)]
pub struct MonitorResponse {
    pub success: bool,
    pub message: String,
    pub object_id: String,
    pub object_type: Option<String>,
}

/// Request to analyze a package
#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub package_id: String,
    pub module_name: Option<String>,  // If not provided, analyze all modules
    pub llm_provider: LlmProvider,
    pub api_key: Option<String>,      // For OpenRouter
    pub model: String,                // e.g., "anthropic/claude-3.5-sonnet" or "llama3.2"
    pub ollama_url: Option<String>,   // For Ollama, default: http://localhost:11434
}

/// Response from analysis
#[derive(Serialize)]
pub struct AnalyzeResponse {
    pub success: bool,
    pub message: String,
    pub modules: Vec<ModuleMetadata>,
    pub analysis_results: Vec<AnalysisResult>,
}

/// Health check endpoint
pub async fn health(State(state): State<Arc<RwLock<MonitorState>>>) -> Json<HealthResponse> {
    let state = state.read().await;
    Json(HealthResponse {
        status: "ok",
        uptime_secs: state.start_time.elapsed().as_secs(),
    })
}

/// Get overall monitoring status
pub async fn status(State(state): State<Arc<RwLock<MonitorState>>>) -> Json<StatusResponse> {
    let state = state.read().await;
    
    let violations = state.results
        .iter()
        .filter(|r| r.status == crate::invariants::InvariantStatus::Violated)
        .count();

    let all_ok = violations == 0 && !state.results.is_empty();

    Json(StatusResponse {
        last_check: state.last_check.map(|t| t.to_rfc3339()),
        violations,
        total_invariants: state.results.len(),
        all_ok,
        monitored_objects: state.monitored_objects.clone(),
    })
}

/// List all invariant results
pub async fn list_invariants(
    State(state): State<Arc<RwLock<MonitorState>>>,
) -> Json<Vec<InvariantResult>> {
    let state = state.read().await;
    Json(state.results.clone())
}

/// Get a specific invariant by ID
pub async fn get_invariant(
    State(state): State<Arc<RwLock<MonitorState>>>,
    Path(id): Path<String>,
) -> Result<Json<InvariantResult>, StatusCode> {
    let state = state.read().await;
    
    state.results
        .iter()
        .find(|r| r.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Add a new object to monitor dynamically
pub async fn add_monitored_object(
    State(state): State<Arc<RwLock<MonitorState>>>,
    Json(request): Json<MonitorRequest>,
) -> Json<MonitorResponse> {
    let object_id = request.object_id.trim().to_string();
    
    // Validate object ID format
    if !object_id.starts_with("0x") || object_id.len() != 66 {
        return Json(MonitorResponse {
            success: false,
            message: "Invalid object ID format. Should be 0x followed by 64 hex characters.".to_string(),
            object_id,
            object_type: None,
        });
    }
    
    let mut state = state.write().await;
    
    if state.monitored_objects.contains(&object_id) {
        return Json(MonitorResponse {
            success: true,
            message: "Object is already being monitored.".to_string(),
            object_id,
            object_type: None,
        });
    }
    
    state.monitored_objects.push(object_id.clone());
    state.pending_evaluation = true;
    
    Json(MonitorResponse {
        success: true,
        message: format!("Added object {} to monitoring. Will evaluate on next cycle.", object_id),
        object_id,
        object_type: None,
    })
}

/// Analyze a package using LLM
pub async fn analyze_package(
    State(state): State<Arc<RwLock<MonitorState>>>,
    Json(request): Json<AnalyzeRequest>,
) -> Json<AnalyzeResponse> {
    let state_read = state.read().await;
    let rpc_url = state_read.rpc_url.clone();
    drop(state_read);

    let fetcher = MetadataFetcher::new(&rpc_url);
    
    // Fetch module list if no specific module provided
    let module_names: Vec<String> = if let Some(ref name) = request.module_name {
        vec![name.clone()]
    } else {
        match fetcher.fetch_package_modules(&request.package_id).await {
            Ok(names) => names,
            Err(e) => {
                return Json(AnalyzeResponse {
                    success: false,
                    message: format!("Failed to fetch package modules: {}", e),
                    modules: vec![],
                    analysis_results: vec![],
                });
            }
        }
    };

    if module_names.is_empty() {
        return Json(AnalyzeResponse {
            success: false,
            message: "No modules found in package".to_string(),
            modules: vec![],
            analysis_results: vec![],
        });
    }

    // Fetch metadata for each module
    let mut modules = Vec::new();
    for module_name in &module_names {
        match fetcher.fetch_module_metadata(&request.package_id, module_name).await {
            Ok(metadata) => modules.push(metadata),
            Err(e) => {
                tracing::warn!("Failed to fetch module {}: {}", module_name, e);
            }
        }
    }

    if modules.is_empty() {
        return Json(AnalyzeResponse {
            success: false,
            message: "Failed to fetch any module metadata".to_string(),
            modules: vec![],
            analysis_results: vec![],
        });
    }

    // Create LLM client
    let llm_config = LlmConfig {
        provider: request.llm_provider,
        api_key: request.api_key,
        model: request.model,
        base_url: request.ollama_url,
    };

    let llm_client = match create_llm_client(&llm_config) {
        Ok(client) => client,
        Err(e) => {
            return Json(AnalyzeResponse {
                success: false,
                message: format!("Failed to create LLM client: {}", e),
                modules,
                analysis_results: vec![],
            });
        }
    };

    // Analyze each module
    let mut analysis_results = Vec::new();
    for module in &modules {
        match llm_client.analyze_module(module).await {
            Ok(result) => analysis_results.push(result),
            Err(e) => {
                tracing::warn!("Failed to analyze module {}: {}", module.module_name, e);
            }
        }
    }

    Json(AnalyzeResponse {
        success: true,
        message: format!("Analyzed {} module(s), found {} invariants", 
            analysis_results.len(),
            analysis_results.iter().map(|r| r.suggested_invariants.len()).sum::<usize>()
        ),
        modules,
        analysis_results,
    })
}

/// Get module metadata (without LLM analysis)
pub async fn get_module_metadata(
    State(state): State<Arc<RwLock<MonitorState>>>,
    Path((package_id, module_name)): Path<(String, String)>,
) -> Result<Json<ModuleMetadata>, StatusCode> {
    let state_read = state.read().await;
    let rpc_url = state_read.rpc_url.clone();
    drop(state_read);

    let fetcher = MetadataFetcher::new(&rpc_url);
    
    match fetcher.fetch_module_metadata(&package_id, &module_name).await {
        Ok(metadata) => Ok(Json(metadata)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
