use crate::{SqliteAgentStore, insert_new_agent_run_tx};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{path::PathBuf, sync::Arc, time::Duration};
use thiserror::Error;
use uuid::Uuid;
use ww_agent_core::{
    AgentEntry, AgentRunId, AgentRunRecord, AgentStore, AgentStoreError, NewAgentRun,
};
use ww_store::{NewExecution, RuntimeStore, StoreError};
use ww_store_sqlite::{SqliteRuntimeStore, insert_new_execution_tx};
use ww_types::{EventId, ExecutionId, ExecutionKind, ExecutionRecord};

const COORDINATOR_MIGRATION_0001: &str = r#"
create table if not exists agent_execution_links (
    agent_run_id  text primary key references agent_runs(id),
    execution_id  text not null unique references executions(id)
);
"#;

#[derive(Clone, Debug)]
pub struct NewCoordinatedAgentRun {
    pub execution_id: ExecutionId,
    pub run_id: AgentRunId,
    pub configuration: Value,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub initial_entry: AgentEntry,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoordinatedAgentRun {
    pub execution: ExecutionRecord,
    pub agent: AgentRunRecord,
}

#[derive(Debug, Error)]
pub enum SqliteAgentCoordinatorError {
    #[error("runtime and Agent SQLite stores must use the same database path")]
    PathMismatch,
    #[error("invalid coordinated Agent run: {0}")]
    Invalid(String),
    #[error(transparent)]
    Runtime(#[from] StoreError),
    #[error(transparent)]
    Agent(#[from] AgentStoreError),
    #[error("SQLite coordinator backend error: {0}")]
    Backend(String),
}

#[derive(Clone, Debug)]
pub struct SqliteAgentCoordinator {
    path: Arc<PathBuf>,
    runtime_store: SqliteRuntimeStore,
    agent_store: SqliteAgentStore,
}

impl SqliteAgentCoordinator {
    pub fn new(
        runtime_store: SqliteRuntimeStore,
        agent_store: SqliteAgentStore,
    ) -> Result<Self, SqliteAgentCoordinatorError> {
        if runtime_store.path() != agent_store.path() {
            return Err(SqliteAgentCoordinatorError::PathMismatch);
        }
        Ok(Self {
            path: Arc::new(runtime_store.path().to_path_buf()),
            runtime_store,
            agent_store,
        })
    }

    pub fn runtime_store(&self) -> &SqliteRuntimeStore {
        &self.runtime_store
    }

    pub fn agent_store(&self) -> &SqliteAgentStore {
        &self.agent_store
    }

    pub async fn migrate(&self) -> Result<(), SqliteAgentCoordinatorError> {
        self.runtime_store.migrate().await?;
        self.agent_store.migrate().await?;
        self.run(|connection| {
            connection
                .execute_batch(COORDINATOR_MIGRATION_0001)
                .map_err(backend)
        })
        .await
    }

    pub async fn create_run(
        &self,
        new: NewCoordinatedAgentRun,
    ) -> Result<CoordinatedAgentRun, SqliteAgentCoordinatorError> {
        if new.initial_entry.run_id != new.run_id {
            return Err(SqliteAgentCoordinatorError::Invalid(format!(
                "initial entry run {} does not match Agent run {}",
                new.initial_entry.run_id, new.run_id
            )));
        }
        let configuration_bytes = serde_json::to_vec(&new.configuration)
            .map_err(|error| SqliteAgentCoordinatorError::Backend(error.to_string()))?;
        let configuration_digest = format!("{:x}", Sha256::digest(&configuration_bytes));
        let execution = NewExecution {
            id: new.execution_id,
            kind: ExecutionKind::new("agent").expect("static Agent execution kind is valid"),
            configuration_digest,
            created_at: new.created_at,
            deadline: new.deadline.clone(),
            event_id: EventId::new(),
        };
        let agent = NewAgentRun {
            id: new.run_id,
            configuration: new.configuration,
            created_at: new.created_at,
            initial_entry: new.initial_entry,
        };
        let execution_id = execution.id;
        let run_id = agent.id;

        self.run(move |connection| {
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(backend)?;
            insert_new_execution_tx(&transaction, &execution)
                .map_err(SqliteAgentCoordinatorError::Runtime)?;
            insert_new_agent_run_tx(&transaction, &agent)
                .map_err(SqliteAgentCoordinatorError::Agent)?;
            transaction
                .execute(
                    "insert into agent_execution_links (agent_run_id, execution_id) values (?1, ?2)",
                    params![run_id.to_string(), execution_id.to_string()],
                )
                .map_err(backend)?;
            transaction.commit().map_err(backend)?;
            Ok(())
        })
        .await?;

        Ok(CoordinatedAgentRun {
            execution: self.runtime_store.get_execution(execution_id).await?,
            agent: self.agent_store.get_run(run_id).await?,
        })
    }

    pub async fn execution_for_agent(
        &self,
        run_id: AgentRunId,
    ) -> Result<Option<ExecutionId>, SqliteAgentCoordinatorError> {
        self.run(move |connection| {
            let value = connection
                .query_row(
                    "select execution_id from agent_execution_links where agent_run_id = ?1",
                    [run_id.to_string()],
                    |row| row.get::<_, String>(0),
                )
                .optional()
                .map_err(backend)?;
            value
                .map(|value| {
                    Uuid::parse_str(&value)
                        .map(ExecutionId::from_uuid)
                        .map_err(|error| {
                            SqliteAgentCoordinatorError::Backend(format!(
                                "invalid execution link id: {error}"
                            ))
                        })
                })
                .transpose()
        })
        .await
    }

    pub async fn agent_for_execution(
        &self,
        execution_id: ExecutionId,
    ) -> Result<Option<AgentRunId>, SqliteAgentCoordinatorError> {
        self.run(move |connection| {
            let value = connection
                .query_row(
                    "select agent_run_id from agent_execution_links where execution_id = ?1",
                    [execution_id.to_string()],
                    |row| row.get::<_, String>(0),
                )
                .optional()
                .map_err(backend)?;
            value
                .map(|value| {
                    Uuid::parse_str(&value)
                        .map(AgentRunId::from_uuid)
                        .map_err(|error| {
                            SqliteAgentCoordinatorError::Backend(format!(
                                "invalid Agent run link id: {error}"
                            ))
                        })
                })
                .transpose()
        })
        .await
    }

    async fn run<R, F>(&self, f: F) -> Result<R, SqliteAgentCoordinatorError>
    where
        R: Send + 'static,
        F: FnOnce(&mut Connection) -> Result<R, SqliteAgentCoordinatorError> + Send + 'static,
    {
        let path = Arc::clone(&self.path);
        tokio::task::spawn_blocking(move || {
            let mut connection = Connection::open(path.as_path()).map_err(backend)?;
            connection
                .busy_timeout(Duration::from_secs(5))
                .map_err(backend)?;
            connection
                .execute_batch("pragma foreign_keys = on;")
                .map_err(backend)?;
            f(&mut connection)
        })
        .await
        .map_err(|error| {
            SqliteAgentCoordinatorError::Backend(format!("sqlite worker join error: {error}"))
        })?
    }
}

fn backend(error: rusqlite::Error) -> SqliteAgentCoordinatorError {
    SqliteAgentCoordinatorError::Backend(error.to_string())
}
