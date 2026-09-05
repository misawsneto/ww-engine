use chrono::{TimeZone, Utc};
use serde_json::json;
use uuid::Uuid;
use ww_agent_core::{
    AgentAssistantContent, AgentEntry, AgentEntryData, AgentEntryId, AgentPhase, AgentRecord,
    AgentRecordData, AgentRunId, AgentTerminalResult, AgentToolCall, CorruptionError,
    DurableAssistantMessage, LogicalToolCallId, ModelAttemptId, ToolAttemptId, ToolAttemptStatus,
    ToolEffectResult, reduce_agent_history,
};
use ww_agent_provider::{CompletionReason, ModelUsage, ToolCallId};
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

fn user(id: u128, ordinal: u64) -> AgentEntry {
    AgentEntry {
        id: entry_id(id),
        run_id: run_id(),
        ordinal,
        created_at: ts(ordinal as i64),
        data: AgentEntryData::UserInput {
            text: "hello".to_owned(),
        },
    }
}

fn assistant_text(id: u128, ordinal: u64, attempt_id: ModelAttemptId) -> AgentEntry {
    AgentEntry {
        id: entry_id(id),
        run_id: run_id(),
        ordinal,
        created_at: ts(ordinal as i64),
        data: AgentEntryData::AssistantMessage {
            attempt_id,
            message: DurableAssistantMessage {
                content: vec![AgentAssistantContent::Text {
                    text: "done".to_owned(),
                }],
                stop_reason: CompletionReason::Stop,
                usage: Some(ModelUsage {
                    input_tokens: 5,
                    output_tokens: 2,
                    ..ModelUsage::default()
                }),
                provider_request_id: Some("req-1".to_owned()),
            },
        },
    }
}

fn assistant_tool(
    id: u128,
    ordinal: u64,
    attempt_id: ModelAttemptId,
    calls: &[LogicalToolCallId],
) -> AgentEntry {
    AgentEntry {
        id: entry_id(id),
        run_id: run_id(),
        ordinal,
        created_at: ts(ordinal as i64),
        data: AgentEntryData::AssistantMessage {
            attempt_id,
            message: DurableAssistantMessage {
                content: calls
                    .iter()
                    .enumerate()
                    .map(|(index, logical_id)| AgentAssistantContent::ToolCall {
                        call: AgentToolCall {
                            logical_id: *logical_id,
                            provider_call_id: ToolCallId::new(format!("provider-{index}")).unwrap(),
                            name: format!("test.{index}"),
                            arguments_json: "{}".to_owned(),
                            arguments: json!({}),
                        },
                    })
                    .collect(),
                stop_reason: CompletionReason::ToolUse,
                usage: None,
                provider_request_id: Some("req-tool".to_owned()),
            },
        },
    }
}

