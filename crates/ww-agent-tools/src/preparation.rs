use crate::{
    EffectDescriptor, PolicyDecision, ReplayPolicy, Tool, ToolIdentity, ToolPolicy,
    ToolPolicyInput, ToolRegistry, arguments_digest,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// The preparation stage that failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolPreparationStage {
    Resolve,
    Validate,
    Classify,
    Policy,
}

/// The outcome of preparing one finalized model tool call.
///
/// This is the single public preparation taxonomy. `ww-agent-core` embeds
/// these exact values in its durable records and defines no equivalent.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum ToolPreparationDisposition {
    Executable {
        identity: ToolIdentity,
        arguments_digest: String,
        effect: EffectDescriptor,
        replay: ReplayPolicy,
        /// Always `Allow` in this variant.
        policy: PolicyDecision,
    },
    NoEffect {
        failed_at: ToolPreparationStage,
        code: String,
        message: String,
        /// Fields stay `None` when the failing stage ran before them.
        identity: Option<ToolIdentity>,
        arguments_digest: Option<String>,
        effect: Option<EffectDescriptor>,
        replay: Option<ReplayPolicy>,
        policy: Option<PolicyDecision>,
    },
}

/// A prepared call plus, when executable, the resolved executor.
///
/// The executor is not part of the durable taxonomy and is not serializable.
pub struct PreparedToolCall {
    disposition: ToolPreparationDisposition,
    executor: Option<Arc<dyn Tool>>,
}

impl PreparedToolCall {
    pub fn disposition(&self) -> &ToolPreparationDisposition {
        &self.disposition
    }

    pub fn into_disposition(self) -> ToolPreparationDisposition {
        self.disposition
    }

    pub fn executor(&self) -> Option<&Arc<dyn Tool>> {
        self.executor.as_ref()
    }
}

/// Prepare one finalized model tool call.
///
/// The stages run in exactly this order, and a failure short-circuits:
///
/// ```text
/// resolve → validate → digest → effect → replay → policy
/// ```
///
/// Preparation performs no external effect and owns no Agent identity.
pub fn prepare_tool_call(
    registry: &ToolRegistry,
    pins: &[ToolIdentity],
    policy: &dyn ToolPolicy,
    requested_tool_name: &str,
    arguments: &Value,
) -> PreparedToolCall {
    // Resolve — only a configured pin is addressable, at its exact version.
    let Some(pin) = pins
        .iter()
        .find(|pin| pin.id.as_str() == requested_tool_name)
    else {
        return no_effect(
            ToolPreparationStage::Resolve,
            "tool_not_found",
            format!("no configured tool named {requested_tool_name}"),
            None,
        );
    };
    let registered = match registry.resolve(pin) {
        Ok(registered) => registered,
        Err(error) => {
            return no_effect(
                ToolPreparationStage::Resolve,
                "tool_not_found",
                error.to_string(),
                None,
            );
        }
    };
    let identity = registered.spec().identity.clone();

    // Validate — the authoritative parsed value, never a reparse.
    if let Err(error) = registered.schema().validate(arguments) {
        return no_effect(
            ToolPreparationStage::Validate,
            "invalid_arguments",
            error.to_string(),
            Some(identity),
        );
    }

    // Digest — derived only from arguments that already validated.
    let digest = arguments_digest(arguments);

    // Classify — effect first, then replay.
    let effect = match registered.tool().effect(arguments) {
        Ok(effect) => effect,
        Err(error) => {
            return ToolPreparationDisposition::NoEffect {
                failed_at: ToolPreparationStage::Classify,
                code: "classification_failed".to_owned(),
                message: error.to_string(),
                identity: Some(identity),
                arguments_digest: Some(digest),
                effect: None,
                replay: None,
                policy: None,
            }
            .into();
        }
    };
    let replay = registered.tool().replay_policy(arguments);

    // Policy — exactly once, and only with the classified values.
    let decision = policy.evaluate(&ToolPolicyInput {
        identity: &identity,
        arguments,
        arguments_digest: &digest,
        effect: &effect,
        replay,
    });

    match decision {
        PolicyDecision::Allow => PreparedToolCall {
            disposition: ToolPreparationDisposition::Executable {
                identity,
                arguments_digest: digest,
                effect,
                replay,
                policy: PolicyDecision::Allow,
            },
            executor: Some(Arc::clone(registered.tool())),
        },
        PolicyDecision::Deny { code, message } => ToolPreparationDisposition::NoEffect {
            failed_at: ToolPreparationStage::Policy,
            code: "policy_denied".to_owned(),
            message: message.clone(),
            identity: Some(identity),
            arguments_digest: Some(digest),
            effect: Some(effect),
            replay: Some(replay),
            policy: Some(PolicyDecision::Deny { code, message }),
        }
        .into(),
    }
}

impl From<ToolPreparationDisposition> for PreparedToolCall {
    fn from(disposition: ToolPreparationDisposition) -> Self {
        Self {
            disposition,
            executor: None,
        }
    }
}

fn no_effect(
    failed_at: ToolPreparationStage,
    code: &str,
    message: String,
    identity: Option<ToolIdentity>,
) -> PreparedToolCall {
    ToolPreparationDisposition::NoEffect {
        failed_at,
        code: code.to_owned(),
        message,
        identity,
        arguments_digest: None,
        effect: None,
        replay: None,
        policy: None,
    }
    .into()
}
