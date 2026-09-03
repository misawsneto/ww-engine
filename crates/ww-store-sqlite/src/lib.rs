use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use ww_store::{ExecutionMutation, NewExecution, RuntimeStore, StoreError};
use ww_types::{
    ArtifactId, ArtifactRef, CancelReason, EventId, EventVisibility, ExecutionEvent,
    ExecutionEventData, ExecutionId, ExecutionKind, ExecutionRecord, ExecutionStatus,
};

const MIGRATION_0001: &str = include_str!("../migrations/0001_runtime.sql");

#[derive(Clone, Debug)]
pub struct SqliteRuntimeStore {
    path: Arc<PathBuf>,
}

impl SqliteRuntimeStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Arc::new(path.into()),
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    async fn run<R, F>(&self, f: F) -> Result<R, StoreError>
    where
        R: Send + 'static,
        F: FnOnce(&mut Connection) -> Result<R, StoreError> + Send + 'static,
    {
        let path = Arc::clone(&self.path);
        tokio::task::spawn_blocking(move || {
            let mut connection = Connection::open(path.as_path()).map_err(backend)?;
            configure_connection(&connection)?;
            f(&mut connection)
        })
        .await
        .map_err(|error| StoreError::Backend(format!("sqlite worker join error: {error}")))?
    }
}

fn configure_connection(connection: &Connection) -> Result<(), StoreError> {
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(backend)?;
    connection
        .execute_batch("pragma foreign_keys = on;")
        .map_err(backend)?;
    Ok(())
}

pub fn migrate_runtime_schema(connection: &Connection) -> Result<(), StoreError> {
    connection.execute_batch(MIGRATION_0001).map_err(backend)
}

pub fn insert_new_execution_tx(
    transaction: &rusqlite::Transaction<'_>,
    new: &NewExecution,
) -> Result<(), StoreError> {
    transaction
        .execute(
            "insert into executions (id, kind, status, configuration_digest, created_at, deadline, version) values (?1, ?2, 'pending', ?3, ?4, ?5, 1)",
            params![
                new.id.to_string(),
                new.kind.as_str(),
                new.configuration_digest.as_str(),
                new.created_at.to_rfc3339(),
                new.deadline.as_ref().map(|value| value.to_rfc3339()),
            ],
        )
        .map_err(backend)?;
    let data = ExecutionEventData::Created {
        kind: new.kind.clone(),
        configuration_digest: new.configuration_digest.clone(),
    };
    transaction
        .execute(
            "insert into execution_events (id, execution_id, sequence, occurred_at, kind, payload_version, visibility, payload_json) values (?1, ?2, 1, ?3, ?4, 1, 'public', ?5)",
            params![
                new.event_id.to_string(),
                new.id.to_string(),
                new.created_at.to_rfc3339(),
                data.kind(),
                serde_json::to_string(&data)
                    .map_err(|error| StoreError::Backend(error.to_string()))?,
            ],
        )
        .map_err(backend)?;
    Ok(())
}

fn backend(error: rusqlite::Error) -> StoreError {
    StoreError::Backend(error.to_string())
}

fn parse_time(value: String, field: &str) -> Result<DateTime<Utc>, StoreError> {
    DateTime::parse_from_rfc3339(&value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| StoreError::Corrupt(format!("invalid {field}: {error}")))
}

fn parse_optional_time(
    value: Option<String>,
    field: &str,
) -> Result<Option<DateTime<Utc>>, StoreError> {
    value.map(|value| parse_time(value, field)).transpose()
}

fn parse_visibility(value: &str) -> Result<EventVisibility, StoreError> {
    match value {
        "public" => Ok(EventVisibility::Public),
        "internal" => Ok(EventVisibility::Internal),
        "sensitive" => Ok(EventVisibility::Sensitive),
        other => Err(StoreError::Corrupt(format!(
            "unknown event visibility: {other}"
        ))),
    }
}

