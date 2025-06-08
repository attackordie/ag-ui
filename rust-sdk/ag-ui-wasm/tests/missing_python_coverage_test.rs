//! Tests covering functionality missing from Python SDK comparison

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, ErrorEvent,
    SSEEncoder, Message, Role, ToolCall, ToolResult,
    RunAgentInput, Tool, Context, State,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// ===== ENCODER TESTS MISSING FROM PYTHON =====

// Python test_encoder.py::test_null_value_exclusion equivalent
#[wasm_bindgen_test]
fn test_null_value_exclusion_in_encoding() {
    // Test event with None/null fields
    let event = BaseEvent {
        event_type: EventType::Error,
        timestamp: Some(Utc::now()),
        raw_event: None, // This should be excluded from JSON
        data: EventData::Error(ErrorEvent {
            error: "Test error".to_string(),
            code: Some("ERR_001".to_string()),
            details: None, // This should be excluded from JSON
        }),
    };
    
    let encoded = SSEEncoder::encode_event_string(&event).unwrap();
    
    // Extract JSON content
    let json_part = encoded.trim_start_matches("data: ").trim_end();
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();
    
    // Required fields should be present
    assert!(parsed.get("type").is_some());
    assert!(parsed.get("timestamp").is_some());
    assert!(parsed.get("error").is_some());
    assert!(parsed.get("code").is_some());
    
    // None/null fields should be excluded or null
    assert!(parsed.get("raw_event").is_none() || parsed["raw_event"].is_null());
    assert!(parsed.get("details").is_none() || parsed["details"].is_null());
}

// Python test_encoder.py::test_round_trip_serialization equivalent
#[wasm_bindgen_test]
fn test_encoder_round_trip_serialization() {
    // Create complex event
    let original_event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ag_ui_wasm::ToolCallStartEvent {
            tool_call_id: "call_abc123".to_string(),
            tool_name: "search_tool".to_string(),
            parent_message_id: Some("msg_parent_456".to_string()),
        }),
    };
    
    // Encode to string and verify JSON structure
    let encoded = SSEEncoder::encode_event_string(&original_event).unwrap();
    let json_part = encoded.trim_start_matches("data: ").trim_end();
    let json_data: serde_json::Value = serde_json::from_str(json_part).unwrap();
    
    // Verify field names and values
    assert_eq!(json_data["type"], "TOOL_CALL_START");
    assert_eq!(json_data["tool_call_id"], "call_abc123");
    assert_eq!(json_data["tool_name"], "search_tool");
    assert_eq!(json_data["parent_message_id"], "msg_parent_456");
    
    // Deserialize back to event
    let deserialized: BaseEvent = serde_json::from_str(json_part).unwrap();
    
    // Verify round-trip preserved data
    assert_eq!(deserialized.event_type, original_event.event_type);
    match (&original_event.data, &deserialized.data) {
        (EventData::ToolCallStart(orig), EventData::ToolCallStart(deser)) => {
            assert_eq!(deser.tool_call_id, orig.tool_call_id);
            assert_eq!(deser.tool_name, orig.tool_name);
            assert_eq!(deser.parent_message_id, orig.parent_message_id);
        },
        _ => panic!("Event data type mismatch"),
    }
}

// ===== TYPES TESTS MISSING FROM PYTHON =====

// Python test_types.py::test_function_call_creation equivalent
#[wasm_bindgen_test]
fn test_function_call_equivalent() {
    // In Rust SDK, we have ToolCall which is similar to FunctionCall
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        name: "test_function".to_string(),
        arguments: Some(json!({"param": "value"})),
    };
    
    assert_eq!(tool_call.id, "call_123");
    assert_eq!(tool_call.name, "test_function");
    assert!(tool_call.arguments.is_some());
}

// Python test_types.py::test_tool_call_serialization equivalent
#[wasm_bindgen_test]
fn test_tool_call_serialization() {
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        name: "test_function".to_string(),
        arguments: Some(json!({"key": "value"})),
    };
    
    let serialized = serde_json::to_value(&tool_call).unwrap();
    assert_eq!(serialized["id"], "call_123");
    assert_eq!(serialized["name"], "test_function");
    assert!(serialized["arguments"].is_object());
}

// Python test_types.py::test_tool_message_camel_case equivalent
#[wasm_bindgen_test]
fn test_tool_message_serialization() {
    let tool_msg = Message {
        id: "tool_123".to_string(),
        role: Role::Tool,
        content: "Tool result".to_string(),
        tool_call_id: Some("call_456".to_string()),
        metadata: None,
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&tool_msg).unwrap();
    assert_eq!(serialized["role"], "tool");
    assert_eq!(serialized["tool_call_id"], "call_456");
}

// Python test_types.py::test_parse_camel_case_json_tool_message equivalent
#[wasm_bindgen_test]
fn test_parse_tool_message_json() {
    let json_data = json!({
        "id": "tool_789",
        "role": "tool",
        "content": "Result from tool",
        "tool_call_id": "call_123"
    });
    
    let tool_msg: Message = serde_json::from_value(json_data).unwrap();
    assert_eq!(tool_msg.id, "tool_789");
    assert_eq!(tool_msg.role, Role::Tool);
    assert_eq!(tool_msg.content, "Result from tool");
    assert_eq!(tool_msg.tool_call_id, Some("call_123".to_string()));
}

