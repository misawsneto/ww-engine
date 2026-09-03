mod assembler;
mod finalize;
mod protocol;
#[cfg(feature = "test-support")]
mod recorded;

pub use assembler::{AssemblyError, ResponseAssembler};
pub use finalize::{StreamFinalizationError, finalize_stream};
pub use protocol::{
    AssistantMessage, CompletionReason, MessageContent, MessageRole, ModelCapabilities, ModelEvent,
    ModelEventStream, ModelId, ModelMessage, ModelProvider, ModelRequest, ModelResponse,
    ModelToolSpec, ModelUsage, ProviderContext, ProviderError, ProviderFailure, ProviderId,
    ProviderStarted, ToolCall, ToolCallId,
};
#[cfg(feature = "test-support")]
pub use recorded::{ExpectedRequest, RecordedOutcome, RecordedProvider};