fn get_execution_conn(
    connection: &Connection,
    id: ExecutionId,
) -> Result<ExecutionRecord, StoreError> {
    let row = connection
        .query_row(
            "select kind, status, configuration_digest, cancel_requested, cancel_reason_json, result_artifact_json, error_json, created_at, started_at, finished_at, deadline, version from executions where id = ?1",
            [id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<String>>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<String>>(10)?,
                    row.get::<_, i64>(11)?,
                ))
            },
        )
        .optional()
        .map_err(backend)?
        .ok_or_else(|| StoreError::NotFound(id.to_string()))?;

    let kind = ExecutionKind::new(row.0).map_err(|error| StoreError::Corrupt(error.to_owned()))?;
    let status = ExecutionStatus::from_str(&row.1)
        .map_err(|error| StoreError::Corrupt(error.to_string()))?;
    let cancel_reason = row
        .4
        .map(|value| {
            serde_json::from_str::<CancelReason>(&value)
                .map_err(|error| StoreError::Corrupt(error.to_string()))
        })
        .transpose()?;
    let result_artifact = row
        .5
        .map(|value| {
            serde_json::from_str::<ArtifactRef>(&value)
                .map_err(|error| StoreError::Corrupt(error.to_string()))
        })
        .transpose()?;
    let error = row
        .6
        .map(|value| {
            serde_json::from_str(&value).map_err(|error| StoreError::Corrupt(error.to_string()))
        })
        .transpose()?;

    Ok(ExecutionRecord {
        id,
        kind,
        status,
        configuration_digest: row.2,
        cancel_requested: row.3 != 0,
        cancel_reason,
        result_artifact,
        error,
        created_at: parse_time(row.7, "created_at")?,
        started_at: parse_optional_time(row.8, "started_at")?,
        finished_at: parse_optional_time(row.9, "finished_at")?,
        deadline: parse_optional_time(row.10, "deadline")?,
        version: u64::try_from(row.11)
            .map_err(|_| StoreError::Corrupt("negative execution version".to_owned()))?,
    })
}

#[async_trait]
impl RuntimeStore for SqliteRuntimeStore {
    async fn migrate(&self) -> Result<(), StoreError> {
        self.run(|connection| migrate_runtime_schema(connection))
            .await
    }

    async fn create_execution(&self, new: NewExecution) -> Result<ExecutionRecord, StoreError> {
        self.run(move |connection| {
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(backend)?;
            insert_new_execution_tx(&transaction, &new)?;
            transaction.commit().map_err(backend)?;
            get_execution_conn(connection, new.id)
        })
        .await
    }

    async fn get_execution(&self, id: ExecutionId) -> Result<ExecutionRecord, StoreError> {
        self.run(move |connection| get_execution_conn(connection, id))
            .await
    }

    async fn mutate_execution(
        &self,
        mutation: ExecutionMutation,
    ) -> Result<ExecutionRecord, StoreError> {
        self.run(move |connection| {
            let transaction = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(backend)?;
            let current = get_execution_conn(&transaction, mutation.execution_id)?;
            if current.version != mutation.expected_version {
                return Err(StoreError::Conflict {
                    id: mutation.execution_id.to_string(),
                    expected: mutation.expected_version,
                    actual: current.version,
                });
            }

            let status = mutation.patch.status.map(|value| value.as_str().to_owned());
            let started_at = mutation.patch.started_at.map(|value| value.to_rfc3339());
            let finished_at = mutation.patch.finished_at.map(|value| value.to_rfc3339());
            let cancel_reason = mutation.patch.cancel_reason
                .as_ref()
                .map(serde_json::to_string)
                .transpose()
                .map_err(|error| StoreError::Backend(error.to_string()))?;
            let result_artifact = mutation.patch.result_artifact
                .as_ref()
                .map(serde_json::to_string)
                .transpose()
                .map_err(|error| StoreError::Backend(error.to_string()))?;
            let error_json = mutation.patch.error
                .as_ref()
                .map(serde_json::to_string)
                .transpose()
                .map_err(|error| StoreError::Backend(error.to_string()))?;
            let next_version = current.version + 1;

            let changed = transaction
                .execute(
                    "update executions set status = coalesce(?1, status), started_at = coalesce(?2, started_at), finished_at = coalesce(?3, finished_at), cancel_requested = case when ?4 is null then cancel_requested else 1 end, cancel_reason_json = coalesce(?4, cancel_reason_json), result_artifact_json = coalesce(?5, result_artifact_json), error_json = coalesce(?6, error_json), version = ?7 where id = ?8 and version = ?9",
                    params![
                        status,
                        started_at,
                        finished_at,
                        cancel_reason,
                        result_artifact,
                        error_json,
                        i64::try_from(next_version).map_err(|_| StoreError::Backend("execution version overflow".to_owned()))?,
                        mutation.execution_id.to_string(),
                        i64::try_from(mutation.expected_version).map_err(|_| StoreError::Backend("execution version overflow".to_owned()))?,
                    ],
                )
                .map_err(backend)?;
            if changed != 1 {
                let actual = get_execution_conn(&transaction, mutation.execution_id)?.version;
                return Err(StoreError::Conflict {
                    id: mutation.execution_id.to_string(),
                    expected: mutation.expected_version,
                    actual,
                });
            }

            let sequence = next_version;
            transaction
                .execute(
                    "insert into execution_events (id, execution_id, sequence, occurred_at, kind, payload_version, visibility, payload_json) values (?1, ?2, ?3, ?4, ?5, 1, ?6, ?7)",
                    params![
                        mutation.event_id.to_string(),
                        mutation.execution_id.to_string(),
                        i64::try_from(sequence).map_err(|_| StoreError::Backend("event sequence overflow".to_owned()))?,
                        mutation.occurred_at.to_rfc3339(),
                        mutation.event.kind(),
                        mutation.visibility.as_str(),
                        serde_json::to_string(&mutation.event).map_err(|error| StoreError::Backend(error.to_string()))?,
                    ],
                )
                .map_err(backend)?;
            transaction.commit().map_err(backend)?;
            get_execution_conn(connection, mutation.execution_id)
        }).await
    }

