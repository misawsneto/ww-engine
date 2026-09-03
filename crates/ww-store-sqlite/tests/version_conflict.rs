use chrono::Utc;
use rusqlite::Connection;
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

    let started_at = Utc::now();
    let started = store
        .mutate_execution(ExecutionMutation {
            execution_id: id,
            expected_version: 1,
            patch: ExecutionPatch {
                status: Some(ExecutionStatus::Running),
                started_at: Some(started_at),
                ..ExecutionPatch::default()
            },
            event_id: EventId::new(),
            occurred_at: started_at,
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

#[tokio::test]
async fn mismatched_state_patch_and_event_is_rejected_without_mutation() {
    let temp = TempDir::new().expect("temp dir");
    let store = SqliteRuntimeStore::new(temp.path().join("runtime.db"));
    store.migrate().await.expect("migrate");
    let id = ExecutionId::new();
    store
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

    let error = store
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
            event: ExecutionEventData::Started,
        })
        .await
        .expect_err("mismatched patch must fail");
    assert!(matches!(error, StoreError::Invalid(_)));

    let snapshot = store
        .load_execution_history(id)
        .await
        .expect("unchanged snapshot");
    assert_eq!(snapshot.record.status, ExecutionStatus::Pending);
    assert_eq!(snapshot.record.version, 1);
    assert_eq!(snapshot.events.len(), 1);
}

#[tokio::test]
async fn unsupported_event_payload_version_fails_closed() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.db");
    let store = SqliteRuntimeStore::new(&path);
    store.migrate().await.expect("migrate");
    let id = ExecutionId::new();
    store
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
    Connection::open(&path)
        .expect("open database")
        .execute(
            "update execution_events set payload_version = 2 where execution_id = ?1",
            [id.to_string()],
        )
        .expect("tamper version");

    assert!(matches!(
        store.list_execution_events(id, 0, 10).await,
        Err(StoreError::UnsupportedVersion { version: 2, .. })
    ));
}

#[tokio::test]
async fn event_kind_must_match_deserialized_payload() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.db");
    let store = SqliteRuntimeStore::new(&path);
    store.migrate().await.expect("migrate");
    let id = ExecutionId::new();
    store
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
    Connection::open(&path)
        .expect("open database")
        .execute(
            "update execution_events set kind = 'execution_failed' where execution_id = ?1",
            [id.to_string()],
        )
        .expect("tamper kind");

    assert!(matches!(
        store.list_execution_events(id, 0, 10).await,
        Err(StoreError::Corrupt(_))
    ));
}
