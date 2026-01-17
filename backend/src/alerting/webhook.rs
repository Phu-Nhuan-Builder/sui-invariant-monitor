use async_trait::async_trait;
use reqwest::Client;
use serde::Serialize;

use crate::error::{MonitorError, Result};
use crate::invariants::InvariantResult;
use super::Alerter;

/// Generic webhook alerter
pub struct WebhookAlerter {
    client: Client,
    url: String,
}

#[derive(Serialize)]
struct WebhookPayload<'a> {
    invariant_id: &'a str,
    invariant_name: &'a str,
    status: &'a str,
    violation_reason: Option<&'a str>,
    timestamp: String,
    computation: &'a crate::invariants::InvariantComputation,
}

impl WebhookAlerter {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }
}

#[async_trait]
impl Alerter for WebhookAlerter {
    async fn send_alert(&self, result: &InvariantResult) -> Result<()> {
        let payload = WebhookPayload {
            invariant_id: &result.id,
            invariant_name: &result.name,
            status: match result.status {
                crate::invariants::InvariantStatus::Ok => "ok",
                crate::invariants::InvariantStatus::Violated => "violated",
                crate::invariants::InvariantStatus::Error => "error",
            },
            violation_reason: result.violation_reason.as_deref(),
            timestamp: result.evaluated_at.to_rfc3339(),
            computation: &result.computation,
        };

        let response = self.client
            .post(&self.url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| MonitorError::AlertError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(MonitorError::AlertError(format!(
                "Webhook returned status {}",
                response.status()
            )));
        }

        Ok(())
    }
}
