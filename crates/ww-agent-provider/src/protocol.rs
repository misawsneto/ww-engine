use async_trait::async_trait;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt, pin::Pin};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

macro_rules! string_id {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(concat!($label, " must not be empty"));
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_id!(ProviderId, "provider id");
string_id!(ModelId, "model id");
string_id!(ToolCallId, "tool call id");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub tool_calls: bool,
    pub usage: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    Tool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    Text { text: String },
    ToolCall { call: ToolCall },
    ToolResult {
        call_id: ToolCallId,
        tool_name: String,
        content: Value,
        is_error: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelMessage {
    pub role: MessageRole,
    pub content: Vec<MessageContent>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelToolSpec {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelRequest {
    pub model: ModelId,
    pub system_prompt: Option<String>,
    pub messages: Vec<ModelMessage>,
    pub tools: Vec<ModelToolSpec>,
}

#[derive(Clone, Debug)]
pub struct ProviderContext {
    pub cancellation: CancellationToken,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderStarted {
    pub request_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionReason {
    Stop,
    ToolUse,
    Length,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModelUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_input_tokens: u64,
    pub cache_write_input_tokens: u64,
}

impl ModelUsage {
    pub const fn total_tokens(self) -> u64 {
        self.input_tokens.saturating_add(self.output_tokens)
    }

    pub(crate) const fn dominates(self, previous: Self) -> bool {
        self.input_tokens >= previous.input_tokens
            && self.output_tokens >= previous.output_tokens
            && self.cache_read_input_tokens >= previous.cache_read_input_tokens
            && self.cache_write_input_tokens >= previous.cache_write_input_tokens
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: ToolCallId,
    pub name: String,
    pub arguments_json: String,
    pub arguments: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantMessage {
    pub content: Vec<MessageContent>,
    pub stop_reason: CompletionReason,
    pub usage: Option<ModelUsage>,
    pub provider_request_id: Option<String>,
}

impl AssistantMessage {
    pub fn tool_calls(&self) -> impl Iterator<Item = &ToolCall> {
        self.content.iter().filter_map(|content| match content {
            MessageContent::ToolCall { call } => Some(call),
            _ => None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderFailure {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl ProviderFailure {
    pub fn new(code: impl Into<String>, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ModelResponse {
    Completed { message: AssistantMessage },
    Failed { failure: ProviderFailure },
    Aborted { message: Option<String> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ModelEvent {
    Started { started: ProviderStarted },
    TextDelta { delta: String },
    ToolCallStarted { id: ToolCallId, name: String },
    ToolCallArgumentsDelta { id: ToolCallId, delta: String },
    ToolCallCompleted { id: ToolCallId },
    Usage { usage: ModelUsage },
    Completed { reason: CompletionReason },
    Failed { failure: ProviderFailure },
    Aborted { message: Option<String> },
}

pub type ModelEventStream = Pin<Box<dyn Stream<Item = Result<ModelEvent, ProviderError>> + Send>>;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ProviderError {
    #[error("provider rejected request: {0}")]
    Request(String),
    #[error("provider transport failed before a normalized stream was available: {0}")]
    Transport(String),
}

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    fn capabilities(&self, model: &ModelId) -> ModelCapabilities;

    async fn stream(
        &self,
        request: ModelRequest,
        context: ProviderContext,
    ) -> Result<ModelEventStream, ProviderError>;
}
