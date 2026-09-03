//! G003 T006 — RecordedProvider conformance.
//!
//! Every scenario drives the real `ModelProvider` contract and feeds the
//! resulting normalized events through the real `ResponseAssembler`, so these
//! tests exercise the same path the Agent kernel will take.

use futures_util::StreamExt;
use serde_json::json;
use tokio_util::sync::CancellationToken;
use ww_agent_provider::{
    AssemblyError, CompletionReason, ExpectedRequest, MessageContent, ModelEvent, ModelEventStream,
    ModelId, ModelMessage, ModelProvider, ModelRequest, ModelResponse, ModelToolSpec, ModelUsage,
    ProviderContext, ProviderError, ProviderFailure, ProviderId, ProviderStarted, RecordedOutcome,
    RecordedProvider, ResponseAssembler, ToolCall, ToolCallId,
};

fn model() -> ModelId {
    ModelId::new("recorded-1").unwrap()
}

fn call_id(value: &str) -> ToolCallId {
    ToolCallId::new(value).unwrap()
}

fn provider() -> RecordedProvider {
    RecordedProvider::new(ProviderId::new("recorded").unwrap())
}

fn request(messages: Vec<ModelMessage>) -> ModelRequest {
    ModelRequest {
        model: model(),
        system_prompt: None,
        messages,
        tools: Vec::new(),
    }
}

fn user(text: &str) -> ModelMessage {
    ModelMessage {
        role: ww_agent_provider::MessageRole::User,
        content: vec![MessageContent::Text {
            text: text.to_owned(),
        }],
    }
}

fn tool_result(id: &str, name: &str) -> MessageContent {
    MessageContent::ToolResult {
        call_id: call_id(id),
        tool_name: name.to_owned(),
        content: json!({"ok": true}),
        is_error: false,
    }
}

fn started() -> ModelEvent {
    ModelEvent::Started {
        started: ProviderStarted {
            request_id: Some("req-1".to_owned()),
        },
    }
}

/// Emit a whole tool call as the provider would: start, argument fragments,
/// completion.
fn tool_call_events(id: &str, name: &str, arguments: &str) -> Vec<ModelEvent> {
    let mid = arguments.len() / 2;
    vec![
        ModelEvent::ToolCallStarted {
            id: call_id(id),
            name: name.to_owned(),
        },
        ModelEvent::ToolCallArgumentsDelta {
            id: call_id(id),
            delta: arguments[..mid].to_owned(),
        },
        ModelEvent::ToolCallArgumentsDelta {
            id: call_id(id),
            delta: arguments[mid..].to_owned(),
        },
        ModelEvent::ToolCallCompleted { id: call_id(id) },
    ]
}

fn context() -> (ProviderContext, CancellationToken) {
    let token = CancellationToken::new();
    (
        ProviderContext {
            cancellation: token.clone(),
        },
        token,
    )
}

/// `ModelEventStream` is not `Debug`, so `expect_err` cannot be used on it.
fn stream_err(result: Result<ModelEventStream, ProviderError>) -> ProviderError {
    match result {
        Ok(_) => panic!("expected a provider error, got a stream"),
        Err(error) => error,
    }
}

/// Drive one exchange and assemble the stream into a terminal response.
/// `Ok(None)` means the stream ended without a terminal event.
async fn drive(
    provider: &RecordedProvider,
    request: ModelRequest,
    context: ProviderContext,
) -> Result<Option<ModelResponse>, AssemblyError> {
    let mut stream = provider
        .stream(request, context)
        .await
        .expect("scripted stream");
    let mut assembler = ResponseAssembler::new();
    while let Some(event) = stream.next().await {
        let event = event.expect("recorded providers never emit stream errors");
        if let Some(terminal) = assembler.push(event)? {
            return Ok(Some(terminal));
        }
    }
    Ok(None)
}

