use crate::{
    EffectDescriptor, ReplayPolicy, Tool, ToolContext, ToolExecutionError, ToolId, ToolIdentity,
    ToolInvocationOutcome, ToolOutput, ToolRequest, ToolSpec, ToolVersion,
};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;

/// Test-only observable effect seam used by `test.unsafe_once`.
///
/// The fixture is deterministic and exposes no filesystem, process, network,
/// storage, runtime, Agent, Flow, or Orchestration capability.
pub trait EffectProbe: Send + Sync {
    fn observe(&self, key: &str);
}

/// Pure, replay-safe deterministic fixture.
#[derive(Clone, Copy, Debug, Default)]
pub struct EchoTool;

impl EchoTool {
    pub fn identity() -> ToolIdentity {
        fixture_identity("test.echo")
    }
}

#[async_trait]
impl Tool for EchoTool {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            identity: Self::identity(),
            description: "Return the input value.".to_owned(),
            input_schema: json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": { "value": {} },
                "required": ["value"],
                "additionalProperties": false
            }),
        }
    }

    fn effect(&self, _arguments: &Value) -> Result<EffectDescriptor, ToolExecutionError> {
        Ok(EffectDescriptor::Pure {
            kind: "test.echo".to_owned(),
        })
    }

    fn replay_policy(&self, _arguments: &Value) -> ReplayPolicy {
        ReplayPolicy::Safe
    }

    async fn execute(&self, request: ToolRequest, context: ToolContext) -> ToolInvocationOutcome {
        if context.cancellation.is_cancelled() {
            return ToolInvocationOutcome::Cancelled;
        }
        let Some(value) = request.arguments.get("value") else {
            return ToolInvocationOutcome::OrdinaryError(ToolExecutionError::new(
                "invalid_arguments",
                "test.echo requires value",
            ));
        };
        ToolInvocationOutcome::Output(ToolOutput {
            content: json!({ "value": value.clone() }),
        })
    }
}

/// Synthetic fixture whose direct execution is observable and never replay-safe.
pub struct UnsafeOnceTool {
    probe: Arc<dyn EffectProbe>,
}

impl UnsafeOnceTool {
    pub fn new(probe: Arc<dyn EffectProbe>) -> Self {
        Self { probe }
    }

    pub fn identity() -> ToolIdentity {
        fixture_identity("test.unsafe_once")
    }
}

#[async_trait]
impl Tool for UnsafeOnceTool {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            identity: Self::identity(),
            description: "Apply one synthetic observable effect for a key.".to_owned(),
            input_schema: json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "key": { "type": "string", "minLength": 1 }
                },
                "required": ["key"],
                "additionalProperties": false
            }),
        }
    }

    fn effect(&self, arguments: &Value) -> Result<EffectDescriptor, ToolExecutionError> {
        let key = required_key(arguments)?;
        Ok(EffectDescriptor::Synthetic {
            kind: "test.unsafe_once".to_owned(),
            attributes: json!({ "key": key }),
        })
    }

    fn replay_policy(&self, _arguments: &Value) -> ReplayPolicy {
        ReplayPolicy::Never
    }

    async fn execute(&self, request: ToolRequest, context: ToolContext) -> ToolInvocationOutcome {
        if context.cancellation.is_cancelled() {
            return ToolInvocationOutcome::Cancelled;
        }
        let key = match required_key(&request.arguments) {
            Ok(key) => key,
            Err(error) => return ToolInvocationOutcome::OrdinaryError(error),
        };
        self.probe.observe(key);
        ToolInvocationOutcome::Output(ToolOutput {
            content: json!({ "applied": true, "key": key }),
        })
    }
}

fn fixture_identity(id: &str) -> ToolIdentity {
    ToolIdentity {
        id: ToolId::new(id).expect("fixture tool id is non-empty"),
        version: ToolVersion::new("1").expect("fixture tool version is non-empty"),
        implementation_digest: None,
    }
}

fn required_key(arguments: &Value) -> Result<&str, ToolExecutionError> {
    arguments
        .get("key")
        .and_then(Value::as_str)
        .filter(|key| !key.is_empty())
        .ok_or_else(|| ToolExecutionError::new("invalid_arguments", "test.unsafe_once requires non-empty key"))
}
