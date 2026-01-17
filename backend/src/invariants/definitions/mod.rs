pub mod total_supply;
pub mod collateralization;
pub mod accounting_balance;
pub mod interest_monotonicity;
pub mod liquidity_constraint;

use crate::aggregator::ProtocolState;
use crate::invariants::types::InvariantResult;

/// Trait for all invariant definitions
pub trait Invariant {
    /// Unique identifier for this invariant
    fn id(&self) -> &'static str;
    
    /// Human-readable name
    fn name(&self) -> &'static str;
    
    /// Description of what this invariant checks
    fn description(&self) -> &'static str;
    
    /// Evaluate the invariant against the current state
    fn evaluate(&self, state: &ProtocolState, previous: Option<&ProtocolState>) -> InvariantResult;
}

/// All defined invariants
pub fn all_invariants() -> Vec<Box<dyn Invariant + Send + Sync>> {
    vec![
        Box::new(total_supply::TotalSupplyConservation),
        Box::new(collateralization::CollateralizationRatio),
        Box::new(accounting_balance::AccountingBalanceIntegrity),
        Box::new(interest_monotonicity::InterestMonotonicity),
        Box::new(liquidity_constraint::LiquidityConstraint),
    ]
}
