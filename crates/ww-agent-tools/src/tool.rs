use crate::ToolIdentity;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// What a tool does when it runs. Classification happens after validation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EffectDescriptor {
    Pure { kind: String },
    Synthetic { kind: String, attributes: Value },
}

/// Whether an interrupted attempt may be retried.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPolicy {
    Safe,
    Never,
}

/// Arguments handed to one tool invocation.
///
/// This type deliberately carries no Agent run, logical-call, attempt, or
/// entry identity. Those belong to `ww-agent-core`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolRequest {
    pub identity: ToolIdentity,
    pub arguments: Value,
}

/// Ambient execution context.
#[derive(Clone, Debug)]
pub struct ToolContext {
    pub cancellation: CancellationToken,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolOutput {
    pub content: Value,
}

/// An ordinary tool failure. It never encodes cancellation.
#[derive(Clone, Debug, Eq, Error, PartialEq, Serialize, Deserialize)]
#[error("{code}: {message}")]
pub struct ToolExecutionError {
    pub code: String,
    pub message: String,
}

impl ToolExecutionError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

/// The three normal outcomes of one tool invocation.
///
/// Cancellation is a control outcome and is kept distinct from an ordinary
/// error. A panic or broken invariant is not represented here at all.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolInvocationOutcome {
    Output(ToolOutput),
    OrdinaryError(ToolExecutionError),
    Cancelled,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn spec(&self) -> crate::ToolSpec;

    fn effect(&self, arguments: &Value) -> Result<EffectDescriptor, ToolExecutionError>;

    fn replay_policy(&self, arguments: &Value) -> ReplayPolicy;

    async fn execute(&self, request: ToolRequest, context: ToolContext) -> ToolInvocationOutcome;
}
