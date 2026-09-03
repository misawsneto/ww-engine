mod assembler;
mod protocol;
mod recorded;

pub use assembler::{AssemblyError, ResponseAssembler};
pub use protocol::{
    AssistantMessage, CompletionReason, MessageContent, MessageRole, ModelCapabilities, ModelEvent,
    ModelEventStream, ModelId, ModelMessage, ModelProvider, ModelRequest, ModelResponse,
    ModelToolSpec, ModelUsage, ProviderContext, ProviderError, ProviderFailure, ProviderId,
    ProviderStarted, ToolCall, ToolCallId,
};
pub use recorded::{ExpectedRequest, RecordedOutcome, RecordedProvider};
