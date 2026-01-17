use crate::aggregator::ProtocolState;
use crate::invariants::types::{InvariantComputation, InvariantResult};
use super::Invariant;

/// INV-004: Interest Monotonicity
/// 
/// Interest index must never decrease.
/// This ensures interest accrual is always non-negative.
pub struct InterestMonotonicity;

impl Invariant for InterestMonotonicity {
    fn id(&self) -> &'static str {
        "INV-004"
    }

    fn name(&self) -> &'static str {
        "Interest Index Monotonicity"
    }

    fn description(&self) -> &'static str {
        "Interest index must never decrease over time"
    }

    fn evaluate(&self, state: &ProtocolState, previous: Option<&ProtocolState>) -> InvariantResult {
        let current_index = state.interest_index;

        // If no previous state, we can't check monotonicity
        let Some(prev) = previous else {
            let computation = InvariantComputation::new("current_index >= previous_index")
                .with_input("current_index", current_index)
                .with_input("previous_index", "N/A (first check)")
                .with_result("First evaluation - no previous state to compare");

            return InvariantResult::ok(self.id(), self.name(), self.description(), computation);
        };

        let previous_index = prev.interest_index;
        let is_monotonic = current_index >= previous_index;

        let computation = InvariantComputation::new("current_index >= previous_index")
            .with_input("current_index", current_index)
            .with_input("previous_index", previous_index)
            .with_input("delta", 
                if current_index >= previous_index {
                    current_index - previous_index
                } else {
                    0
                }
            )
            .with_result(format!(
                "{} {} {}",
                current_index,
                if is_monotonic { ">=" } else { "<" },
                previous_index
            ));

        if is_monotonic {
            InvariantResult::ok(self.id(), self.name(), self.description(), computation)
        } else {
            InvariantResult::violated(
                self.id(),
                self.name(),
                self.description(),
                computation,
                &format!(
                    "Interest index decreased: {} -> {} (delta: {})",
                    previous_index,
                    current_index,
                    previous_index - current_index
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_increasing() {
        let prev = ProtocolState {
            interest_index: 1_000_000_000, // 1.0
            ..Default::default()
        };
        let current = ProtocolState {
            interest_index: 1_010_000_000, // 1.01
            ..Default::default()
        };

        let inv = InterestMonotonicity;
        let result = inv.evaluate(&current, Some(&prev));
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }

    #[test]
    fn test_interest_decreasing() {
        let prev = ProtocolState {
            interest_index: 1_010_000_000, // 1.01
            ..Default::default()
        };
        let current = ProtocolState {
            interest_index: 1_000_000_000, // 1.0 - decreased!
            ..Default::default()
        };

        let inv = InterestMonotonicity;
        let result = inv.evaluate(&current, Some(&prev));
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Violated);
    }

    #[test]
    fn test_no_previous_state() {
        let current = ProtocolState {
            interest_index: 1_000_000_000,
            ..Default::default()
        };

        let inv = InterestMonotonicity;
        let result = inv.evaluate(&current, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }
}
