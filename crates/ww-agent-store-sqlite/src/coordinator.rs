use crate::{SqliteAgentStore, get_run_conn, insert_new_agent_run_tx, load_entries};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;
use uuid::Uuid;
use ww_agent_core::{
    AgentEntry, AgentRunId, AgentRunRecord, AgentStore, AgentStoreError, NewAgentRun,
};
use ww_store::{NewExecution, RuntimeStore, StoreError};
use ww_store_sqlite::{
    ComponentMigration, SqliteRuntimeStore, apply_component_migrations, configure_connection,
    get_execution_on_connection, insert_new_execution_tx, is_transient_sqlite_error,
};
use ww_types::{EventId, ExecutionId, ExecutionKind, ExecutionRecord};

const COORDINATOR_MIGRATION_0001: &str = include_str!("../migrations/0001_coordinator.sql");
const COORDINATOR_MIGRATIONS: &[ComponentMigration] = &[ComponentMigration {
    version: 1,
    sql: COORDINATOR_MIGRATION_0001,
}];

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
    #[error("coordinated Agent run conflict: {0}")]
    Conflict(String),
    #[error(transparent)]
    Runtime(#[from] StoreError),
    #[error(transparent)]
    Agent(#[from] AgentStoreError),
    #[error("SQLite coordinator data is corrupt: {0}")]
    Corrupt(String),
    #[error("transient SQLite coordinator backend error: {0}")]
    TransientBackend(String),
    #[error("permanent SQLite coordinator backend error: {0}")]
    PermanentBackend(String),
    #[error("SQLite coordinator migration error: {0}")]
    Migration(String),
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
            apply_component_migrations(connection, "agent_coordinator", COORDINATOR_MIGRATIONS)
                .map_err(|error| SqliteAgentCoordinatorError::Migration(error.to_string()))
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
            .map_err(|error| SqliteAgentCoordinatorError::PermanentBackend(error.to_string()))?;
        let configuration_digest = format!("{:x}", Sha256::digest(&configuration_bytes));
        let execution = NewExecution {
            id: new.execution_id,
            kind: ExecutionKind::new("agent").expect("static Agent execution kind is valid"),
            configuration_digest,
            created_at: new.created_at,
            deadline: new.deadline,
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
            let existing_link = find_link(&transaction, run_id, execution_id)?;
            let result = if let Some((linked_run, linked_execution)) = existing_link {
                if linked_run != run_id || linked_execution != execution_id {
                    return Err(SqliteAgentCoordinatorError::Conflict(format!(
                        "requested {run_id}/{execution_id}, existing link is {linked_run}/{linked_execution}"
                    )));
                }
                let existing_execution = get_execution_on_connection(&transaction, execution_id)
                    .map_err(SqliteAgentCoordinatorError::Runtime)?;
                let existing_agent = get_run_conn(&transaction, run_id)
                    .map_err(SqliteAgentCoordinatorError::Agent)?;
                let initial_entry = load_entries(&transaction, run_id)
                    .map_err(SqliteAgentCoordinatorError::Agent)?
                    .into_iter()
                    .next()
                    .ok_or_else(|| {
                        SqliteAgentCoordinatorError::Conflict(format!(
                            "linked Agent run {run_id} has no initial entry"
                        ))
                    })?;
                if existing_execution.kind != execution.kind
                    || existing_execution.configuration_digest != execution.configuration_digest
                    || existing_execution.created_at != execution.created_at
                    || existing_execution.deadline != execution.deadline
                    || existing_agent.configuration != agent.configuration
                    || existing_agent.created_at != agent.created_at
                    || initial_entry != agent.initial_entry
                {
                    return Err(SqliteAgentCoordinatorError::Conflict(format!(
                        "retry inputs do not match existing coordinated Agent run {run_id}/{execution_id}"
                    )));
                }
                CoordinatedAgentRun {
                    execution: existing_execution,
                    agent: existing_agent,
                }
            } else {
                if execution_exists(&transaction, execution_id)? || agent_exists(&transaction, run_id)?
                {
                    return Err(SqliteAgentCoordinatorError::Conflict(format!(
                        "execution {execution_id} or Agent run {run_id} already exists without the requested link"
                    )));
                }
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
                CoordinatedAgentRun {
                    execution: get_execution_on_connection(&transaction, execution_id)
                        .map_err(SqliteAgentCoordinatorError::Runtime)?,
                    agent: get_run_conn(&transaction, run_id)
                        .map_err(SqliteAgentCoordinatorError::Agent)?,
                }
            };
            transaction.commit().map_err(backend)?;
            Ok(result)
        })
        .await
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
                            SqliteAgentCoordinatorError::Corrupt(format!(
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
                            SqliteAgentCoordinatorError::Corrupt(format!(
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
            configure_connection(&connection).map_err(|error| {
                if error.is_transient() {
                    SqliteAgentCoordinatorError::TransientBackend(error.to_string())
                } else {
                    SqliteAgentCoordinatorError::PermanentBackend(error.to_string())
                }
            })?;
            f(&mut connection)
        })
        .await
        .map_err(|error| {
            SqliteAgentCoordinatorError::PermanentBackend(format!(
                "sqlite worker join error: {error}"
            ))
        })?
    }
}

fn backend(error: rusqlite::Error) -> SqliteAgentCoordinatorError {
    if is_transient_sqlite_error(&error) {
        SqliteAgentCoordinatorError::TransientBackend(error.to_string())
    } else {
        SqliteAgentCoordinatorError::PermanentBackend(error.to_string())
    }
}

fn find_link(
    connection: &Connection,
    run_id: AgentRunId,
    execution_id: ExecutionId,
) -> Result<Option<(AgentRunId, ExecutionId)>, SqliteAgentCoordinatorError> {
    let mut statement = connection
        .prepare("select agent_run_id, execution_id from agent_execution_links where agent_run_id = ?1 or execution_id = ?2")
        .map_err(backend)?;
    let rows = statement
        .query_map(
            params![run_id.to_string(), execution_id.to_string()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .map_err(backend)?;
    let mut links = Vec::new();
    for row in rows {
        let (run, execution) = row.map_err(backend)?;
        links.push((
            Uuid::parse_str(&run)
                .map(AgentRunId::from_uuid)
                .map_err(|error| {
                    SqliteAgentCoordinatorError::Corrupt(format!(
                        "invalid linked Agent run id: {error}"
                    ))
                })?,
            Uuid::parse_str(&execution)
                .map(ExecutionId::from_uuid)
                .map_err(|error| {
                    SqliteAgentCoordinatorError::Corrupt(format!(
                        "invalid linked execution id: {error}"
                    ))
                })?,
        ));
    }
    match links.as_slice() {
        [] => Ok(None),
        [link] => Ok(Some(*link)),
        _ => Err(SqliteAgentCoordinatorError::Conflict(format!(
            "requested identifiers {run_id}/{execution_id} resolve to different existing links"
        ))),
    }
}

fn execution_exists(
    connection: &Connection,
    execution_id: ExecutionId,
) -> Result<bool, SqliteAgentCoordinatorError> {
    connection
        .query_row(
            "select exists(select 1 from executions where id = ?1)",
            [execution_id.to_string()],
            |row| row.get::<_, bool>(0),
        )
        .map_err(backend)
}

fn agent_exists(
    connection: &Connection,
    run_id: AgentRunId,
) -> Result<bool, SqliteAgentCoordinatorError> {
    connection
        .query_row(
            "select exists(select 1 from agent_runs where id = ?1)",
            [run_id.to_string()],
            |row| row.get::<_, bool>(0),
        )
        .map_err(backend)
}
