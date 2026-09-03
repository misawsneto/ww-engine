use serde_json::json;
use ww_agent_provider::{
    AssemblyError, CompletionReason, MessageContent, ModelEvent, ModelResponse, ModelUsage,
    ProviderFailure, ProviderStarted, ResponseAssembler, StreamFinalizationError, ToolCallId,
    finalize_stream,
};

fn started() -> ModelEvent {
    ModelEvent::Started {
        started: ProviderStarted {
            request_id: Some("req-1".to_owned()),
        },
    }
}

fn call_id(value: &str) -> ToolCallId {
    ToolCallId::new(value).unwrap()
}

#[test]
fn assembles_text_response() {
    let mut assembler = ResponseAssembler::new();
    assert_eq!(assembler.push(started()).unwrap(), None);
    assembler
        .push(ModelEvent::TextDelta {
            delta: "hel".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::TextDelta {
            delta: "lo".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::Usage {
            usage: ModelUsage {
                input_tokens: 4,
                output_tokens: 2,
                ..ModelUsage::default()
            },
        })
        .unwrap();
    let terminal = assembler
        .push(ModelEvent::Completed {
            reason: CompletionReason::Stop,
        })
        .unwrap()
        .expect("terminal response");

    let ModelResponse::Completed { message } = terminal else {
        panic!("expected completed response")
    };
    assert_eq!(message.provider_request_id.as_deref(), Some("req-1"));
    assert_eq!(message.usage.unwrap().total_tokens(), 6);
    assert_eq!(
        message.content,
        vec![MessageContent::Text {
            text: "hello".to_owned()
        }]
    );
}

#[test]
fn assembles_tool_calls_in_provider_source_order() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();

    for (id, name, fragments) in [
        ("call-1", "test.first", vec!["{\"value\":", "1}"]),
        ("call-2", "test.second", vec!["{\"value\":", "2}"]),
    ] {
        let id = call_id(id);
        assembler
            .push(ModelEvent::ToolCallStarted {
                id: id.clone(),
                name: name.to_owned(),
            })
            .unwrap();
        for fragment in fragments {
            assembler
                .push(ModelEvent::ToolCallArgumentsDelta {
                    id: id.clone(),
                    delta: fragment.to_owned(),
                })
                .unwrap();
        }
        assembler
            .push(ModelEvent::ToolCallCompleted { id })
            .unwrap();
    }

    let terminal = assembler
        .push(ModelEvent::Completed {
            reason: CompletionReason::ToolUse,
        })
        .unwrap()
        .expect("terminal response");
    let ModelResponse::Completed { message } = terminal else {
        panic!("expected completed response")
    };
    let calls = message.tool_calls().collect::<Vec<_>>();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].name, "test.first");
    assert_eq!(calls[0].arguments, json!({"value": 1}));
    assert_eq!(calls[1].name, "test.second");
    assert_eq!(calls[1].arguments, json!({"value": 2}));
}

#[test]
fn rejects_delta_before_start_and_poisoned_assembler_stays_closed() {
    let mut assembler = ResponseAssembler::new();
    assert_eq!(
        assembler
            .push(ModelEvent::TextDelta {
                delta: "nope".to_owned()
            })
            .unwrap_err(),
        AssemblyError::EventBeforeStart {
            event: "text_delta"
        }
    );
    assert_eq!(
        assembler.push(started()).unwrap_err(),
        AssemblyError::Poisoned
    );
}

#[test]
fn rejects_duplicate_started() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    assert_eq!(
        assembler.push(started()).unwrap_err(),
        AssemblyError::DuplicateStart
    );
}

#[test]
fn rejects_duplicate_tool_call_id() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    let id = call_id("call-1");
    assembler
        .push(ModelEvent::ToolCallStarted {
            id: id.clone(),
            name: "test.echo".to_owned(),
        })
        .unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::ToolCallStarted {
                id: id.clone(),
                name: "test.echo".to_owned(),
            })
            .unwrap_err(),
        AssemblyError::DuplicateToolCallId(id)
    );
}

#[test]
fn rejects_unknown_tool_argument_delta() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    let id = call_id("missing");
    assert_eq!(
        assembler
            .push(ModelEvent::ToolCallArgumentsDelta {
                id: id.clone(),
                delta: "{}".to_owned(),
            })
            .unwrap_err(),
        AssemblyError::UnknownToolCall(id)
    );
}

#[test]
fn rejects_invalid_tool_json_before_terminal_response() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    let id = call_id("call-1");
    assembler
        .push(ModelEvent::ToolCallStarted {
            id: id.clone(),
            name: "test.echo".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::ToolCallArgumentsDelta {
            id: id.clone(),
            delta: "{not-json".to_owned(),
        })
        .unwrap();
    assert!(matches!(
        assembler
            .push(ModelEvent::ToolCallCompleted { id })
            .unwrap_err(),
        AssemblyError::InvalidToolArguments(_, _)
    ));
}

