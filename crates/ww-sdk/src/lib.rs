use chrono::{DateTime, Utc};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc, time::Duration};
use ww_runtime::{ExecutionInspection, ExecutionService, RuntimeError, RuntimeEventStream};
use ww_store_sqlite::SqliteRuntimeStore;
use ww_types::{
    ArtifactRef, CancelReason, ExecutionEvent, ExecutionId, ExecutionKind, ExecutionRecord,
};

#[derive(Clone)]
pub struct WorkWeaveSdk {
    runtime: ExecutionService,
}

impl WorkWeaveSdk {
    pub async fn open_local(
        db_path: impl Into<PathBuf>,
        artifact_root: impl Into<PathBuf>,
    ) -> Result<Self, RuntimeError> {
        let store = Arc::new(SqliteRuntimeStore::new(db_path));
        let runtime = ExecutionService::new(store, artifact_root).await?;
        Ok(Self { runtime })
    }

    pub async fn create_execution(
        &self,
        kind: ExecutionKind,
        configuration: &[u8],
        deadline: Option<DateTime<Utc>>,
    ) -> Result<ExecutionRecord, RuntimeError> {
        self.runtime
            .create_execution(kind, configuration, deadline)
            .await
    }

    pub async fn start_execution(&self, id: ExecutionId) -> Result<ExecutionRecord, RuntimeError> {
        self.runtime.start(id).await
    }

    pub async fn request_cancel(
        &self,
        id: ExecutionId,
        reason: CancelReason,
    ) -> Result<ExecutionRecord, RuntimeError> {
        self.runtime.request_cancel(id, reason).await
    }

    pub async fn succeed_execution(
        &self,
        id: ExecutionId,
        result: Option<ArtifactRef>,
    ) -> Result<ExecutionRecord, RuntimeError> {
        self.runtime.succeed(id, result).await
    }

    pub async fn fail_execution(
        &self,
        id: ExecutionId,
        error: Value,
    ) -> Result<ExecutionRecord, RuntimeError> {
        self.runtime.fail(id, error).await
    }

    pub async fn settle_cancelled(
        &self,
        id: ExecutionId,
    ) -> Result<ExecutionRecord, RuntimeError> {
        self.runtime.settle_cancelled(id).await
    }

    pub async fn inspect_execution(
        &self,
        id: ExecutionId,
    ) -> Result<ExecutionInspection, RuntimeError> {
        self.runtime.inspect(id).await
    }

    pub async fn execution_events(
        &self,
        id: ExecutionId,
        after: u64,
        limit: usize,
    ) -> Result<Vec<ExecutionEvent>, RuntimeError> {
        self.runtime.events(id, after, limit).await
    }

    pub fn watch_execution_events(&self, id: ExecutionId, after: u64) -> RuntimeEventStream {
        self.runtime
            .watch_events(id, after, Duration::from_millis(100))
    }

    pub async fn put_artifact(
        &self,
        bytes: &[u8],
        media_type: impl Into<String>,
    ) -> Result<ArtifactRef, RuntimeError> {
        self.runtime.artifacts().put_bytes(bytes, media_type).await
    }
}
