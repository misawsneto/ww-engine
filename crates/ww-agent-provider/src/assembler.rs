use crate::{
    AssistantMessage, CompletionReason, MessageContent, ModelEvent, ModelResponse, ModelUsage,
    ProviderStarted, ToolCall, ToolCallId,
};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AssemblyError {
    #[error("provider stream event {event} arrived before Started")]
    EventBeforeStart { event: &'static str },
    #[error("provider stream emitted Started more than once")]
    DuplicateStart,
    #[error("provider stream emitted an event after terminal response")]
    EventAfterTerminal,
    #[error("provider stream ended without a terminal event")]
    UnexpectedEnd,
    #[error("tool call id {0} is duplicated in one assistant response")]
    DuplicateToolCallId(ToolCallId),
    #[error("tool call id {0} is unknown")]
    UnknownToolCall(ToolCallId),
    #[error("tool call id {0} received argument data after completion")]
    ToolCallAlreadyCompleted(ToolCallId),
    #[error("tool call id {0} completed more than once")]
    DuplicateToolCallCompletion(ToolCallId),
    #[error("tool call id {0} has invalid JSON arguments: {1}")]
    InvalidToolArguments(ToolCallId, String),
    #[error("terminal response arrived while tool call id {0} was incomplete")]
    IncompleteToolCall(ToolCallId),
    #[error("length-truncated response contains tool calls and is not executable")]
    TruncatedToolCalls,
    #[error("tool_use completion reason requires at least one completed tool call")]
    ToolUseWithoutToolCall,
    #[error("stop completion reason cannot contain tool calls")]
    StopWithToolCall,
    #[error("usage counters regressed within one provider response")]
    UsageRegression,
    #[error("response assembler is poisoned by a previous protocol error")]
    Poisoned,
}

#[derive(Clone, Debug)]
struct ToolCallAssembly {
    id: ToolCallId,
    name: String,
    arguments_json: String,
    completed: bool,
    arguments: Option<Value>,
}

#[derive(Clone, Debug)]
struct StreamingState {
    started: ProviderStarted,
    text: String,
    tool_calls: Vec<ToolCallAssembly>,
    tool_indexes: HashMap<ToolCallId, usize>,
    usage: Option<ModelUsage>,
}

#[derive(Clone, Debug, Default)]
enum State {
    #[default]
    AwaitingStart,
    Streaming(StreamingState),
    Terminal(ModelResponse),
    Poisoned,
}

#[derive(Clone, Debug, Default)]
pub struct ResponseAssembler {
    state: State,
}

impl ResponseAssembler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: ModelEvent) -> Result<Option<ModelResponse>, AssemblyError> {
        let result = self.push_inner(event);
        if result.is_err() {
            self.state = State::Poisoned;
        }
        result
    }

    pub fn finish(self) -> Result<ModelResponse, AssemblyError> {
        match self.state {
            State::Terminal(response) => Ok(response),
            State::Poisoned => Err(AssemblyError::Poisoned),
            State::AwaitingStart | State::Streaming(_) => Err(AssemblyError::UnexpectedEnd),
        }
    }

    fn push_inner(&mut self, event: ModelEvent) -> Result<Option<ModelResponse>, AssemblyError> {
        if matches!(&self.state, State::Poisoned) {
            return Err(AssemblyError::Poisoned);
        }
        if matches!(&self.state, State::Terminal(_)) {
            return Err(AssemblyError::EventAfterTerminal);
        }

        if let ModelEvent::Started { started } = &event {
            match &self.state {
                State::AwaitingStart => {}
                State::Streaming(_) => return Err(AssemblyError::DuplicateStart),
                State::Terminal(_) => return Err(AssemblyError::EventAfterTerminal),
                State::Poisoned => return Err(AssemblyError::Poisoned),
            }
            self.state = State::Streaming(StreamingState {
                started: started.clone(),
                text: String::new(),
                tool_calls: Vec::new(),
                tool_indexes: HashMap::new(),
                usage: None,
            });
            return Ok(None);
        }

        let event_name = event.kind();
        let State::Streaming(state) = &mut self.state else {
            return Err(AssemblyError::EventBeforeStart { event: event_name });
        };

        match event {
            ModelEvent::Started { .. } => unreachable!("Started handled above"),
            ModelEvent::TextDelta { delta } => {
                state.text.push_str(&delta);
                Ok(None)
            }
            ModelEvent::ToolCallStarted { id, name } => {
                if state.tool_indexes.contains_key(&id) {
                    return Err(AssemblyError::DuplicateToolCallId(id));
                }
                let index = state.tool_calls.len();
                state.tool_indexes.insert(id.clone(), index);
                state.tool_calls.push(ToolCallAssembly {
                    id,
                    name,
                    arguments_json: String::new(),
                    completed: false,
                    arguments: None,
                });
                Ok(None)
            }
            ModelEvent::ToolCallArgumentsDelta { id, delta } => {
                let tool = tool_mut(state, &id)?;
                if tool.completed {
                    return Err(AssemblyError::ToolCallAlreadyCompleted(id));
                }
                tool.arguments_json.push_str(&delta);
                Ok(None)
            }
            ModelEvent::ToolCallCompleted { id } => {
                let tool = tool_mut(state, &id)?;
                if tool.completed {
                    return Err(AssemblyError::DuplicateToolCallCompletion(id));
                }
                let arguments = serde_json::from_str::<Value>(&tool.arguments_json).map_err(|error| {
                    AssemblyError::InvalidToolArguments(id.clone(), error.to_string())
                })?;
                tool.arguments = Some(arguments);
                tool.completed = true;
                Ok(None)
            }
            ModelEvent::Usage { usage } => {
                if let Some(previous) = state.usage
                    && !usage.dominates(previous)
                {
                    return Err(AssemblyError::UsageRegression);
                }
                state.usage = Some(usage);
                Ok(None)
            }
            ModelEvent::Completed { reason } => self.complete_success(reason),
            ModelEvent::Failed { failure } => {
                self.complete_terminal(ModelResponse::Failed { failure })
            }
            ModelEvent::Aborted { message } => {
                self.complete_terminal(ModelResponse::Aborted { message })
            }
        }
    }

    fn complete_success(
        &mut self,
        reason: CompletionReason,
    ) -> Result<Option<ModelResponse>, AssemblyError> {
        let State::Streaming(state) = &self.state else {
            unreachable!("completion requires streaming state")
        };

        if let Some(incomplete) = state.tool_calls.iter().find(|tool| !tool.completed) {
            return Err(AssemblyError::IncompleteToolCall(incomplete.id.clone()));
        }

        if reason == CompletionReason::Length && !state.tool_calls.is_empty() {
            return Err(AssemblyError::TruncatedToolCalls);
        }
        if reason == CompletionReason::ToolUse && state.tool_calls.is_empty() {
            return Err(AssemblyError::ToolUseWithoutToolCall);
        }
        if reason == CompletionReason::Stop && !state.tool_calls.is_empty() {
            return Err(AssemblyError::StopWithToolCall);
        }

        let mut content = Vec::new();
        if !state.text.is_empty() {
            content.push(MessageContent::Text {
                text: state.text.clone(),
            });
        }
        for tool in &state.tool_calls {
            content.push(MessageContent::ToolCall {
                call: ToolCall {
                    id: tool.id.clone(),
                    name: tool.name.clone(),
                    arguments_json: tool.arguments_json.clone(),
                    arguments: tool
                        .arguments
                        .clone()
                        .expect("completed tool call has parsed arguments"),
                },
            });
        }

        let response = ModelResponse::Completed {
            message: AssistantMessage {
                content,
                stop_reason: reason,
                usage: state.usage,
                provider_request_id: state.started.request_id.clone(),
            },
        };
        self.complete_terminal(response)
    }

    fn complete_terminal(
        &mut self,
        response: ModelResponse,
    ) -> Result<Option<ModelResponse>, AssemblyError> {
        self.state = State::Terminal(response.clone());
        Ok(Some(response))
    }
}

impl ModelEvent {
    const fn kind(&self) -> &'static str {
        match self {
            Self::Started { .. } => "started",
            Self::TextDelta { .. } => "text_delta",
            Self::ToolCallStarted { .. } => "tool_call_started",
            Self::ToolCallArgumentsDelta { .. } => "tool_call_arguments_delta",
            Self::ToolCallCompleted { .. } => "tool_call_completed",
            Self::Usage { .. } => "usage",
            Self::Completed { .. } => "completed",
            Self::Failed { .. } => "failed",
            Self::Aborted { .. } => "aborted",
        }
    }
}

fn tool_mut<'a>(
    state: &'a mut StreamingState,
    id: &ToolCallId,
) -> Result<&'a mut ToolCallAssembly, AssemblyError> {
    let Some(index) = state.tool_indexes.get(id).copied() else {
        return Err(AssemblyError::UnknownToolCall(id.clone()));
    };
    Ok(&mut state.tool_calls[index])
}
