use crate::aggregator::ProtocolState;
use crate::invariants::types::{InvariantComputation, InvariantResult};
use super::Invariant;

/// INV-001: Total Supply Conservation
/// 
/// Protocol total supply must equal reserves plus outstanding borrows.
/// This ensures no tokens are created or destroyed outside of proper accounting.
pub struct TotalSupplyConservation;

impl Invariant for TotalSupplyConservation {
    fn id(&self) -> &'static str {
        "INV-001"
    }

    fn name(&self) -> &'static str {
        "Total Supply Conservation"
    }

    fn description(&self) -> &'static str {
        "Protocol total supply equals reserves plus outstanding borrows"
    }

    fn evaluate(&self, state: &ProtocolState, _previous: Option<&ProtocolState>) -> InvariantResult {
        let expected = state.total_reserves.saturating_add(state.total_borrowed);
        let actual = state.total_supply;

        let computation = InvariantComputation::new("total_supply == total_reserves + total_borrowed")
            .with_input("total_supply", actual)
            .with_input("total_reserves", state.total_reserves)
            .with_input("total_borrowed", state.total_borrowed)
            .with_input("expected", expected)
            .with_result(format!("{} {} {}", actual, if actual == expected { "==" } else { "!=" }, expected));

        if actual == expected {
            InvariantResult::ok(self.id(), self.name(), self.description(), computation)
        } else {
            InvariantResult::violated(
                self.id(),
                self.name(),
                self.description(),
                computation,
                &format!("Supply mismatch: actual {} != expected {}", actual, expected),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_conservation_ok() {
        let state = ProtocolState {
            total_supply: 1000,
            total_reserves: 400,
            total_borrowed: 600,
            ..Default::default()
        };

        let inv = TotalSupplyConservation;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }

    #[test]
    fn test_supply_conservation_violated() {
        let state = ProtocolState {
            total_supply: 1000,
            total_reserves: 400,
            total_borrowed: 500, // Should be 600
            ..Default::default()
        };

        let inv = TotalSupplyConservation;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Violated);
    }
}
