//! Provider-independent tool contracts for the WorkWeave Agent kernel.
//!
//! This crate owns tool identity, the offline JSON Schema profile, the
//! registry, effect/replay classification, and the policy seam. It owns no
//! Agent operational identity and depends on no runtime, storage, transport,
//! capability, Flow, or Orchestration crate.

mod digest;
mod error;
mod fixtures;
mod identity;
mod policy;
mod preparation;
mod registry;
mod schema;
mod tool;

pub use digest::{arguments_digest, canonical_bytes};
pub use error::{
    ArgumentValidationError, ArgumentViolation, ToolDefinitionError, ToolResolutionError,
};
pub use fixtures::{EchoTool, EffectProbe, UnsafeOnceTool};
pub use identity::{ToolId, ToolIdentity, ToolSpec, ToolVersion};
pub use policy::{PolicyDecision, ToolPolicy, ToolPolicyInput};
pub use preparation::{
    PreparedToolCall, ToolPreparationDisposition, ToolPreparationStage, prepare_tool_call,
};
pub use registry::{RegisteredTool, ToolRegistry};
pub use schema::CompiledSchema;
pub use tool::{
    EffectDescriptor, ReplayPolicy, Tool, ToolContext, ToolExecutionError, ToolInvocationOutcome,
    ToolOutput, ToolRequest,
};
