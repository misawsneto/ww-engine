use crate::{
    AgentEntry, AgentEntryData, AgentEntryId, AgentRecord, AgentRecordData, AgentRunId,
    AgentTerminalResult, LogicalToolCallId, ModelAttemptId, ToolAttemptId, ToolEffectResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;
use ww_agent_provider::{ModelUsage, ToolCallId};
use ww_agent_tools::{ToolPreparationDisposition, ToolPreparationStage};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentPhase {
    ReadyForModel,
    ModelInFlight,
    ToolsPending,
    ToolInFlight,
    ReadyToCommitTurn,
    ReadyToCommitResult,
    InterventionPending,
    Terminal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ModelAttemptStatus {
    Started,
    Interrupted,
    Completed { assistant_entry_id: AgentEntryId },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModelAttemptState {
    pub id: ModelAttemptId,
    pub request_ordinal: u64,
    pub status: ModelAttemptStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ToolAttemptStatus {
    Started,
    Prepared,
    /// The effect crossed the ambiguity boundary. It may or may not have run.
    EffectInFlight,
    /// The effect settled durably but its model-visible entry is not committed.
    EffectSettled {
        result: ToolEffectResult,
    },
    Rejected {
        result_entry_id: AgentEntryId,
        failed_at: ToolPreparationStage,
    },
    Denied {
        result_entry_id: AgentEntryId,
    },
    Completed {
        result_entry_id: AgentEntryId,
    },
    /// A Safe attempt abandoned without a result; the logical call stays pending.
    Interrupted {
        reason: String,
    },
    Intervention {
        reason: String,
    },
}

/// The durable preparation of one logical tool call.
///
/// Every attempt of one logical call must reproduce these exact values.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolCallPreparation {
    pub assistant_entry_id: AgentEntryId,
    pub source_index: u32,
    pub provider_call_id: ToolCallId,
    pub requested_tool_name: String,
    pub result_entry_id: AgentEntryId,
    pub disposition: ToolPreparationDisposition,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolAttemptState {
    pub id: ToolAttemptId,
    pub logical_call_id: LogicalToolCallId,
    pub status: ToolAttemptStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRecoveryState {
    pub run_id: AgentRunId,
    pub phase: AgentPhase,
    pub context_entry_ids: Vec<AgentEntryId>,
    pub next_model_request_ordinal: u64,
    pub next_turn_ordinal: u64,
    pub active_model_attempt: Option<ModelAttemptId>,
    pub last_model_attempt: Option<ModelAttemptId>,
    pub current_assistant_entry: Option<AgentEntryId>,
    pub pending_tool_calls: Vec<LogicalToolCallId>,
    pub completed_tool_results: BTreeMap<LogicalToolCallId, AgentEntryId>,
    pub active_tool_attempt: Option<ToolAttemptId>,
    pub model_attempts: BTreeMap<ModelAttemptId, ModelAttemptState>,
    pub tool_attempts: BTreeMap<LogicalToolCallId, Vec<ToolAttemptState>>,
    pub tool_preparations: BTreeMap<LogicalToolCallId, ToolCallPreparation>,
    pub usage: ModelUsage,
    pub model_request_count: u64,
    pub turn_count: u64,
    pub tool_attempt_count: u64,
    pub terminal_result: Option<AgentTerminalResult>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CorruptionError {
    #[error("entry ordinal {actual} is not expected contiguous ordinal {expected}")]
    EntryOrdinal { expected: u64, actual: u64 },
    #[error("record sequence {actual} is not expected contiguous sequence {expected}")]
    RecordSequence { expected: u64, actual: u64 },
    #[error("entry or record belongs to run {actual}, expected {expected}")]
    RunMismatch {
        expected: AgentRunId,
        actual: AgentRunId,
    },
    #[error("duplicate entry id {0}")]
    DuplicateEntryId(AgentEntryId),
    #[error("duplicate model attempt id {0}")]
    DuplicateModelAttempt(ModelAttemptId),
    #[error("duplicate tool attempt id {0}")]
    DuplicateToolAttempt(ToolAttemptId),
    #[error("unknown tool attempt id {0}")]
    UnknownToolAttempt(ToolAttemptId),
    #[error("duplicate logical tool call id {0}")]
    DuplicateLogicalToolCall(LogicalToolCallId),
    #[error("logical tool call {0} has more than one model-visible result entry")]
    DuplicateLogicalToolResult(LogicalToolCallId),
    #[error("user input entry appears after model-generated context")]
    UserInputAfterGeneratedContext,
    #[error("record {record} is invalid while recovery phase is {phase:?}")]
    InvalidRecord {
        record: &'static str,
        phase: AgentPhase,
    },
    #[error("unknown entry id {0}")]
    UnknownEntry(AgentEntryId),
    #[error("unknown logical tool call id {0}")]
    UnknownLogicalToolCall(LogicalToolCallId),
    #[error("attempt {actual} does not match active attempt {expected}")]
    AttemptMismatch { expected: String, actual: String },
    #[error("model request ordinal {actual} is not expected ordinal {expected}")]
    ModelRequestOrdinal { expected: u64, actual: u64 },
    #[error("turn ordinal {actual} is not expected ordinal {expected}")]
    TurnOrdinal { expected: u64, actual: u64 },
    #[error("assistant entry {entry_id} does not belong to model attempt {attempt_id}")]
    AssistantAttemptMismatch {
        entry_id: AgentEntryId,
        attempt_id: ModelAttemptId,
    },
    #[error("entry {0} is not the next context entry")]
    ContextOrder(AgentEntryId),
    #[error(
        "tool result entry {entry_id} does not match tool attempt {attempt_id} / call {logical_call_id}"
    )]
    ToolResultMismatch {
        entry_id: AgentEntryId,
        attempt_id: ToolAttemptId,
        logical_call_id: LogicalToolCallId,
    },
    #[error("tool call {actual} is not the next pending call {expected}")]
    ToolSourceOrder {
        expected: LogicalToolCallId,
        actual: LogicalToolCallId,
    },
    #[error("turn result entries do not match committed tool-call source order")]
    TurnResultOrder,
    #[error("succeeded Agent result references wrong assistant entry {0}")]
    TerminalAssistantMismatch(AgentEntryId),
    #[error("record {0} exists after terminal Agent result")]
    RecordAfterTerminal(&'static str),
    #[error("generated entry {0} is durable but never finalized by an operational record")]
    OrphanGeneratedEntry(AgentEntryId),
    #[error("logical tool call {0} is prepared differently across attempts")]
    ToolPreparationConflict(LogicalToolCallId),
    #[error("tool attempt {0} is prepared more than once")]
    DuplicateToolPreparation(ToolAttemptId),
    #[error("tool attempt {0} starts an effect without an executable preparation")]
    EffectStartWithoutExecutable(ToolAttemptId),
    #[error("tool attempt {0} completes an effect that never started")]
    EffectCompletionWithoutStart(ToolAttemptId),
    #[error("result entry {actual} is not reserved result entry {expected}")]
    ReservedResultMismatch {
        expected: AgentEntryId,
        actual: AgentEntryId,
    },
    #[error("tool attempt {0} settles with the wrong record for its preparation stage")]
    WrongNoEffectRecord(ToolAttemptId),
}

/// How one tool attempt reaches its model-visible result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ToolSettlement {
    Completed,
    Denied,
    Rejected(ToolPreparationStage),
}

impl ToolSettlement {
    const fn kind(self) -> &'static str {
        match self {
            Self::Completed => "tool_attempt_completed",
            Self::Denied => "tool_attempt_denied",
            Self::Rejected(_) => "tool_attempt_rejected",
        }
    }
}

#[derive(Clone, Debug)]
struct ToolDefinition {
    assistant_entry_id: AgentEntryId,
}

pub fn reduce_agent_history(
    run_id: AgentRunId,
    entries: &[AgentEntry],
    records: &[AgentRecord],
) -> Result<AgentRecoveryState, CorruptionError> {
    let mut entry_by_id = BTreeMap::new();
    let mut tool_definitions = BTreeMap::new();
    let mut tool_result_entries = BTreeMap::new();
    let mut user_prefix = true;
    let mut initial_context = Vec::new();

    for (index, entry) in entries.iter().enumerate() {
        let expected = index as u64 + 1;
        if entry.ordinal != expected {
            return Err(CorruptionError::EntryOrdinal {
                expected,
                actual: entry.ordinal,
            });
        }
        if entry.run_id != run_id {
            return Err(CorruptionError::RunMismatch {
                expected: run_id,
                actual: entry.run_id,
            });
        }
        if entry_by_id.insert(entry.id, entry).is_some() {
            return Err(CorruptionError::DuplicateEntryId(entry.id));
        }

        match &entry.data {
            AgentEntryData::UserInput { .. } => {
                if !user_prefix {
                    return Err(CorruptionError::UserInputAfterGeneratedContext);
                }
                initial_context.push(entry.id);
            }
            AgentEntryData::AssistantMessage { message, .. } => {
                user_prefix = false;
                for call in message.tool_calls() {
                    if tool_definitions
                        .insert(
                            call.logical_id,
                            ToolDefinition {
                                assistant_entry_id: entry.id,
                            },
                        )
                        .is_some()
                    {
                        return Err(CorruptionError::DuplicateLogicalToolCall(call.logical_id));
                    }
                }
            }
            AgentEntryData::ModelVisibleToolResult {
                logical_call_id, ..
            } => {
                user_prefix = false;
                if tool_result_entries
                    .insert(*logical_call_id, entry.id)
                    .is_some()
                {
                    return Err(CorruptionError::DuplicateLogicalToolResult(
                        *logical_call_id,
                    ));
                }
            }
        }
    }

    let mut state = AgentRecoveryState {
        run_id,
        phase: AgentPhase::ReadyForModel,
        context_entry_ids: initial_context,
        next_model_request_ordinal: 1,
        next_turn_ordinal: 1,
        active_model_attempt: None,
        last_model_attempt: None,
        current_assistant_entry: None,
        pending_tool_calls: Vec::new(),
        completed_tool_results: BTreeMap::new(),
        active_tool_attempt: None,
        model_attempts: BTreeMap::new(),
        tool_attempts: BTreeMap::new(),
        tool_preparations: BTreeMap::new(),
        usage: ModelUsage::default(),
        model_request_count: 0,
        turn_count: 0,
        tool_attempt_count: 0,
        terminal_result: None,
    };

    let mut finalized_entries = BTreeSet::new();
    finalized_entries.extend(state.context_entry_ids.iter().copied());
    let mut current_turn_result_ids = Vec::new();
    let mut seen_tool_attempt_ids = BTreeSet::new();

    for (index, record) in records.iter().enumerate() {
        let expected_sequence = index as u64 + 1;
        if record.sequence != expected_sequence {
            return Err(CorruptionError::RecordSequence {
                expected: expected_sequence,
                actual: record.sequence,
            });
        }
        if record.run_id != run_id {
            return Err(CorruptionError::RunMismatch {
                expected: run_id,
                actual: record.run_id,
            });
        }
        if state.phase == AgentPhase::Terminal {
            return Err(CorruptionError::RecordAfterTerminal(record.data.kind()));
        }

        match &record.data {
            AgentRecordData::ModelAttemptStarted {
                attempt_id,
                request_ordinal,
            } => {
                require_phase(&state, AgentPhase::ReadyForModel, record.data.kind())?;
                if state.model_attempts.contains_key(attempt_id) {
                    return Err(CorruptionError::DuplicateModelAttempt(*attempt_id));
                }
                if *request_ordinal != state.next_model_request_ordinal {
                    return Err(CorruptionError::ModelRequestOrdinal {
                        expected: state.next_model_request_ordinal,
                        actual: *request_ordinal,
                    });
                }
                state.model_attempts.insert(
                    *attempt_id,
                    ModelAttemptState {
                        id: *attempt_id,
                        request_ordinal: *request_ordinal,
                        status: ModelAttemptStatus::Started,
                    },
                );
                state.active_model_attempt = Some(*attempt_id);
                state.last_model_attempt = Some(*attempt_id);
                state.next_model_request_ordinal += 1;
                state.model_request_count += 1;
                state.phase = AgentPhase::ModelInFlight;
            }
            AgentRecordData::ModelAttemptInterrupted { attempt_id, .. } => {
                require_phase(&state, AgentPhase::ModelInFlight, record.data.kind())?;
                require_attempt(state.active_model_attempt, *attempt_id)?;
                let attempt = state
                    .model_attempts
                    .get_mut(attempt_id)
                    .expect("active model attempt exists");
                attempt.status = ModelAttemptStatus::Interrupted;
                state.active_model_attempt = None;
                state.phase = AgentPhase::ReadyForModel;
            }
            AgentRecordData::ModelAttemptCompleted {
                attempt_id,
                assistant_entry_id,
            } => {
                require_phase(&state, AgentPhase::ModelInFlight, record.data.kind())?;
                require_attempt(state.active_model_attempt, *attempt_id)?;
                let entry = *entry_by_id
                    .get(assistant_entry_id)
                    .ok_or(CorruptionError::UnknownEntry(*assistant_entry_id))?;
                let AgentEntryData::AssistantMessage {
                    attempt_id: entry_attempt,
                    message,
                } = &entry.data
                else {
                    return Err(CorruptionError::AssistantAttemptMismatch {
                        entry_id: *assistant_entry_id,
                        attempt_id: *attempt_id,
                    });
                };
                if entry_attempt != attempt_id {
                    return Err(CorruptionError::AssistantAttemptMismatch {
                        entry_id: *assistant_entry_id,
                        attempt_id: *attempt_id,
                    });
                }
                append_context_entry(&mut state, entry)?;
                finalized_entries.insert(entry.id);

                let attempt = state
                    .model_attempts
                    .get_mut(attempt_id)
                    .expect("active model attempt exists");
                attempt.status = ModelAttemptStatus::Completed {
                    assistant_entry_id: *assistant_entry_id,
                };
                state.active_model_attempt = None;
                state.current_assistant_entry = Some(*assistant_entry_id);
                if let Some(usage) = message.usage {
                    state.usage = add_usage(state.usage, usage);
                }

                state.pending_tool_calls =
                    message.tool_calls().map(|call| call.logical_id).collect();
                current_turn_result_ids.clear();
                state.phase = if state.pending_tool_calls.is_empty() {
                    AgentPhase::ReadyToCommitResult
                } else {
                    AgentPhase::ToolsPending
                };
            }
            AgentRecordData::ToolAttemptStarted {
                attempt_id,
                logical_call_id,
            } => {
                require_phase(&state, AgentPhase::ToolsPending, record.data.kind())?;
                if !seen_tool_attempt_ids.insert(*attempt_id) {
                    return Err(CorruptionError::DuplicateToolAttempt(*attempt_id));
                }
                let expected_call = *state
                    .pending_tool_calls
                    .first()
                    .ok_or(CorruptionError::UnknownLogicalToolCall(*logical_call_id))?;
                if expected_call != *logical_call_id {
                    return Err(CorruptionError::ToolSourceOrder {
                        expected: expected_call,
                        actual: *logical_call_id,
                    });
                }
                let definition = tool_definitions
                    .get(logical_call_id)
                    .ok_or(CorruptionError::UnknownLogicalToolCall(*logical_call_id))?;
                if Some(definition.assistant_entry_id) != state.current_assistant_entry {
                    return Err(CorruptionError::UnknownLogicalToolCall(*logical_call_id));
                }
                state
                    .tool_attempts
                    .entry(*logical_call_id)
                    .or_default()
                    .push(ToolAttemptState {
                        id: *attempt_id,
                        logical_call_id: *logical_call_id,
                        status: ToolAttemptStatus::Started,
                    });
                state.active_tool_attempt = Some(*attempt_id);
                state.tool_attempt_count += 1;
                state.phase = AgentPhase::ToolInFlight;
            }
            AgentRecordData::ToolCallPrepared {
                attempt_id,
                logical_call_id,
                assistant_entry_id,
                source_index,
                provider_call_id,
                requested_tool_name,
                result_entry_id,
                disposition,
            } => {
                require_phase(&state, AgentPhase::ToolInFlight, record.data.kind())?;
                require_attempt(state.active_tool_attempt, *attempt_id)?;
                if active_tool_call_id(&state, *attempt_id)? != *logical_call_id
                    || Some(*assistant_entry_id) != state.current_assistant_entry
                {
                    return Err(CorruptionError::UnknownLogicalToolCall(*logical_call_id));
                }
                if active_tool_attempt_mut(&mut state, *attempt_id)?.status
                    != ToolAttemptStatus::Started
                {
                    return Err(CorruptionError::DuplicateToolPreparation(*attempt_id));
                }
                let preparation = ToolCallPreparation {
                    assistant_entry_id: *assistant_entry_id,
                    source_index: *source_index,
                    provider_call_id: provider_call_id.clone(),
                    requested_tool_name: requested_tool_name.clone(),
                    result_entry_id: *result_entry_id,
                    disposition: disposition.as_ref().clone(),
                };
                // Every attempt of one logical call must reproduce the same
                // pinned tool, digest, effect, replay, policy, and reservation.
                if state
                    .tool_preparations
                    .get(logical_call_id)
                    .is_some_and(|existing| *existing != preparation)
                {
                    return Err(CorruptionError::ToolPreparationConflict(*logical_call_id));
                }
                state
                    .tool_preparations
                    .insert(*logical_call_id, preparation);
                active_tool_attempt_mut(&mut state, *attempt_id)?.status =
                    ToolAttemptStatus::Prepared;
            }
            AgentRecordData::ToolEffectStarted { attempt_id } => {
                require_phase(&state, AgentPhase::ToolInFlight, record.data.kind())?;
                require_attempt(state.active_tool_attempt, *attempt_id)?;
                let logical_call_id = active_tool_call_id(&state, *attempt_id)?;
                let executable = matches!(
                    state.tool_preparations.get(&logical_call_id),
                    Some(ToolCallPreparation {
                        disposition: ToolPreparationDisposition::Executable { .. },
                        ..
                    })
                );
                let attempt = active_tool_attempt_mut(&mut state, *attempt_id)?;
                if !executable || attempt.status != ToolAttemptStatus::Prepared {
                    return Err(CorruptionError::EffectStartWithoutExecutable(*attempt_id));
                }
                attempt.status = ToolAttemptStatus::EffectInFlight;
            }
            AgentRecordData::ToolEffectCompleted { attempt_id, result } => {
                require_phase(&state, AgentPhase::ToolInFlight, record.data.kind())?;
                require_attempt(state.active_tool_attempt, *attempt_id)?;
                let attempt = active_tool_attempt_mut(&mut state, *attempt_id)?;
                if attempt.status != ToolAttemptStatus::EffectInFlight {
                    return Err(CorruptionError::EffectCompletionWithoutStart(*attempt_id));
                }
                attempt.status = ToolAttemptStatus::EffectSettled {
                    result: result.clone(),
                };
            }
            AgentRecordData::ToolAttemptInterrupted { attempt_id, reason } => {
                require_phase(&state, AgentPhase::ToolInFlight, record.data.kind())?;
                require_attempt(state.active_tool_attempt, *attempt_id)?;
                active_tool_attempt_mut(&mut state, *attempt_id)?.status =
                    ToolAttemptStatus::Interrupted {
                        reason: reason.clone(),
                    };
                state.active_tool_attempt = None;
                // The logical call stays pending; a later attempt may retry it.
                state.phase = AgentPhase::ToolsPending;
            }
            AgentRecordData::ToolAttemptRejected {
                attempt_id,
                result_entry_id,
                failed_at,
            } => {
                finish_tool_attempt(
                    &mut state,
                    &entry_by_id,
                    *attempt_id,
                    *result_entry_id,
                    ToolSettlement::Rejected(*failed_at),
                    &mut finalized_entries,
                    &mut current_turn_result_ids,
                )?;
            }
            AgentRecordData::ToolAttemptDenied {
                attempt_id,
                result_entry_id,
            } => {
                finish_tool_attempt(
                    &mut state,
                    &entry_by_id,
                    *attempt_id,
                    *result_entry_id,
                    ToolSettlement::Denied,
                    &mut finalized_entries,
                    &mut current_turn_result_ids,
                )?;
            }
            AgentRecordData::ToolAttemptCompleted {
                attempt_id,
                result_entry_id,
            } => {
                finish_tool_attempt(
                    &mut state,
                    &entry_by_id,
                    *attempt_id,
                    *result_entry_id,
                    ToolSettlement::Completed,
                    &mut finalized_entries,
                    &mut current_turn_result_ids,
                )?;
            }
            AgentRecordData::ToolAttemptIntervention { attempt_id, reason } => {
                require_phase(&state, AgentPhase::ToolInFlight, record.data.kind())?;
                require_attempt(state.active_tool_attempt, *attempt_id)?;
                let logical_call_id = active_tool_call_id(&state, *attempt_id)?;
                {
                    let attempt = active_tool_attempt_mut(&mut state, *attempt_id)?;
                    attempt.status = ToolAttemptStatus::Intervention {
                        reason: reason.clone(),
                    };
                }
                state.active_tool_attempt = None;
                state.pending_tool_calls.retain(|id| *id != logical_call_id);
                state.phase = AgentPhase::InterventionPending;
            }
            AgentRecordData::TurnCommitted {
                turn_ordinal,
                assistant_entry_id,
                tool_result_entry_ids,
            } => {
                require_phase(&state, AgentPhase::ReadyToCommitTurn, record.data.kind())?;
                if *turn_ordinal != state.next_turn_ordinal {
                    return Err(CorruptionError::TurnOrdinal {
                        expected: state.next_turn_ordinal,
                        actual: *turn_ordinal,
                    });
                }
                if Some(*assistant_entry_id) != state.current_assistant_entry {
                    return Err(CorruptionError::UnknownEntry(*assistant_entry_id));
                }
                if tool_result_entry_ids != &current_turn_result_ids {
                    return Err(CorruptionError::TurnResultOrder);
                }
                state.next_turn_ordinal += 1;
                state.turn_count += 1;
                state.current_assistant_entry = None;
                state.pending_tool_calls.clear();
                current_turn_result_ids.clear();
                state.phase = AgentPhase::ReadyForModel;
            }
            AgentRecordData::AgentResultCommitted { result } => {
                if state.active_model_attempt.is_some() || state.active_tool_attempt.is_some() {
                    return Err(CorruptionError::InvalidRecord {
                        record: record.data.kind(),
                        phase: state.phase,
                    });
                }
                match result {
                    AgentTerminalResult::Succeeded { assistant_entry_id } => {
                        require_phase(&state, AgentPhase::ReadyToCommitResult, record.data.kind())?;
                        if Some(*assistant_entry_id) != state.current_assistant_entry {
                            return Err(CorruptionError::TerminalAssistantMismatch(
                                *assistant_entry_id,
                            ));
                        }
                    }
                    AgentTerminalResult::RequiresIntervention { .. } => {
                        require_phase(&state, AgentPhase::InterventionPending, record.data.kind())?;
                    }
                    AgentTerminalResult::Failed { .. }
                    | AgentTerminalResult::Cancelled
                    | AgentTerminalResult::TimedOut
                    | AgentTerminalResult::BudgetExhausted { .. } => {}
                }
                state.terminal_result = Some(result.clone());
                state.phase = AgentPhase::Terminal;
            }
        }
    }

    for entry in entries {
        if !matches!(entry.data, AgentEntryData::UserInput { .. })
            && !finalized_entries.contains(&entry.id)
        {
            return Err(CorruptionError::OrphanGeneratedEntry(entry.id));
        }
    }

    Ok(state)
}

fn require_phase(
    state: &AgentRecoveryState,
    expected: AgentPhase,
    record: &'static str,
) -> Result<(), CorruptionError> {
    if state.phase == expected {
        Ok(())
    } else {
        Err(CorruptionError::InvalidRecord {
            record,
            phase: state.phase,
        })
    }
}

fn require_attempt<T: std::fmt::Display + Copy + Eq>(
    active: Option<T>,
    actual: T,
) -> Result<(), CorruptionError> {
    match active {
        Some(expected) if expected == actual => Ok(()),
        Some(expected) => Err(CorruptionError::AttemptMismatch {
            expected: expected.to_string(),
            actual: actual.to_string(),
        }),
        None => Err(CorruptionError::AttemptMismatch {
            expected: "none".to_owned(),
            actual: actual.to_string(),
        }),
    }
}

fn append_context_entry(
    state: &mut AgentRecoveryState,
    entry: &AgentEntry,
) -> Result<(), CorruptionError> {
    let expected_ordinal = state.context_entry_ids.len() as u64 + 1;
    if entry.ordinal != expected_ordinal {
        return Err(CorruptionError::ContextOrder(entry.id));
    }
    state.context_entry_ids.push(entry.id);
    Ok(())
}

fn active_tool_call_id(
    state: &AgentRecoveryState,
    attempt_id: ToolAttemptId,
) -> Result<LogicalToolCallId, CorruptionError> {
    for (call_id, attempts) in &state.tool_attempts {
        if attempts.iter().any(|attempt| attempt.id == attempt_id) {
            return Ok(*call_id);
        }
    }
    Err(CorruptionError::UnknownToolAttempt(attempt_id))
}

fn active_tool_attempt_mut(
    state: &mut AgentRecoveryState,
    attempt_id: ToolAttemptId,
) -> Result<&mut ToolAttemptState, CorruptionError> {
    for attempts in state.tool_attempts.values_mut() {
        if let Some(attempt) = attempts.iter_mut().find(|attempt| attempt.id == attempt_id) {
            return Ok(attempt);
        }
    }
    Err(CorruptionError::UnknownToolAttempt(attempt_id))
}

fn finish_tool_attempt(
    state: &mut AgentRecoveryState,
    entry_by_id: &BTreeMap<AgentEntryId, &AgentEntry>,
    attempt_id: ToolAttemptId,
    result_entry_id: AgentEntryId,
    settlement: ToolSettlement,
    finalized_entries: &mut BTreeSet<AgentEntryId>,
    current_turn_result_ids: &mut Vec<AgentEntryId>,
) -> Result<(), CorruptionError> {
    require_phase(state, AgentPhase::ToolInFlight, settlement.kind())?;
    require_attempt(state.active_tool_attempt, attempt_id)?;
    let logical_call_id = active_tool_call_id(state, attempt_id)?;
    if let Some(preparation) = state.tool_preparations.get(&logical_call_id) {
        if preparation.result_entry_id != result_entry_id {
            return Err(CorruptionError::ReservedResultMismatch {
                expected: preparation.result_entry_id,
                actual: result_entry_id,
            });
        }
        // Q008 — Resolve/Validate/Classify settle as Rejected, Policy as Denied.
        let failed_at = match &preparation.disposition {
            ToolPreparationDisposition::Executable { .. } => None,
            ToolPreparationDisposition::NoEffect { failed_at, .. } => Some(*failed_at),
        };
        let consistent = match settlement {
            ToolSettlement::Completed => failed_at.is_none(),
            ToolSettlement::Denied => failed_at == Some(ToolPreparationStage::Policy),
            ToolSettlement::Rejected(stage) => {
                stage != ToolPreparationStage::Policy && failed_at == Some(stage)
            }
        };
        if !consistent {
            return Err(CorruptionError::WrongNoEffectRecord(attempt_id));
        }
    }
    let entry = *entry_by_id
        .get(&result_entry_id)
        .ok_or(CorruptionError::UnknownEntry(result_entry_id))?;
    let AgentEntryData::ModelVisibleToolResult {
        logical_call_id: entry_call_id,
        attempt_id: entry_attempt_id,
        ..
    } = &entry.data
    else {
        return Err(CorruptionError::ToolResultMismatch {
            entry_id: result_entry_id,
            attempt_id,
            logical_call_id,
        });
    };
    if *entry_call_id != logical_call_id || *entry_attempt_id != attempt_id {
        return Err(CorruptionError::ToolResultMismatch {
            entry_id: result_entry_id,
            attempt_id,
            logical_call_id,
        });
    }
    if state.completed_tool_results.contains_key(&logical_call_id) {
        return Err(CorruptionError::DuplicateLogicalToolResult(logical_call_id));
    }
    append_context_entry(state, entry)?;
    finalized_entries.insert(result_entry_id);
    state
        .completed_tool_results
        .insert(logical_call_id, result_entry_id);
    current_turn_result_ids.push(result_entry_id);
    {
        let attempt = active_tool_attempt_mut(state, attempt_id)?;
        attempt.status = match settlement {
            ToolSettlement::Completed => ToolAttemptStatus::Completed { result_entry_id },
            ToolSettlement::Denied => ToolAttemptStatus::Denied { result_entry_id },
            ToolSettlement::Rejected(failed_at) => ToolAttemptStatus::Rejected {
                result_entry_id,
                failed_at,
            },
        };
    }
    state.active_tool_attempt = None;
    state.pending_tool_calls.remove(0);
    state.phase = if state.pending_tool_calls.is_empty() {
        AgentPhase::ReadyToCommitTurn
    } else {
        AgentPhase::ToolsPending
    };
    Ok(())
}

fn add_usage(left: ModelUsage, right: ModelUsage) -> ModelUsage {
    ModelUsage {
        input_tokens: left.input_tokens.saturating_add(right.input_tokens),
        output_tokens: left.output_tokens.saturating_add(right.output_tokens),
        cache_read_input_tokens: left
            .cache_read_input_tokens
            .saturating_add(right.cache_read_input_tokens),
        cache_write_input_tokens: left
            .cache_write_input_tokens
            .saturating_add(right.cache_write_input_tokens),
    }
}