fn tool_result(
    id: u128,
    ordinal: u64,
    logical_call_id: LogicalToolCallId,
    attempt_id: ToolAttemptId,
) -> AgentEntry {
    AgentEntry {
        id: entry_id(id),
        run_id: run_id(),
        ordinal,
        created_at: ts(ordinal as i64),
        data: AgentEntryData::ModelVisibleToolResult {
            logical_call_id,
            attempt_id,
            tool_name: "test.echo".to_owned(),
            content: json!({"ok": true}),
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

#[test]
fn text_only_history_reduces_to_terminal_result() {
    let attempt = model_attempt(10);
    let entries = vec![user(2, 1), assistant_text(3, 2, attempt)];
    let records = vec![
        record(
            1,
            AgentRecordData::ModelAttemptStarted {
                attempt_id: attempt,
                request_ordinal: 1,
            },
        ),
        record(
            2,
            AgentRecordData::ModelAttemptCompleted {
                attempt_id: attempt,
                assistant_entry_id: entry_id(3),
            },
        ),
        record(
            3,
            AgentRecordData::AgentResultCommitted {
                result: AgentTerminalResult::Succeeded {
                    assistant_entry_id: entry_id(3),
                },
            },
        ),
    ];
    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::Terminal);
    assert_eq!(state.context_entry_ids, vec![entry_id(2), entry_id(3)]);
    assert_eq!(state.next_model_request_ordinal, 2);
    assert_eq!(state.model_request_count, 1);
    assert_eq!(state.usage.total_tokens(), 7);
}

#[test]
fn one_tool_turn_reduces_to_ready_for_next_model_in_source_order() {
    let model = model_attempt(10);
    let tool = tool_attempt(20);
    let logical = call_id(30);
    let entries = vec![
        user(2, 1),
        assistant_tool(3, 2, model, &[logical]),
        tool_result(4, 3, logical, tool),
    ];
    let records = vec![
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
                assistant_entry_id: entry_id(3),
            },
        ),
        record(
            3,
            AgentRecordData::ToolAttemptStarted {
                attempt_id: tool,
                logical_call_id: logical,
            },
        ),
        record(
            4,
            AgentRecordData::ToolAttemptCompleted {
                attempt_id: tool,
                result_entry_id: entry_id(4),
            },
        ),
        record(
            5,
            AgentRecordData::TurnCommitted {
                turn_ordinal: 1,
                assistant_entry_id: entry_id(3),
                tool_result_entry_ids: vec![entry_id(4)],
            },
        ),
    ];
    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::ReadyForModel);
    assert_eq!(
        state.context_entry_ids,
        vec![entry_id(2), entry_id(3), entry_id(4)]
    );
    assert_eq!(state.turn_count, 1);
    assert_eq!(state.tool_attempt_count, 1);
    assert_eq!(
        state.completed_tool_results.get(&logical),
        Some(&entry_id(4))
    );
}

#[test]
fn interrupted_model_attempt_is_recoverable_as_new_model_boundary() {
    let attempt = model_attempt(10);
    let entries = vec![user(2, 1)];
    let records = vec![
        record(
            1,
            AgentRecordData::ModelAttemptStarted {
                attempt_id: attempt,
                request_ordinal: 1,
            },
        ),
        record(
            2,
            AgentRecordData::ModelAttemptInterrupted {
                attempt_id: attempt,
                reason: ww_agent_core::ModelAttemptInterruptReason::RuntimeRestart,
            },
        ),
    ];
    let state = reduce_agent_history(run_id(), &entries, &records).unwrap();
    assert_eq!(state.phase, AgentPhase::ReadyForModel);
    assert_eq!(state.next_model_request_ordinal, 2);
}

#[test]
fn rejects_non_contiguous_entry_ordinal() {
    let entries = vec![user(2, 2)];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &[]).unwrap_err(),
        CorruptionError::EntryOrdinal {
            expected: 1,
            actual: 2
        }
    );
}

#[test]
fn rejects_non_contiguous_record_sequence() {
    let entries = vec![user(2, 1)];
    let records = vec![record(
        2,
        AgentRecordData::ModelAttemptStarted {
            attempt_id: model_attempt(10),
            request_ordinal: 1,
        },
    )];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::RecordSequence {
            expected: 1,
            actual: 2
        }
    );
}

#[test]
fn rejects_unknown_assistant_entry_reference() {
    let attempt = model_attempt(10);
    let entries = vec![user(2, 1)];
    let records = vec![
        record(
            1,
            AgentRecordData::ModelAttemptStarted {
                attempt_id: attempt,
                request_ordinal: 1,
            },
        ),
        record(
            2,
            AgentRecordData::ModelAttemptCompleted {
                attempt_id: attempt,
                assistant_entry_id: entry_id(99),
            },
        ),
    ];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::UnknownEntry(entry_id(99))
    );
}

#[test]
fn rejects_assistant_entry_from_different_model_attempt() {
    let active = model_attempt(10);
    let other = model_attempt(11);
    let entries = vec![user(2, 1), assistant_text(3, 2, other)];
    let records = vec![
        record(
            1,
            AgentRecordData::ModelAttemptStarted {
                attempt_id: active,
                request_ordinal: 1,
            },
        ),
        record(
            2,
            AgentRecordData::ModelAttemptCompleted {
                attempt_id: active,
                assistant_entry_id: entry_id(3),
            },
        ),
    ];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::AssistantAttemptMismatch {
            entry_id: entry_id(3),
            attempt_id: active
        }
    );
}

