use std::env;

/// Get RPC URL for the specified network
pub fn get_rpc_url(network: Option<&str>) -> String {
    match network {
        Some("mainnet") => "https://fullnode.mainnet.sui.io:443".to_string(),
        Some("testnet") => "https://fullnode.testnet.sui.io:443".to_string(),
        _ => {
            // Fallback to env var or mainnet
            env::var("SUI_RPC_URL")
                .unwrap_or_else(|_| "https://fullnode.mainnet.sui.io:443".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mainnet() {
        assert_eq!(
            get_rpc_url(Some("mainnet")),
            "https://fullnode.mainnet.sui.io:443"
        );
    }

    #[test]
    fn test_testnet() {
        assert_eq!(
            get_rpc_url(Some("testnet")),
            "https://fullnode.testnet.sui.io:443"
        );
    }

    #[test]
    fn test_none_defaults_to_mainnet() {
        assert_eq!(
            get_rpc_url(None),
            "https://fullnode.mainnet.sui.io:443"
        );
    }
}