// 1. text-only completion
#[tokio::test]
async fn text_only_completion() {
    let provider = provider().expect(
        ExpectedRequest::any().model(model()).message_count(1),
        RecordedOutcome::Stream(vec![
            started(),
            ModelEvent::TextDelta {
                delta: "hello ".to_owned(),
            },
            ModelEvent::TextDelta {
                delta: "world".to_owned(),
            },
            ModelEvent::Completed {
                reason: CompletionReason::Stop,
            },
        ]),
    );
    let (ctx, _token) = context();

    let terminal = drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal response");

    let ModelResponse::Completed { message } = terminal else {
        panic!("expected completed response")
    };
    assert_eq!(
        message.content,
        vec![MessageContent::Text {
            text: "hello world".to_owned()
        }]
    );
    assert_eq!(message.stop_reason, CompletionReason::Stop);
    provider.verify().expect("script satisfied");
}

// 2. one tool call followed by a later final response
#[tokio::test]
async fn tool_call_then_final_response() {
    let mut events = vec![started()];
    events.extend(tool_call_events("call-1", "test.echo", r#"{"value":"a"}"#));
    events.push(ModelEvent::Completed {
        reason: CompletionReason::ToolUse,
    });

    let provider = provider()
        .expect(
            ExpectedRequest::any().message_count(1),
            RecordedOutcome::Stream(events),
        )
        .expect(
            // the second request must carry the tool result back
            ExpectedRequest::any()
                .message_count(3)
                .tool_results_in_order([call_id("call-1")]),
            RecordedOutcome::Stream(vec![
                started(),
                ModelEvent::TextDelta {
                    delta: "done".to_owned(),
                },
                ModelEvent::Completed {
                    reason: CompletionReason::Stop,
                },
            ]),
        );

    let (ctx, _token) = context();
    let first = drive(&provider, request(vec![user("go")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal");
    let ModelResponse::Completed { message } = first else {
        panic!("expected completed response")
    };
    let calls: Vec<&ToolCall> = message.tool_calls().collect();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "test.echo");
    assert_eq!(calls[0].arguments, json!({"value": "a"}));

    // model → tool → model: append the assistant turn and the tool result
    let (ctx, _token) = context();
    let second = drive(
        &provider,
        request(vec![
            user("go"),
            ModelMessage {
                role: ww_agent_provider::MessageRole::Assistant,
                content: message.content.clone(),
            },
            ModelMessage {
                role: ww_agent_provider::MessageRole::Tool,
                content: vec![tool_result("call-1", "test.echo")],
            },
        ]),
        ctx,
    )
    .await
    .expect("assembles")
    .expect("terminal");
    assert!(matches!(second, ModelResponse::Completed { .. }));
    provider.verify().expect("script satisfied");
}

// 3. multiple tool calls with stable provider source ordering
#[tokio::test]
async fn multiple_tool_calls_preserve_source_order() {
    let mut events = vec![started()];
    events.extend(tool_call_events("call-a", "test.echo", r#"{"value":1}"#));
    events.extend(tool_call_events("call-b", "test.echo", r#"{"value":2}"#));
    events.extend(tool_call_events("call-c", "test.echo", r#"{"value":3}"#));
    events.push(ModelEvent::Completed {
        reason: CompletionReason::ToolUse,
    });

    let provider = provider()
        .expect_any(RecordedOutcome::Stream(events))
        .expect(
            // results must return in the same order the provider emitted them
            ExpectedRequest::any().tool_results_in_order([
                call_id("call-a"),
                call_id("call-b"),
                call_id("call-c"),
            ]),
            RecordedOutcome::Stream(vec![
                started(),
                ModelEvent::Completed {
                    reason: CompletionReason::Stop,
                },
            ]),
        );

    let (ctx, _token) = context();
    let terminal = drive(&provider, request(vec![user("go")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal");
    let ModelResponse::Completed { message } = terminal else {
        panic!("expected completed response")
    };
    let ids: Vec<&str> = message.tool_calls().map(|call| call.id.as_str()).collect();
    assert_eq!(ids, vec!["call-a", "call-b", "call-c"]);

    let (ctx, _token) = context();
    drive(
        &provider,
        request(vec![
            user("go"),
            ModelMessage {
                role: ww_agent_provider::MessageRole::Tool,
                content: vec![
                    tool_result("call-a", "test.echo"),
                    tool_result("call-b", "test.echo"),
                    tool_result("call-c", "test.echo"),
                ],
            },
        ]),
        ctx,
    )
    .await
    .expect("assembles")
    .expect("terminal");
    provider.verify().expect("script satisfied");
}

// 4. usage accounting
#[tokio::test]
async fn usage_is_finalized_on_the_response() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::Usage {
            usage: ModelUsage {
                input_tokens: 10,
                output_tokens: 2,
                ..ModelUsage::default()
            },
        },
        ModelEvent::Usage {
            usage: ModelUsage {
                input_tokens: 10,
                output_tokens: 7,
                cache_read_input_tokens: 3,
                ..ModelUsage::default()
            },
        },
        ModelEvent::Completed {
            reason: CompletionReason::Stop,
        },
    ]));
    let (ctx, _token) = context();

    let terminal = drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal");
    let ModelResponse::Completed { message } = terminal else {
        panic!("expected completed response")
    };
    let usage = message.usage.expect("usage present");
    assert_eq!(usage.input_tokens, 10);
    assert_eq!(usage.output_tokens, 7);
    assert_eq!(usage.cache_read_input_tokens, 3);
    assert_eq!(usage.total_tokens(), 17);
    provider.verify().expect("script satisfied");
}

// 5. provider-declared failure
#[tokio::test]
async fn provider_declared_failure_is_terminal() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::Failed {
            failure: ProviderFailure::new("overloaded", "try later", true),
        },
    ]));
    let (ctx, _token) = context();

    let terminal = drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal");
    let ModelResponse::Failed { failure } = terminal else {
        panic!("expected failed response")
    };
    assert_eq!(failure.code, "overloaded");
    assert!(failure.retryable);
    provider.verify().expect("script satisfied");
}

// 5b. transport unavailable before any normalized stream exists
#[tokio::test]
async fn unavailable_provider_yields_no_stream() {
    let provider = provider().expect_any(RecordedOutcome::Unavailable(ProviderError::Transport(
        "connection refused".to_owned(),
    )));
    let (ctx, _token) = context();

    let error = stream_err(provider.stream(request(vec![user("hi")]), ctx).await);
    assert_eq!(error, ProviderError::Transport("connection refused".into()));
    provider.verify().expect("script satisfied");
}

// 6. cancellation
#[tokio::test]
async fn cancellation_aborts_the_stream_between_events() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::TextDelta {
            delta: "partial".to_owned(),
        },
        ModelEvent::Completed {
            reason: CompletionReason::Stop,
        },
    ]));
    let (ctx, token) = context();

    let mut stream = provider
        .stream(request(vec![user("hi")]), ctx)
        .await
        .expect("scripted stream");
    let mut assembler = ResponseAssembler::new();

    // consume Started, then cancel before the provider yields anything else
    let first = stream.next().await.expect("started").expect("event");
    assert!(assembler.push(first).expect("push started").is_none());
    token.cancel();

    let mut terminal = None;
    while let Some(event) = stream.next().await {
        if let Some(response) = assembler.push(event.expect("event")).expect("push") {
            terminal = Some(response);
        }
    }
    let ModelResponse::Aborted { message } = terminal.expect("aborted terminal") else {
        panic!("expected aborted response")
    };
    assert_eq!(message.as_deref(), Some("cancelled"));
    provider.verify().expect("script satisfied");
}

// 7. truncated response must not yield an executable tool call
#[tokio::test]
async fn length_truncated_tool_call_fails_closed() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::ToolCallStarted {
            id: call_id("call-1"),
            name: "test.echo".to_owned(),
        },
        ModelEvent::ToolCallArgumentsDelta {
            id: call_id("call-1"),
            delta: r#"{"value":"#.to_owned(),
        },
        ModelEvent::Completed {
            reason: CompletionReason::Length,
        },
    ]));
    let (ctx, _token) = context();

    let error = drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect_err("truncated tool call must fail closed");
    assert!(
        matches!(
            error,
            AssemblyError::IncompleteToolCall(_) | AssemblyError::TruncatedToolCalls
        ),
        "unexpected error: {error:?}"
    );
    provider.verify().expect("script satisfied");
}

