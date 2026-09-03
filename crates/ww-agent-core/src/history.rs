use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use thiserror::Error;
use uuid::Uuid;
use ww_agent_provider::{AssistantMessage, CompletionReason, MessageContent, ModelUsage};

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
    pub provider_call_id: String,
    pub name: String,
    pub arguments: Value,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCompletionReason {
    Stop,
    ToolUse,
    Length,
}

impl From<CompletionReason> for AgentCompletionReason {
    fn from(value: CompletionReason) -> Self {
        match value {
            CompletionReason::Stop => Self::Stop,
            CompletionReason::ToolUse => Self::ToolUse,
            CompletionReason::Length => Self::Length,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_input_tokens: u64,
    pub cache_write_input_tokens: u64,
}

impl AgentUsage {
    pub const fn total_tokens(self) -> u64 {
        self.input_tokens.saturating_add(self.output_tokens)
    }
}

impl From<ModelUsage> for AgentUsage {
    fn from(value: ModelUsage) -> Self {
        Self {
            input_tokens: value.input_tokens,
            output_tokens: value.output_tokens,
            cache_read_input_tokens: value.cache_read_input_tokens,
            cache_write_input_tokens: value.cache_write_input_tokens,
        }
    }
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
    pub stop_reason: AgentCompletionReason,
    pub usage: Option<AgentUsage>,
    pub provider_request_id: Option<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum DurableMessageConversionError {
    #[error("provider assistant message contains a tool result")]
    ToolResultInAssistantMessage,
}

impl TryFrom<AssistantMessage> for DurableAssistantMessage {
    type Error = DurableMessageConversionError;

    fn try_from(message: AssistantMessage) -> Result<Self, Self::Error> {
        let mut content = Vec::with_capacity(message.content.len());
        for item in message.content {
            match item {
                MessageContent::Text { text } => {
                    content.push(AgentAssistantContent::Text { text });
                }
                MessageContent::ToolCall { call } => {
                    content.push(AgentAssistantContent::ToolCall {
                        call: AgentToolCall {
                            logical_id: LogicalToolCallId::new(),
                            provider_call_id: call.id.to_string(),
                            name: call.name,
                            arguments: call.arguments,
                        },
                    });
                }
                MessageContent::ToolResult { .. } => {
                    return Err(DurableMessageConversionError::ToolResultInAssistantMessage);
                }
            }
        }
        Ok(Self {
            content,
            stop_reason: message.stop_reason.into(),
            usage: message.usage.map(Into::into),
            provider_request_id: message.provider_request_id,
        })
    }
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
