use async_trait::async_trait;
use reqwest::Client;
use serde::Serialize;

use crate::error::{MonitorError, Result};
use crate::invariants::{InvariantResult, InvariantStatus};
use super::Alerter;

/// Discord webhook alerter with rich embeds
pub struct DiscordAlerter {
    client: Client,
    url: String,
}

#[derive(Serialize)]
struct DiscordMessage {
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: u32,
    fields: Vec<DiscordField>,
    timestamp: String,
}

#[derive(Serialize)]
struct DiscordField {
    name: String,
    value: String,
    inline: bool,
}

impl DiscordAlerter {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    fn status_color(status: &InvariantStatus) -> u32 {
        match status {
            InvariantStatus::Ok => 0x00FF00,      // Green
            InvariantStatus::Violated => 0xFF0000, // Red
            InvariantStatus::Error => 0xFFAA00,   // Orange
        }
    }

    fn status_emoji(status: &InvariantStatus) -> &'static str {
        match status {
            InvariantStatus::Ok => "✅",
            InvariantStatus::Violated => "🚨",
            InvariantStatus::Error => "⚠️",
        }
    }
}

#[async_trait]
impl Alerter for DiscordAlerter {
    async fn send_alert(&self, result: &InvariantResult) -> Result<()> {
        let emoji = Self::status_emoji(&result.status);
        let status_str = match result.status {
            InvariantStatus::Ok => "OK",
            InvariantStatus::Violated => "VIOLATED",
            InvariantStatus::Error => "ERROR",
        };

        let mut fields = vec![
            DiscordField {
                name: "Status".to_string(),
                value: format!("{} {}", emoji, status_str),
                inline: true,
            },
            DiscordField {
                name: "Invariant ID".to_string(),
                value: result.id.clone(),
                inline: true,
            },
            DiscordField {
                name: "Formula".to_string(),
                value: format!("`{}`", result.computation.formula),
                inline: false,
            },
            DiscordField {
                name: "Result".to_string(),
                value: format!("`{}`", result.computation.result),
                inline: false,
            },
        ];

        if let Some(reason) = &result.violation_reason {
            fields.push(DiscordField {
                name: "Violation Reason".to_string(),
                value: reason.clone(),
                inline: false,
            });
        }

        // Add input values
        if !result.computation.inputs.is_empty() {
            let inputs_str = result.computation.inputs
                .iter()
                .map(|(k, v)| format!("• **{}**: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n");
            
            fields.push(DiscordField {
                name: "Computation Inputs".to_string(),
                value: inputs_str,
                inline: false,
            });
        }

        let embed = DiscordEmbed {
            title: format!("{} {}", emoji, result.name),
            description: result.description.clone(),
            color: Self::status_color(&result.status),
            fields,
            timestamp: result.evaluated_at.to_rfc3339(),
        };

        let message = DiscordMessage {
            content: if result.status == InvariantStatus::Violated {
                Some("🚨 **Invariant Violation Detected**".to_string())
            } else {
                None
            },
            embeds: vec![embed],
        };

        let response = self.client
            .post(&self.url)
            .json(&message)
            .send()
            .await
            .map_err(|e| MonitorError::AlertError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(MonitorError::AlertError(format!(
                "Discord webhook returned status {}",
                response.status()
            )));
        }

        Ok(())
    }
}
