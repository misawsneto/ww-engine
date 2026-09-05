//! T007 work unit 5 — Agent-owned durable tool grammar and reduction.
//!
//! These histories are handcrafted. They prove durable vocabulary,
//! reconstruction, and corruption rejection only. T008 owns production
//! commit-before-effect ordering.

use chrono::{TimeZone, Utc};
use serde_json::json;
use uuid::Uuid;
use ww_agent_core::{
    AgentAssistantContent, AgentEntry, AgentEntryData, AgentEntryId, AgentPhase, AgentRecord,
    AgentRecordData, AgentRunId, AgentToolCall, CorruptionError, DurableAssistantMessage,
    LogicalToolCallId, ModelAttemptId, ToolAttemptId, ToolAttemptStatus, ToolEffectResult,
    reduce_agent_history,
};
use ww_agent_provider::{CompletionReason, ToolCallId};
use ww_agent_tools::{
    EffectDescriptor, PolicyDecision, ReplayPolicy, ToolId, ToolIdentity,
    ToolPreparationDisposition, ToolPreparationStage, ToolVersion,
};

fn run_id() -> AgentRunId {
    AgentRunId::from_uuid(Uuid::from_u128(1))
}
fn entry_id(value: u128) -> AgentEntryId {
    AgentEntryId::from_uuid(Uuid::from_u128(value))
}
fn model_attempt(value: u128) -> ModelAttemptId {
    ModelAttemptId::from_uuid(Uuid::from_u128(value))
}
fn tool_attempt(value: u128) -> ToolAttemptId {
    ToolAttemptId::from_uuid(Uuid::from_u128(value))
}
fn call_id(value: u128) -> LogicalToolCallId {
    LogicalToolCallId::from_uuid(Uuid::from_u128(value))
}
fn ts(value: i64) -> chrono::DateTime<Utc> {
    Utc.timestamp_opt(value, 0).unwrap()
}
fn provider_call() -> ToolCallId {
    ToolCallId::new("provider-0").unwrap()
}
fn identity() -> ToolIdentity {
    ToolIdentity {
        id: ToolId::new("test.unsafe_once").unwrap(),
        version: ToolVersion::new("1").unwrap(),
        implementation_digest: None,
    }
}

fn user() -> AgentEntry {
    AgentEntry {
        id: entry_id(10),
        run_id: run_id(),
        ordinal: 1,
        created_at: ts(1),
        data: AgentEntryData::UserInput {
            text: "run tool".to_owned(),
        },
    }
}

fn assistant_tool(logical: LogicalToolCallId, model: ModelAttemptId) -> AgentEntry {
    AgentEntry {
        id: entry_id(20),
        run_id: run_id(),
        ordinal: 2,
        created_at: ts(2),
        data: AgentEntryData::AssistantMessage {
            attempt_id: model,
            message: DurableAssistantMessage {
                content: vec![AgentAssistantContent::ToolCall {
                    call: AgentToolCall {
                        logical_id: logical,
                        provider_call_id: provider_call(),
                        name: "test.unsafe_once".to_owned(),
                        arguments_json: r#"{"key":"alpha"}"#.to_owned(),
                        arguments: json!({"key": "alpha"}),
                    },
                }],
                stop_reason: CompletionReason::ToolUse,
                usage: None,
                provider_request_id: Some("request-1".to_owned()),
            },
        },
    }
}

fn result_entry(
    id: AgentEntryId,
    logical: LogicalToolCallId,
    attempt: ToolAttemptId,
) -> AgentEntry {
    AgentEntry {
        id,
        run_id: run_id(),
        ordinal: 3,
        created_at: ts(3),
        data: AgentEntryData::ModelVisibleToolResult {
            logical_call_id: logical,
            attempt_id: attempt,
            tool_name: "test.unsafe_once".to_owned(),
            content: json!({"applied": true, "key": "alpha"}),
            is_error: false,
        },
    }
}

fn record(sequence: u64, data: AgentRecordData) -> AgentRecord {
    AgentRecord {
        run_id: run_id(),
        sequence,
        recorded_at: ts(100 + sequence as i64),
        data,
    }
}