#[test]
fn rejects_terminal_response_with_incomplete_tool_call() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    let id = call_id("call-1");
    assembler
        .push(ModelEvent::ToolCallStarted {
            id: id.clone(),
            name: "test.echo".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::ToolCallArgumentsDelta {
            id: id.clone(),
            delta: "{}".to_owned(),
        })
        .unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::Completed {
                reason: CompletionReason::ToolUse,
            })
            .unwrap_err(),
        AssemblyError::IncompleteToolCall(id)
    );
}

#[test]
fn rejects_length_truncated_response_with_tool_calls_even_when_json_is_complete() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    let id = call_id("call-1");
    assembler
        .push(ModelEvent::ToolCallStarted {
            id: id.clone(),
            name: "test.echo".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::ToolCallArgumentsDelta {
            id: id.clone(),
            delta: "{}".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::ToolCallCompleted { id })
        .unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::Completed {
                reason: CompletionReason::Length,
            })
            .unwrap_err(),
        AssemblyError::TruncatedToolCalls
    );
}

#[test]
fn rejects_tool_use_without_tool_call() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::Completed {
                reason: CompletionReason::ToolUse,
            })
            .unwrap_err(),
        AssemblyError::ToolUseWithoutToolCall
    );
}

#[test]
fn rejects_stop_with_tool_call() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    let id = call_id("call-1");
    assembler
        .push(ModelEvent::ToolCallStarted {
            id: id.clone(),
            name: "test.echo".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::ToolCallArgumentsDelta {
            id: id.clone(),
            delta: "{}".to_owned(),
        })
        .unwrap();
    assembler
        .push(ModelEvent::ToolCallCompleted { id })
        .unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::Completed {
                reason: CompletionReason::Stop,
            })
            .unwrap_err(),
        AssemblyError::StopWithToolCall
    );
}

#[test]
fn rejects_duplicate_terminal_event() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    assembler
        .push(ModelEvent::Completed {
            reason: CompletionReason::Stop,
        })
        .unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::Failed {
                failure: ProviderFailure::new("late", "too late", false),
            })
            .unwrap_err(),
        AssemblyError::EventAfterTerminal
    );
}

#[test]
fn rejects_stream_end_without_terminal_event() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    assembler
        .push(ModelEvent::TextDelta {
            delta: "partial".to_owned(),
        })
        .unwrap();
    assert_eq!(
        assembler.finish().unwrap_err(),
        AssemblyError::UnexpectedEnd
    );
}

#[test]
fn rejects_usage_regression() {
    let mut assembler = ResponseAssembler::new();
    assembler.push(started()).unwrap();
    assembler
        .push(ModelEvent::Usage {
            usage: ModelUsage {
                input_tokens: 10,
                output_tokens: 2,
                ..ModelUsage::default()
            },
        })
        .unwrap();
    assert_eq!(
        assembler
            .push(ModelEvent::Usage {
                usage: ModelUsage {
                    input_tokens: 9,
                    output_tokens: 2,
                    ..ModelUsage::default()
                },
            })
            .unwrap_err(),
        AssemblyError::UsageRegression
    );
}

#[test]
fn failed_and_aborted_events_are_terminal_without_executable_message() {
    let mut failed = ResponseAssembler::new();
    failed.push(started()).unwrap();
    let response = failed
        .push(ModelEvent::Failed {
            failure: ProviderFailure::new("provider", "failed", true),
        })
        .unwrap()
        .unwrap();
    assert!(matches!(response, ModelResponse::Failed { .. }));

    let mut aborted = ResponseAssembler::new();
    aborted.push(started()).unwrap();
    let response = aborted
        .push(ModelEvent::Aborted {
            message: Some("cancelled".to_owned()),
        })
        .unwrap()
        .unwrap();
    assert!(matches!(response, ModelResponse::Aborted { .. }));
}

#[tokio::test]
async fn production_finalizer_requires_terminal_eof() {
    let stream = Box::pin(futures_util::stream::iter(vec![Ok(started())]));
    assert_eq!(
        finalize_stream(stream).await.unwrap_err(),
        StreamFinalizationError::Assembly(AssemblyError::UnexpectedEnd)
    );
}

#[tokio::test]
async fn production_finalizer_rejects_post_terminal_output() {
    let stream = Box::pin(futures_util::stream::iter(vec![
        Ok(started()),
        Ok(ModelEvent::Completed {
            reason: CompletionReason::Stop,
        }),
        Ok(ModelEvent::TextDelta {
            delta: "late".to_owned(),
        }),
    ]));
    assert_eq!(
        finalize_stream(stream).await.unwrap_err(),
        StreamFinalizationError::Assembly(AssemblyError::EventAfterTerminal)
    );
}
