use crate::error::{ArgumentValidationError, ArgumentViolation, ToolDefinitionError};
use jsonschema::{Draft, Validator};
use serde_json::Value;

/// One tool input schema, compiled once and reused.
///
/// The profile is offline. `jsonschema` is pinned with default resolver
/// features disabled, and non-fragment references are rejected before
/// compilation so that no retrieval is ever attempted.
#[derive(Debug)]
pub struct CompiledSchema {
    validator: Validator,
}

impl CompiledSchema {
    pub fn compile(schema: &Value) -> Result<Self, ToolDefinitionError> {
        reject_non_fragment_references(schema)?;
        let validator = jsonschema::options()
            .with_draft(Draft::Draft202012)
            .build(schema)
            .map_err(|error| ToolDefinitionError::InvalidSchema {
                id: None,
                message: error.to_string(),
            })?;
        Ok(Self { validator })
    }

    /// Validate the authoritative parsed value.
    ///
    /// The instance is borrowed and never mutated: this profile forbids
    /// coercion and default injection.
    pub fn validate(&self, instance: &Value) -> Result<(), ArgumentValidationError> {
        let mut violations: Vec<ArgumentViolation> = self
            .validator
            .iter_errors(instance)
            .map(|error| ArgumentViolation {
                instance_path: error.instance_path().to_string(),
                message: error.to_string(),
            })
            .collect();
        if violations.is_empty() {
            return Ok(());
        }
        violations.sort_by(|left, right| {
            left.instance_path
                .cmp(&right.instance_path)
                .then_with(|| left.message.cmp(&right.message))
        });
        Err(ArgumentValidationError { violations })
    }
}

/// Reject `$ref`/`$dynamicRef` values that are not self-contained fragments.
///
/// This runs before compilation so a rejected schema never reaches a
/// resolver. `$id` is deliberately not inspected: declaring a base URI is not
/// a retrieval request and must not relax the fragment-only rule.
fn reject_non_fragment_references(schema: &Value) -> Result<(), ToolDefinitionError> {
    match schema {
        Value::Object(map) => {
            for key in ["$ref", "$dynamicRef"] {
                if let Some(value) = map.get(key) {
                    let Value::String(reference) = value else {
                        return Err(ToolDefinitionError::InvalidSchema {
                            id: None,
                            message: format!("{key} must be a string"),
                        });
                    };
                    if !reference.starts_with('#') {
                        return Err(ToolDefinitionError::ExternalReference {
                            reference: reference.clone(),
                        });
                    }
                }
            }
            map.values().try_for_each(reject_non_fragment_references)
        }
        Value::Array(items) => items.iter().try_for_each(reject_non_fragment_references),
        _ => Ok(()),
    }
}
