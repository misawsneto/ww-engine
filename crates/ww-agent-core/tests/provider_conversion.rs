use serde_json::json;
use ww_agent_core::{
    AgentAssistantContent, AgentCompletionReason, DurableAssistantMessage,
    DurableMessageConversionError,
};
use ww_agent_provider::{
    AssistantMessage, CompletionReason, MessageContent, ModelUsage, ToolCall, ToolCallId,
};

#[test]
fn provider_message_converts_to_agent_owned_durable_types() {
    let durable = DurableAssistantMessage::try_from(AssistantMessage {
        content: vec![MessageContent::ToolCall {
            call: ToolCall {
                id: ToolCallId::new("provider-call-1").expect("call id"),
                name: "test.echo".to_owned(),
                arguments: json!({"message": "hello"}),
            },
        }],
        stop_reason: CompletionReason::ToolUse,
        usage: Some(ModelUsage {
            input_tokens: 5,
            output_tokens: 3,
            cache_read_input_tokens: 2,
            cache_write_input_tokens: 1,
        }),
        provider_request_id: Some("request-1".to_owned()),
    })
    .expect("convert message");

    assert_eq!(durable.stop_reason, AgentCompletionReason::ToolUse);
    assert_eq!(durable.usage.expect("usage").total_tokens(), 8);
    let AgentAssistantContent::ToolCall { call } = &durable.content[0] else {
        panic!("expected tool call")
    };
    assert_eq!(call.provider_call_id, "provider-call-1");
    assert_eq!(call.arguments, json!({"message": "hello"}));
}

#[test]
fn provider_tool_result_cannot_be_persisted_as_assistant_content() {
    let error = DurableAssistantMessage::try_from(AssistantMessage {
        content: vec![MessageContent::ToolResult {
            call_id: ToolCallId::new("provider-call-1").expect("call id"),
            tool_name: "test.echo".to_owned(),
            content: json!({"message": "hello"}),
            is_error: false,
        }],
        stop_reason: CompletionReason::Stop,
        usage: None,
        provider_request_id: None,
    })
    .expect_err("tool result must not cross assistant durability boundary");
    assert_eq!(
        error,
        DurableMessageConversionError::ToolResultInAssistantMessage
    );
}