fn base_history(
    logical: LogicalToolCallId,
    model: ModelAttemptId,
    attempt: ToolAttemptId,
) -> (Vec<AgentEntry>, Vec<AgentRecord>) {
    (
        vec![user(), assistant_tool(logical, model)],
        vec![
            record(
                1,
                AgentRecordData::ModelAttemptStarted {
                    attempt_id: model,
                    request_ordinal: 1,
                },
            ),
            record(
                2,
                AgentRecordData::ModelAttemptCompleted {
                    attempt_id: model,
                    assistant_entry_id: entry_id(20),
                },
            ),
            record(
                3,
                AgentRecordData::ToolAttemptStarted {
                    attempt_id: attempt,
                    logical_call_id: logical,
                },
            ),
        ],
    )
}

fn executable(replay: ReplayPolicy, digest: &str) -> ToolPreparationDisposition {
    ToolPreparationDisposition::Executable {
        identity: identity(),
        arguments_digest: digest.to_owned(),
        effect: EffectDescriptor::Synthetic {
            kind: "test.unsafe_once".to_owned(),
            attributes: json!({"key": "alpha"}),
        },
        replay,
        policy: PolicyDecision::Allow,
    }
}

fn no_effect(stage: ToolPreparationStage) -> ToolPreparationDisposition {
    match stage {
        ToolPreparationStage::Resolve => ToolPreparationDisposition::NoEffect {
            failed_at: stage,
            code: "tool_not_found".to_owned(),
            message: "no configured tool".to_owned(),
            identity: None,
            arguments_digest: None,
            effect: None,
            replay: None,
            policy: None,
        },
        ToolPreparationStage::Validate => ToolPreparationDisposition::NoEffect {
            failed_at: stage,
            code: "invalid_arguments".to_owned(),
            message: "invalid arguments".to_owned(),
            identity: Some(identity()),
            arguments_digest: None,
            effect: None,
            replay: None,
            policy: None,
        },
        ToolPreparationStage::Classify => ToolPreparationDisposition::NoEffect {
            failed_at: stage,
            code: "classification_failed".to_owned(),
            message: "classification failed".to_owned(),
            identity: Some(identity()),
            arguments_digest: Some("digest-a".to_owned()),
            effect: None,
            replay: None,
            policy: None,
        },
        ToolPreparationStage::Policy => ToolPreparationDisposition::NoEffect {
            failed_at: stage,
            code: "policy_denied".to_owned(),
            message: "denied".to_owned(),
            identity: Some(identity()),
            arguments_digest: Some("digest-a".to_owned()),
            effect: Some(EffectDescriptor::Synthetic {
                kind: "test.unsafe_once".to_owned(),
                attributes: json!({"key": "alpha"}),
            }),
            replay: Some(ReplayPolicy::Never),
            policy: Some(PolicyDecision::Deny {
                code: "blocked".to_owned(),
                message: "denied".to_owned(),
            }),
        },
    }
}

fn prepared(
    attempt: ToolAttemptId,
    logical: LogicalToolCallId,
    result: AgentEntryId,
    disposition: ToolPreparationDisposition,
) -> AgentRecordData {
    AgentRecordData::ToolCallPrepared {
        attempt_id: attempt,
        logical_call_id: logical,
        assistant_entry_id: entry_id(20),
        source_index: 0,
        provider_call_id: provider_call(),
        requested_tool_name: "test.unsafe_once".to_owned(),
        result_entry_id: result,
        disposition,
    }
}

fn latest_status(
    state: &ww_agent_core::AgentRecoveryState,
    logical: LogicalToolCallId,
) -> &ToolAttemptStatus {
    &state.tool_attempts[&logical].last().unwrap().status
}