// 8. interrupted model attempt — stream ends with no terminal event
#[tokio::test]
async fn interrupted_attempt_has_no_terminal_event() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::TextDelta {
            delta: "half a th".to_owned(),
        },
    ]));
    let (ctx, _token) = context();

    let terminal = drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect("stream ends cleanly");
    assert!(
        terminal.is_none(),
        "an interrupted attempt must not produce a terminal response"
    );
    provider.verify().expect("script satisfied");
}

// determinism: the same script yields byte-identical responses across runs
#[tokio::test]
async fn script_is_deterministic_across_runs() {
    async fn run() -> ModelResponse {
        let provider = provider().expect_any(RecordedOutcome::Stream(vec![
            started(),
            ModelEvent::TextDelta {
                delta: "stable".to_owned(),
            },
            ModelEvent::Usage {
                usage: ModelUsage {
                    input_tokens: 1,
                    output_tokens: 1,
                    ..ModelUsage::default()
                },
            },
            ModelEvent::Completed {
                reason: CompletionReason::Stop,
            },
        ]));
        let (ctx, _token) = context();
        drive(&provider, request(vec![user("hi")]), ctx)
            .await
            .expect("assembles")
            .expect("terminal")
    }

    let first = serde_json::to_string(&run().await).expect("serialize");
    let second = serde_json::to_string(&run().await).expect("serialize");
    assert_eq!(first, second);
}

