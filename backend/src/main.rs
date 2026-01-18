mod config;
mod error;
mod sui_client;
mod aggregator;
mod invariants;
mod alerting;
mod api;
mod analysis;
mod network;

use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use tracing::{info, error, warn};

use config::Config;
use sui_client::SuiFetcher;
use aggregator::StateAggregator;
use invariants::{InvariantEngine, InvariantResult, InvariantStatus};
use alerting::{DiscordAlerter, WebhookAlerter, Alerter};

/// Shared state for the monitoring service
pub struct MonitorState {
    pub results: Vec<InvariantResult>,
    pub last_check: Option<DateTime<Utc>>,
    pub start_time: Instant,
    pub monitored_objects: Vec<String>,
    pub pending_evaluation: bool,
    pub rpc_url: String,
}

impl MonitorState {
    fn new(initial_objects: Vec<String>, rpc_url: String) -> Self {
        Self {
            results: Vec::new(),
            last_check: None,
            start_time: Instant::now(),
            monitored_objects: initial_objects,
            pending_evaluation: false,
            rpc_url,
        }
    }

    fn update(&mut self, results: Vec<InvariantResult>) {
        self.results = results;
        self.last_check = Some(Utc::now());
        self.pending_evaluation = false;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("sui_invariant_monitor=info".parse().unwrap())
        )
        .init();

    info!("Starting Sui Invariant Monitor");

    // Load configuration
    let config = Config::from_env().map_err(|e| anyhow::anyhow!("Config error: {}", e))?;
    info!("Loaded configuration");
    info!("  RPC URL: {}", config.sui_rpc_url);
    info!("  Polling interval: {}s", config.polling_interval_secs);
    info!("  Initial monitored objects: {:?}", config.monitored_object_ids);

    // Initialize shared state with RPC URL
    let state = Arc::new(RwLock::new(MonitorState::new(
        config.monitored_object_ids.clone(),
        config.sui_rpc_url.clone(),
    )));

    // Start API server
    let api_state = state.clone();
    let api_port = config.port;
    tokio::spawn(async move {
        let router = api::create_router(api_state);
        let addr = format!("0.0.0.0:{}", api_port);
        info!("Starting API server on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    });

    // Run the monitoring service loop
    run_service_loop(&config, state).await;

    Ok(())
}

async fn run_service_loop(
    config: &Config,
    state: Arc<RwLock<MonitorState>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.polling_interval_secs));
    let mut engine = InvariantEngine::new();

    // Initialize alerters
    let webhook_alerter = config.webhook_url.as_ref().map(|url| WebhookAlerter::new(url.clone()));
    let discord_alerter = config.discord_webhook_url.as_ref().map(|url| DiscordAlerter::new(url.clone()));

    info!("Starting service loop");

    loop {
        interval.tick().await;

        // Get current monitored objects from state
        let monitored_objects = {
            let s = state.read().await;
            s.monitored_objects.clone()
        };

        match fetch_and_evaluate(config, &mut engine, &monitored_objects).await {
            Ok(results) => {
                let violations = InvariantEngine::violation_count(&results);
                let errors = InvariantEngine::error_count(&results);

                // Update shared state
                state.write().await.update(results.clone());

                // Log status
                if violations > 0 {
                    warn!(
                        "Evaluation complete: {} invariants, {} violations, {} errors",
                        results.len(), violations, errors
                    );
                } else {
                    info!(
                        "Evaluation complete: {} invariants, {} violations, {} errors",
                        results.len(), violations, errors
                    );
                }

                // Send alerts for violations
                for result in &results {
                    if result.status == InvariantStatus::Violated {
                        if let Some(alerter) = &webhook_alerter {
                            if let Err(e) = alerter.send_alert(result).await {
                                error!("Failed to send webhook alert: {}", e);
                            }
                        }
                        if let Some(alerter) = &discord_alerter {
                            if let Err(e) = alerter.send_alert(result).await {
                                error!("Failed to send Discord alert: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Evaluation failed: {}", e);
            }
        }
    }
}

async fn fetch_and_evaluate(
    config: &Config,
    engine: &mut InvariantEngine,
    monitored_objects: &[String],
) -> error::Result<Vec<InvariantResult>> {
    let fetcher = SuiFetcher::new(&config.sui_rpc_url).await?;

    let objects = if monitored_objects.is_empty() {
        info!("No monitored object IDs configured, using empty state");
        Vec::new()
    } else {
        fetcher.fetch_objects(monitored_objects).await?
    };

    let on_chain_balance = 0;
    let protocol_state = StateAggregator::aggregate(&objects, on_chain_balance)?;

    info!(
        "Fetched state: supply={}, borrowed={}, reserves={}, collateral={}",
        protocol_state.total_supply,
        protocol_state.total_borrowed,
        protocol_state.total_reserves,
        protocol_state.collateral_value
    );

    let results = engine.evaluate_all(&protocol_state);

    Ok(results)
}
