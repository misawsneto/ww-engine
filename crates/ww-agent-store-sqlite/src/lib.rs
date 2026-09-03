mod coordinator;

pub use coordinator::{
    CoordinatedAgentRun, NewCoordinatedAgentRun, SqliteAgentCoordinator, SqliteAgentCoordinatorError,
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use uuid::Uuid;
use ww_agent_core::{
    AgentAppend, AgentEntry, AgentEntryData, AgentEntryId, AgentHistorySnapshot, AgentRecord,
    AgentRecordData, AgentRunId, AgentRunRecord, AgentStore, AgentStoreError, NewAgentRun,
};

const MIGRATION_0001: &str = include_str!("../migrations/0001_agent.sql");

#[derive(Clone, Debug)]
pub struct SqliteAgentStore {
    path: Arc<PathBuf>,
}

impl SqliteAgentStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Arc::new(path.into()),
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    async fn run<R, F>(&self, f: F) -> Result<R, AgentStoreError>
    where
        R: Send + 'static,
        F: FnOnce(&mut Connection) -> Result<R, AgentStoreError> + Send + 'static,
    {
        let path = Arc::clone(&self.path);
        tokio::task::spawn_blocking(move || {
            let mut connection = Connection::open(path.as_path()).map_err(backend)?;
            configure_connection(&connection)?;
            f(&mut connection)
        })
        .await
        .map_err(|error| AgentStoreError::Backend(format!("sqlite worker join error: {error}")))?
    }
}

fn configure_connection(connection: &Connection) -> Result<(), AgentStoreError> {
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(backend)?;
    connection
        .execute_batch("pragma foreign_keys = on;")
        .map_err(backend)?;
    Ok(())
}

fn backend(error: rusqlite::Error) -> AgentStoreError {
    AgentStoreError::Backend(error.to_string())
}

fn corrupt(message: impl Into<String>) -> AgentStoreError {
    AgentStoreError::Corrupt(message.into())
}

fn parse_time(value: String, field: &str) -> Result<DateTime<Utc>, AgentStoreError> {
    DateTime::parse_from_rfc3339(&value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| corrupt(format!("invalid {field}: {error}")))
}

fn parse_uuid(value: String, field: &str) -> Result<Uuid, AgentStoreError> {
    Uuid::parse_str(&value).map_err(|error| corrupt(format!("invalid {field}: {error}")))
}

fn to_i64(value: u64, field: &str) -> Result<i64, AgentStoreError> {
    i64::try_from(value).map_err(|_| AgentStoreError::Backend(format!("{field} overflow")))
}

fn to_u64(value: i64, field: &str) -> Result<u64, AgentStoreError> {
    u64::try_from(value).map_err(|_| corrupt(format!("negative {field}")))
}