#[test]
fn rejects_tool_attempt_that_skips_provider_source_order() {
    let model = model_attempt(10);
    let first = call_id(30);
    let second = call_id(31);
    let entries = vec![user(2, 1), assistant_tool(3, 2, model, &[first, second])];
    let records = vec![
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
                assistant_entry_id: entry_id(3),
            },
        ),
        record(
            3,
            AgentRecordData::ToolAttemptStarted {
                attempt_id: tool_attempt(20),
                logical_call_id: second,
            },
        ),
    ];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::ToolSourceOrder {
            expected: first,
            actual: second
        }
    );
}

#[test]
fn rejects_duplicate_model_visible_result_for_one_logical_call() {
    let logical = call_id(30);
    let attempt = tool_attempt(20);
    let model = model_attempt(10);
    let entries = vec![
        user(2, 1),
        assistant_tool(3, 2, model, &[logical]),
        tool_result(4, 3, logical, attempt),
        tool_result(5, 4, logical, attempt),
    ];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &[]).unwrap_err(),
        CorruptionError::DuplicateLogicalToolResult(logical)
    );
}

#[test]
fn rejects_record_after_terminal_result() {
    let attempt = model_attempt(10);
    let entries = vec![user(2, 1), assistant_text(3, 2, attempt)];
    let records = vec![
        record(
            1,
            AgentRecordData::ModelAttemptStarted {
                attempt_id: attempt,
                request_ordinal: 1,
            },
        ),
        record(
            2,
            AgentRecordData::ModelAttemptCompleted {
                attempt_id: attempt,
                assistant_entry_id: entry_id(3),
            },
        ),
        record(
            3,
            AgentRecordData::AgentResultCommitted {
                result: AgentTerminalResult::Succeeded {
                    assistant_entry_id: entry_id(3),
                },
            },
        ),
        record(
            4,
            AgentRecordData::ModelAttemptStarted {
                attempt_id: model_attempt(11),
                request_ordinal: 2,
            },
        ),
    ];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &records).unwrap_err(),
        CorruptionError::RecordAfterTerminal("model_attempt_started")
    );
}

#[test]
fn rejects_orphan_generated_entry() {
    let attempt = model_attempt(10);
    let entries = vec![user(2, 1), assistant_text(3, 2, attempt)];
    assert_eq!(
        reduce_agent_history(run_id(), &entries, &[]).unwrap_err(),
        CorruptionError::OrphanGeneratedEntry(entry_id(3))
    );
}

// ---------------------------------------------------------------------------
// T007 work unit 5 — durable tool grammar and reducer.
//
// Covers V-T007-24, 25, 26, 27, 31, 35, 38, 39, 40, and the core half of 41.
// These histories are handcrafted. They prove reduction, not that a production
// driver persisted them in this order; that remains T008.
// ---------------------------------------------------------------------------

fn tool_identity(name: &str) -> ToolIdentity {
    ToolIdentity {
        id: ToolId::new(name).expect("tool id"),
        version: ToolVersion::new("1").expect("tool version"),
        implementation_digest: None,
    }
}

fn executable(name: &str) -> ToolPreparationDisposition {
    ToolPreparationDisposition::Executable {
        identity: tool_identity(name),
        arguments_digest: "digest-a".to_owned(),
        effect: EffectDescriptor::Pure {
            kind: name.to_owned(),
        },
        replay: ReplayPolicy::Safe,
        policy: PolicyDecision::Allow,
    }
}

fn executable_never(name: &str) -> ToolPreparationDisposition {
    ToolPreparationDisposition::Executable {
        identity: tool_identity(name),
        arguments_digest: "digest-a".to_owned(),
        effect: EffectDescriptor::Synthetic {
            kind: name.to_owned(),
            attributes: json!({"key": "alpha"}),
        },
        replay: ReplayPolicy::Never,
        policy: PolicyDecision::Allow,
    }
}

