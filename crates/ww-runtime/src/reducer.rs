use thiserror::Error;
use serde_json::Value;
use ww_types::{
    ArtifactRef, CancelReason, ExecutionEvent, ExecutionEventData, ExecutionKind, ExecutionStatus,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionProjection {
    pub kind: ExecutionKind,
    pub configuration_digest: String,
    pub status: ExecutionStatus,
    pub cancel_requested: bool,
    pub cancel_reason: Option<CancelReason>,
    pub result_artifact: Option<ArtifactRef>,
    pub error: Option<Value>,
    pub version: u64,
}

#[derive(Debug, Error, PartialEq)]
pub enum ReductionError {
    #[error("execution event history is empty")]
    Empty,
    #[error("event sequence {actual} is not the expected contiguous sequence {expected}")]
    Sequence { expected: u64, actual: u64 },
    #[error("first event is not execution_created")]
    MissingCreated,
    #[error("invalid event {event} while execution is {status}")]
    InvalidTransition {
        event: &'static str,
        status: ExecutionStatus,
    },
    #[error("event exists after terminal status {0}")]
    EventAfterTerminal(ExecutionStatus),
}

pub fn reduce_execution_events(
    events: &[ExecutionEvent],
) -> Result<ExecutionProjection, ReductionError> {
    let first = events.first().ok_or(ReductionError::Empty)?;
    if first.sequence != 1 {
        return Err(ReductionError::Sequence {
            expected: 1,
            actual: first.sequence,
        });
    }
    let ExecutionEventData::Created {
        kind,
        configuration_digest,
    } = &first.data
    else {
        return Err(ReductionError::MissingCreated);
    };
    let mut projection = ExecutionProjection {
        kind: kind.clone(),
        configuration_digest: configuration_digest.clone(),
        status: ExecutionStatus::Pending,
        cancel_requested: false,
        cancel_reason: None,
        result_artifact: None,
        error: None,
        version: 1,
    };

    for event in &events[1..] {
        let expected = projection.version + 1;
        if event.sequence != expected {
            return Err(ReductionError::Sequence {
                expected,
                actual: event.sequence,
            });
        }
        if projection.status.is_terminal() {
            return Err(ReductionError::EventAfterTerminal(projection.status));
        }

        match &event.data {
            ExecutionEventData::Created { .. } => {
                return Err(ReductionError::InvalidTransition {
                    event: event.data.kind(),
                    status: projection.status,
                });
            }
            ExecutionEventData::Started if projection.status == ExecutionStatus::Pending => {
                projection.status = ExecutionStatus::Running;
            }
            ExecutionEventData::CancelRequested { reason } => {
                projection.cancel_requested = true;
                projection.cancel_reason = Some(reason.clone());
            }
            ExecutionEventData::Succeeded { result_artifact }
                if matches!(
                    projection.status,
                    ExecutionStatus::Running | ExecutionStatus::Waiting
                ) =>
            {
                projection.status = ExecutionStatus::Succeeded;
                projection.result_artifact = result_artifact.clone();
            }
            ExecutionEventData::Failed { error }
                if matches!(
                    projection.status,
                    ExecutionStatus::Running | ExecutionStatus::Waiting
                ) =>
            {
                projection.status = ExecutionStatus::Failed;
                projection.error = Some(error.clone());
            }
            ExecutionEventData::Cancelled { reason }
                if projection.cancel_requested
                    && matches!(
                        projection.status,
                        ExecutionStatus::Pending
                            | ExecutionStatus::Running
                            | ExecutionStatus::Waiting
                    ) =>
            {
                projection.status = ExecutionStatus::Cancelled;
                if let Some(reason) = reason {
                    projection.cancel_reason = Some(reason.clone());
                }
            }
            ExecutionEventData::TimedOut
                if matches!(
                    projection.status,
                    ExecutionStatus::Running | ExecutionStatus::Waiting
                ) =>
            {
                projection.status = ExecutionStatus::TimedOut;
            }
            ExecutionEventData::RequiresIntervention { .. }
                if matches!(
                    projection.status,
                    ExecutionStatus::Running | ExecutionStatus::Waiting
                ) =>
            {
                projection.status = ExecutionStatus::RequiresIntervention;
            }
            _ => {
                return Err(ReductionError::InvalidTransition {
                    event: event.data.kind(),
                    status: projection.status,
                });
            }
        }
        projection.version = event.sequence;
    }

    Ok(projection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use proptest::prelude::*;
    use uuid::Uuid;
    use ww_types::{EventId, EventVisibility, ExecutionId};

    fn event(sequence: u64, data: ExecutionEventData) -> ExecutionEvent {
        ExecutionEvent {
            id: EventId::from_uuid(Uuid::from_u128(sequence as u128 + 100)),
            execution_id: ExecutionId::from_uuid(Uuid::from_u128(1)),
            sequence,
            occurred_at: Utc.timestamp_opt(sequence as i64, 0).unwrap(),
            payload_version: 1,
            visibility: EventVisibility::Public,
            data,
        }
    }

    proptest! {
        #[test]
        fn repeated_cancel_requests_reduce_deterministically(cancel_count in 1u8..8) {
            let mut events = vec![
                event(1, ExecutionEventData::Created {
                    kind: ExecutionKind::synthetic(),
                    configuration_digest: "abc".to_owned(),
                }),
                event(2, ExecutionEventData::Started),
            ];
            for index in 0..cancel_count {
                events.push(event(
                    events.len() as u64 + 1,
                    ExecutionEventData::CancelRequested {
                        reason: CancelReason::new(format!("cancel-{index}"), None),
                    },
                ));
            }
            events.push(event(
                events.len() as u64 + 1,
                ExecutionEventData::Cancelled { reason: None },
            ));

            let projection = reduce_execution_events(&events).unwrap();
            prop_assert_eq!(projection.status, ExecutionStatus::Cancelled);
            prop_assert_eq!(projection.version, events.len() as u64);
            prop_assert_eq!(projection.cancel_requested, cancel_count > 0);
        }
    }
}
