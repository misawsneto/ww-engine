//! T007 work unit 2 — canonical bytes/digest and the single preparation seam.
//!
//! Covers `V-T007-11` .. `V-T007-15`, `V-T007-32`, and `V-T007-34`.
//! Effect-aware policy conformance and the real fixtures land in work unit 3.

use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use ww_agent_tools::{
    EffectDescriptor, PolicyDecision, PreparedToolCall, ReplayPolicy, Tool, ToolContext, ToolId,
    ToolIdentity, ToolInvocationOutcome, ToolOutput, ToolPolicy, ToolPolicyInput,
    ToolPreparationDisposition, ToolPreparationStage, ToolRegistry, ToolRequest, ToolSpec,
    ToolVersion, arguments_digest, canonical_bytes, prepare_tool_call,
};

fn identity(id: &str) -> ToolIdentity {
    ToolIdentity {
        id: ToolId::new(id).expect("tool id"),
        version: ToolVersion::new("1").expect("tool version"),
        implementation_digest: None,
    }
}

/// Counts every stage the seam drives, so a test can prove short-circuiting.
#[derive(Default)]
struct Counters {
    classify: AtomicUsize,
    policy: AtomicUsize,
    execute: AtomicUsize,
}

impl Counters {
    fn read(&self) -> (usize, usize, usize) {
        (
            self.classify.load(Ordering::SeqCst),
            self.policy.load(Ordering::SeqCst),
            self.execute.load(Ordering::SeqCst),
        )
    }
}

struct CountingTool {
    counters: Arc<Counters>,
    classification_fails: bool,
}

#[async_trait::async_trait]
impl Tool for CountingTool {
    fn spec(&self) -> ToolSpec {
        ToolSpec {
            identity: identity("test.counted"),
            description: "counted".to_owned(),
            input_schema: json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": { "value": { "type": "string" } },
                "required": ["value"],
                "additionalProperties": false
            }),
        }
    }

    fn effect(
        &self,
        _arguments: &Value,
    ) -> Result<EffectDescriptor, ww_agent_tools::ToolExecutionError> {
        self.counters.classify.fetch_add(1, Ordering::SeqCst);
        if self.classification_fails {
            return Err(ww_agent_tools::ToolExecutionError::new(
                "unsupported",
                "cannot classify",
            ));
        }
        Ok(EffectDescriptor::Pure {
            kind: "test.counted".to_owned(),
        })
    }

    fn replay_policy(&self, _arguments: &Value) -> ReplayPolicy {
        ReplayPolicy::Safe
    }

    async fn execute(&self, request: ToolRequest, _context: ToolContext) -> ToolInvocationOutcome {
        self.counters.execute.fetch_add(1, Ordering::SeqCst);
        ToolInvocationOutcome::Output(ToolOutput {
            content: request.arguments,
        })
    }
}

struct CountingPolicy {
    counters: Arc<Counters>,
    seen: std::sync::Mutex<Vec<(EffectDescriptor, ReplayPolicy)>>,
}

impl ToolPolicy for CountingPolicy {
    fn evaluate(&self, input: &ToolPolicyInput<'_>) -> PolicyDecision {
        self.counters.policy.fetch_add(1, Ordering::SeqCst);
        self.seen
            .lock()
            .expect("policy log")
            .push((input.effect.clone(), input.replay));
        PolicyDecision::Allow
    }
}

struct Harness {
    registry: ToolRegistry,
    pins: Vec<ToolIdentity>,
    policy: CountingPolicy,
    counters: Arc<Counters>,
}

fn harness(classification_fails: bool) -> Harness {
    let counters = Arc::new(Counters::default());
    let registry = ToolRegistry::build(vec![Arc::new(CountingTool {
        counters: Arc::clone(&counters),
        classification_fails,
    })])
    .expect("registry builds");
    Harness {
        registry,
        pins: vec![identity("test.counted")],
        policy: CountingPolicy {
            counters: Arc::clone(&counters),
            seen: std::sync::Mutex::new(Vec::new()),
        },
        counters,
    }
}

fn prepare(harness: &Harness, name: &str, arguments: &Value) -> PreparedToolCall {
    prepare_tool_call(
        &harness.registry,
        &harness.pins,
        &harness.policy,
        name,
        arguments,
    )
}