fn no_effect(stage: ToolPreparationStage, code: &str) -> ToolPreparationDisposition {
    let denied = stage == ToolPreparationStage::Policy;
    ToolPreparationDisposition::NoEffect {
        failed_at: stage,
        code: code.to_owned(),
        message: "no effect".to_owned(),
        identity: Some(tool_identity("test.echo")),
        arguments_digest: denied.then(|| "digest-a".to_owned()),
        effect: denied.then(|| EffectDescriptor::Pure {
            kind: "test.echo".to_owned(),
        }),
        replay: denied.then_some(ReplayPolicy::Safe),
        policy: denied.then(|| PolicyDecision::Deny {
            code: "policy_denied".to_owned(),
            message: "denied".to_owned(),
        }),
    }
}

fn prepared(
    attempt_id: ToolAttemptId,
    logical: LogicalToolCallId,
    result_entry_id: AgentEntryId,
    disposition: ToolPreparationDisposition,
) -> AgentRecordData {
    AgentRecordData::ToolCallPrepared {
        attempt_id,
        logical_call_id: logical,
        assistant_entry_id: entry_id(3),
        source_index: 0,
        provider_call_id: ToolCallId::new("provider-0").unwrap(),
        requested_tool_name: "test.0".to_owned(),
        result_entry_id,
        disposition: Box::new(disposition),
    }
}

/// Assistant turn with one tool call, plus the records that open one attempt.
fn opening(
    attempt: ToolAttemptId,
    logical: LogicalToolCallId,
) -> (Vec<AgentEntry>, Vec<AgentRecord>) {
    let model = model_attempt(10);
    let entries = vec![user(2, 1), assistant_tool(3, 2, model, &[logical])];
    let records = vec![
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
                assistant_entry_id: entry_id(3),
            },
        ),
        record(
            3,
            AgentRecordData::ToolAttemptStarted {
                attempt_id: attempt,
                logical_call_id: logical,
            },
        ),
    ];
    (entries, records)
}

// V-T007-38 / V-T007-39
#[test]
fn executable_preparation_and_effect_start_reduce_to_ambiguity() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(attempt, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted {
            attempt_id: attempt,
        },
    ));

    let state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
    let preparation = state
        .tool_preparations
        .get(&logical)
        .expect("preparation is durable");
    assert_eq!(preparation.result_entry_id, entry_id(4));
    assert_eq!(preparation.source_index, 0);
    assert_eq!(preparation.assistant_entry_id, entry_id(3));
    assert_eq!(preparation.disposition, executable("test.echo"));

    let attempts = state.tool_attempts.get(&logical).expect("attempt state");
    assert_eq!(
        attempts.last().expect("attempt").status,
        ToolAttemptStatus::EffectInFlight,
        "an effect start is an ambiguity boundary, not proof the effect ran"
    );
    assert!(state.completed_tool_results.is_empty());
}

// V-T007-24
#[test]
fn effect_completion_without_result_entry_is_repairable() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(attempt, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted {
            attempt_id: attempt,
        },
    ));
    records.push(record(
        6,
        AgentRecordData::ToolEffectCompleted {
            attempt_id: attempt,
            result: ToolEffectResult::Output {
                content: json!({"value": "a"}),
            },
        },
    ));

    let state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
    let attempts = state.tool_attempts.get(&logical).expect("attempt state");
    assert_eq!(
        attempts.last().expect("attempt").status,
        ToolAttemptStatus::EffectSettled {
            result: ToolEffectResult::Output {
                content: json!({"value": "a"})
            }
        },
        "a durable effect result without its model-visible entry awaits repair"
    );
    assert!(
        state.completed_tool_results.is_empty(),
        "no model-visible result exists yet"
    );
}

