use crate::{ArtifactRef, ExecutionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionKind(String);

impl ExecutionKind {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("execution kind must not be empty");
        }
        Ok(Self(value))
    }

    pub fn synthetic() -> Self {
        Self("synthetic".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for ExecutionKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Waiting,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    BudgetExhausted,
    PolicyDenied,
    RequiresIntervention,
}

impl ExecutionStatus {
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded
                | Self::Failed
                | Self::Cancelled
                | Self::TimedOut
                | Self::BudgetExhausted
                | Self::PolicyDenied
                | Self::RequiresIntervention
        )
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Waiting => "waiting",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::BudgetExhausted => "budget_exhausted",
            Self::PolicyDenied => "policy_denied",
            Self::RequiresIntervention => "requires_intervention",
        }
    }
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Error)]
#[error("unknown execution status: {0}")]
pub struct ExecutionStatusParseError(String);

impl FromStr for ExecutionStatus {
    type Err = ExecutionStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Self::Pending),
            "running" => Ok(Self::Running),
            "waiting" => Ok(Self::Waiting),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "timed_out" => Ok(Self::TimedOut),
            "budget_exhausted" => Ok(Self::BudgetExhausted),
            "policy_denied" => Ok(Self::PolicyDenied),
            "requires_intervention" => Ok(Self::RequiresIntervention),
            other => Err(ExecutionStatusParseError(other.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CancelReason {
    pub code: String,
    pub message: Option<String>,
}

impl CancelReason {
    pub fn new(code: impl Into<String>, message: Option<String>) -> Self {
        Self {
            code: code.into(),
            message,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub id: ExecutionId,
    pub kind: ExecutionKind,
    pub status: ExecutionStatus,
    pub configuration_digest: String,
    pub cancel_requested: bool,
    pub cancel_reason: Option<CancelReason>,
    pub result_artifact: Option<ArtifactRef>,
    pub error: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub version: u64,
}
