//! Comprehensive types testing suite that matches the actual Rust SDK implementation

use ag_ui_wasm::{
    Message, Role, RunAgentInput, Tool, Context, State, ToolCall, ToolResult,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// Test Message creation and serialization
#[wasm_bindgen_test]
fn test_message_creation() {
    let msg = Message::new(Role::User, "Hello, world!".to_string());
    
    assert_eq!(msg.role, Role::User);
    assert_eq!(msg.content, "Hello, world!");
    assert!(!msg.id.is_empty());
    assert!(msg.tool_call_id.is_none());
    assert!(msg.metadata.is_none());
    assert!(msg.created_at.is_some());
}

// Test Message serialization
#[wasm_bindgen_test]
fn test_message_serialization() {
    let msg = Message {
        id: "msg_123".to_string(),
        role: Role::Assistant,
        content: "Hello there!".to_string(),
        tool_call_id: Some("call_456".to_string()),
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("key".to_string(), json!("value"));
            map
        }),
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["id"], "msg_123");
    assert_eq!(serialized["role"], "assistant");
    assert_eq!(serialized["content"], "Hello there!");
    assert_eq!(serialized["tool_call_id"], "call_456");
    assert!(serialized["metadata"].is_object());
    assert!(serialized["created_at"].is_string());
}

// Test Message deserialization
#[wasm_bindgen_test]
fn test_message_deserialization() {
    let json_data = json!({
        "id": "msg_789",
        "role": "user",
        "content": "How are you?",
        "tool_call_id": null,
        "metadata": null,
        "created_at": "2023-10-01T12:00:00Z"
    });
    
    let msg: Message = serde_json::from_value(json_data).unwrap();
    assert_eq!(msg.id, "msg_789");
    assert_eq!(msg.role, Role::User);
    assert_eq!(msg.content, "How are you?");
    assert!(msg.tool_call_id.is_none());
    assert!(msg.metadata.is_none());
}

// Test different Role variants
#[wasm_bindgen_test]
fn test_role_variants() {
    let roles = vec![
        (Role::User, "user"),
        (Role::Assistant, "assistant"),
        (Role::System, "system"),
        (Role::Tool, "tool"),
    ];
    
    for (role, expected_str) in roles {
        let msg = Message {
            id: "test".to_string(),
            role,
            content: "test".to_string(),
            tool_call_id: None,
            metadata: None,
            created_at: None,
        };
        
        let serialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(serialized["role"], expected_str);
    }
}

// Test Tool structure
#[wasm_bindgen_test]
fn test_tool_structure() {
    let tool = Tool {
        name: "search_database".to_string(),
        description: "Search the database for information".to_string(),
        parameters: Some(json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                },
                "limit": {
                    "type": "integer", 
                    "default": 10
                }
            },
            "required": ["query"]
        })),
    };
    
    let serialized = serde_json::to_value(&tool).unwrap();
    assert_eq!(serialized["name"], "search_database");
    assert_eq!(serialized["description"], "Search the database for information");
    assert!(serialized["parameters"]["properties"]["query"].is_object());
    assert_eq!(serialized["parameters"]["properties"]["limit"]["default"], 10);
}

// Test Context structure
#[wasm_bindgen_test]
fn test_context_structure() {
    let context = Context {
        user_id: Some("user_123".to_string()),
        session_id: Some("session_456".to_string()),
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("ip_address".to_string(), json!("192.168.1.1"));
            map.insert("user_agent".to_string(), json!("Mozilla/5.0..."));
            map
        }),
    };
    
    let serialized = serde_json::to_value(&context).unwrap();
    assert_eq!(serialized["user_id"], "user_123");
    assert_eq!(serialized["session_id"], "session_456");
    assert_eq!(serialized["metadata"]["ip_address"], "192.168.1.1");
}

