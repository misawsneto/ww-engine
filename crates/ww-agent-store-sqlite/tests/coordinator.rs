use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};
use tempfile::TempDir;
use ww_agent_core::{
    AgentEntry, AgentEntryData, AgentEntryId, AgentRunId, AgentStore, NewAgentRun,
};
use ww_agent_store_sqlite::{
    NewCoordinatedAgentRun, SqliteAgentCoordinator, SqliteAgentCoordinatorError, SqliteAgentStore,
};
use ww_store::{RuntimeStore, StoreError};
use ww_store_sqlite::SqliteRuntimeStore;
use ww_types::{ExecutionId, ExecutionStatus};

fn initial_entry(run_id: AgentRunId) -> AgentEntry {
    AgentEntry {
        id: AgentEntryId::new(),
        run_id,
        ordinal: 1,
        created_at: Utc::now(),
        data: AgentEntryData::UserInput {
            text: "coordinated".to_owned(),
        },
    }
}

async fn coordinator(
    temp: &TempDir,
) -> (SqliteAgentCoordinator, SqliteRuntimeStore, SqliteAgentStore) {
    let path = temp.path().join("runtime.db");
    let runtime = SqliteRuntimeStore::new(&path);
    let agent = SqliteAgentStore::new(&path);
    let coordinator =
        SqliteAgentCoordinator::new(runtime.clone(), agent.clone()).expect("same database path");
    coordinator.migrate().await.expect("migrate coordinator");
    (coordinator, runtime, agent)
}

#[tokio::test]
async fn common_execution_agent_run_and_link_commit_atomically() {
    let temp = TempDir::new().expect("temp dir");
    let (coordinator, runtime, agent) = coordinator(&temp).await;
    let execution_id = ExecutionId::new();
    let run_id = AgentRunId::new();
    let configuration = json!({"provider": "recorded", "model": "fixture"});
    let expected_digest = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&configuration).expect("serialize configuration"))
    );

    let created = coordinator
        .create_run(NewCoordinatedAgentRun {
            execution_id,
            run_id,
            configuration: configuration.clone(),
            created_at: Utc::now(),
            deadline: None,
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("coordinated create");

    assert_eq!(created.execution.id, execution_id);
    assert_eq!(created.execution.kind.as_str(), "agent");
    assert_eq!(created.execution.status, ExecutionStatus::Pending);
    assert_eq!(created.execution.configuration_digest, expected_digest);
    assert_eq!(created.execution.version, 1);
    assert_eq!(created.agent.id, run_id);
    assert_eq!(created.agent.configuration, configuration);
    assert_eq!(created.agent.version, 1);

    let events = runtime
        .list_execution_events(execution_id, 0, 10)
        .await
        .expect("common creation event");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].sequence, 1);

    let history = agent.load_history(run_id).await.expect("Agent history");
    assert_eq!(history.entries.len(), 1);
    assert!(history.records.is_empty());
    assert_eq!(
        coordinator
            .execution_for_agent(run_id)
            .await
            .expect("lookup execution"),
        Some(execution_id)
    );
    assert_eq!(
        coordinator
            .agent_for_execution(execution_id)
            .await
            .expect("lookup Agent"),
        Some(run_id)
    );
}

#[tokio::test]
async fn agent_insert_failure_rolls_back_preceding_common_creation() {
    let temp = TempDir::new().expect("temp dir");
    let (coordinator, runtime, agent) = coordinator(&temp).await;
    let run_id = AgentRunId::new();
    agent
        .create_run(NewAgentRun {
            id: run_id,
            configuration: json!({"preexisting": true}),
            created_at: Utc::now(),
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect("seed conflicting Agent run");

    let execution_id = ExecutionId::new();
    let error = coordinator
        .create_run(NewCoordinatedAgentRun {
            execution_id,
            run_id,
            configuration: json!({"new": true}),
            created_at: Utc::now(),
            deadline: None,
            initial_entry: initial_entry(run_id),
        })
        .await
        .expect_err("Agent primary-key conflict must abort whole transaction");
    assert!(matches!(error, SqliteAgentCoordinatorError::Conflict(_)));

    assert!(matches!(
        runtime.get_execution(execution_id).await,
        Err(StoreError::NotFound(_))
    ));
    assert_eq!(
        coordinator
            .agent_for_execution(execution_id)
            .await
            .expect("link lookup after rollback"),
        None
    );
    let existing = agent
        .get_run(run_id)
        .await
        .expect("existing Agent run remains");
    assert_eq!(existing.version, 1);
}

#[tokio::test]
async fn exact_create_retry_returns_existing_run_without_duplicate_history() {
    let temp = TempDir::new().expect("temp dir");
    let (coordinator, runtime, agent) = coordinator(&temp).await;
    let execution_id = ExecutionId::new();
    let run_id = AgentRunId::new();
    let request = NewCoordinatedAgentRun {
        execution_id,
        run_id,
        configuration: json!({"provider": "recorded"}),
        created_at: Utc::now(),
        deadline: None,
        initial_entry: initial_entry(run_id),
    };

    let first = coordinator
        .create_run(request.clone())
        .await
        .expect("first create");
    let retry = coordinator.create_run(request).await.expect("exact retry");

    assert_eq!(retry, first);
    assert_eq!(
        runtime
            .load_execution_history(execution_id)
            .await
            .expect("runtime history")
            .events
            .len(),
        1
    );
    let history = agent.load_history(run_id).await.expect("Agent history");
    assert_eq!(history.entries.len(), 1);
    assert!(history.records.is_empty());
}

#[tokio::test]
async fn conflicting_create_retry_rejects_without_mutating_existing_run() {
    let temp = TempDir::new().expect("temp dir");
    let (coordinator, runtime, agent) = coordinator(&temp).await;
    let execution_id = ExecutionId::new();
    let run_id = AgentRunId::new();
    let created_at = Utc::now();
    let initial_entry = initial_entry(run_id);
    let request = NewCoordinatedAgentRun {
        execution_id,
        run_id,
        configuration: json!({"provider": "recorded"}),
        created_at,
        deadline: None,
        initial_entry: initial_entry.clone(),
    };
    coordinator.create_run(request).await.expect("first create");

    let error = coordinator
        .create_run(NewCoordinatedAgentRun {
            execution_id,
            run_id,
            configuration: json!({"provider": "different"}),
            created_at,
            deadline: None,
            initial_entry,
        })
        .await
        .expect_err("changed retry must conflict");
    assert!(matches!(error, SqliteAgentCoordinatorError::Conflict(_)));

    assert_eq!(
        runtime
            .load_execution_history(execution_id)
            .await
            .expect("runtime history")
            .events
            .len(),
        1
    );
    let history = agent.load_history(run_id).await.expect("Agent history");
    assert_eq!(history.run.configuration, json!({"provider": "recorded"}));
    assert_eq!(history.entries.len(), 1);
    assert!(history.records.is_empty());
}

#[test]
fn coordinator_rejects_different_database_paths() {
    let temp = TempDir::new().expect("temp dir");
    let runtime = SqliteRuntimeStore::new(temp.path().join("runtime.db"));
    let agent = SqliteAgentStore::new(temp.path().join("agent.db"));
    assert!(matches!(
        SqliteAgentCoordinator::new(runtime, agent),
        Err(SqliteAgentCoordinatorError::PathMismatch)
    ));
}