    async fn list_execution_events(
        &self,
        id: ExecutionId,
        after_sequence: u64,
        limit: usize,
    ) -> Result<Vec<ExecutionEvent>, StoreError> {
        self.run(move |connection| {
            let mut statement = connection
                .prepare("select id, sequence, occurred_at, payload_version, visibility, payload_json from execution_events where execution_id = ?1 and sequence > ?2 order by sequence asc limit ?3")
                .map_err(backend)?;
            let rows = statement
                .query_map(
                    params![
                        id.to_string(),
                        i64::try_from(after_sequence).map_err(|_| StoreError::Backend("event sequence overflow".to_owned()))?,
                        i64::try_from(limit).map_err(|_| StoreError::Backend("event limit overflow".to_owned()))?,
                    ],
                    |row| {
                        Ok((
                            row.get::<_, String>(0)?,
                            row.get::<_, i64>(1)?,
                            row.get::<_, String>(2)?,
                            row.get::<_, i64>(3)?,
                            row.get::<_, String>(4)?,
                            row.get::<_, String>(5)?,
                        ))
                    },
                )
                .map_err(backend)?;

            let mut events = Vec::new();
            for row in rows {
                let row = row.map_err(backend)?;
                events.push(ExecutionEvent {
                    id: EventId::from_str(&row.0).map_err(|error| StoreError::Corrupt(error.to_string()))?,
                    execution_id: id,
                    sequence: u64::try_from(row.1).map_err(|_| StoreError::Corrupt("negative event sequence".to_owned()))?,
                    occurred_at: parse_time(row.2, "event occurred_at")?,
                    payload_version: u16::try_from(row.3).map_err(|_| StoreError::Corrupt("invalid event payload version".to_owned()))?,
                    visibility: parse_visibility(&row.4)?,
                    data: serde_json::from_str(&row.5).map_err(|error| StoreError::Corrupt(error.to_string()))?,
                });
            }
            Ok(events)
        }).await
    }

    async fn put_artifact(&self, artifact: ArtifactRef) -> Result<ArtifactRef, StoreError> {
        self.run(move |connection| {
            let created_at = Utc::now().to_rfc3339();
            connection
                .execute(
                    "insert or ignore into artifacts (id, digest, media_type, size_bytes, storage_uri, created_at) values (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![
                        artifact.id.to_string(),
                        artifact.digest,
                        artifact.media_type,
                        i64::try_from(artifact.size_bytes).map_err(|_| StoreError::Backend("artifact too large".to_owned()))?,
                        artifact.storage_uri,
                        created_at,
                    ],
                )
                .map_err(backend)?;
            get_artifact_by_digest_conn(connection, &artifact.digest)?.ok_or_else(|| StoreError::Corrupt("artifact insert did not become visible".to_owned()))
        }).await
    }

    async fn get_artifact_by_digest(
        &self,
        digest: &str,
    ) -> Result<Option<ArtifactRef>, StoreError> {
        let digest = digest.to_owned();
        self.run(move |connection| get_artifact_by_digest_conn(connection, &digest))
            .await
    }
}

fn get_artifact_by_digest_conn(
    connection: &Connection,
    digest: &str,
) -> Result<Option<ArtifactRef>, StoreError> {
    let row = connection
        .query_row(
            "select id, digest, media_type, size_bytes, storage_uri from artifacts where digest = ?1",
            [digest],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .optional()
        .map_err(backend)?;

    row.map(|row| {
        Ok(ArtifactRef {
            id: ArtifactId::from_str(&row.0)
                .map_err(|error| StoreError::Corrupt(error.to_string()))?,
            digest: row.1,
            media_type: row.2,
            size_bytes: u64::try_from(row.3)
                .map_err(|_| StoreError::Corrupt("negative artifact size".to_owned()))?,
            storage_uri: row.4,
        })
    })
    .transpose()
}
