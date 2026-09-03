use chrono::Utc;
use serde_json::json;
use tempfile::TempDir;
use ww_agent_core::{
    AgentAppend, AgentAssistantContent, AgentEntry, AgentEntryData, AgentEntryId, AgentPhase,
    AgentRecord, AgentRecordData, AgentRunId, AgentStore, AgentStoreError, AgentTerminalResult,
    CompletionReason, DurableAssistantMessage, ModelAttemptId, NewAgentRun, reduce_agent_history,
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
            text: "hello".to_owned(),
        },
    }
}

fn terminal_append(run_id: AgentRunId, assistant_id: AgentEntryId) -> AgentAppend {
    let attempt_id = ModelAttemptId::new();
    let now = Utc::now();
    AgentAppend {
        run_id,
        expected_version: 1,
        entries: vec![AgentEntry {
            id: assistant_id,
            run_id,
            ordinal: 2,
            created_at: now,
            data: AgentEntryData::AssistantMessage {
                attempt_id,
                message: DurableAssistantMessage {
                    content: vec![AgentAssistantContent::Text {
                        text: "done".to_owned(),
                    }],
                    stop_reason: CompletionReason::Stop,
                    usage: None,
                    provider_request_id: None,
                },
            },
        }],
        records: vec![
            AgentRecord {
                run_id,
                sequence: 1,
                recorded_at: now,
                data: AgentRecordData::ModelAttemptStarted {
                    attempt_id,
                    request_ordinal: 1,
                },
            },
            AgentRecord {
                run_id,
                sequence: 2,
                recorded_at: now,
                data: AgentRecordData::ModelAttemptCompleted {
                    attempt_id,
                    assistant_entry_id: assistant_id,
                },
            },
            AgentRecord {
                run_id,
                sequence: 3,
                recorded_at: now,
                data: AgentRecordData::AgentResultCommitted {
                    result: AgentTerminalResult::Succeeded {
                        assistant_entry_id: assistant_id,
                    },
                },
            },
        ],
    }
}

#[tokio::test]
async fn create_append_reopen_reconstructs_identical_terminal_state() {
    let temp = TempDir::new().expect("temp dir");
    let store = migrated_store(&temp).await;
    let run_id = AgentRunId::new();
    let assistant_id = AgentEntryId::new();
    store
        .create_run(NewAgentRun {
            id: run_id,
            configuration: json!({"model": "recorded"}),
            created_at: Utc::now(),
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("create run");
    let updated = store
        .append(terminal_append(run_id, assistant_id))
        .await
        .expect("append terminal history");
    assert_eq!(updated.version, 2);

    let before = store
        .load_history(run_id)
        .await
        .expect("load before reopen");
    let before_state = reduce_agent_history(run_id, &before.entries, &before.records)
        .expect("reduce before reopen");
    drop(store);

    let reopened = migrated_store(&temp).await;
    let after = reopened
        .load_history(run_id)
        .await
        .expect("load after reopen");
    let after_state =
        reduce_agent_history(run_id, &after.entries, &after.records).expect("reduce after reopen");

    assert_eq!(before, after);
    assert_eq!(before_state, after_state);
    assert_eq!(after_state.phase, AgentPhase::Terminal);
}

#[tokio::test]
async fn stale_expected_version_rejects_without_partial_agent_mutation() {
    let temp = TempDir::new().expect("temp dir");
    let store = migrated_store(&temp).await;
    let run_id = AgentRunId::new();
    store
        .create_run(NewAgentRun {
            id: run_id,
            configuration: json!({}),
            created_at: Utc::now(),
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("create run");

    let assistant_id = AgentEntryId::new();
    store
        .append(terminal_append(run_id, assistant_id))
        .await
        .expect("first append");

    let error = store
        .append(AgentAppend {
            run_id,
            expected_version: 1,
            entries: vec![],
            records: vec![AgentRecord {
                run_id,
                sequence: 4,
                recorded_at: Utc::now(),
                data: AgentRecordData::AgentResultCommitted {
                    result: AgentTerminalResult::Cancelled,
                },
            }],
        })
        .await
        .expect_err("stale append must conflict");
    assert!(matches!(
        error,
        AgentStoreError::Conflict {
            expected: 1,
            actual: 2,
            ..
        }
    ));

    let history = store
        .load_history(run_id)
        .await
        .expect("history after conflict");
    assert_eq!(history.run.version, 2);
    assert_eq!(history.entries.len(), 2);
    assert_eq!(history.records.len(), 3);
}

#[tokio::test]
async fn failed_batch_rolls_back_inserted_entries_records_and_version() {
    let temp = TempDir::new().expect("temp dir");
    let store = migrated_store(&temp).await;
    let run_id = AgentRunId::new();
    store
        .create_run(NewAgentRun {
            id: run_id,
            configuration: json!({}),
            created_at: Utc::now(),
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("create run");

    let duplicate_id = AgentEntryId::new();
    let now = Utc::now();
    let error = store
        .append(AgentAppend {
            run_id,
            expected_version: 1,
            entries: vec![
                AgentEntry {
                    id: duplicate_id,
                    run_id,
                    ordinal: 2,
                    created_at: now,
                    data: AgentEntryData::UserInput {
                        text: "first insert succeeds inside transaction".to_owned(),
                    },
                },
                AgentEntry {
                    id: duplicate_id,
                    run_id,
                    ordinal: 3,
                    created_at: now,
                    data: AgentEntryData::UserInput {
                        text: "duplicate id forces rollback".to_owned(),
                    },
                },
            ],
            records: vec![AgentRecord {
                run_id,
                sequence: 1,
                recorded_at: now,
                data: AgentRecordData::ModelAttemptStarted {
                    attempt_id: ModelAttemptId::new(),
                    request_ordinal: 1,
                },
            }],
        })
        .await
        .expect_err("duplicate primary key must abort batch");
    assert!(matches!(error, AgentStoreError::Backend(_)));

    let history = store
        .load_history(run_id)
        .await
        .expect("history after rollback");
    assert_eq!(history.run.version, 1);
    assert_eq!(history.entries.len(), 1);
    assert!(history.records.is_empty());
}

#[tokio::test]
async fn append_rejects_non_contiguous_ordinals_before_mutation() {
    let temp = TempDir::new().expect("temp dir");
    let store = migrated_store(&temp).await;
    let run_id = AgentRunId::new();
    store
        .create_run(NewAgentRun {
            id: run_id,
            configuration: json!({}),
            created_at: Utc::now(),
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("create run");

    let error = store
        .append(AgentAppend {
            run_id,
            expected_version: 1,
            entries: vec![AgentEntry {
                id: AgentEntryId::new(),
                run_id,
                ordinal: 3,
                created_at: Utc::now(),
                data: AgentEntryData::UserInput {
                    text: "gap".to_owned(),
                },
            }],
            records: vec![],
        })
        .await
        .expect_err("ordinal gap must fail");
    assert!(matches!(error, AgentStoreError::Corrupt(_)));

    let history = store
        .load_history(run_id)
        .await
        .expect("history after rejection");
    assert_eq!(history.run.version, 1);
    assert_eq!(history.entries.len(), 1);
}
