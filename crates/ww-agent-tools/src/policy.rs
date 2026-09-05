use crate::{EffectDescriptor, ReplayPolicy, ToolIdentity};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The decision a policy returns for one prepared call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum PolicyDecision {
    Allow,
    Deny { code: String, message: String },
}

/// What a policy may inspect.
///
/// `effect` and `replay` are non-optional, so a caller cannot construct this
/// input before classification has run.
#[derive(Clone, Debug)]
pub struct ToolPolicyInput<'a> {
    pub identity: &'a ToolIdentity,
    pub arguments: &'a Value,
    pub arguments_digest: &'a str,
    pub effect: &'a EffectDescriptor,
    pub replay: ReplayPolicy,
}

pub trait ToolPolicy: Send + Sync {
    fn evaluate(&self, input: &ToolPolicyInput<'_>) -> PolicyDecision;
}
