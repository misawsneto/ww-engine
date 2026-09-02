use chrono::Utc;
use tempfile::TempDir;
use ww_store::{ExecutionMutation, ExecutionPatch, NewExecution, RuntimeStore, StoreError};
use ww_store_sqlite::SqliteRuntimeStore;
use ww_types::{
    EventId, EventVisibility, ExecutionEventData, ExecutionId, ExecutionKind, ExecutionStatus,
};

#[tokio::test]
async fn stale_expected_version_is_rejected_without_partial_commit() {
    let temp = TempDir::new().expect("temp dir");
    let store = SqliteRuntimeStore::new(temp.path().join("runtime.db"));
    store.migrate().await.expect("migrate");

    let id = ExecutionId::new();
    let created = store
        .create_execution(NewExecution {
            id,
            kind: ExecutionKind::synthetic(),
            configuration_digest: "configuration".to_owned(),
            created_at: Utc::now(),
            deadline: None,
            event_id: EventId::new(),
        })
        .await
        .expect("create");
    assert_eq!(created.version, 1);

    let started = store
        .mutate_execution(ExecutionMutation {
            execution_id: id,
            expected_version: 1,
            patch: ExecutionPatch {
                status: Some(ExecutionStatus::Running),
                started_at: Some(Utc::now()),
                ..ExecutionPatch::default()
            },
            event_id: EventId::new(),
            occurred_at: Utc::now(),
            visibility: EventVisibility::Public,
            event: ExecutionEventData::Started,
        })
        .await
        .expect("start");
    assert_eq!(started.version, 2);

    let conflict = store
        .mutate_execution(ExecutionMutation {
            execution_id: id,
            expected_version: 1,
            patch: ExecutionPatch {
                status: Some(ExecutionStatus::Failed),
                finished_at: Some(Utc::now()),
                ..ExecutionPatch::default()
            },
            event_id: EventId::new(),
            occurred_at: Utc::now(),
            visibility: EventVisibility::Public,
            event: ExecutionEventData::Failed {
                error: serde_json::json!({"code": "stale-writer"}),
            },
        })
        .await
        .expect_err("stale mutation must conflict");

    assert!(matches!(
        conflict,
        StoreError::Conflict {
            expected: 1,
            actual: 2,
            ..
        }
    ));

    let after = store.get_execution(id).await.expect("read after conflict");
    assert_eq!(after.status, ExecutionStatus::Running);
    assert_eq!(after.version, 2);

    let events = store
        .list_execution_events(id, 0, 10)
        .await
        .expect("events after conflict");
    assert_eq!(events.len(), 2);
    assert_eq!(events.last().expect("last event").sequence, 2);
}
