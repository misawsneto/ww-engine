use chrono::{TimeZone, Utc};
use serde_json::json;
use uuid::Uuid;
use ww_agent_core::{
    AgentAssistantContent, AgentEntry, AgentEntryData, AgentEntryId, AgentPhase, AgentRecord,
    AgentRecordData, AgentRunId, AgentTerminalResult, AgentToolCall, CorruptionError,
    DurableAssistantMessage, LogicalToolCallId, ModelAttemptId, ToolAttemptId,
    reduce_agent_history,
};
use ww_agent_provider::{CompletionReason, ModelUsage, ToolCallId};

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
