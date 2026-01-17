use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub sui_rpc_url: String,
    pub polling_interval_secs: u64,
    pub webhook_url: Option<String>,
    pub discord_webhook_url: Option<String>,
    pub monitored_object_ids: Vec<String>,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let sui_rpc_url = env::var("SUI_RPC_URL")
            .unwrap_or_else(|_| "https://fullnode.mainnet.sui.io:443".to_string());

        let polling_interval_secs = env::var("POLLING_INTERVAL_SECS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("POLLING_INTERVAL_SECS".to_string()))?;

        let webhook_url = env::var("WEBHOOK_URL").ok().filter(|s| !s.is_empty());
        let discord_webhook_url = env::var("DISCORD_WEBHOOK_URL").ok().filter(|s| !s.is_empty());

        let monitored_object_ids: Vec<String> = env::var("MONITORED_OBJECT_IDS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("PORT".to_string()))?;

        Ok(Self {
            sui_rpc_url,
            polling_interval_secs,
            webhook_url,
            discord_webhook_url,
            monitored_object_ids,
            port,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid value for {0}")]
    InvalidValue(String),

    #[error("Missing required environment variable: {0}")]
    MissingRequired(String),
}