// the fixture asserts on requests, and rejects ones the script did not expect
#[tokio::test]
async fn mismatched_request_is_rejected_and_recorded() {
    let provider = provider().expect(
        ExpectedRequest::any().message_count(2),
        RecordedOutcome::Stream(vec![started()]),
    );
    let (ctx, _token) = context();

    let error = stream_err(provider.stream(request(vec![user("only one")]), ctx).await);
    assert!(matches!(error, ProviderError::Request(_)), "{error:?}");
    assert!(
        provider.verify().is_err(),
        "a violated expectation must fail verification"
    );
}

#[tokio::test]
async fn extra_request_beyond_the_script_is_rejected() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::Completed {
            reason: CompletionReason::Stop,
        },
    ]));
    let (ctx, _token) = context();
    drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal");
    assert!(provider.verify().is_ok(), "script should be satisfied here");

    let (ctx, _token) = context();
    let error = stream_err(provider.stream(request(vec![user("again")]), ctx).await);
    assert!(matches!(error, ProviderError::Request(_)), "{error:?}");
    assert!(provider.verify().is_err(), "extra request must be recorded");
    assert_eq!(provider.requests().len(), 2);
}

#[tokio::test]
async fn unused_script_entries_fail_verification() {
    let provider = provider()
        .expect_any(RecordedOutcome::Stream(vec![
            started(),
            ModelEvent::Completed {
                reason: CompletionReason::Stop,
            },
        ]))
        .expect_any(RecordedOutcome::Stream(vec![started()]));
    let (ctx, _token) = context();
    drive(&provider, request(vec![user("hi")]), ctx)
        .await
        .expect("assembles")
        .expect("terminal");

    let error = provider.verify().expect_err("one exchange is unused");
    assert!(error.contains("never used"), "{error}");
}

#[tokio::test]
async fn requests_are_captured_for_assertions() {
    let provider = provider().expect_any(RecordedOutcome::Stream(vec![
        started(),
        ModelEvent::Completed {
            reason: CompletionReason::Stop,
        },
    ]));
    let (ctx, _token) = context();
    let mut req = request(vec![user("hi")]);
    req.tools = vec![ModelToolSpec {
        name: "test.echo".to_owned(),
        description: "echo".to_owned(),
        input_schema: json!({"type": "object"}),
    }];
    drive(&provider, req, ctx)
        .await
        .expect("assembles")
        .expect("terminal");

    let captured = provider.requests();
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].tools[0].name, "test.echo");
    provider.verify().expect("script satisfied");
}
