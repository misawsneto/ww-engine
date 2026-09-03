//! Deterministic recorded provider used to drive the Agent kernel without
//! transport, credentials, or provider nondeterminism.
//!
//! A recorded provider is scripted as an ordered list of exchanges:
//!
//! ```text
//! expected request 1 -> stream events
//! expected request 2 -> stream events
//! ...
//! ```
//!
//! Each request the Agent sends is matched against the next scripted
//! expectation. A request that does not match, or that arrives after the
//! script is exhausted, is rejected and recorded as a violation so a test can
//! fail on it even if the caller swallows the provider error.

use crate::{
    MessageContent, ModelCapabilities, ModelEvent, ModelEventStream, ModelId, ModelProvider,
    ModelRequest, ProviderContext, ProviderError, ProviderId, ToolCallId,
};
use async_stream::try_stream;
use async_trait::async_trait;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

/// Declarative match against one request the Agent sends.
///
/// Every field is opt-in; an unset field is not checked. Assertions that do
/// not fit here belong in the test, using [`RecordedProvider::requests`].
#[derive(Clone, Debug, Default)]
pub struct ExpectedRequest {
    model: Option<ModelId>,
    message_count: Option<usize>,
    tool_names: Option<Vec<String>>,
    tool_result_call_ids: Option<Vec<ToolCallId>>,
}

impl ExpectedRequest {
    /// Accept any request.
    pub fn any() -> Self {
        Self::default()
    }

    /// Require an exact model pin.
    pub fn model(mut self, model: ModelId) -> Self {
        self.model = Some(model);
        self
    }

    /// Require an exact conversation length.
    pub fn message_count(mut self, count: usize) -> Self {
        self.message_count = Some(count);
        self
    }

    /// Require exactly these tool specs, in this order.
    pub fn tools<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tool_names = Some(names.into_iter().map(Into::into).collect());
        self
    }

    /// Require that the model-visible tool results carried by this request
    /// appear in exactly this order. This is the provider-source-order
    /// invariant the kernel must preserve.
    pub fn tool_results_in_order<I>(mut self, call_ids: I) -> Self
    where
        I: IntoIterator<Item = ToolCallId>,
    {
        self.tool_result_call_ids = Some(call_ids.into_iter().collect());
        self
    }

    fn check(&self, index: usize, request: &ModelRequest) -> Result<(), String> {
        if let Some(model) = &self.model
            && model != &request.model
        {
            return Err(format!(
                "request {index}: expected model {model}, got {}",
                request.model
            ));
        }
        if let Some(count) = self.message_count
            && count != request.messages.len()
        {
            return Err(format!(
                "request {index}: expected {count} messages, got {}",
                request.messages.len()
            ));
        }
        if let Some(expected) = &self.tool_names {
            let actual: Vec<&str> = request
                .tools
                .iter()
                .map(|spec| spec.name.as_str())
                .collect();
            if expected != &actual {
                return Err(format!(
                    "request {index}: expected tools {expected:?}, got {actual:?}"
                ));
            }
        }
        if let Some(expected) = &self.tool_result_call_ids {
            let actual: Vec<ToolCallId> = request
                .messages
                .iter()
                .flat_map(|message| message.content.iter())
                .filter_map(|content| match content {
                    MessageContent::ToolResult { call_id, .. } => Some(call_id.clone()),
                    _ => None,
                })
                .collect();
            if expected != &actual {
                return Err(format!(
                    "request {index}: expected tool results {expected:?}, got {actual:?}"
                ));
            }
        }
        Ok(())
    }
}

/// What the provider does once a request matches its expectation.
#[derive(Clone, Debug)]
pub enum RecordedOutcome {
    /// Emit these normalized events, in order.
    ///
    /// A script whose last event is not terminal reproduces an interrupted
    /// model attempt. Cancellation is honored between events: once the
    /// context token is cancelled the stream emits `Aborted` and stops.
    Stream(Vec<ModelEvent>),
    /// Fail before any normalized stream exists.
    Unavailable(ProviderError),
}

