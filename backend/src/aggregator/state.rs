use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Result;
use crate::sui_client::SuiObjectResponse;

/// Normalized protocol state derived from on-chain objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolState {
    /// Unix timestamp when state was fetched
    pub timestamp: u64,
    /// Total supply in the lending pool
    pub total_supply: u128,
    /// Total amount currently borrowed
    pub total_borrowed: u128,
    /// Protocol reserves
    pub total_reserves: u128,
    /// Total collateral value
    pub collateral_value: u128,
    /// Outstanding LP shares
    pub outstanding_shares: u128,
    /// Interest accumulation index
    pub interest_index: u128,
    /// Last update epoch
    pub last_update_epoch: u64,
    /// On-chain balance (for accounting verification)
    pub on_chain_balance: u128,
}

impl Default for ProtocolState {
    fn default() -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp() as u64,
            total_supply: 0,
            total_borrowed: 0,
            total_reserves: 0,
            collateral_value: 0,
            outstanding_shares: 0,
            interest_index: 1_000_000_000, // 1.0 scaled by 1e9
            last_update_epoch: 0,
            on_chain_balance: 0,
        }
    }
}

pub struct StateAggregator;

impl StateAggregator {
    /// Aggregate protocol state from raw Sui object responses
    /// 
    /// This function parses Move struct fields from the object content
    /// and normalizes them into a unified ProtocolState.
    pub fn aggregate(objects: &[SuiObjectResponse], on_chain_balance: u128) -> Result<ProtocolState> {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let mut state = ProtocolState {
            timestamp,
            on_chain_balance,
            ..Default::default()
        };

        for obj_response in objects {
            if let Some(data) = &obj_response.data {
                if let Some(content) = &data.content {
                    if let Some(fields) = &content.fields {
                        Self::parse_fields(&mut state, fields)?;
                    }
                }
            }
        }

        Ok(state)
    }

    fn parse_fields(state: &mut ProtocolState, fields: &Value) -> Result<()> {
        if let Some(obj) = fields.as_object() {
            // Try to extract common lending protocol fields
            if let Some(total_supply) = Self::extract_u128(obj, "total_supply") {
                state.total_supply = total_supply;
            }
            if let Some(total_borrowed) = Self::extract_u128(obj, "total_borrowed") {
                state.total_borrowed = total_borrowed;
            }
            if let Some(total_reserves) = Self::extract_u128(obj, "total_reserves") {
                state.total_reserves = total_reserves;
            }
            if let Some(collateral) = Self::extract_u128(obj, "collateral_value") {
                state.collateral_value = collateral;
            }
            if let Some(shares) = Self::extract_u128(obj, "outstanding_shares") {
                state.outstanding_shares = shares;
            }
            if let Some(index) = Self::extract_u128(obj, "interest_index") {
                state.interest_index = index;
            }
            if let Some(epoch) = Self::extract_u64(obj, "last_update_epoch") {
                state.last_update_epoch = epoch;
            }
        }
        Ok(())
    }

    fn extract_u128(fields: &serde_json::Map<String, Value>, key: &str) -> Option<u128> {
        fields.get(key).and_then(|v| {
            // Handle both string and number representations
            v.as_str()
                .and_then(|s| s.parse().ok())
                .or_else(|| v.as_u64().map(|n| n as u128))
        })
    }

    fn extract_u64(fields: &serde_json::Map<String, Value>, key: &str) -> Option<u64> {
        fields.get(key).and_then(|v| {
            v.as_str()
                .and_then(|s| s.parse().ok())
                .or_else(|| v.as_u64())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = ProtocolState::default();
        assert_eq!(state.total_supply, 0);
        assert_eq!(state.interest_index, 1_000_000_000);
    }
}
