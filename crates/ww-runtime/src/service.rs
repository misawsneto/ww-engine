use crate::{
    CancellationRegistry, ExecutionProjection, LocalArtifactService, reduce_execution_events,
};
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures_core::Stream;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{pin::Pin, sync::Arc, time::Duration};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use ww_store::{ExecutionMutation, ExecutionPatch, NewExecution, RuntimeStore, StoreError};
use ww_types::{
    ArtifactRef, CancelReason, EventId, EventVisibility, ExecutionEvent, ExecutionEventData,
    ExecutionId, ExecutionKind, ExecutionRecord, ExecutionStatus,
};

pub type RuntimeEventStream =
    Pin<Box<dyn Stream<Item = Result<ExecutionEvent, RuntimeError>> + Send>>;

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Clone, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionInspection {
    pub record: ExecutionRecord,
    pub reduced: ExecutionProjection,
}

#[derive(Debug, Error)]
#[error("invalid execution transition from {from} via {action}")]
pub struct InvalidTransition {
    pub from: ExecutionStatus,
    pub action: &'static str,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    InvalidTransition(#[from] InvalidTransition),
    #[error("execution history does not match current record: {0}")]
    CorruptProjection(String),
    #[error("artifact I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone)]
pub struct ExecutionService {
    store: Arc<dyn RuntimeStore>,
    cancellations: CancellationRegistry,
    artifacts: LocalArtifactService,
    clock: Arc<dyn Clock>,
}

impl ExecutionService {
    pub async fn new(
        store: Arc<dyn RuntimeStore>,
        artifact_root: impl Into<std::path::PathBuf>,
    ) -> Result<Self, RuntimeError> {
        store.migrate().await?;
        Ok(Self {
            artifacts: LocalArtifactService::new(artifact_root, Arc::clone(&store)),
            store,
            cancellations: CancellationRegistry::default(),
            clock: Arc::new(SystemClock),
        })
    }

    pub fn artifacts(&self) -> &LocalArtifactService {
        &self.artifacts
    }

    pub async fn create_execution(
        &self,
        kind: ExecutionKind,
        configuration: &[u8],
        deadline: Option<DateTime<Utc>>,
    ) -> Result<ExecutionRecord, RuntimeError> {
        let digest = format!("{:x}", Sha256::digest(configuration));
        self.store
            .create_execution(NewExecution {
                id: ExecutionId::new(),
                kind,
                configuration_digest: digest,
                created_at: self.clock.now(),
                deadline,
                event_id: EventId::new(),
            })
            .await
            .map_err(Into::into)
    }

    pub async fn start(&self, id: ExecutionId) -> Result<ExecutionRecord, RuntimeError> {
        let current = self.store.get_execution(id).await?;
        if current.status != ExecutionStatus::Pending || current.cancel_requested {
            return Err(InvalidTransition {
                from: current.status,
                action: "start",
            }
            .into());
        }
        self.mutate(
            &current,
            ExecutionPatch {
                status: Some(ExecutionStatus::Running),
                started_at: Some(self.clock.now()),
                ..ExecutionPatch::default()
            },
            ExecutionEventData::Started,
        )
        .await
    }

    pub async fn request_cancel(
        &self,
        id: ExecutionId,
        reason: CancelReason,
    ) -> Result<ExecutionRecord, RuntimeError> {
        let current = self.store.get_execution(id).await?;
        if current.status.is_terminal() {
            return Err(InvalidTransition {
                from: current.status,
                action: "request_cancel",
            }
            .into());
        }
        let updated = self
            .mutate(
                &current,
                ExecutionPatch {
                    cancel_reason: Some(reason.clone()),
                    ..ExecutionPatch::default()
                },
                ExecutionEventData::CancelRequested { reason },
            )
            .await?;
        self.cancellations.signal(id).await;
        Oi(updated)
    }

    pub async fn succeed(
        &self,
        id: ExecutionId,
        result_artifact: Option<ArtifactRef>,
    ) -> Result<ExecutionRecord, RuntimeError> {
        let current = self.store.get_execution(id).await?;
        self.require_active(&current, "succeed")?;
        self.mutate(
            &current,
            ExecutionPatch {
                status: Some(ExecutionStatus::Succeeded),
                finished_at: Some(self.clock.now()),
                result_artifact: result_artifact.clone(),
                ..ExecutionPatch::default()
            },
            ExecutionEventData::Succeeded { result_artifact },
        )
        .await
    }

