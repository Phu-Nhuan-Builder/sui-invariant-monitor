use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of evaluating an invariant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantResult {
    /// Unique identifier for this invariant
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what this invariant checks
    pub description: String,
    /// Current status
    pub status: InvariantStatus,
    /// When this invariant was last evaluated
    pub evaluated_at: DateTime<Utc>,
    /// Details of the computation
    pub computation: InvariantComputation,
    /// Reason for violation, if any
    pub violation_reason: Option<String>,
}

/// Status of an invariant check
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InvariantStatus {
    /// Invariant holds
    Ok,
    /// Invariant is violated
    Violated,
    /// Error occurred during evaluation
    Error,
}

/// Details of the computation performed for an invariant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantComputation {
    /// Input values used in the computation
    pub inputs: HashMap<String, String>,
    /// Human-readable formula
    pub formula: String,
    /// Computed result
    pub result: String,
}

impl InvariantResult {
    pub fn ok(
        id: &str,
        name: &str,
        description: &str,
        computation: InvariantComputation,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: InvariantStatus::Ok,
            evaluated_at: Utc::now(),
            computation,
            violation_reason: None,
        }
    }

    pub fn violated(
        id: &str,
        name: &str,
        description: &str,
        computation: InvariantComputation,
        reason: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: InvariantStatus::Violated,
            evaluated_at: Utc::now(),
            computation,
            violation_reason: Some(reason.to_string()),
        }
    }

    pub fn error(id: &str, name: &str, description: &str, error_msg: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: InvariantStatus::Error,
            evaluated_at: Utc::now(),
            computation: InvariantComputation {
                inputs: HashMap::new(),
                formula: String::new(),
                result: format!("Error: {}", error_msg),
            },
            violation_reason: Some(error_msg.to_string()),
        }
    }
}

impl InvariantComputation {
    pub fn new(formula: &str) -> Self {
        Self {
            inputs: HashMap::new(),
            formula: formula.to_string(),
            result: String::new(),
        }
    }

    pub fn with_input(mut self, key: &str, value: impl ToString) -> Self {
        self.inputs.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_result(mut self, result: impl ToString) -> Self {
        self.result = result.to_string();
        self
    }
}
