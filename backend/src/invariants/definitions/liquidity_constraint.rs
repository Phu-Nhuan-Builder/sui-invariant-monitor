use crate::aggregator::ProtocolState;
use crate::invariants::types::{InvariantComputation, InvariantResult};
use super::Invariant;

/// INV-005: Liquidity Constraint
/// 
/// Available liquidity must be non-negative.
/// Total borrowed must not exceed total supply.
pub struct LiquidityConstraint;

impl Invariant for LiquidityConstraint {
    fn id(&self) -> &'static str {
        "INV-005"
    }

    fn name(&self) -> &'static str {
        "Liquidity Constraint"
    }

    fn description(&self) -> &'static str {
        "Total borrowed must not exceed total supply"
    }

    fn evaluate(&self, state: &ProtocolState, _previous: Option<&ProtocolState>) -> InvariantResult {
        let is_valid = state.total_borrowed <= state.total_supply;

        let available_liquidity = if state.total_supply >= state.total_borrowed {
            state.total_supply - state.total_borrowed
        } else {
            0
        };

        let utilization_percent = if state.total_supply > 0 {
            state.total_borrowed.saturating_mul(100) / state.total_supply
        } else if state.total_borrowed == 0 {
            0
        } else {
            u128::MAX // Borrowed without supply = infinite utilization
        };

        let computation = InvariantComputation::new("total_borrowed <= total_supply")
            .with_input("total_supply", state.total_supply)
            .with_input("total_borrowed", state.total_borrowed)
            .with_input("available_liquidity", available_liquidity)
            .with_input("utilization_percent", utilization_percent)
            .with_result(format!(
                "{} {} {} ({}% utilization)",
                state.total_borrowed,
                if is_valid { "<=" } else { ">" },
                state.total_supply,
                utilization_percent
            ));

        if is_valid {
            InvariantResult::ok(self.id(), self.name(), self.description(), computation)
        } else {
            InvariantResult::violated(
                self.id(),
                self.name(),
                self.description(),
                computation,
                &format!(
                    "Over-borrowed: {} > {} (negative liquidity)",
                    state.total_borrowed, state.total_supply
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidity_ok() {
        let state = ProtocolState {
            total_supply: 1_000_000,
            total_borrowed: 800_000, // 80% utilization
            ..Default::default()
        };

        let inv = LiquidityConstraint;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }

    #[test]
    fn test_over_borrowed() {
        let state = ProtocolState {
            total_supply: 1_000_000,
            total_borrowed: 1_100_000, // More borrowed than supplied!
            ..Default::default()
        };

        let inv = LiquidityConstraint;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Violated);
    }

    #[test]
    fn test_empty_pool() {
        let state = ProtocolState {
            total_supply: 0,
            total_borrowed: 0,
            ..Default::default()
        };

        let inv = LiquidityConstraint;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }
}
