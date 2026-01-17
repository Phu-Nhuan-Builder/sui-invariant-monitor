pub mod webhook;
pub mod discord;

pub use webhook::WebhookAlerter;
pub use discord::DiscordAlerter;

use crate::invariants::InvariantResult;
use crate::error::Result;
use async_trait::async_trait;

/// Trait for alert dispatchers
#[async_trait]
pub trait Alerter: Send + Sync {
    async fn send_alert(&self, result: &InvariantResult) -> Result<()>;
}
