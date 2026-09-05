use crate::ToolId;
use thiserror::Error;

/// A tool definition is wrong. Registry construction fails; no run starts.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ToolDefinitionError {
    #[error("{field} must not be empty")]
    EmptyIdentity { field: &'static str },
    #[error("duplicate tool id {id}")]
    DuplicateId { id: ToolId },
    #[error("invalid tool input schema: {message}")]
    InvalidSchema { message: String },
    #[error("schema reference {reference} is not a self-contained fragment")]
    ExternalReference { reference: String },
}

/// A configured pin does not resolve. No substitution is ever attempted.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ToolResolutionError {
    #[error("no tool registered with id {id}")]
    NotFound { id: ToolId },
    #[error("tool {id} is registered at version {available}, not the pinned {requested}")]
    VersionMismatch {
        id: ToolId,
        requested: String,
        available: String,
    },
}

/// One JSON Schema violation, owned by WorkWeave rather than by `jsonschema`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArgumentViolation {
    pub instance_path: String,
    pub message: String,
}

/// Arguments failed schema validation. Policy and execution never run.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("tool arguments failed validation ({} violation(s))", .violations.len())]
pub struct ArgumentValidationError {
    pub violations: Vec<ArgumentViolation>,
}