// V-T007-38 / V-T007-39 / V-T007-41
#[test]
fn executable_preparation_and_effect_start_reconstruct_full_ambiguity_state() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let attempt = tool_attempt(50);
    let reserved = entry_id(60);
    let (entries, mut records) = base_history(logical, model, attempt);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            executable(ReplayPolicy::Never, "digest-a"),
        ),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted { attempt_id: attempt },
    ));

    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::ToolInFlight);
    assert_eq!(state.active_tool_attempt, Some(attempt));
    assert!(matches!(
        latest_status(&state, logical),
        ToolAttemptStatus::EffectInFlight {
            result_entry_id,
            replay: ReplayPolicy::Never,
        } if *result_entry_id == reserved
    ));

    let AgentRecordData::ToolCallPrepared {
        assistant_entry_id,
        source_index,
        provider_call_id,
        requested_tool_name,
        result_entry_id,
        disposition,
        ..
    } = &records[3].data
    else {
        unreachable!()
    };
    assert_eq!(*assistant_entry_id, entry_id(20));
    assert_eq!(*source_index, 0);
    assert_eq!(provider_call_id, &provider_call());
    assert_eq!(requested_tool_name, "test.unsafe_once");
    assert_eq!(*result_entry_id, reserved);
    assert!(matches!(
        disposition,
        ToolPreparationDisposition::Executable {
            identity: durable_identity,
            arguments_digest,
            replay: ReplayPolicy::Never,
            policy: PolicyDecision::Allow,
            ..
        } if durable_identity == &identity() && arguments_digest == "digest-a"
    ));
}

// V-T007-24
#[test]
fn completed_effect_without_visible_result_is_repairable_state() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let attempt = tool_attempt(50);
    let reserved = entry_id(60);
    let (entries, mut records) = base_history(logical, model, attempt);
    records.extend([
        record(
            4,
            prepared(
                attempt,
                logical,
                reserved,
                executable(ReplayPolicy::Safe, "digest-a"),
            ),
        ),
        record(
            5,
            AgentRecordData::ToolEffectStarted { attempt_id: attempt },
        ),
        record(
            6,
            AgentRecordData::ToolEffectCompleted {
                attempt_id: attempt,
                result: ToolEffectResult::Output {
                    content: json!({"applied": true}),
                },
            },
        ),
    ]);

    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::ToolResultPending);
    assert!(state.completed_tool_results.is_empty());
    assert!(matches!(
        latest_status(&state, logical),
        ToolAttemptStatus::EffectCompleted {
            result_entry_id,
            replay: ReplayPolicy::Safe,
            ..
        } if *result_entry_id == reserved
    ));
}

// V-T007-25
#[test]
fn ambiguous_safe_attempt_can_interrupt_but_never_attempt_requires_intervention() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let reserved = entry_id(60);

    let safe_attempt = tool_attempt(50);
    let (safe_entries, mut safe_records) = base_history(logical, model, safe_attempt);
    safe_records.extend([
        record(
            4,
            prepared(
                safe_attempt,
                logical,
                reserved,
                executable(ReplayPolicy::Safe, "digest-a"),
            ),
        ),
        record(
            5,
            AgentRecordData::ToolEffectStarted {
                attempt_id: safe_attempt,
            },
        ),
        record(
            6,
            AgentRecordData::ToolAttemptInterrupted {
                attempt_id: safe_attempt,
                reason: "runtime_restart".to_owned(),
            },
        ),
    ]);
    let safe = reduce_agent_history(run_id(), &safe_entries, &safe_records).unwrap();
    assert_eq!(safe.phase, AgentPhase::ToolsPending);
    assert_eq!(safe.active_tool_attempt, None);
    assert!(matches!(
        latest_status(&safe, logical),
        ToolAttemptStatus::Interrupted {
            replay: ReplayPolicy::Safe,
            ..
        }
    ));

    let never_attempt = tool_attempt(51);
    let (never_entries, mut never_records) = base_history(logical, model, never_attempt);
    never_records.extend([
        record(
            4,
            prepared(
                never_attempt,
                logical,
                reserved,
                executable(ReplayPolicy::Never, "digest-a"),
            ),
        ),
        record(
            5,
            AgentRecordData::ToolEffectStarted {
                attempt_id: never_attempt,
            },
        ),
        record(
            6,
            AgentRecordData::ToolAttemptIntervention {
                attempt_id: never_attempt,
                reason: "ambiguous_never_replay".to_owned(),
            },
        ),
    ]);
    let never = reduce_agent_history(run_id(), &never_entries, &never_records).unwrap();
    assert_eq!(never.phase, AgentPhase::InterventionPending);
    assert!(matches!(
        latest_status(&never, logical),
        ToolAttemptStatus::Intervention { .. }
    ));
}