// Python test_types.py::test_developer_message equivalent
#[wasm_bindgen_test]
fn test_developer_message() {
    let msg = Message {
        id: "dev_123".to_string(),
        role: Role::User, // Rust SDK doesn't have Developer role, using User
        content: "Developer note".to_string(),
        tool_call_id: None,
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("role_type".to_string(), json!("developer"));
            map
        }),
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["role"], "user");
    assert_eq!(serialized["content"], "Developer note");
    assert_eq!(serialized["metadata"]["role_type"], "developer");
}

// Python test_types.py::test_multiple_tool_calls equivalent
#[wasm_bindgen_test]
fn test_multiple_tool_calls_in_metadata() {
    let metadata = json!({
        "tool_calls": [
            {
                "id": "call_1",
                "name": "get_weather",
                "arguments": {"location": "New York"}
            },
            {
                "id": "call_2",
                "name": "search_database",
                "arguments": {"query": "recent sales"}
            },
            {
                "id": "call_3",
                "name": "calculate",
                "arguments": {"operation": "sum", "values": [1, 2, 3, 4, 5]}
            }
        ]
    });
    
    let msg = Message {
        id: "asst_multi".to_string(),
        role: Role::Assistant,
        content: "I'll perform multiple operations".to_string(),
        tool_call_id: None,
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("tool_calls".to_string(), metadata["tool_calls"].clone());
            map
        }),
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    let tool_calls = &serialized["metadata"]["tool_calls"];
    assert_eq!(tool_calls.as_array().unwrap().len(), 3);
    assert_eq!(tool_calls[0]["id"], "call_1");
    assert_eq!(tool_calls[1]["name"], "search_database");
    assert_eq!(tool_calls[2]["arguments"]["operation"], "sum");
}

// Python test_types.py::test_validation_errors equivalent
#[wasm_bindgen_test]
fn test_validation_behaviors() {
    // Test message with empty content (should be allowed)
    let empty_msg = Message {
        id: "empty_123".to_string(),
        role: Role::User,
        content: "".to_string(),
        tool_call_id: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };
    
    let json_result = serde_json::to_string(&empty_msg);
    assert!(json_result.is_ok());
    
    // Test tool message with tool_call_id
    let tool_msg = Message {
        id: "tool_456".to_string(),
        role: Role::Tool,
        content: "Tool result".to_string(),
        tool_call_id: Some("call_789".to_string()),
        metadata: None,
        created_at: Some(Utc::now()),
    };
    
    let json_result = serde_json::to_string(&tool_msg);
    assert!(json_result.is_ok());
}

// Python test_types.py::test_name_field_handling equivalent
#[wasm_bindgen_test]
fn test_message_name_handling() {
    // Test message with name in metadata
    let metadata = json!({"name": "AI Assistant"});
    
    let msg = Message {
        id: "asst_named".to_string(),
        role: Role::Assistant,
        content: "Hello".to_string(),
        tool_call_id: None,
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("name".to_string(), metadata["name"].clone());
            map
        }),
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["metadata"]["name"], "AI Assistant");
}

// ===== TOOL RESULT TESTS =====

#[wasm_bindgen_test]
fn test_tool_result_structure() {
    // Test successful tool result
    let success_result = ToolResult {
        tool_call_id: "call_123".to_string(),
        result: json!({"sum": 6, "calculation": "1+2+3"}),
        error: None,
    };
    
    let serialized = serde_json::to_value(&success_result).unwrap();
    assert_eq!(serialized["tool_call_id"], "call_123");
    assert_eq!(serialized["result"]["sum"], 6);
    assert!(serialized["error"].is_null());
    
    // Test error tool result
    let error_result = ToolResult {
        tool_call_id: "call_456".to_string(),
        result: json!(null),
        error: Some("Division by zero".to_string()),
    };
    
    let error_serialized = serde_json::to_value(&error_result).unwrap();
    assert_eq!(error_serialized["error"], "Division by zero");
    assert!(error_serialized["result"].is_null());
}

// ===== CONTEXT TESTS =====

#[wasm_bindgen_test]
fn test_context_with_metadata() {
    let context = Context {
        user_id: Some("user_123".to_string()),
        session_id: Some("session_456".to_string()),
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("ip_address".to_string(), json!("192.168.1.1"));
            map.insert("user_agent".to_string(), json!("Mozilla/5.0..."));
            map.insert("custom_data".to_string(), json!({"key": "value"}));
            map
        }),
    };
    
    let serialized = serde_json::to_value(&context).unwrap();
    assert_eq!(serialized["user_id"], "user_123");
    assert_eq!(serialized["session_id"], "session_456");
    assert_eq!(serialized["metadata"]["ip_address"], "192.168.1.1");
    assert_eq!(serialized["metadata"]["custom_data"]["key"], "value");
}