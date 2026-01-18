use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use super::handlers;
use crate::MonitorState;

pub fn create_router(state: Arc<tokio::sync::RwLock<MonitorState>>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/status", get(handlers::status))
        .route("/api/invariants", get(handlers::list_invariants))
        .route("/api/invariants/:id", get(handlers::get_invariant))
        .route("/api/invariants/remove", post(handlers::remove_invariant))
        .route("/api/monitor", post(handlers::add_monitored_object))
        .route("/api/analyze", post(handlers::analyze_package))
        .route("/api/invariants/add", post(handlers::add_suggested_invariants))
        .route("/api/metadata/:package_id/:module_name", get(handlers::get_module_metadata))
        .layer(cors)
        .with_state(state)
}