    pub async fn fail(
        &self,
        id: ExecutionId,
        error: Value,
    ) -> Result<ExecutionRecord, RuntimeError> {
        let current = self.store.get_execution(id).await?;
        self.require_active(&current, "fail")?;
        self.mutate(
            &current,
            ExecutionPatch {
                status: Some(ExecutionStatus::Failed),
                finished_at: Some(self.clock.now()),
                error: Some(error.clone()),
                ..ExecutionPatch::default()
            },
            ExecutionEventData::Failed { error },
        )
        .await
    }

    pub async fn settle_cancelled(&self, id: ExecutionId) -> Result<ExecutionRecord, RuntimeError> {
        let current = self.store.get_execution(id).await?;
        if current.status.is_terminal() || !current.cancel_requested {
            return Err(InvalidTransition {
                from: current.status,
                action: "settle_cancelled",
            }
            .into());
        }
        let updated = self
            .mutate(
                &current,
                ExecutionPatch {
                    status: Some(ExecutionStatus::Cancelled),
                    finished_at: Some(self.clock.now()),
                    ..ExecutionPatch::default()
                },
                ExecutionEventData::Cancelled {
                    reason: current.cancel_reason.clone(),
                },
            )
            .await?;
        self.cancellations.unregister(id).await;
        Ok(updated)
    }

    pub async fn inspect(&self, id: ExecutionId) -> Result<ExecutionInspection, RuntimeError> {
        let record = self.store.get_execution(id).await?;
        let events = self.all_events(id).await?;
        let reduced = reduce_execution_events(&events)
            .map_err(|error| RuntimeError::CorruptProjection(error.to_string()))?;
        if reduced.kind != record.kind
            || reduced.configuration_digest != record.configuration_digest
            || reduced.status != record.status
            || reduced.cancel_requested != record.cancel_requested
            || reduced.cancel_reason != record.cancel_reason
            || reduced.result_artifact != record.result_artifact
            || reduced.error != record.error
            || reduced.version != record.version
        {
            return Err(RuntimeError::CorruptProjection(format!(
                "record={record:?} reduced={reduced:?}"
            )));
        }
        Ok(ExecutionInspection { record, reduced })
    }

    pub async fn events(
        &self,
        id: ExecutionId,
        after_sequence: u64,
        limit: usize,
    ) -> Result<Vec<ExecutionEvent>, RuntimeError> {
        self.store
            .list_execution_events(id, after_sequence, limit)
            .await
            .map_err(Into::into)
    }

    pub fn watch_events(
        &self,
        id: ExecutionId,
        after_sequence: u64,
        poll_interval: Duration,
    ) -> RuntimeEventStream {
        let service = self.clone();
        Box::pin(try_stream! {
            let mut cursor = after_sequence;
            loop {
                let events = service.events(id, cursor, 128).await?;
                for event in events {
                    cursor = event.sequence;
                    yield event;
                }
                let current = service.store.get_execution(id).await?;
                if current.status.is_terminal() {
                    let remaining = service.events(id, cursor, 1).await?;
                    if remaining.is_empty() {
                        break;
                    }
                }
                tokio::time::sleep(poll_interval).await;
            }
        })
    }

    pub async fn register_local_cancellation(
        &self,
        id: ExecutionId,
    ) -> Result<CancellationToken, RuntimeError> {
        let current = self.store.get_execution(id).await?;
        Ok(self
            .cancellations
            .register(id, current.cancel_requested)
            .await)
    }

    async fn all_events(&self, id: ExecutionId) -> Result<Vec<ExecutionEvent>, RuntimeError> {
        let mut cursor = 0;
        let mut all = Vec::new();
        loop {
            let page = self.events(id, cursor, 256).await?;
            if page.is_empty() {
                break;
            }
            cursor = page.last().expect("non-empty page").sequence;
            all.extend(page);
        }
        Ok(all)
    }

    fn require_active(
        &self,
        current: &ExecutionRecord,
        action: &'static str,
    ) -> Result<(), RuntimeError> {
        if matches!(
            current.status,
            ExecutionStatus::Running | ExecutionStatus::Waiting
        ) {
            Ok(())
        } else {
            Err(InvalidTransition {
                from: current.status,
                action,
            }
            .into())
        }
    }

