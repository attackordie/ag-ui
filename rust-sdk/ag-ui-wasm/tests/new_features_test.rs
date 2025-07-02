//! Tests for new features added for TypeScript SDK parity

use ag_ui_wasm::{
    Message, Role, RunAgentInput, FunctionCall, 
    EventType, BaseEvent, EventData,
    ThinkingStartEvent, ThinkingEndEvent, ThinkingTextMessageStartEvent,
    ThinkingTextMessageContentEvent, ThinkingTextMessageEndEvent,
    StepStartedEvent, StepFinishedEvent, RunErrorEvent,
    ToolCallArgsEvent, RawEvent, CustomEvent,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// Test Developer role
#[wasm_bindgen_test]
fn test_developer_role() {
    let msg = Message::new(Role::Developer, "Development note".to_string());
    assert_eq!(msg.role, Role::Developer);
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["role"], "developer");
}

// Test Message with name field
#[wasm_bindgen_test]
fn test_message_with_name() {
    let msg = Message {
        id: "msg_123".to_string(),
        role: Role::Assistant,
        content: "Hello!".to_string(),
        name: Some("Claude".to_string()),
        tool_call_id: None,
        tool_calls: None,
        function_call: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["name"], "Claude");
}

// Test Message with tool_calls
#[wasm_bindgen_test]
fn test_message_with_tool_calls() {
    use ag_ui_wasm::ToolCall;
    
    let tool_calls = vec![
        ToolCall {
            id: "call_1".to_string(),
            name: "search".to_string(),
            arguments: Some(json!({"query": "rust programming"})),
        },
        ToolCall {
            id: "call_2".to_string(),
            name: "calculate".to_string(),
            arguments: Some(json!({"expression": "2 + 2"})),
        },
    ];
    
    let msg = Message {
        id: "msg_123".to_string(),
        role: Role::Assistant,
        content: "I'll search for that and calculate the result.".to_string(),
        name: None,
        tool_call_id: None,
        tool_calls: Some(tool_calls),
        function_call: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    let calls = serialized["tool_calls"].as_array().unwrap();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0]["name"], "search");
    assert_eq!(calls[1]["name"], "calculate");
}

// Test Message with function_call (legacy)
#[wasm_bindgen_test]
fn test_message_with_function_call() {
    let function_call = FunctionCall {
        name: "get_weather".to_string(),
        arguments: Some("{\"location\": \"San Francisco\"}".to_string()),
    };
    
    let msg = Message {
        id: "msg_123".to_string(),
        role: Role::Assistant,
        content: "Let me check the weather for you.".to_string(),
        name: None,
        tool_call_id: None,
        tool_calls: None,
        function_call: Some(function_call),
        metadata: None,
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["function_call"]["name"], "get_weather");
    assert_eq!(serialized["function_call"]["arguments"], "{\"location\": \"San Francisco\"}");
}

// Test RunAgentInput with context array
#[wasm_bindgen_test]
fn test_run_agent_input_context_array() {
    use ag_ui_wasm::Context;
    
    let contexts = vec![
        Context {
            user_id: Some("user_1".to_string()),
            session_id: Some("session_1".to_string()),
            metadata: None,
        },
        Context {
            user_id: Some("user_2".to_string()),
            session_id: Some("session_2".to_string()),
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("role".to_string(), json!("admin"));
                map
            }),
        },
    ];
    
    let input = RunAgentInput {
        thread_id: "thread_123".to_string(),
        run_id: "run_123".to_string(),
        messages: None,
        tools: None,
        context: Some(contexts),
        state: None,
        forwarded_props: None,
    };
    
    let serialized = serde_json::to_value(&input).unwrap();
    let ctx_array = serialized["context"].as_array().unwrap();
    assert_eq!(ctx_array.len(), 2);
    assert_eq!(ctx_array[0]["user_id"], "user_1");
    assert_eq!(ctx_array[1]["user_id"], "user_2");
    assert_eq!(ctx_array[1]["metadata"]["role"], "admin");
}

// Test RunAgentInput with forwarded_props
#[wasm_bindgen_test]
fn test_run_agent_input_forwarded_props() {
    let mut forwarded_props = HashMap::new();
    forwarded_props.insert("custom_header".to_string(), json!("x-custom-value"));
    forwarded_props.insert("feature_flags".to_string(), json!({
        "new_ui": true,
        "beta_features": false
    }));
    
    let input = RunAgentInput {
        thread_id: "thread_123".to_string(),
        run_id: "run_123".to_string(),
        messages: None,
        tools: None,
        context: None,
        state: None,
        forwarded_props: Some(forwarded_props),
    };
    
    let serialized = serde_json::to_value(&input).unwrap();
    assert_eq!(serialized["forwarded_props"]["custom_header"], "x-custom-value");
    assert_eq!(serialized["forwarded_props"]["feature_flags"]["new_ui"], true);
}