// V-T007-26
#[test]
fn changed_preparation_contract_across_attempts_is_corruption() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let first = tool_attempt(50);
    let second = tool_attempt(51);
    let reserved = entry_id(60);
    let (entries, mut records) = base_history(logical, model, first);
    records.extend([
        record(
            4,
            prepared(
                first,
                logical,
                reserved,
                executable(ReplayPolicy::Safe, "digest-a"),
            ),
        ),
        record(
            5,
            AgentRecordData::ToolEffectStarted { attempt_id: first },
        ),
        record(
            6,
            AgentRecordData::ToolAttemptInterrupted {
                attempt_id: first,
                reason: "runtime_restart".to_owned(),
            },
        ),
        record(
            7,
            AgentRecordData::ToolAttemptStarted {
                attempt_id: second,
                logical_call_id: logical,
            },
        ),
        record(
            8,
            prepared(
                second,
                logical,
                reserved,
                executable(ReplayPolicy::Safe, "digest-b"),
            ),
        ),
    ]);

    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::PreparationConflict(logical)
    );
}

// V-T007-27
#[test]
fn duplicate_preparation_effect_after_no_effect_and_missing_start_are_corruption() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let attempt = tool_attempt(50);
    let reserved = entry_id(60);

    let (entries, mut duplicate) = base_history(logical, model, attempt);
    duplicate.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            executable(ReplayPolicy::Safe, "digest-a"),
        ),
    ));
    duplicate.push(record(
        5,
        prepared(
            attempt,
            logical,
            reserved,
            executable(ReplayPolicy::Safe, "digest-a"),
        ),
    ));
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &duplicate).unwrap_err(),
        CorruptionError::DuplicatePreparation(attempt)
    );

    let (entries, mut no_effect_records) = base_history(logical, model, attempt);
    no_effect_records.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            no_effect(ToolPreparationStage::Resolve),
        ),
    ));
    no_effect_records.push(record(
        5,
        AgentRecordData::ToolEffectStarted { attempt_id: attempt },
    ));
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &no_effect_records).unwrap_err(),
        CorruptionError::EffectAfterNoEffect(attempt)
    );

    let (entries, mut missing_start) = base_history(logical, model, attempt);
    missing_start.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            executable(ReplayPolicy::Safe, "digest-a"),
        ),
    ));
    missing_start.push(record(
        5,
        AgentRecordData::ToolEffectCompleted {
            attempt_id: attempt,
            result: ToolEffectResult::Output {
                content: json!({"ok": true}),
            },
        },
    ));
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &missing_start).unwrap_err(),
        CorruptionError::EffectCompletionWithoutStart(attempt)
    );
}

// V-T007-27
#[test]
fn wrong_reserved_result_identity_is_corruption() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let attempt = tool_attempt(50);
    let reserved = entry_id(60);
    let wrong = entry_id(61);
    let (entries, mut records) = base_history(logical, model, attempt);
    records.extend([
        record(
            4,
            prepared(
                attempt,
                logical,
                reserved,
                executable(ReplayPolicy::Safe, "digest-a"),
            ),
        ),
        record(
            5,
            AgentRecordData::ToolEffectStarted { attempt_id: attempt },
        ),
        record(
            6,
            AgentRecordData::ToolEffectCompleted {
                attempt_id: attempt,
                result: ToolEffectResult::Output {
                    content: json!({"ok": true}),
                },
            },
        ),
        record(
            7,
            AgentRecordData::ToolAttemptCompleted {
                attempt_id: attempt,
                result_entry_id: wrong,
            },
        ),
    ]);
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::ReservedResultMismatch {
            attempt_id: attempt,
            expected: reserved,
            actual: wrong,
        }
    );
}