// V-T007-25
#[test]
fn interrupted_safe_and_intervention_never_are_distinct() {
    let logical = call_id(30);

    let safe = tool_attempt(20);
    let (entries, mut records) = opening(safe, logical);
    records.push(record(
        4,
        prepared(safe, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted { attempt_id: safe },
    ));
    records.push(record(
        6,
        AgentRecordData::ToolAttemptInterrupted {
            attempt_id: safe,
            reason: "runtime restart".to_owned(),
        },
    ));
    let safe_state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
    assert_eq!(
        safe_state.tool_attempts[&logical].last().unwrap().status,
        ToolAttemptStatus::Interrupted {
            reason: "runtime restart".to_owned()
        }
    );

    let never = tool_attempt(21);
    let (entries, mut records) = opening(never, logical);
    records.push(record(
        4,
        prepared(
            never,
            logical,
            entry_id(4),
            executable_never("test.unsafe_once"),
        ),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted { attempt_id: never },
    ));
    records.push(record(
        6,
        AgentRecordData::ToolAttemptIntervention {
            attempt_id: never,
            reason: "never-replayable ambiguity".to_owned(),
        },
    ));
    let never_state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
    assert_eq!(
        never_state.tool_attempts[&logical].last().unwrap().status,
        ToolAttemptStatus::Intervention {
            reason: "never-replayable ambiguity".to_owned()
        }
    );
    assert_ne!(
        safe_state.tool_attempts[&logical].last().unwrap().status,
        never_state.tool_attempts[&logical].last().unwrap().status
    );
}

// V-T007-26
#[test]
fn preparation_conflict_across_attempts_rejects() {
    let first = tool_attempt(20);
    let second = tool_attempt(21);
    let logical = call_id(30);
    let (entries, mut records) = opening(first, logical);
    records.push(record(
        4,
        prepared(first, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted { attempt_id: first },
    ));
    records.push(record(
        6,
        AgentRecordData::ToolAttemptInterrupted {
            attempt_id: first,
            reason: "restart".to_owned(),
        },
    ));
    records.push(record(
        7,
        AgentRecordData::ToolAttemptStarted {
            attempt_id: second,
            logical_call_id: logical,
        },
    ));
    // same logical call, different pinned tool — must reject
    records.push(record(
        8,
        prepared(second, logical, entry_id(4), executable("test.other")),
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(error, CorruptionError::ToolPreparationConflict(logical));
}

// V-T007-27 — the reserved result entry id is not negotiable.
#[test]
fn wrong_reserved_result_entry_rejects() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (mut entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(attempt, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted {
            attempt_id: attempt,
        },
    ));
    records.push(record(
        6,
        AgentRecordData::ToolEffectCompleted {
            attempt_id: attempt,
            result: ToolEffectResult::Output { content: json!({}) },
        },
    ));
    entries.push(tool_result(9, 3, logical, attempt));
    records.push(record(
        7,
        AgentRecordData::ToolAttemptCompleted {
            attempt_id: attempt,
            result_entry_id: entry_id(9),
        },
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(
        error,
        CorruptionError::ReservedResultMismatch {
            expected: entry_id(4),
            actual: entry_id(9),
        }
    );
}

// V-T007-27 — an effect may not start on a no-effect disposition.
#[test]
fn effect_start_after_no_effect_rejects() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            entry_id(4),
            no_effect(ToolPreparationStage::Policy, "policy_denied"),
        ),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectStarted {
            attempt_id: attempt,
        },
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(
        error,
        CorruptionError::EffectStartWithoutExecutable(attempt)
    );
}

#[test]
fn duplicate_preparation_for_one_attempt_rejects() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(attempt, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        prepared(attempt, logical, entry_id(4), executable("test.echo")),
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(error, CorruptionError::DuplicateToolPreparation(attempt));
}

#[test]
fn effect_completion_without_start_rejects() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(attempt, logical, entry_id(4), executable("test.echo")),
    ));
    records.push(record(
        5,
        AgentRecordData::ToolEffectCompleted {
            attempt_id: attempt,
            result: ToolEffectResult::Output { content: json!({}) },
        },
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(
        error,
        CorruptionError::EffectCompletionWithoutStart(attempt)
    );
}

// V-T007-31 / V-T007-35 — Rejected is Resolve/Validate/Classify; Denied is Policy.
#[test]
fn rejected_carries_the_failed_stage_and_denied_is_policy_only() {
    for (stage, code) in [
        (ToolPreparationStage::Resolve, "tool_not_found"),
        (ToolPreparationStage::Validate, "invalid_arguments"),
        (ToolPreparationStage::Classify, "classification_failed"),
    ] {
        let attempt = tool_attempt(20);
        let logical = call_id(30);
        let (mut entries, mut records) = opening(attempt, logical);
        records.push(record(
            4,
            prepared(attempt, logical, entry_id(4), no_effect(stage, code)),
        ));
        entries.push(tool_result(4, 3, logical, attempt));
        records.push(record(
            5,
            AgentRecordData::ToolAttemptRejected {
                attempt_id: attempt,
                result_entry_id: entry_id(4),
                failed_at: stage,
            },
        ));
        let state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
        assert_eq!(
            state.tool_attempts[&logical].last().unwrap().status,
            ToolAttemptStatus::Rejected {
                result_entry_id: entry_id(4),
                failed_at: stage,
            }
        );
    }

    // Policy failure terminates as Denied, and ToolAttemptDenied carries no stage.
    let attempt = tool_attempt(21);
    let logical = call_id(30);
    let (mut entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            entry_id(4),
            no_effect(ToolPreparationStage::Policy, "policy_denied"),
        ),
    ));
    entries.push(tool_result(4, 3, logical, attempt));
    records.push(record(
        5,
        AgentRecordData::ToolAttemptDenied {
            attempt_id: attempt,
            result_entry_id: entry_id(4),
        },
    ));
    let state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
    assert_eq!(
        state.tool_attempts[&logical].last().unwrap().status,
        ToolAttemptStatus::Denied {
            result_entry_id: entry_id(4)
        }
    );
}

