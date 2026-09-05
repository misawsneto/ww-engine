//! T007 work unit 1 — offline Draft 2020-12 schema profile.
//!
//! Covers `V-T007-05` .. `V-T007-10`. `V-T007-11` needs the preparation seam
//! and lands in work unit 2.

use serde_json::json;
use ww_agent_tools::{CompiledSchema, ToolDefinitionError};

fn object_schema() -> serde_json::Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": { "value": { "type": "string" } },
        "required": ["value"],
        "additionalProperties": false
    })
}

// V-T007-05
#[test]
fn valid_schema_compiles_once_and_validates_repeatedly() {
    let schema = CompiledSchema::compile(&object_schema()).expect("schema compiles");
    assert!(schema.validate(&json!({"value": "a"})).is_ok());
    assert!(schema.validate(&json!({"value": "b"})).is_ok());
    assert!(schema.validate(&json!({"value": 1})).is_err());
}

// V-T007-06
#[test]
fn malformed_schema_rejects() {
    let error =
        CompiledSchema::compile(&json!({"type": 42})).expect_err("a malformed schema must reject");
    assert!(
        matches!(error, ToolDefinitionError::InvalidSchema { .. }),
        "unexpected error: {error:?}"
    );
}

// V-T007-07 — non-fragment references reject before compilation, with no retrieval.
#[test]
fn non_fragment_reference_rejects_before_compilation() {
    for reference in [
        "https://example.com/schema.json",
        "file:///etc/passwd",
        "other.json#/$defs/thing",
    ] {
        let error = CompiledSchema::compile(&json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": { "value": { "$ref": reference } }
        }))
        .expect_err("external $ref must reject");
        assert!(
            matches!(error, ToolDefinitionError::ExternalReference { .. }),
            "{reference} produced {error:?}"
        );
    }
}

#[test]
fn non_fragment_dynamic_reference_rejects_before_compilation() {
    let error = CompiledSchema::compile(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": { "value": { "$dynamicRef": "https://example.com/x#thing" } }
    }))
    .expect_err("external $dynamicRef must reject");
    assert!(matches!(
        error,
        ToolDefinitionError::ExternalReference { .. }
    ));
}

// V-T007-07 — `$id` alone is not an external retrieval request.
#[test]
fn id_alone_does_not_reject_or_retrieve() {
    let schema = CompiledSchema::compile(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://workweave.invalid/tools/test.echo",
        "type": "object",
        "properties": { "value": { "type": "string" } },
        "required": ["value"],
        "additionalProperties": false
    }))
    .expect("$id alone must not reject");
    assert!(schema.validate(&json!({"value": "a"})).is_ok());
}

// V-T007-08
#[test]
fn local_fragment_reference_validates() {
    let schema = CompiledSchema::compile(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": { "value": { "$ref": "#/$defs/label" } },
        "required": ["value"],
        "additionalProperties": false,
        "$defs": { "label": { "type": "string", "minLength": 1 } }
    }))
    .expect("local fragment $ref compiles");

    assert!(schema.validate(&json!({"value": "ok"})).is_ok());
    assert!(schema.validate(&json!({"value": ""})).is_err());
}

#[test]
fn local_dynamic_anchor_validates() {
    let schema = CompiledSchema::compile(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": { "value": { "$dynamicRef": "#label" } },
        "required": ["value"],
        "additionalProperties": false,
        "$defs": {
            "label": { "$dynamicAnchor": "label", "type": "string", "minLength": 1 }
        }
    }))
    .expect("local $dynamicRef compiles");

    assert!(schema.validate(&json!({"value": "ok"})).is_ok());
    assert!(schema.validate(&json!({"value": ""})).is_err());
}

// V-T007-09
#[test]
fn violations_are_workweave_owned_and_deterministically_ordered() {
    let schema = CompiledSchema::compile(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {
            "a": { "type": "string" },
            "b": { "type": "string" },
            "c": { "type": "string" }
        },
        "required": ["a", "b", "c"],
        "additionalProperties": false
    }))
    .expect("schema compiles");

    let instance = json!({"a": 1, "b": 2, "c": 3});
    let first = schema.validate(&instance).expect_err("instance is invalid");
    let second = schema.validate(&instance).expect_err("instance is invalid");

    let paths: Vec<&str> = first
        .violations
        .iter()
        .map(|v| v.instance_path.as_str())
        .collect();
    assert_eq!(
        paths,
        vec!["/a", "/b", "/c"],
        "paths must sort deterministically"
    );

    let repeat: Vec<&str> = second
        .violations
        .iter()
        .map(|v| v.instance_path.as_str())
        .collect();
    assert_eq!(paths, repeat, "ordering must be stable across runs");
    assert!(first.violations.iter().all(|v| !v.message.is_empty()));
}

// V-T007-10
#[test]
fn validation_never_coerces_or_injects_defaults() {
    let schema = CompiledSchema::compile(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {
            "value": { "type": "string" },
            "extra": { "type": "string", "default": "injected" }
        },
        "required": ["value"]
    }))
    .expect("schema compiles");

    let original = json!({"value": "a"});
    let mut instance = original.clone();
    schema.validate(&instance).expect("valid instance");
    assert_eq!(
        instance, original,
        "validation must not mutate the instance"
    );

    // a numeric string is not coerced into a number, and vice versa
    instance = json!({"value": 1});
    assert!(schema.validate(&instance).is_err());
    assert_eq!(instance, json!({"value": 1}));
}