// V-T007-31 / V-T007-35 / V-T007-40
#[test]
fn no_effect_stage_taxonomy_reconstructs_rejected_and_denied_without_effect_records() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let reserved = entry_id(60);

    for stage in [
        ToolPreparationStage::Resolve,
        ToolPreparationStage::Validate,
        ToolPreparationStage::Classify,
    ] {
        let attempt = tool_attempt(50 + stage as u128);
        let (entries, mut records) = base_history(logical, model, attempt);
        records.push(record(
            4,
            prepared(attempt, logical, reserved, no_effect(stage)),
        ));
        records.push(record(
            5,
            AgentRecordData::ToolAttemptRejected {
                attempt_id: attempt,
                result_entry_id: reserved,
                failed_at: stage,
            },
        ));
        let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
        assert_eq!(state.phase, AgentPhase::ToolResultPending);
        assert!(matches!(
            latest_status(&state, logical),
            ToolAttemptStatus::Rejected { failed_at, .. } if *failed_at == stage
        ));
        assert!(!records.iter().any(|record| matches!(
            record.data,
            AgentRecordData::ToolEffectStarted { .. }
                | AgentRecordData::ToolEffectCompleted { .. }
        )));
    }

    let attempt = tool_attempt(70);
    let (entries, mut records) = base_history(logical, model, attempt);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            no_effect(ToolPreparationStage::Policy),
        ),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolAttemptDenied {
            attempt_id: attempt,
            result_entry_id: reserved,
        },
    ));
    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::ToolResultPending);
    assert!(matches!(
        latest_status(&state, logical),
        ToolAttemptStatus::Denied { .. }
    ));
}

// V-T007-31 / V-T007-35
#[test]
fn rejection_and_denial_taxonomies_cannot_be_swapped() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let attempt = tool_attempt(50);
    let reserved = entry_id(60);

    let (entries, mut policy_as_rejected) = base_history(logical, model, attempt);
    policy_as_rejected.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            no_effect(ToolPreparationStage::Policy),
        ),
    ));
    policy_as_rejected.push(record(
        5,
        AgentRecordData::ToolAttemptRejected {
            attempt_id: attempt,
            result_entry_id: reserved,
            failed_at: ToolPreparationStage::Policy,
        },
    ));
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &policy_as_rejected).unwrap_err(),
        CorruptionError::WrongNoEffectRecord(attempt)
    );

    let (entries, mut resolve_as_denied) = base_history(logical, model, attempt);
    resolve_as_denied.push(record(
        4,
        prepared(
            attempt,
            logical,
            reserved,
            no_effect(ToolPreparationStage::Resolve),
        ),
    ));
    resolve_as_denied.push(record(
        5,
        AgentRecordData::ToolAttemptDenied {
            attempt_id: attempt,
            result_entry_id: reserved,
        },
    ));
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &resolve_as_denied).unwrap_err(),
        CorruptionError::WrongNoEffectRecord(attempt)
    );
}

// T007 durable grammar must also distinguish the settled logical result state.
#[test]
fn handcrafted_effect_history_settles_only_the_reserved_visible_result() {
    let logical = call_id(30);
    let model = model_attempt(40);
    let attempt = tool_attempt(50);
    let reserved = entry_id(60);
    let (mut entries, mut records) = base_history(logical, model, attempt);
    entries.push(result_entry(reserved, logical, attempt));
    records.extend([
        record(
            4,
            prepared(
                attempt,
                logical,
                reserved,
                executable(ReplayPolicy::Safe, "digest-a"),
            ),
        ),
        record(
            5,
            AgentRecordData::ToolEffectStarted { attempt_id: attempt },
        ),
        record(
            6,
            AgentRecordData::ToolEffectCompleted {
                attempt_id: attempt,
                result: ToolEffectResult::Output {
                    content: json!({"applied": true, "key": "alpha"}),
                },
            },
        ),
        record(
            7,
            AgentRecordData::ToolAttemptCompleted {
                attempt_id: attempt,
                result_entry_id: reserved,
            },
        ),
    ]);

    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::ReadyToCommitTurn);
    assert_eq!(state.completed_tool_results.get(&logical), Some(&reserved));
    assert!(matches!(
        latest_status(&state, logical),
        ToolAttemptStatus::Completed { result_entry_id } if *result_entry_id == reserved
    ));
}