    async fn mutate(
        &self,
        current: &ExecutionRecord,
        patch: ExecutionPatch,
        event: ExecutionEventData,
    ) -> Result<ExecutionRecord, RuntimeError> {
        self.store
            .mutate_execution(ExecutionMutation {
                execution_id: current.id,
                expected_version: current.version,
                patch,
                event_id: EventId::new(),
                occurred_at: self.clock.now(),
                visibility: EventVisibility::Public,
                event,
            })
            .await
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use serde_json::json;
    use tempfile::TempDir;
    use ww_store_sqlite::SqliteRuntimeStore;

    async fn service(temp: &TempDir) -> ExecutionService {
        let store = Arc::new(SqliteRuntimeStore::new(temp.path().join("runtime.db")));
        ExecutionService::new(store, temp.path().join("artifacts"))
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn lifecycle_survives_reopen_and_matches_reducer() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let created = runtime
            .create_execution(ExecutionKind::synthetic(), b"{}", None)
            .await
            .unwrap();
        runtime.start(created.id).await.unwrap();
        runtime.succeed(created.id, None).await.unwrap();
        let before = runtime.inspect(created.id).await.unwrap();
        drop(runtime);

        let reopened = service(&temp).await;
        let after = reopened.inspect(created.id).await.unwrap();
        assert_eq!(before, after);
        assert_eq!(after.record.status, ExecutionStatus::Succeeded);
        assert_eq!(after.record.version, 3);
    }

    #[tokio::test]
    async fn invalid_transition_does_not_append_event() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let created = runtime
            .create_execution(ExecutionKind::synthetic(), b"{}", None)
            .await
            .unwrap();
        let error = runtime.succeed(created.id, None).await.unwrap_err();
        assert!(matches!(error, RuntimeError::InvalidTransition(_)));
        let inspection = runtime.inspect(created.id).await.unwrap();
        assert_eq!(inspection.record.version, 1);
        assert_eq!(runtime.events(created.id, 0, 10).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn cancellation_cannot_settle_without_durable_request() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let created = runtime
            .create_execution(ExecutionKind::synthetic(), b"{}", None)
            .await
            .unwrap();
        runtime.start(created.id).await.unwrap();

        let error = runtime.settle_cancelled(created.id).await.unwrap_err();
        assert!(matches!(error, RuntimeError::InvalidTransition(_)));

        let inspection = runtime.inspect(created.id).await.unwrap();
        assert_eq!(inspection.record.status, ExecutionStatus::Running);
        assert!(!inspection.record.cancel_requested);
        assert_eq!(inspection.record.version, 2);
    }

    #[tokio::test]
    async fn durable_cancel_signals_local_token() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let created = runtime
            .create_execution(ExecutionKind::synthetic(), b"{}", None)
            .await
            .unwrap();
        runtime.start(created.id).await.unwrap();
        let token = runtime
            .register_local_cancellation(created.id)
            .await
            .unwrap();
        assert!(!token.is_cancelled());
        runtime
            .request_cancel(
                created.id,
                CancelReason::new("operator", Some("stop".to_owned())),
            )
            .await
            .unwrap();
        assert!(token.is_cancelled());
        runtime.settle_cancelled(created.id).await.unwrap();
        let inspection = runtime.inspect(created.id).await.unwrap();
        assert!(inspection.record.cancel_requested);
        assert_eq!(inspection.record.status, ExecutionStatus::Cancelled);
    }

    #[tokio::test]
    async fn event_cursor_has_no_duplicates() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let created = runtime
            .create_execution(ExecutionKind::synthetic(), b"{}", None)
            .await
            .unwrap();
        runtime.start(created.id).await.unwrap();
        runtime
            .fail(created.id, json!({"code": "boom"}))
            .await
            .unwrap();
        let first = runtime.events(created.id, 0, 2).await.unwrap();
        let second = runtime
            .events(created.id, first.last().unwrap().sequence, 10)
            .await
            .unwrap();
        assert_eq!(
            first.iter().map(|event| event.sequence).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            second
                .iter()
                .map(|event| event.sequence)
                .collect::<Vec<_>>(),
            vec![3]
        );
    }

    #[tokio::test]
    async fn watch_stream_finishes_at_terminal_event() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let created = runtime
            .create_execution(ExecutionKind::synthetic(), b"{}", None)
            .await
            .unwrap();
        runtime.start(created.id).await.unwrap();
        runtime.succeed(created.id, None).await.unwrap();
        let events = runtime
            .watch_events(created.id, 0, Duration::from_millis(1))
            .collect::<Vec<_>>()
            .await;
        assert_eq!(events.len(), 3);
        assert!(events.into_iter().all(|event| event.is_ok()));
    }

    #[tokio::test]
    async fn artifacts_are_content_addressed_and_deduplicated() {
        let temp = TempDir::new().unwrap();
        let runtime = service(&temp).await;
        let first = runtime
            .artifacts()
            .put_bytes(b"hello", "text/plain")
            .await
            .unwrap();
        let second = runtime
            .artifacts()
            .put_bytes(b"hello", "text/plain")
            .await
            .unwrap();
        assert_eq!(first, second);
        assert!(first.storage_uri.contains(&first.digest));
    }
}