fn get_run_conn(
    connection: &Connection,
    id: AgentRunId,
) -> Result<AgentRunRecord, AgentStoreError> {
    let row = connection
        .query_row(
            "select configuration_json, created_at, version from agent_runs where id = ?1",
            [id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .optional()
        .map_err(backend)?
        .ok_or_else(|| AgentStoreError::NotFound(id.to_string()))?;

    Ok(AgentRunRecord {
        id,
        configuration: serde_json::from_str(&row.0)
            .map_err(|error| corrupt(format!("invalid agent run configuration: {error}")))?,
        created_at: parse_time(row.1, "agent run created_at")?,
        version: to_u64(row.2, "agent run version")?,
    })
}

fn entry_kind(data: &AgentEntryData) -> &'static str {
    match data {
        AgentEntryData::UserInput { .. } => "user_input",
        AgentEntryData::AssistantMessage { .. } => "assistant_message",
        AgentEntryData::ModelVisibleToolResult { .. } => "model_visible_tool_result",
    }
}

fn validate_initial_run(new: &NewAgentRun) -> Result<(), AgentStoreError> {
    if new.initial_entry.run_id != new.id {
        return Err(corrupt(format!(
            "initial entry run {} does not match new run {}",
            new.initial_entry.run_id, new.id
        )));
    }
    if new.initial_entry.ordinal != 1 {
        return Err(corrupt(format!(
            "initial entry ordinal must be 1, got {}",
            new.initial_entry.ordinal
        )));
    }
    if !matches!(new.initial_entry.data, AgentEntryData::UserInput { .. }) {
        return Err(corrupt("initial Agent entry must be user_input"));
    }
    Ok(())
}

fn max_ordinal(
    connection: &Connection,
    run_id: AgentRunId,
) -> Result<u64, AgentStoreError> {
    let value = connection
        .query_row(
            "select coalesce(max(ordinal), 0) from agent_entries where run_id = ?1",
            [run_id.to_string()],
            |row| row.get::<_, i64>(0),
        )
        .map_err(backend)?;
    to_u64(value, "Agent entry ordinal")
}

fn max_sequence(
    connection: &Connection,
    run_id: AgentRunId,
) -> Result<u64, AgentStoreError> {
    let value = connection
        .query_row(
            "select coalesce(max(sequence), 0) from agent_records where run_id = ?1",
            [run_id.to_string()],
            |row| row.get::<_, i64>(0),
        )
        .map_err(backend)?;
    to_u64(value, "Agent record sequence")
}

fn validate_append(
    append: &AgentAppend,
    current_entry_ordinal: u64,
    current_record_sequence: u64,
) -> Result<(), AgentStoreError> {
    if append.entries.is_empty() && append.records.is_empty() {
        return Err(corrupt("Agent append must contain at least one entry or record"));
    }

    for (index, entry) in append.entries.iter().enumerate() {
        if entry.run_id != append.run_id {
            return Err(corrupt(format!(
                "entry run {} does not match append run {}",
                entry.run_id, append.run_id
            )));
        }
        let expected = current_entry_ordinal
            .checked_add(index as u64 + 1)
            .ok_or_else(|| AgentStoreError::Backend("Agent entry ordinal overflow".to_owned()))?;
        if entry.ordinal != expected {
            return Err(corrupt(format!(
                "Agent entry ordinal {} does not continue from {} (expected {expected})",
                entry.ordinal, current_entry_ordinal
            )));
        }
    }

    for (index, record) in append.records.iter().enumerate() {
        if record.run_id != append.run_id {
            return Err(corrupt(format!(
                "record run {} does not match append run {}",
                record.run_id, append.run_id
            )));
        }
        let expected = current_record_sequence
            .checked_add(index as u64 + 1)
            .ok_or_else(|| AgentStoreError::Backend("Agent record sequence overflow".to_owned()))?;
        if record.sequence != expected {
            return Err(corrupt(format!(
                "Agent record sequence {} does not continue from {} (expected {expected})",
                record.sequence, current_record_sequence
            )));
        }
    }
    Ok(())
}

fn insert_entry(
    connection: &Connection,
    entry: &AgentEntry,
) -> Result<(), AgentStoreError> {
    connection
        .execute(
            "insert into agent_entries (id, run_id, ordinal, created_at, kind, payload_json) values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                entry.id.to_string(),
                entry.run_id.to_string(),
                to_i64(entry.ordinal, "Agent entry ordinal")?,
                entry.created_at.to_rfc3339(),
                entry_kind(&entry.data),
                serde_json::to_string(&entry.data)
                    .map_err(|error| AgentStoreError::Backend(error.to_string()))?,
            ],
        )
        .map_err(backend)?;
    Ok(())
}

fn insert_record(
    connection: &Connection,
    record: &AgentRecord,
) -> Result<(), AgentStoreError> {
    connection
        .execute(
            "insert into agent_records (run_id, sequence, recorded_at, kind, payload_json) values (?1, ?2, ?3, ?4, ?5)",
            params![
                record.run_id.to_string(),
                to_i64(record.sequence, "Agent record sequence")?,
                record.recorded_at.to_rfc3339(),
                record.data.kind(),
                serde_json::to_string(&record.data)
                    .map_err(|error| AgentStoreError::Backend(error.to_string()))?,
            ],
        )
        .map_err(backend)?;
    Ok(())
}