#[derive(Clone, Debug)]
struct Exchange {
    expect: ExpectedRequest,
    outcome: RecordedOutcome,
}

#[derive(Debug, Default)]
struct Journal {
    requests: Vec<ModelRequest>,
    violations: Vec<String>,
}

/// A `ModelProvider` that replays a fixed script and asserts on the requests
/// it receives.
#[derive(Clone)]
pub struct RecordedProvider {
    id: ProviderId,
    capabilities: ModelCapabilities,
    exchanges: Arc<Mutex<VecDeque<Exchange>>>,
    journal: Arc<Mutex<Journal>>,
}

impl RecordedProvider {
    pub fn new(id: ProviderId) -> Self {
        Self {
            id,
            capabilities: ModelCapabilities {
                tool_calls: true,
                usage: true,
            },
            exchanges: Arc::new(Mutex::new(VecDeque::new())),
            journal: Arc::new(Mutex::new(Journal::default())),
        }
    }

    pub fn with_capabilities(mut self, capabilities: ModelCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Append one scripted exchange.
    pub fn expect(self, expect: ExpectedRequest, outcome: RecordedOutcome) -> Self {
        self.exchanges
            .lock()
            .expect("recorded provider script lock")
            .push_back(Exchange { expect, outcome });
        self
    }

    /// Append a scripted exchange that accepts any request.
    pub fn expect_any(self, outcome: RecordedOutcome) -> Self {
        self.expect(ExpectedRequest::any(), outcome)
    }

    /// Every request received so far, in order.
    pub fn requests(&self) -> Vec<ModelRequest> {
        self.journal
            .lock()
            .expect("recorded provider journal lock")
            .requests
            .clone()
    }

    /// Scripted exchanges not yet consumed.
    pub fn remaining(&self) -> usize {
        self.exchanges
            .lock()
            .expect("recorded provider script lock")
            .len()
    }

    /// Confirm the script was followed exactly: no mismatched or extra
    /// request, and no scripted exchange left unused.
    pub fn verify(&self) -> Result<(), String> {
        let violations = self
            .journal
            .lock()
            .expect("recorded provider journal lock")
            .violations
            .clone();
        if !violations.is_empty() {
            return Err(violations.join("; "));
        }
        let remaining = self.remaining();
        if remaining > 0 {
            return Err(format!("{remaining} scripted exchange(s) were never used"));
        }
        Ok(())
    }

    fn record_violation(&self, detail: String) -> ProviderError {
        self.journal
            .lock()
            .expect("recorded provider journal lock")
            .violations
            .push(detail.clone());
        ProviderError::Request(detail)
    }
}

#[async_trait]
impl ModelProvider for RecordedProvider {
    fn id(&self) -> ProviderId {
        self.id.clone()
    }

    fn capabilities(&self, _model: &ModelId) -> ModelCapabilities {
        self.capabilities
    }

    async fn stream(
        &self,
        request: ModelRequest,
        context: ProviderContext,
    ) -> Result<ModelEventStream, ProviderError> {
        let index = {
            let mut journal = self.journal.lock().expect("recorded provider journal lock");
            journal.requests.push(request.clone());
            journal.requests.len() - 1
        };

        let exchange = self
            .exchanges
            .lock()
            .expect("recorded provider script lock")
            .pop_front();
        let Some(exchange) = exchange else {
            return Err(self.record_violation(format!(
                "request {index}: script exhausted, no exchange remains"
            )));
        };

        if let Err(detail) = exchange.expect.check(index, &request) {
            return Err(self.record_violation(detail));
        }

        match exchange.outcome {
            RecordedOutcome::Unavailable(error) => Err(error),
            RecordedOutcome::Stream(events) => {
                let cancellation = context.cancellation;
                Ok(Box::pin(try_stream! {
                    for event in events {
                        if cancellation.is_cancelled() {
                            yield ModelEvent::Aborted {
                                message: Some("cancelled".to_owned()),
                            };
                            break;
                        }
                        yield event;
                    }
                }))
            }
        }
    }
}
