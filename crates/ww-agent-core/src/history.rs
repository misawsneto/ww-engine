use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;
use ww_agent_provider::{CompletionReason, ModelUsage, ToolCallId};
use ww_agent_tools::{ToolPreparationDisposition, ToolPreparationStage};

macro_rules! uuid_id {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

uuid_id!(AgentRunId);
uuid_id!(AgentEntryId);
uuid_id!(ModelAttemptId);
uuid_id!(ToolAttemptId);
uuid_id!(LogicalToolCallId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentToolCall {
    pub logical_id: LogicalToolCallId,
    pub provider_call_id: ToolCallId,
    pub name: String,
    pub arguments_json: String,
    pub arguments: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentAssistantContent {
    Text { text: String },
    ToolCall { call: AgentToolCall },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DurableAssistantMessage {
    pub content: Vec<AgentAssistantContent>,
    pub stop_reason: CompletionReason,
    pub usage: Option<ModelUsage>,
    pub provider_request_id: Option<String>,
}

impl DurableAssistantMessage {
    pub fn tool_calls(&self) -> impl Iterator<Item = &AgentToolCall> {
        self.content.iter().filter_map(|content| match content {
            AgentAssistantContent::ToolCall { call } => Some(call),
            AgentAssistantContent::Text { .. } => None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentEntryData {
    UserInput {
        text: String,
    },
    AssistantMessage {
        attempt_id: ModelAttemptId,
        message: DurableAssistantMessage,
    },
    ModelVisibleToolResult {
        logical_call_id: LogicalToolCallId,
        attempt_id: ToolAttemptId,
        tool_name: String,
        content: Value,
        is_error: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentEntry {
    pub id: AgentEntryId,
    pub run_id: AgentRunId,
    pub ordinal: u64,
    pub created_at: DateTime<Utc>,
    pub data: AgentEntryData,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ModelAttemptInterruptReason {
    ProviderDisconnected,
    ProviderFailed { code: String },
    Cancelled,
    RuntimeRestart,
    Other { code: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AgentTerminalResult {
    Succeeded { assistant_entry_id: AgentEntryId },
    Failed { code: String, message: String },
    Cancelled,
    TimedOut,
    BudgetExhausted { limit: String },
    RequiresIntervention { reason: String },
}

/// The normalized outcome of one tool effect, before any model-visible entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ToolEffectResult {
    Output { content: Value },
    Error { code: String, message: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentRecordData {
    ModelAttemptStarted {
        attempt_id: ModelAttemptId,
        request_ordinal: u64,
    },
    ModelAttemptInterrupted {
        attempt_id: ModelAttemptId,
        reason: ModelAttemptInterruptReason,
    },
    ModelAttemptCompleted {
        attempt_id: ModelAttemptId,
        assistant_entry_id: AgentEntryId,
    },
    ToolAttemptStarted {
        attempt_id: ToolAttemptId,
        logical_call_id: LogicalToolCallId,
    },
    /// Agent-owned record embedding the tools-owned preparation taxonomy.
    ToolCallPrepared {
        attempt_id: ToolAttemptId,
        logical_call_id: LogicalToolCallId,
        assistant_entry_id: AgentEntryId,
        source_index: u32,
        provider_call_id: ToolCallId,
        requested_tool_name: String,
        result_entry_id: AgentEntryId,
        /// Boxed so one large preparation does not widen every record variant.
        disposition: Box<ToolPreparationDisposition>,
    },
    /// The ambiguity boundary. It never proves the external effect occurred.
    ToolEffectStarted {
        attempt_id: ToolAttemptId,
    },
    ToolEffectCompleted {
        attempt_id: ToolAttemptId,
        result: ToolEffectResult,
    },
    /// Resolve, Validate, or Classify no-effect settlement. Policy uses
    /// `ToolAttemptDenied`, which carries no duplicate stage field.
    ToolAttemptRejected {
        attempt_id: ToolAttemptId,
        result_entry_id: AgentEntryId,
        failed_at: ToolPreparationStage,
    },
    ToolAttemptInterrupted {
        attempt_id: ToolAttemptId,
        reason: String,
    },
    ToolAttemptDenied {
        attempt_id: ToolAttemptId,
        result_entry_id: AgentEntryId,
    },
    ToolAttemptCompleted {
        attempt_id: ToolAttemptId,
        result_entry_id: AgentEntryId,
    },
    ToolAttemptIntervention {
        attempt_id: ToolAttemptId,
        reason: String,
    },
    TurnCommitted {
        turn_ordinal: u64,
        assistant_entry_id: AgentEntryId,
        tool_result_entry_ids: Vec<AgentEntryId>,
    },
    AgentResultCommitted {
        result: AgentTerminalResult,
    },
}

impl AgentRecordData {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::ModelAttemptStarted { .. } => "model_attempt_started",
            Self::ModelAttemptInterrupted { .. } => "model_attempt_interrupted",
            Self::ModelAttemptCompleted { .. } => "model_attempt_completed",
            Self::ToolAttemptStarted { .. } => "tool_attempt_started",
            Self::ToolCallPrepared { .. } => "tool_call_prepared",
            Self::ToolEffectStarted { .. } => "tool_effect_started",
            Self::ToolEffectCompleted { .. } => "tool_effect_completed",
            Self::ToolAttemptRejected { .. } => "tool_attempt_rejected",
            Self::ToolAttemptInterrupted { .. } => "tool_attempt_interrupted",
            Self::ToolAttemptDenied { .. } => "tool_attempt_denied",
            Self::ToolAttemptCompleted { .. } => "tool_attempt_completed",
            Self::ToolAttemptIntervention { .. } => "tool_attempt_intervention",
            Self::TurnCommitted { .. } => "turn_committed",
            Self::AgentResultCommitted { .. } => "agent_result_committed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRecord {
    pub run_id: AgentRunId,
    pub sequence: u64,
    pub recorded_at: DateTime<Utc>,
    pub data: AgentRecordData,
}