#[test]
fn rejected_for_a_policy_failure_rejects() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (mut entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            entry_id(4),
            no_effect(ToolPreparationStage::Policy, "policy_denied"),
        ),
    ));
    entries.push(tool_result(4, 3, logical, attempt));
    records.push(record(
        5,
        AgentRecordData::ToolAttemptRejected {
            attempt_id: attempt,
            result_entry_id: entry_id(4),
            failed_at: ToolPreparationStage::Policy,
        },
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(error, CorruptionError::WrongNoEffectRecord(attempt));
}

#[test]
fn denied_for_a_validation_failure_rejects() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (mut entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            entry_id(4),
            no_effect(ToolPreparationStage::Validate, "invalid_arguments"),
        ),
    ));
    entries.push(tool_result(4, 3, logical, attempt));
    records.push(record(
        5,
        AgentRecordData::ToolAttemptDenied {
            attempt_id: attempt,
            result_entry_id: entry_id(4),
        },
    ));

    let error = reduce_agent_history(run_id(), &entries, &records).expect_err("must reject");
    assert_eq!(error, CorruptionError::WrongNoEffectRecord(attempt));
}

// V-T007-40
#[test]
fn no_effect_histories_contain_no_effect_records() {
    let attempt = tool_attempt(20);
    let logical = call_id(30);
    let (mut entries, mut records) = opening(attempt, logical);
    records.push(record(
        4,
        prepared(
            attempt,
            logical,
            entry_id(4),
            no_effect(ToolPreparationStage::Validate, "invalid_arguments"),
        ),
    ));
    entries.push(tool_result(4, 3, logical, attempt));
    records.push(record(
        5,
        AgentRecordData::ToolAttemptRejected {
            attempt_id: attempt,
            result_entry_id: entry_id(4),
            failed_at: ToolPreparationStage::Validate,
        },
    ));

    let state = reduce_agent_history(run_id(), &entries, &records).expect("valid history");
    assert!(
        !records.iter().any(|r| matches!(
            r.data,
            AgentRecordData::ToolEffectStarted { .. } | AgentRecordData::ToolEffectCompleted { .. }
        )),
        "a no-effect settlement records no effect boundary"
    );
    assert_eq!(
        state.completed_tool_results.get(&logical),
        Some(&entry_id(4))
    );
}

// V-T007-41 — core embeds the tools-owned taxonomy without redefining it.
#[test]
fn core_embeds_the_tools_owned_preparation_taxonomy() {
    let disposition: ToolPreparationDisposition = executable("test.echo");
    let stage: ToolPreparationStage = ToolPreparationStage::Policy;
    let record = prepared(tool_attempt(20), call_id(30), entry_id(4), disposition);
    let encoded = serde_json::to_value(&record).expect("record serializes");
    assert_eq!(encoded["disposition"]["disposition"], "executable");
    assert_eq!(
        serde_json::to_value(stage).expect("stage serializes"),
        json!("policy")
    );
}
