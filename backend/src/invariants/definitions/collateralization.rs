use crate::aggregator::ProtocolState;
use crate::invariants::types::{InvariantComputation, InvariantResult};
use super::Invariant;

/// INV-002: Collateralization Ratio
/// 
/// Outstanding borrows must always be over-collateralized.
/// Minimum collateral ratio is 150% (configurable).
pub struct CollateralizationRatio;

/// Minimum collateral ratio as percentage (150 = 150%)
const MIN_COLLATERAL_RATIO: u128 = 150;

impl Invariant for CollateralizationRatio {
    fn id(&self) -> &'static str {
        "INV-002"
    }

    fn name(&self) -> &'static str {
        "Collateralization Ratio"
    }

    fn description(&self) -> &'static str {
        "Outstanding borrows must be over-collateralized (minimum 150%)"
    }

    fn evaluate(&self, state: &ProtocolState, _previous: Option<&ProtocolState>) -> InvariantResult {
        // If no borrows, invariant is trivially satisfied
        if state.total_borrowed == 0 {
            let computation = InvariantComputation::new("collateral_value >= total_borrowed * 1.5")
                .with_input("collateral_value", state.collateral_value)
                .with_input("total_borrowed", 0)
                .with_input("required_collateral", 0)
                .with_result("No borrows outstanding - OK");

            return InvariantResult::ok(self.id(), self.name(), self.description(), computation);
        }

        let required_collateral = state.total_borrowed
            .saturating_mul(MIN_COLLATERAL_RATIO)
            .saturating_div(100);

        let is_valid = state.collateral_value >= required_collateral;

        let current_ratio = if state.total_borrowed > 0 {
            state.collateral_value.saturating_mul(100) / state.total_borrowed
        } else {
            u128::MAX
        };

        let computation = InvariantComputation::new("collateral_value >= total_borrowed * 1.5")
            .with_input("collateral_value", state.collateral_value)
            .with_input("total_borrowed", state.total_borrowed)
            .with_input("required_collateral", required_collateral)
            .with_input("current_ratio_percent", current_ratio)
            .with_input("min_ratio_percent", MIN_COLLATERAL_RATIO)
            .with_result(format!(
                "{} {} {} ({}% {} {}%)",
                state.collateral_value,
                if is_valid { ">=" } else { "<" },
                required_collateral,
                current_ratio,
                if is_valid { ">=" } else { "<" },
                MIN_COLLATERAL_RATIO
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
                    "Under-collateralized: {}% < {}% minimum",
                    current_ratio, MIN_COLLATERAL_RATIO
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collateralization_ok() {
        let state = ProtocolState {
            total_borrowed: 1_000_000,
            collateral_value: 1_600_000, // 160% > 150%
            ..Default::default()
        };

        let inv = CollateralizationRatio;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }

    #[test]
    fn test_collateralization_violated() {
        let state = ProtocolState {
            total_borrowed: 1_000_000,
            collateral_value: 1_400_000, // 140% < 150%
            ..Default::default()
        };

        let inv = CollateralizationRatio;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Violated);
    }

    #[test]
    fn test_no_borrows() {
        let state = ProtocolState {
            total_borrowed: 0,
            collateral_value: 0,
            ..Default::default()
        };

        let inv = CollateralizationRatio;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }
}
