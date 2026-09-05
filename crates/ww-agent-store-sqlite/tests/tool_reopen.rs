//! T007 V-T007-28 — durable tool reconstruction survives real SQLite reopen.

use chrono::Utc;
use serde_json::json;
use tempfile::TempDir;
use ww_agent_core::{
    AgentAppend, AgentEntry, AgentEntryData, AgentEntryId, AgentPhase, AgentRecord,
    AgentRecordData, AgentRunId, AgentStore, LogicalToolCallId, ModelAttemptId, NewAgentRun,
    ToolAttemptId, ToolAttemptStatus, reduce_agent_history,
};
use ww_agent_store_sqlite::SqliteAgentStore;

async fn migrated_store(temp: &TempDir) -> SqliteAgentStore {
    let store = SqliteAgentStore::new(temp.path().join("runtime.db"));
    store.migrate().await.expect("migrate Agent store");
    store
}

fn initial_entry(run_id: AgentRunId) -> AgentEntry {
    AgentEntry {
        id: AgentEntryId::new(),
        run_id,
        ordinal: 1,
        created_at: Utc::now(),
        data: AgentEntryData::UserInput {
            text: "run fixture".to_owned(),
        },
    }
}

fn assistant_entry(
    run_id: AgentRunId,
    id: AgentEntryId,
    attempt_id: ModelAttemptId,
    logical_call_id: LogicalToolCallId,
) -> AgentEntry {
    let data: AgentEntryData = serde_json::from_value(json!({
        "type": "assistant_message",
        "attempt_id": attempt_id.to_string(),
        "message": {
            "content": [{
                "type": "tool_call",
                "call": {
                    "logical_id": logical_call_id.to_string(),
                    "provider_call_id": "provider-0",
                    "name": "test.unsafe_once",
                    "arguments_json": "{\"key\":\"alpha\"}",
                    "arguments": {"key": "alpha"}
                }
            }],
            "stop_reason": "tool_use",
            "usage": null,
            "provider_request_id": "request-1"
        }
    }))
    .expect("deserialize assistant entry data");
    AgentEntry {
        id,
        run_id,
        ordinal: 2,
        created_at: Utc::now(),
        data,
    }
}

fn prepared_record_data(
    attempt_id: ToolAttemptId,
    logical_call_id: LogicalToolCallId,
    assistant_entry_id: AgentEntryId,
    result_entry_id: AgentEntryId,
) -> AgentRecordData {
    serde_json::from_value(json!({
        "type": "tool_call_prepared",
        "attempt_id": attempt_id.to_string(),
        "logical_call_id": logical_call_id.to_string(),
        "assistant_entry_id": assistant_entry_id.to_string(),
        "source_index": 0,
        "provider_call_id": "provider-0",
        "requested_tool_name": "test.unsafe_once",
        "result_entry_id": result_entry_id.to_string(),
        "disposition": {
            "disposition": "executable",
            "identity": {
                "id": "test.unsafe_once",
                "version": "1",
                "implementation_digest": null
            },
            "arguments_digest": "digest-a",
            "effect": {
                "type": "synthetic",
                "kind": "test.unsafe_once",
                "attributes": {"key": "alpha"}
            },
            "replay": "never",
            "policy": {"decision": "allow"}
        }
    }))
    .expect("deserialize prepared record")
}

#[tokio::test]
async fn prepared_effect_in_flight_reconstructs_identically_after_reopen() {
    let temp = TempDir::new().expect("temp dir");
    let store = migrated_store(&temp).await;
    let run_id = AgentRunId::new();
    let assistant_id = AgentEntryId::new();
    let reserved_result_id = AgentEntryId::new();
    let model_attempt_id = ModelAttemptId::new();
    let tool_attempt_id = ToolAttemptId::new();
    let logical_call_id = LogicalToolCallId::new();

    store
        .create_run(NewAgentRun {
            id: run_id,
            configuration: json!({"model": "recorded"}),
            created_at: Utc::now(),
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("create run");

    let now = Utc::now();
    store
        .append(AgentAppend {
            run_id,
            expected_version: 1,
            entries: vec![assistant_entry(
                run_id,
                assistant_id,
                model_attempt_id,
                logical_call_id,
            )],
            records: vec![
                AgentRecord {
                    run_id,
                    sequence: 1,
                    recorded_at: now,
                    data: AgentRecordData::ModelAttemptStarted {
                        attempt_id: model_attempt_id,
                        request_ordinal: 1,
                    },
                },
                AgentRecord {
                    run_id,
                    sequence: 2,
                    recorded_at: now,
                    data: AgentRecordData::ModelAttemptCompleted {
                        attempt_id: model_attempt_id,
                        assistant_entry_id: assistant_id,
                    },
                },
                AgentRecord {
                    run_id,
                    sequence: 3,
                    recorded_at: now,
                    data: AgentRecordData::ToolAttemptStarted {
                        attempt_id: tool_attempt_id,
                        logical_call_id,
                    },
                },
                AgentRecord {
                    run_id,
                    sequence: 4,
                    recorded_at: now,
                    data: prepared_record_data(
                        tool_attempt_id,
                        logical_call_id,
                        assistant_id,
                        reserved_result_id,
                    ),
                },
                AgentRecord {
                    run_id,
                    sequence: 5,
                    recorded_at: now,
                    data: AgentRecordData::ToolEffectStarted {
                        attempt_id: tool_attempt_id,
                    },
                },
            ],
        })
        .await
        .expect("append durable tool ambiguity state");

    let before = store.load_history(run_id).await.expect("load before reopen");
    let before_state = reduce_agent_history(run_id, &before.entries, &before.records)
        .expect("reduce before reopen");
    assert_eq!(before_state.phase, AgentPhase::ToolInFlight);
    assert!(matches!(
        before_state.tool_attempts[&logical_call_id]
            .last()
            .unwrap()
            .status,
        ToolAttemptStatus::EffectInFlight { .. }
    ));
    drop(store);

    let reopened = migrated_store(&temp).await;
    let after = reopened.load_history(run_id).await.expect("load after reopen");
    let after_state =
        reduce_agent_history(run_id, &after.entries, &after.records).expect("reduce after reopen");

    assert_eq!(before, after);
    assert_eq!(before_state, after_state);
}
