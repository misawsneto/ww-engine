//! T007 work unit 1 — identity, registry, and configured-order projection.
//!
//! Covers `V-T007-01` .. `V-T007-04`.

use serde_json::json;
use std::sync::Arc;
use ww_agent_tools::{
    EffectDescriptor, ReplayPolicy, Tool, ToolContext, ToolDefinitionError, ToolExecutionError,
    ToolId, ToolIdentity, ToolInvocationOutcome, ToolOutput, ToolRegistry, ToolRequest,
    ToolResolutionError, ToolSpec, ToolVersion,
};

/// Minimal registrable tool. Work unit 3 replaces this with the real fixtures.
struct StubTool {
    identity: ToolIdentity,
    schema: serde_json::Value,
}

impl StubTool {
    fn new(id: &str, version: &str) -> Self {
        Self {
            identity: identity(id, version),
            schema: json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": { "value": {} },
                "required": ["value"],
                "additionalProperties": false
            }),
        }
    }
}

#[async_trait::async_trait]
impl Tool for StubTool {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            identity: self.identity.clone(),
            description: "stub".to_owned(),
            input_schema: self.schema.clone(),
        }
    }

    fn effect(
        &self,
        _arguments: &serde_json::Value,
    ) -> Result<EffectDescriptor, ToolExecutionError> {
        Ok(EffectDescriptor::Pure {
            kind: self.identity.id.as_str().to_owned(),
        })
    }

    fn replay_policy(&self, _arguments: &serde_json::Value) -> ReplayPolicy {
        ReplayPolicy::Safe
    }

    async fn execute(&self, request: ToolRequest, _context: ToolContext) -> ToolInvocationOutcome {
        ToolInvocationOutcome::Output(ToolOutput {
            content: request.arguments,
        })
    }
}

fn identity(id: &str, version: &str) -> ToolIdentity {
    ToolIdentity {
        id: ToolId::new(id).expect("tool id"),
        version: ToolVersion::new(version).expect("tool version"),
        implementation_digest: None,
    }
}

fn stub(id: &str, version: &str) -> Arc<dyn Tool> {
    Arc::new(StubTool::new(id, version))
}

// V-T007-01
#[test]
fn empty_identity_components_reject() {
    assert!(ToolId::new("").is_err());
    assert!(ToolId::new("   ").is_err());
    assert!(ToolVersion::new("").is_err());
    assert!(ToolVersion::new("  ").is_err());
    assert!(ToolId::new("test.echo").is_ok());
    assert!(ToolVersion::new("1").is_ok());
}

// V-T007-02
#[test]
fn duplicate_tool_id_rejects_before_a_run() {
    let error = ToolRegistry::build(vec![stub("test.echo", "1"), stub("test.echo", "2")])
        .expect_err("duplicate id must reject");
    assert!(
        matches!(error, ToolDefinitionError::DuplicateId { ref id } if id.as_str() == "test.echo"),
        "unexpected error: {error:?}"
    );
}

// V-T007-03
#[test]
fn exact_version_resolves_and_mismatch_rejects_without_substitution() {
    let registry = ToolRegistry::build(vec![stub("test.echo", "1")]).expect("registry builds");

    let resolved = registry
        .resolve(&identity("test.echo", "1"))
        .expect("exact pin resolves");
    assert_eq!(resolved.spec().identity.version.as_str(), "1");

    let mismatch = registry
        .resolve(&identity("test.echo", "2"))
        .expect_err("version mismatch must reject");
    assert!(
        matches!(mismatch, ToolResolutionError::VersionMismatch { .. }),
        "a different version must never be substituted: {mismatch:?}"
    );

    let unknown = registry
        .resolve(&identity("test.missing", "1"))
        .expect_err("unknown tool must reject");
    assert!(matches!(unknown, ToolResolutionError::NotFound { .. }));
}

// V-T007-04 — registration order deliberately differs from configured pin order.
#[test]
fn projection_returns_only_configured_pins_in_configured_order() {
    let registry = ToolRegistry::build(vec![
        stub("test.charlie", "1"),
        stub("test.alpha", "1"),
        stub("test.bravo", "1"),
    ])
    .expect("registry builds");

    let pins = vec![identity("test.bravo", "1"), identity("test.charlie", "1")];
    let projected = registry.project(&pins).expect("configured pins project");

    let names: Vec<&str> = projected
        .iter()
        .map(|spec| spec.identity.id.as_str())
        .collect();
    assert_eq!(
        names,
        vec!["test.bravo", "test.charlie"],
        "configured order is authoritative; registration order has none"
    );
    assert_eq!(
        projected.len(),
        2,
        "an unpinned registered tool must not be model-visible"
    );
}

#[test]
fn projection_rejects_an_unavailable_pin() {
    let registry = ToolRegistry::build(vec![stub("test.alpha", "1")]).expect("registry builds");
    let error = registry
        .project(&[identity("test.alpha", "9")])
        .expect_err("unavailable pin must reject");
    assert!(matches!(error, ToolResolutionError::VersionMismatch { .. }));
}
