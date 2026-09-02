use crate::{ArtifactRef, CancelReason, EventId, ExecutionId, ExecutionKind};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventVisibility {
    Public,
    Internal,
    Sensitive,
}

impl EventVisibility {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Sensitive => "sensitive",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExecutionEventData {
    Created {
        kind: ExecutionKind,
        configuration_digest: String,
    },
    Started,
    CancelRequested {
        reason: CancelReason,
    },
    Succeeded {
        result_artifact: Option<ArtifactRef>,
    },
    Failed {
        error: Value,
    },
    Cancelled {
        reason: Option<CancelReason>,
    },
    TimedOut,
    RequiresIntervention {
        reason: String,
    },
}

impl ExecutionEventData {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Created { .. } => "execution_created",
            Self::Started => "execution_started",
            Self::CancelRequested { .. } => "cancel_requested",
            Self::Succeeded { .. } => "execution_succeeded",
            Self::Failed { .. } => "execution_failed",
            Self::Cancelled { .. } => "execution_cancelled",
            Self::TimedOut => "execution_timed_out",
            Self::RequiresIntervention { .. } => "execution_requires_intervention",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub id: EventId,
    pub execution_id: ExecutionId,
    pub sequence: u64,
    pub occurred_at: DateTime<Utc>,
    pub payload_version: u16,
    pub visibility: EventVisibility,
    pub data: ExecutionEventData,
}