fn load_entries(
    connection: &Connection,
    run_id: AgentRunId,
) -> Result<Vec<AgentEntry>, AgentStoreError> {
    let mut statement = connection
        .prepare(
            "select id, ordinal, created_at, kind, payload_json from agent_entries where run_id = ?1 order by ordinal asc",
        )
        .map_err(backend)?;
    let rows = statement
        .query_map([run_id.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(backend)?;

    let mut entries = Vec::new();
    for row in rows {
        let row = row.map_err(backend)?;
        let data: AgentEntryData = serde_json::from_str(&row.4)
            .map_err(|error| corrupt(format!("invalid Agent entry payload: {error}")))?;
        if row.3 != entry_kind(&data) {
            return Err(corrupt(format!(
                "Agent entry kind {} does not match payload kind {}",
                row.3,
                entry_kind(&data)
            )));
        }
        entries.push(AgentEntry {
            id: AgentEntryId::from_uuid(parse_uuid(row.0, "Agent entry id")?),
            run_id,
            ordinal: to_u64(row.1, "Agent entry ordinal")?,
            created_at: parse_time(row.2, "Agent entry created_at")?,
            data,
        });
    }
    Ok(entries)
}

fn load_records(
    connection: &Connection,
    run_id: AgentRunId,
) -> Result<Vec<AgentRecord>, AgentStoreError> {
    let mut statement = connection
        .prepare(
            "select sequence, recorded_at, kind, payload_json from agent_records where run_id = ?1 order by sequence asc",
        )
        .map_err(backend)?;
    let rows = statement
        .query_map([run_id.to_string()], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(backend)?;

    let mut records = Vec::new();
    for row in rows {
        let row = row.map_err(backend)?;
        let data: AgentRecordData = serde_json::from_str(&row.3)
            .map_err(|error| corrupt(format!("invalid Agent record payload: {error}")))?;
        if row.2 != data.kind() {
            return Err(corrupt(format!(
                "Agent record kind {} does not match payload kind {}",
                row.2,
                data.kind()
            )));
        }
        records.push(AgentRecord {
            run_id,
            sequence: to_u64(row.0, "Agent record sequence")?,
            recorded_at: parse_time(row.1, "Agent record recorded_at")?,
            data,
        });
    }
    Ok(records)
}

pub(crate) fn insert_new_agent_run_tx(
    transaction: &rusqlite::Transaction<'_>,
    new: &NewAgentRun,
) -> Result<(), AgentStoreError> {
    validate_initial_run(new)?;
    transaction
        .execute(
            "insert into agent_runs (id, configuration_json, created_at, version) values (?1, ?2, ?3, 1)",
            params![
                new.id.to_string(),
                serde_json::to_string(&new.configuration)
                    .map_err(|error| AgentStoreError::Backend(error.to_string()))?,
                new.created_at.to_rfc3339(),
            ],
        )
        .map_err(backend)?;
    insert_entry(transaction, &new.initial_entry)?;
    Ok(())
}

#[async_trait]
impl AgentStore for SqliteAgentStore {
    async fn migrate(&self) -> Result<(), AgentStoreError> {
        self.run(|connection| connection.execute_batch(MIGRATION_0001).map_err(backend))
            .await
    }

    async fn create_run(&self, new: NewAgentRun) -> Result<AgentRunRecord, AgentStoreError> {
        validate_initial_run(&new)?;
        self.run(move |connection| {
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(backend)?;
            insert_new_agent_run_tx(&transaction, &new)?;
            transaction.commit().map_err(backend)?;
            get_run_conn(connection, new.id)
        })
        .await
    }

    async fn get_run(&self, id: AgentRunId) -> Result<AgentRunRecord, AgentStoreError> {
        self.run(move |connection| get_run_conn(connection, id)).await
    }

    async fn append(&self, append: AgentAppend) -> Result<AgentRunRecord, AgentStoreError> {
        self.run(move |connection| {
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(backend)?;
            let current = get_run_conn(&transaction, append.run_id)?;
            if current.version != append.expected_version {
                return Err(AgentStoreError::Conflict {
                    id: append.run_id.to_string(),
                    expected: append.expected_version,
                    actual: current.version,
                });
            }

            let current_entry_ordinal = max_ordinal(&transaction, append.run_id)?;
            let current_record_sequence = max_sequence(&transaction, append.run_id)?;
            validate_append(&append, current_entry_ordinal, current_record_sequence)?;

            for entry in &append.entries {
                insert_entry(&transaction, entry)?;
            }
            for record in &append.records {
                insert_record(&transaction, record)?;
            }

            let next_version = current
                .version
                .checked_add(1)
                .ok_or_else(|| AgentStoreError::Backend("Agent run version overflow".to_owned()))?;
            let changed = transaction
                .execute(
                    "update agent_runs set version = ?1 where id = ?2 and version = ?3",
                    params![
                        to_i64(next_version, "Agent run version")?,
                        append.run_id.to_string(),
                        to_i64(append.expected_version, "Agent run version")?,
                    ],
                )
                .map_err(backend)?;
            if changed != 1 {
                let actual = get_run_conn(&transaction, append.run_id)?.version;
                return Err(AgentStoreError::Conflict {
                    id: append.run_id.to_string(),
                    expected: append.expected_version,
                    actual,
                });
            }

            transaction.commit().map_err(backend)?;
            get_run_conn(connection, append.run_id)
        })
        .await
    }

    async fn load_history(
        &self,
        id: AgentRunId,
    ) -> Result<AgentHistorySnapshot, AgentStoreError> {
        self.run(move |connection| {
            let run = get_run_conn(connection, id)?;
            let entries = load_entries(connection, id)?;
            let records = load_records(connection, id)?;
            Ok(AgentHistorySnapshot {
                run,
                entries,
                records,
            })
        })
        .await
    }
}
