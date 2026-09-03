use crate::{AgentEntry, AgentRecord, AgentRunId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRunRecord {
    pub id: AgentRunId,
    pub configuration: Value,
    pub created_at: DateTime<Utc>,
    pub version: u64,
}

#[derive(Clone, Debug)]
pub struct NewAgentRun {
    pub id: AgentRunId,
    pub configuration: Value,
    pub created_at: DateTime<Utc>,
    pub initial_entry: AgentEntry,
}

#[derive(Clone, Debug)]
pub struct AgentAppend {
    pub run_id: AgentRunId,
    pub expected_version: u64,
    pub entries: Vec<AgentEntry>,
    pub records: Vec<AgentRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AgentHistorySnapshot {
    pub run: AgentRunRecord,
    pub entries: Vec<AgentEntry>,
    pub records: Vec<AgentRecord>,
}

#[derive(Debug, Error)]
pub enum AgentStoreError {
    #[error("Agent run not found: {0}")]
    NotFound(String),
    #[error("Agent run version conflict for {id}: expected {expected}, actual {actual}")]
    Conflict {
        id: String,
        expected: u64,
        actual: u64,
    },
    #[error("Agent store data is corrupt: {0}")]
    Corrupt(String),
    #[error("Agent store backend error: {0}")]
    Backend(String),
}

#[async_trait]
pub trait AgentStore: Send + Sync {
    async fn migrate(&self) -> Result<(), AgentStoreError>;

    async fn create_run(&self, new: NewAgentRun) -> Result<AgentRunRecord, AgentStoreError>;

    async fn get_run(&self, id: AgentRunId) -> Result<AgentRunRecord, AgentStoreError>;

    async fn append(&self, append: AgentAppend) -> Result<AgentRunRecord, AgentStoreError>;

    async fn load_history(
        &self,
        id: AgentRunId,
    ) -> Result<AgentHistorySnapshot, AgentStoreError>;
}
