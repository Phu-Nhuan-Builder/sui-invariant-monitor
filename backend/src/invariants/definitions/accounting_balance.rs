use crate::aggregator::ProtocolState;
use crate::invariants::types::{InvariantComputation, InvariantResult};
use super::Invariant;

/// INV-003: Accounting Balance Integrity
/// 
/// Internal accounting must match on-chain token balance.
/// The reserves tracked internally should equal the actual tokens held.
pub struct AccountingBalanceIntegrity;

impl Invariant for AccountingBalanceIntegrity {
    fn id(&self) -> &'static str {
        "INV-003"
    }

    fn name(&self) -> &'static str {
        "Accounting Balance Integrity"
    }

    fn description(&self) -> &'static str {
        "Internal balance matches on-chain token balance"
    }

    fn evaluate(&self, state: &ProtocolState, _previous: Option<&ProtocolState>) -> InvariantResult {
        let internal_balance = state.total_reserves;
        let on_chain_balance = state.on_chain_balance;

        let matches = internal_balance == on_chain_balance;

        let computation = InvariantComputation::new("internal_balance == on_chain_balance")
            .with_input("internal_balance", internal_balance)
            .with_input("on_chain_balance", on_chain_balance)
            .with_input("difference", 
                if internal_balance > on_chain_balance {
                    internal_balance - on_chain_balance
                } else {
                    on_chain_balance - internal_balance
                }
            )
            .with_result(format!(
                "{} {} {}",
                internal_balance,
                if matches { "==" } else { "!=" },
                on_chain_balance
            ));

        if matches {
            InvariantResult::ok(self.id(), self.name(), self.description(), computation)
        } else {
            InvariantResult::violated(
                self.id(),
                self.name(),
                self.description(),
                computation,
                &format!(
                    "Balance mismatch: internal {} != on-chain {}",
                    internal_balance, on_chain_balance
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounting_match() {
        let state = ProtocolState {
            total_reserves: 1_000_000,
            on_chain_balance: 1_000_000,
            ..Default::default()
        };

        let inv = AccountingBalanceIntegrity;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Ok);
    }

    #[test]
    fn test_accounting_mismatch() {
        let state = ProtocolState {
            total_reserves: 1_000_000,
            on_chain_balance: 900_000, // Missing 100k!
            ..Default::default()
        };

        let inv = AccountingBalanceIntegrity;
        let result = inv.evaluate(&state, None);
        assert_eq!(result.status, crate::invariants::types::InvariantStatus::Violated);
    }
}
