//! T007 work unit 3 — effect/replay-aware policy and deterministic fixtures.
//!
//! Covers `V-T007-17`, `V-T007-19`, `V-T007-20`, `V-T007-33`, and
//! `V-T007-37`. Production commit-before-effect ordering remains T008.

use serde_json::{Value, json};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use tokio_util::sync::CancellationToken;
use ww_agent_tools::{
    EchoTool, EffectDescriptor, EffectProbe, PolicyDecision, ReplayPolicy, Tool, ToolContext,
    ToolIdentity, ToolInvocationOutcome, ToolPolicy, ToolPolicyInput, ToolPreparationDisposition,
    ToolPreparationStage, ToolRegistry, ToolRequest, UnsafeOnceTool, arguments_digest,
    prepare_tool_call,
};

#[derive(Default)]
struct CountingProbe {
    calls: AtomicUsize,
    keys: Mutex<Vec<String>>,
}

impl CountingProbe {
    fn count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl EffectProbe for CountingProbe {
    fn observe(&self, key: &str) {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.keys.lock().expect("probe log").push(key.to_owned());
    }
}

#[derive(Default)]
struct EffectReplayPolicy {
    seen: Mutex<Vec<(EffectDescriptor, ReplayPolicy)>>,
}

impl EffectReplayPolicy {
    fn decision(effect: &EffectDescriptor, replay: ReplayPolicy) -> PolicyDecision {
        if matches!(effect, EffectDescriptor::Pure { .. }) && replay == ReplayPolicy::Safe {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Deny {
                code: "unsafe_effect".to_owned(),
                message: "synthetic or non-replay-safe call denied".to_owned(),
            }
        }
    }
}

impl ToolPolicy for EffectReplayPolicy {
    fn evaluate(&self, input: &ToolPolicyInput<'_>) -> PolicyDecision {
        self.seen
            .lock()
            .expect("policy log")
            .push((input.effect.clone(), input.replay));
        Self::decision(input.effect, input.replay)
    }
}

fn registry(probe: Arc<CountingProbe>) -> ToolRegistry {
    let tools: Vec<Arc<dyn Tool>> = vec![Arc::new(EchoTool), Arc::new(UnsafeOnceTool::new(probe))];
    ToolRegistry::build(tools).expect("fixture registry")
}

fn pins() -> Vec<ToolIdentity> {
    vec![EchoTool::identity(), UnsafeOnceTool::identity()]
}

// V-T007-17 / V-T007-37
#[test]
fn policy_deny_is_stable_no_effect_and_invokes_probe_zero_times() {
    let probe = Arc::new(CountingProbe::default());
    let registry = registry(Arc::clone(&probe));
    let policy = EffectReplayPolicy::default();
    let arguments = json!({"key": "alpha"});
    let expected_digest = arguments_digest(&arguments);
    let prepared = prepare_tool_call(
        &registry,
        &pins(),
        &policy,
        "test.unsafe_once",
        &arguments,
    );

    let ToolPreparationDisposition::NoEffect {
        failed_at,
        code,
        message,
        identity,
        arguments_digest: digest,
        effect,
        replay,
        policy: decision,
    } = prepared.disposition()
    else {
        panic!("policy denial must be a no-effect disposition")
    };

    assert_eq!(*failed_at, ToolPreparationStage::Policy);
    assert_eq!(code, "policy_denied");
    assert_eq!(message, "synthetic or non-replay-safe call denied");
    assert_eq!(identity.as_ref(), Some(&UnsafeOnceTool::identity()));
    assert_eq!(digest.as_deref(), Some(expected_digest.as_str()));
    assert!(matches!(effect, Some(EffectDescriptor::Synthetic { .. })));
    assert_eq!(*replay, Some(ReplayPolicy::Never));
    assert_eq!(
        decision.as_ref(),
        Some(&PolicyDecision::Deny {
            code: "unsafe_effect".to_owned(),
            message: "synthetic or non-replay-safe call denied".to_owned(),
        })
    );
    assert!(prepared.executor().is_none());
    assert_eq!(probe.count(), 0, "preparation/denial performs no effect");
}

// V-T007-33
#[test]
fn effect_replay_policy_changes_decision_when_classification_is_substituted() {
    let policy = EffectReplayPolicy::default();
    let identity = EchoTool::identity();
    let arguments = json!({"value": "x"});
    let digest = arguments_digest(&arguments);
    let pure = EffectDescriptor::Pure {
        kind: "test.echo".to_owned(),
    };
    let synthetic = EffectDescriptor::Synthetic {
        kind: "test.echo".to_owned(),
        attributes: json!({"substituted": true}),
    };

    let allow = policy.evaluate(&ToolPolicyInput {
        identity: &identity,
        arguments: &arguments,
        arguments_digest: &digest,
        effect: &pure,
        replay: ReplayPolicy::Safe,
    });
    let deny = policy.evaluate(&ToolPolicyInput {
        identity: &identity,
        arguments: &arguments,
        arguments_digest: &digest,
        effect: &synthetic,
        replay: ReplayPolicy::Never,
    });

    assert_eq!(allow, PolicyDecision::Allow);
    assert!(matches!(deny, PolicyDecision::Deny { .. }));
    assert_eq!(
        policy.seen.lock().expect("policy log").as_slice(),
        &[(pure, ReplayPolicy::Safe), (synthetic, ReplayPolicy::Never),]
    );
}

// V-T007-33 — the production seam supplies the exact classifications before policy.
#[test]
fn production_seam_feeds_exact_fixture_classifications_to_policy() {
    let probe = Arc::new(CountingProbe::default());
    let registry = registry(probe);
    let policy = EffectReplayPolicy::default();
    let configured = pins();

    let echo = prepare_tool_call(
        &registry,
        &configured,
        &policy,
        "test.echo",
        &json!({"value": "x"}),
    );
    assert!(matches!(
        echo.disposition(),
        ToolPreparationDisposition::Executable { .. }
    ));

    let unsafe_call = prepare_tool_call(
        &registry,
        &configured,
        &policy,
        "test.unsafe_once",
        &json!({"key": "alpha"}),
    );
    assert!(matches!(
        unsafe_call.disposition(),
        ToolPreparationDisposition::NoEffect {
            failed_at: ToolPreparationStage::Policy,
            ..
        }
    ));

    let seen = policy.seen.lock().expect("policy log");
    assert_eq!(seen.len(), 2);
    assert_eq!(
        seen[0],
        (
            EffectDescriptor::Pure {
                kind: "test.echo".to_owned(),
            },
            ReplayPolicy::Safe,
        )
    );
    assert_eq!(seen[1].1, ReplayPolicy::Never);
    assert!(matches!(
        &seen[1].0,
        EffectDescriptor::Synthetic { kind, attributes }
            if kind == "test.unsafe_once" && attributes == &json!({"key": "alpha"})
    ));
}

// V-T007-19
#[tokio::test]
async fn echo_returns_deterministic_structured_output_and_is_safe() {
    let tool = EchoTool;
    let request = ToolRequest {
        identity: EchoTool::identity(),
        arguments: json!({"value": {"b": 2, "a": [1, 2, 3]}}),
    };
    assert_eq!(tool.replay_policy(&request.arguments), ReplayPolicy::Safe);

    let left = tool
        .execute(
            request.clone(),
            ToolContext {
                cancellation: CancellationToken::new(),
            },
        )
        .await;
    let right = tool
        .execute(
            request,
            ToolContext {
                cancellation: CancellationToken::new(),
            },
        )
        .await;

    assert_eq!(left, right);
    assert_eq!(
        left,
        ToolInvocationOutcome::Output(ww_agent_tools::ToolOutput {
            content: json!({"value": {"a": [1, 2, 3], "b": 2}}),
        })
    );
}

// V-T007-20
#[tokio::test]
async fn unsafe_once_invokes_probe_once_per_direct_execute_and_is_never_replay_safe() {
    let probe = Arc::new(CountingProbe::default());
    let probe_for_tool: Arc<dyn EffectProbe> = probe.clone();
    let tool = UnsafeOnceTool::new(probe_for_tool);
    let arguments = json!({"key": "alpha"});
    assert_eq!(tool.replay_policy(&arguments), ReplayPolicy::Never);

    for expected_count in 1..=2 {
        let outcome = tool
            .execute(
                ToolRequest {
                    identity: UnsafeOnceTool::identity(),
                    arguments: arguments.clone(),
                },
                ToolContext {
                    cancellation: CancellationToken::new(),
                },
            )
            .await;
        assert_eq!(
            outcome,
            ToolInvocationOutcome::Output(ww_agent_tools::ToolOutput {
                content: json!({"applied": true, "key": "alpha"}),
            })
        );
        assert_eq!(probe.count(), expected_count);
    }

    assert_eq!(
        probe.keys.lock().expect("probe log").as_slice(),
        &["alpha".to_owned(), "alpha".to_owned()]
    );
}