// Test State (HashMap) handling
#[wasm_bindgen_test]
fn test_state_handling() {
    let mut state: State = HashMap::new();
    state.insert("conversation_phase".to_string(), json!("greeting"));
    state.insert("user_preferences".to_string(), json!({
        "theme": "dark",
        "language": "en",
        "notifications": true
    }));
    state.insert("session_data".to_string(), json!({
        "start_time": 1234567890,
        "message_count": 5,
        "active_tools": ["search", "calculator"]
    }));
    
    let serialized = serde_json::to_value(&state).unwrap();
    assert_eq!(serialized["conversation_phase"], "greeting");
    assert_eq!(serialized["user_preferences"]["theme"], "dark");
    assert_eq!(serialized["session_data"]["message_count"], 5);
    
    // Test deserialization
    let deserialized: State = serde_json::from_value(serialized).unwrap();
    assert_eq!(deserialized["conversation_phase"], json!("greeting"));
    assert_eq!(deserialized["user_preferences"]["language"], json!("en"));
}

// Test RunAgentInput creation
#[wasm_bindgen_test]
fn test_run_agent_input_creation() {
    let input = RunAgentInput::new("thread_123".to_string(), "run_456".to_string());
    
    assert_eq!(input.thread_id, "thread_123");
    assert_eq!(input.run_id, "run_456");
    assert!(input.messages.is_none());
    assert!(input.tools.is_none());
    assert!(input.context.is_none());
    assert!(input.state.is_none());
}

// Test comprehensive RunAgentInput
#[wasm_bindgen_test]
fn test_comprehensive_run_agent_input() {
    let messages = vec![
        Message {
            id: "msg_1".to_string(),
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
            tool_call_id: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "msg_2".to_string(),
            role: Role::User,
            content: "Hello AI!".to_string(),
            tool_call_id: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
    ];
    
    let tools = vec![
        Tool {
            name: "search".to_string(),
            description: "Search the web".to_string(),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                }
            })),
        }
    ];
    
    let context = Context {
        user_id: Some("user_123".to_string()),
        session_id: Some("session_456".to_string()),
        metadata: None,
    };
    
    let mut state: State = HashMap::new();
    state.insert("status".to_string(), json!("active"));
    
    let input = RunAgentInput {
        thread_id: "thread_complex".to_string(),
        run_id: "run_complex".to_string(),
        messages: Some(messages),
        tools: Some(tools),
        context: Some(context),
        state: Some(state),
    };
    
    // Test serialization
    let serialized = serde_json::to_value(&input).unwrap();
    assert_eq!(serialized["thread_id"], "thread_complex");
    assert_eq!(serialized["run_id"], "run_complex");
    assert_eq!(serialized["messages"].as_array().unwrap().len(), 2);
    assert_eq!(serialized["tools"].as_array().unwrap().len(), 1);
    assert_eq!(serialized["context"]["user_id"], "user_123");
    assert_eq!(serialized["state"]["status"], "active");
    
    // Test deserialization
    let deserialized: RunAgentInput = serde_json::from_value(serialized).unwrap();
    assert_eq!(deserialized.thread_id, "thread_complex");
    assert_eq!(deserialized.messages.unwrap().len(), 2);
    assert_eq!(deserialized.tools.unwrap()[0].name, "search");
}

// Test ToolCall structure
#[wasm_bindgen_test]
fn test_tool_call_structure() {
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        name: "calculate".to_string(),
        arguments: Some(json!({
            "operation": "add",
            "operands": [1, 2, 3]
        })),
    };
    
    let serialized = serde_json::to_value(&tool_call).unwrap();
    assert_eq!(serialized["id"], "call_123");
    assert_eq!(serialized["name"], "calculate");
    assert_eq!(serialized["arguments"]["operation"], "add");
    assert_eq!(serialized["arguments"]["operands"][1], 2);
}