// Test thinking events
#[wasm_bindgen_test]
fn test_thinking_events() {
    // Thinking start
    let thinking_start = BaseEvent {
        event_type: EventType::ThinkingStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ThinkingStart(ThinkingStartEvent {
            thread_id: "thread_123".to_string(),
            run_id: "run_123".to_string(),
        }),
    };
    
    let serialized = serde_json::to_value(&thinking_start).unwrap();
    assert_eq!(serialized["type"], "THINKING_START");
    assert_eq!(serialized["thread_id"], "thread_123");
    
    // Thinking text message
    let thinking_msg = BaseEvent {
        event_type: EventType::ThinkingTextMessageStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ThinkingTextMessageStart(ThinkingTextMessageStartEvent {
            message_id: "thinking_msg_123".to_string(),
            role: Some(Role::Assistant),
        }),
    };
    
    let serialized = serde_json::to_value(&thinking_msg).unwrap();
    assert_eq!(serialized["type"], "THINKING_TEXT_MESSAGE_START");
    assert_eq!(serialized["message_id"], "thinking_msg_123");
}

// Test step events
#[wasm_bindgen_test]
fn test_step_events() {
    let step_started = BaseEvent {
        event_type: EventType::StepStarted,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StepStarted(StepStartedEvent {
            thread_id: "thread_123".to_string(),
            run_id: "run_123".to_string(),
            step_id: "step_456".to_string(),
            step_type: Some("tool_call".to_string()),
        }),
    };
    
    let serialized = serde_json::to_value(&step_started).unwrap();
    assert_eq!(serialized["type"], "STEP_STARTED");
    assert_eq!(serialized["step_id"], "step_456");
    assert_eq!(serialized["step_type"], "tool_call");
}

// Test run error event
#[wasm_bindgen_test]
fn test_run_error_event() {
    let run_error = BaseEvent {
        event_type: EventType::RunError,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::RunError(RunErrorEvent {
            thread_id: "thread_123".to_string(),
            run_id: "run_123".to_string(),
            error: "API rate limit exceeded".to_string(),
            code: Some("rate_limit_error".to_string()),
        }),
    };
    
    let serialized = serde_json::to_value(&run_error).unwrap();
    assert_eq!(serialized["type"], "RUN_ERROR");
    assert_eq!(serialized["error"], "API rate limit exceeded");
    assert_eq!(serialized["code"], "rate_limit_error");
}

// Test tool call args event
#[wasm_bindgen_test]
fn test_tool_call_args_event() {
    let tool_args = BaseEvent {
        event_type: EventType::ToolCallArgs,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallArgs(ToolCallArgsEvent {
            tool_call_id: "call_123".to_string(),
            delta: "{\"query\": \"rust".to_string(),
        }),
    };
    
    let serialized = serde_json::to_value(&tool_args).unwrap();
    assert_eq!(serialized["type"], "TOOL_CALL_ARGS");
    assert_eq!(serialized["tool_call_id"], "call_123");
}

// Test raw event
#[wasm_bindgen_test]
fn test_raw_event() {
    let raw = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent {
            event: json!({
                "custom_type": "special_event",
                "custom_data": {
                    "value": 42,
                    "message": "Custom event data"
                }
            }),
        }),
    };
    
    let serialized = serde_json::to_value(&raw).unwrap();
    assert_eq!(serialized["type"], "RAW");
    assert_eq!(serialized["event"]["custom_type"], "special_event");
    assert_eq!(serialized["event"]["custom_data"]["value"], 42);
}

// Test custom event
#[wasm_bindgen_test]
fn test_custom_event() {
    let custom = BaseEvent {
        event_type: EventType::Custom,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Custom(CustomEvent {
            event_type: "user_feedback".to_string(),
            data: json!({
                "rating": 5,
                "comment": "Great response!"
            }),
        }),
    };
    
    let serialized = serde_json::to_value(&custom).unwrap();
    assert_eq!(serialized["type"], "CUSTOM");
    assert_eq!(serialized["event_type"], "user_feedback");
    assert_eq!(serialized["rating"], 5);
}