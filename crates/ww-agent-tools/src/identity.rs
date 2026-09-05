use crate::error::ToolDefinitionError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

macro_rules! non_empty_string_id {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ToolDefinitionError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(ToolDefinitionError::EmptyIdentity { field: $label });
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

non_empty_string_id!(ToolId, "tool id");
non_empty_string_id!(ToolVersion, "tool version");

/// Exact pin for one tool implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolIdentity {
    pub id: ToolId,
    pub version: ToolVersion,
    /// Reserved for later implementation pinning. G003 fixtures leave it unset.
    pub implementation_digest: Option<String>,
}

/// Model-visible description of one tool.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolSpec {
    pub identity: ToolIdentity,
    pub description: String,
    pub input_schema: Value,
}
