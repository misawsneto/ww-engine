use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use thiserror::Error;
use ww_types::{
    ArtifactRef, CancelReason, EventId, EventVisibility, ExecutionEvent, ExecutionEventData,
    ExecutionId, ExecutionKind, ExecutionRecord, ExecutionStatus,
};

#[derive(Clone, Debug)]
pub struct NewExecution {
    pub id: ExecutionId,
    pub kind: ExecutionKind,
    pub configuration_digest: String,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub event_id: EventId,
}

#[derive(Clone, Debug, Default)]
pub struct ExecutionPatch {
    pub status: Option<ExecutionStatus>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub cancel_reason: Option<CancelReason>,
    pub result_artifact: Option<ArtifactRef>,
    pub error: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct ExecutionMutation {
    pub execution_id: ExecutionId,
    pub expected_version: u64,
    pub patch: ExecutionPatch,
    pub event_id: EventId,
    pub occurred_at: DateTime<Utc>,
    pub visibility: EventVisibility,
    pub event: ExecutionEventData,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("record not found: {0}")]
    NotFound(String),
    #[error("optimistic version conflict for {id}: expected {expected}, actual {actual}")]
    Conflict {
        id: String,
        expected: u64,
        actual: u64,
    },
    #[error("store data is corrupt: {0}")]
    Corrupt(String),
    #[error("store backend error: {0}")]
    Backend(String),
}

#[async_trait]
pub trait RuntimeStore: Send + Sync {
    async fn migrate(&self) -> Result<(), StoreError>;

    async fn create_execution(&self, new: NewExecution) -> Result<ExecutionRecord, StoreError>;

    async fn get_execution(&self, id: ExecutionId) -> Result<ExecutionRecord, StoreError>;

    async fn mutate_execution(
        &self,
        mutation: ExecutionMutation,
    ) -> Result<ExecutionRecord, StoreError>;

    async fn list_execution_events(
        &self,
        id: ExecutionId,
        after_sequence: u64,
        limit: usize,
    ) -> Result<Vec<ExecutionEvent>, StoreError>;

    async fn put_artifact(&self, artifact: ArtifactRef) -> Result<ArtifactRef, StoreError>;

    async fn get_artifact_by_digest(&self, digest: &str)
    -> Result<Option<ArtifactRef>, StoreError>;
}