// V-T007-34 — the bytes themselves are asserted, not just digest equality.
#[test]
fn canonical_bytes_sort_nested_object_keys() {
    let reordered: Value =
        serde_json::from_str(r#"{"b":{"z":1,"a":{"y":2,"x":3}},"a":0}"#).expect("json");
    let canonical = String::from_utf8(canonical_bytes(&reordered)).expect("utf-8");
    assert_eq!(
        canonical, r#"{"a":0,"b":{"a":{"x":3,"y":2},"z":1}}"#,
        "nested object keys must appear in sorted order"
    );
}

// V-T007-12
#[test]
fn insertion_order_does_not_change_bytes_or_digest() {
    let left: Value = serde_json::from_str(r#"{"b":{"z":1,"a":2},"a":3}"#).expect("json");
    let right: Value = serde_json::from_str(r#"{"a":3,"b":{"a":2,"z":1}}"#).expect("json");
    assert_eq!(canonical_bytes(&left), canonical_bytes(&right));
    assert_eq!(arguments_digest(&left), arguments_digest(&right));
}

// V-T007-13
#[test]
fn a_different_value_changes_the_digest() {
    let base = json!({"value": "a"});
    assert_ne!(
        arguments_digest(&base),
        arguments_digest(&json!({"value": "b"}))
    );
    assert_ne!(
        arguments_digest(&base),
        arguments_digest(&json!({"value": 1}))
    );
    assert_eq!(arguments_digest(&base).len(), 64, "sha-256 hex");
}

// V-T007-32 — one seam drives every stage.
#[test]
fn allowed_call_returns_an_executable_disposition() {
    let harness = harness(false);
    let prepared = prepare(&harness, "test.counted", &json!({"value": "a"}));

    let ToolPreparationDisposition::Executable {
        identity: pinned,
        arguments_digest: digest,
        effect,
        replay,
        policy,
    } = prepared.disposition()
    else {
        panic!(
            "expected an executable disposition: {:?}",
            prepared.disposition()
        )
    };
    assert_eq!(pinned.id.as_str(), "test.counted");
    assert_eq!(*digest, arguments_digest(&json!({"value": "a"})));
    assert_eq!(
        *effect,
        EffectDescriptor::Pure {
            kind: "test.counted".to_owned()
        }
    );
    assert_eq!(*replay, ReplayPolicy::Safe);
    assert_eq!(*policy, PolicyDecision::Allow);
    assert!(
        prepared.executor().is_some(),
        "an allowed call retains its executor"
    );

    let (classify, policy_calls, execute) = harness.counters.read();
    assert_eq!((classify, policy_calls), (1, 1));
    assert_eq!(execute, 0, "preparation never executes the tool");
}

// V-T007-11 — invalid arguments stop before classification, policy, and execution.
#[test]
fn invalid_arguments_short_circuit_at_validate() {
    let harness = harness(false);
    let prepared = prepare(&harness, "test.counted", &json!({"value": 1}));

    let ToolPreparationDisposition::NoEffect {
        failed_at,
        code,
        identity: pinned,
        arguments_digest: digest,
        effect,
        replay,
        policy,
        ..
    } = prepared.disposition()
    else {
        panic!("expected a no-effect disposition")
    };
    assert_eq!(*failed_at, ToolPreparationStage::Validate);
    assert_eq!(code, "invalid_arguments");
    assert!(
        pinned.is_some(),
        "the pin resolved before validation failed"
    );
    assert!(
        digest.is_none(),
        "no digest is derived for invalid arguments"
    );
    assert!(effect.is_none() && replay.is_none() && policy.is_none());
    assert!(prepared.executor().is_none());

    assert_eq!(
        harness.counters.read(),
        (0, 0, 0),
        "classification, policy, and execution must all be zero"
    );
}

// V-T007-14 / V-T007-15 — an unknown tool stops at resolve.
#[test]
fn unknown_tool_short_circuits_at_resolve() {
    let harness = harness(false);
    let prepared = prepare(&harness, "test.missing", &json!({"value": "a"}));

    let ToolPreparationDisposition::NoEffect {
        failed_at,
        code,
        identity: pinned,
        ..
    } = prepared.disposition()
    else {
        panic!("expected a no-effect disposition")
    };
    assert_eq!(*failed_at, ToolPreparationStage::Resolve);
    assert_eq!(code, "tool_not_found");
    assert!(pinned.is_none(), "an unknown name pins nothing");
    assert_eq!(harness.counters.read(), (0, 0, 0));
}

// V-T007-15 — classification failure stops before policy.
#[test]
fn classification_failure_short_circuits_before_policy() {
    let harness = harness(true);
    let prepared = prepare(&harness, "test.counted", &json!({"value": "a"}));

    let ToolPreparationDisposition::NoEffect {
        failed_at,
        code,
        arguments_digest: digest,
        effect,
        policy,
        ..
    } = prepared.disposition()
    else {
        panic!("expected a no-effect disposition")
    };
    assert_eq!(*failed_at, ToolPreparationStage::Classify);
    assert_eq!(code, "classification_failed");
    assert!(
        digest.is_some(),
        "the digest is derived before classification"
    );
    assert!(effect.is_none(), "classification produced no descriptor");
    assert!(policy.is_none(), "policy must not run after a failed stage");

    let (classify, policy_calls, execute) = harness.counters.read();
    assert_eq!(classify, 1);
    assert_eq!(policy_calls, 0, "policy must not be consulted");
    assert_eq!(execute, 0);
}

// V-T007-16 — policy runs exactly once per preparation attempt.
#[test]
fn policy_is_evaluated_exactly_once_per_attempt() {
    let harness = harness(false);
    prepare(&harness, "test.counted", &json!({"value": "a"}));
    assert_eq!(harness.counters.read().1, 1);

    prepare(&harness, "test.counted", &json!({"value": "b"}));
    assert_eq!(harness.counters.read().1, 2, "one evaluation per attempt");
}

// The classified values reach policy before it decides.
#[test]
fn policy_observes_the_exact_classified_values() {
    let harness = harness(false);
    prepare(&harness, "test.counted", &json!({"value": "a"}));

    let seen = harness.policy.seen.lock().expect("policy log");
    assert_eq!(seen.len(), 1);
    assert_eq!(
        seen[0],
        (
            EffectDescriptor::Pure {
                kind: "test.counted".to_owned()
            },
            ReplayPolicy::Safe
        )
    );
}
