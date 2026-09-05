//! T007 work unit 4 — tool execution outcome contract.
//!
//! `ToolInvocationOutcome` represents exactly the three normal tool outcomes.
//! Panic/impossible invariant failure is deliberately outside this enum.

use serde_json::json;
use ww_agent_tools::{ToolExecutionError, ToolInvocationOutcome, ToolOutput};

fn normal_outcome_kind(outcome: &ToolInvocationOutcome) -> &'static str {
    match outcome {
        ToolInvocationOutcome::Output(_) => "output",
        ToolInvocationOutcome::OrdinaryError(_) => "ordinary_error",
        ToolInvocationOutcome::Cancelled => "cancelled",
    }
}

// V-T007-36 — the exhaustive match makes the three normal outcomes
// machine-distinguishable. There is no panic/invariant variant to normalize
// contract violations into a normal result.
#[test]
fn execution_outcome_contract_is_output_error_or_cancelled() {
    let output = ToolInvocationOutcome::Output(ToolOutput {
        content: json!({"ok": true}),
    });
    let error = ToolInvocationOutcome::OrdinaryError(ToolExecutionError::new(
        "fixture_error",
        "ordinary tool failure",
    ));
    let cancelled = ToolInvocationOutcome::Cancelled;

    assert_eq!(normal_outcome_kind(&output), "output");
    assert_eq!(normal_outcome_kind(&error), "ordinary_error");
    assert_eq!(normal_outcome_kind(&cancelled), "cancelled");
    assert_ne!(normal_outcome_kind(&error), normal_outcome_kind(&cancelled));
}