// Test ToolResult structure
#[wasm_bindgen_test]
fn test_tool_result_structure() {
    let tool_result = ToolResult {
        tool_call_id: "call_123".to_string(),
        result: json!({"sum": 6, "calculation": "1+2+3"}),
        error: None,
    };
    
    let serialized = serde_json::to_value(&tool_result).unwrap();
    assert_eq!(serialized["tool_call_id"], "call_123");
    assert_eq!(serialized["result"]["sum"], 6);
    assert!(serialized["error"].is_null());
    
    // Test with error
    let error_result = ToolResult {
        tool_call_id: "call_456".to_string(),
        result: json!(null),
        error: Some("Division by zero".to_string()),
    };
    
    let error_serialized = serde_json::to_value(&error_result).unwrap();
    assert_eq!(error_serialized["error"], "Division by zero");
}

// Test Message with complex metadata
#[wasm_bindgen_test]
fn test_message_with_complex_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("tool_calls".to_string(), json!([
        {
            "id": "call_1",
            "name": "search",
            "arguments": {"query": "weather"}
        },
        {
            "id": "call_2", 
            "name": "calculate",
            "arguments": {"expression": "2+2"}
        }
    ]));
    metadata.insert("confidence".to_string(), json!(0.95));
    metadata.insert("processing_time".to_string(), json!(1.5));
    
    let msg = Message {
        id: "msg_complex".to_string(),
        role: Role::Assistant,
        content: "I'll help you with that calculation and weather check.".to_string(),
        tool_call_id: None,
        metadata: Some(metadata),
        created_at: Some(Utc::now()),
    };
    
    let serialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(serialized["metadata"]["tool_calls"].as_array().unwrap().len(), 2);
    assert_eq!(serialized["metadata"]["confidence"], 0.95);
    assert_eq!(serialized["metadata"]["processing_time"], 1.5);
}

// Test round-trip serialization
#[wasm_bindgen_test]
fn test_round_trip_serialization() {
    let original = RunAgentInput {
        thread_id: "thread_round_trip".to_string(),
        run_id: "run_round_trip".to_string(),
        messages: Some(vec![
            Message::new(Role::User, "Test message".to_string())
        ]),
        tools: Some(vec![
            Tool {
                name: "test_tool".to_string(),
                description: "A test tool".to_string(),
                parameters: Some(json!({"type": "object"})),
            }
        ]),
        context: Some(Context {
            user_id: Some("user_test".to_string()),
            session_id: None,
            metadata: None,
        }),
        state: Some({
            let mut map = HashMap::new();
            map.insert("test_key".to_string(), json!("test_value"));
            map
        }),
    };
    
    // Serialize
    let json_str = serde_json::to_string(&original).unwrap();
    
    // Deserialize
    let deserialized: RunAgentInput = serde_json::from_str(&json_str).unwrap();
    
    // Verify
    assert_eq!(deserialized.thread_id, original.thread_id);
    assert_eq!(deserialized.run_id, original.run_id);
    assert_eq!(deserialized.messages.as_ref().unwrap().len(), 1);
    assert_eq!(deserialized.tools.as_ref().unwrap().len(), 1);
    assert_eq!(deserialized.tools.as_ref().unwrap()[0].name, "test_tool");
    assert_eq!(deserialized.context.as_ref().unwrap().user_id, Some("user_test".to_string()));
    assert_eq!(deserialized.state.as_ref().unwrap()["test_key"], json!("test_value"));
}

// Test edge cases
#[wasm_bindgen_test]
fn test_edge_cases() {
    // Empty content message
    let empty_msg = Message::new(Role::User, "".to_string());
    assert_eq!(empty_msg.content, "");
    
    // Large content
    let large_content = "A".repeat(5000);
    let large_msg = Message::new(Role::User, large_content.clone());
    assert_eq!(large_msg.content.len(), 5000);
    
    // Unicode content
    let unicode_content = "Hello 你好 こんにちは 안녕하세요 👋 🌍";
    let unicode_msg = Message::new(Role::User, unicode_content.to_string());
    assert_eq!(unicode_msg.content, unicode_content);
    
    // Tool with no parameters
    let simple_tool = Tool {
        name: "ping".to_string(),
        description: "Simple ping tool".to_string(),
        parameters: None,
    };
    let tool_json = serde_json::to_value(&simple_tool).unwrap();
    assert!(tool_json["parameters"].is_null());
}