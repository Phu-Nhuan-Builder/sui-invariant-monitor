use crate::aggregator::ProtocolState;
use crate::invariants::definitions::{all_invariants, Invariant};
use crate::invariants::types::{InvariantResult, InvariantStatus};

/// Engine that evaluates all invariants against protocol state
pub struct InvariantEngine {
    invariants: Vec<Box<dyn Invariant + Send + Sync>>,
    previous_state: Option<ProtocolState>,
}

impl InvariantEngine {
    pub fn new() -> Self {
        Self {
            invariants: Vec::new(),  // Start with empty list, invariants added via API
            previous_state: None,
        }
    }

    /// Evaluate all invariants against the current state
    pub fn evaluate_all(&mut self, state: &ProtocolState) -> Vec<InvariantResult> {
        let results: Vec<InvariantResult> = self
            .invariants
            .iter()
            .map(|inv| inv.evaluate(state, self.previous_state.as_ref()))
            .collect();

        // Store current state as previous for next evaluation
        self.previous_state = Some(state.clone());

        results
    }

    /// Get count of current violations
    pub fn violation_count(results: &[InvariantResult]) -> usize {
        results
            .iter()
            .filter(|r| r.status == InvariantStatus::Violated)
            .count()
    }

    /// Get count of errors
    pub fn error_count(results: &[InvariantResult]) -> usize {
        results
            .iter()
            .filter(|r| r.status == InvariantStatus::Error)
            .count()
    }

    /// Check if all invariants pass
    pub fn all_ok(results: &[InvariantResult]) -> bool {
        results.iter().all(|r| r.status == InvariantStatus::Ok)
    }

    /// Get only violated invariants
    pub fn get_violations(results: &[InvariantResult]) -> Vec<&InvariantResult> {
        results
            .iter()
            .filter(|r| r.status == InvariantStatus::Violated)
            .collect()
    }
}

impl Default for InvariantEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_evaluates_all() {
        let mut engine = InvariantEngine::new();
        let state = ProtocolState::default();
        
        let results = engine.evaluate_all(&state);
        
        // Should have 5 invariants
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_violation_detection() {
        let mut engine = InvariantEngine::new();
        
        // Create a state that violates collateralization
        let state = ProtocolState {
            total_supply: 1000,
            total_reserves: 400,
            total_borrowed: 600,
            collateral_value: 500, // 83% < 150% required
            on_chain_balance: 400,
            ..Default::default()
        };
        
        let results = engine.evaluate_all(&state);
        
        assert!(InvariantEngine::violation_count(&results) > 0);
    }
}
