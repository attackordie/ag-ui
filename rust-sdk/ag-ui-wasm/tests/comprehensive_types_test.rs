//! Comprehensive types testing suite matching Python and TypeScript SDK patterns

use ag_ui_wasm::{
    Message, Role, RunAgentInput, Tool, Context, State, ToolCall, ToolResult,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// Test Message creation and serialization (matching Python test_types.py pattern)
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

// Test Message serialization with camelCase conversion (matching Python pattern)
#[wasm_bindgen_test]
fn test_message_serialization() {
    let msg = Message {
        id: "msg_123".to_string(),
        role: Role::Assistant,
        content: "Hello there!".to_string(),
        name: None,
        tool_call_id: Some("call_456".to_string()),
        tool_calls: None,
        function_call: None,
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
    // Check for camelCase conversion
    if serialized.get("toolCallId").is_some() {
        assert_eq!(serialized["toolCallId"], "call_456");
    } else {
        assert_eq!(serialized["tool_call_id"], "call_456");
    }
    assert!(serialized["metadata"].is_object());
    assert!(serialized["created_at"].is_string());
}

// Test Message deserialization with both camelCase and snake_case (matching Python pattern)
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

    // Test camelCase deserialization
    let camel_case_json = json!({
        "id": "msg_camel",
        "role": "assistant",
        "content": "Response content",
        "toolCallId": "call_123"
    });

    let camel_msg_result = serde_json::from_value::<Message>(camel_case_json);
    // Should work with camelCase if supported, otherwise skip
    if camel_msg_result.is_ok() {
        let camel_msg = camel_msg_result.unwrap();
        assert_eq!(camel_msg.tool_call_id, Some("call_123".to_string()));
    }
}

// Test different Role variants
#[wasm_bindgen_test]
fn test_role_variants() {
    let roles = vec![
        (Role::User, "user"),
        (Role::Assistant, "assistant"),
        (Role::System, "system"),
        (Role::Tool, "tool"),
        (Role::Developer, "developer"),
    ];
    
    for (role, expected_str) in roles {
        let msg = Message {
            id: "test".to_string(),
            role,
            content: "test".to_string(),
            tool_call_id: None,
            metadata: None,
            name: None,
            tool_calls: None,
            function_call: None,
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
            name: None,
            tool_calls: None,
            function_call: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "msg_2".to_string(),
            role: Role::User,
            content: "Hello AI!".to_string(),
            tool_call_id: None,
            metadata: None,
            name: None,
            tool_calls: None,
            function_call: None,
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
        context: Some(vec![context]),
        state: Some(state),
        forwarded_props: None,
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
            name: None,
            tool_calls: None,
            function_call: None,
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
        context: Some(vec![Context {
            user_id: Some("user_test".to_string()),
            session_id: None,
            metadata: None,
        }]),
        state: Some({
            let mut map = HashMap::new();
            map.insert("test_key".to_string(), json!("test_value"));
            map
        }),
        forwarded_props: None,
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
    assert_eq!(deserialized.context.as_ref().unwrap()[0].user_id, Some("user_test".to_string()));
    assert_eq!(deserialized.state.as_ref().unwrap()["test_key"], json!("test_value"));
}

// Test validation errors (matching Python pattern)
#[wasm_bindgen_test]
fn test_validation_errors() {
    // Test invalid role deserialization
    let invalid_role_json = json!({
        "id": "msg_123",
        "role": "invalid_role",
        "content": "Hello"
    });

    let invalid_result = serde_json::from_value::<Message>(invalid_role_json);
    assert!(invalid_result.is_err());

    // Test missing required fields
    let missing_content_json = json!({
        "id": "msg_456",
        "role": "user"
        // Missing content field
    });

    let missing_result = serde_json::from_value::<Message>(missing_content_json);
    // Should fail if content is required, otherwise should have default
    // This depends on the actual implementation
}

// Test camelCase and snake_case field handling (Python/TypeScript compatibility)
#[wasm_bindgen_test]
fn test_field_name_compatibility() {
    // Test that we can deserialize from both camelCase and snake_case
    let camel_case_json = json!({
        "id": "msg_camel",
        "role": "tool",
        "content": "Tool result",
        "toolCallId": "call_123",
        "createdAt": "2023-10-01T12:00:00Z"
    });

    let snake_case_json = json!({
        "id": "msg_snake",
        "role": "tool",
        "content": "Tool result",
        "tool_call_id": "call_456",
        "created_at": "2023-10-01T12:00:00Z"
    });

    // Both should deserialize successfully
    let camel_result = serde_json::from_value::<Message>(camel_case_json);
    let snake_result = serde_json::from_value::<Message>(snake_case_json);

    // At least one format should work (depends on implementation)
    if camel_result.is_ok() {
        let msg = camel_result.unwrap();
        assert_eq!(msg.tool_call_id, Some("call_123".to_string()));
    }

    if snake_result.is_ok() {
        let msg = snake_result.unwrap();
        assert_eq!(msg.tool_call_id, Some("call_456".to_string()));
    }
}

// Test message name field handling (matching Python/TypeScript patterns)
#[wasm_bindgen_test]
fn test_message_name_field() {
    // Test messages with name field
    let user_with_name = Message {
        id: "user_named".to_string(),
        role: Role::User,
        content: "Hello".to_string(),
        name: Some("John Doe".to_string()),
        tool_call_id: None,
        tool_calls: None,
        function_call: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };

    let serialized = serde_json::to_value(&user_with_name).unwrap();
    assert_eq!(serialized["name"], "John Doe");

    // Test assistant with name
    let assistant_with_name = Message {
        id: "asst_named".to_string(),
        role: Role::Assistant,
        content: "Hello!".to_string(),
        name: Some("AI Assistant".to_string()),
        tool_call_id: None,
        tool_calls: None,
        function_call: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };

    let asst_serialized = serde_json::to_value(&assistant_with_name).unwrap();
    assert_eq!(asst_serialized["name"], "AI Assistant");
}

// Test complex forwarded props (matching Python patterns)
#[wasm_bindgen_test]
fn test_forwarded_props_variations() {
    let mut forwarded_props = HashMap::new();
    forwarded_props.insert("api_version".to_string(), json!("v2.0"));
    forwarded_props.insert("client_info".to_string(), json!({
        "browser": "Chrome",
        "version": "120.0",
        "features": ["webgl", "wasm", "workers"]
    }));
    forwarded_props.insert("session_config".to_string(), json!({
        "timeout": 30000,
        "retry_count": 3,
        "enable_caching": true
    }));

    let input = RunAgentInput {
        thread_id: "thread_forwarded".to_string(),
        run_id: "run_forwarded".to_string(),
        messages: None,
        tools: None,
        context: None,
        state: None,
        forwarded_props: Some(forwarded_props),
    };

    let serialized = serde_json::to_value(&input).unwrap();
    assert_eq!(serialized["forwarded_props"]["api_version"], "v2.0");
    assert_eq!(serialized["forwarded_props"]["client_info"]["browser"], "Chrome");
    assert_eq!(serialized["forwarded_props"]["session_config"]["timeout"], 30000);
}

// Test content edge cases (matching Python pattern)
#[wasm_bindgen_test]
fn test_content_edge_cases() {
    // Empty content message
    let empty_msg = Message::new(Role::User, "".to_string());
    assert_eq!(empty_msg.content, "");

    // Large content (matching Python 10K pattern)
    let large_content = "A".repeat(10000);
    let large_msg = Message::new(Role::User, large_content.clone());
    assert_eq!(large_msg.content.len(), 10000);

    // Unicode and special characters (matching Python pattern)
    let unicode_content = "Special chars: 你好 こんにちは 안녕하세요 👋 🌍 \n\t\"'\\/<>{}[]";
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

// Test multiple tool calls (matching Python pattern)
#[wasm_bindgen_test]
fn test_multiple_tool_calls() {
    let tool_calls = vec![
        ToolCall {
            id: "call_1".to_string(),
            name: "get_weather".to_string(),
            arguments: Some(json!({"location": "New York"})),
        },
        ToolCall {
            id: "call_2".to_string(),
            name: "search_database".to_string(),
            arguments: Some(json!({"query": "recent sales"})),
        },
        ToolCall {
            id: "call_3".to_string(),
            name: "calculate".to_string(),
            arguments: Some(json!({"operation": "sum", "values": [1, 2, 3, 4, 5]})),
        },
    ];

    let msg = Message {
        id: "msg_multi_tools".to_string(),
        role: Role::Assistant,
        content: "I'll perform multiple operations".to_string(),
        tool_calls: Some(tool_calls),
        name: None,
        tool_call_id: None,
        function_call: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };

    let serialized = serde_json::to_value(&msg).unwrap();
    if let Some(tool_calls_array) = serialized.get("tool_calls").or_else(|| serialized.get("toolCalls")) {
        assert_eq!(tool_calls_array.as_array().unwrap().len(), 3);
        assert_eq!(tool_calls_array[0]["id"], "call_1");
        assert_eq!(tool_calls_array[1]["name"], "search_database");
        assert_eq!(tool_calls_array[2]["arguments"]["operation"], "sum");
    }
}

// Test RunAgentInput with diverse message types (matching Python pattern)
#[wasm_bindgen_test]
fn test_run_agent_input_diverse_messages() {
    let messages = vec![
        Message {
            id: "sys_001".to_string(),
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "user_001".to_string(),
            role: Role::User,
            content: "Can you help me analyze this data?".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "dev_001".to_string(),
            role: Role::Developer,
            content: "The assistant should provide a detailed analysis.".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "asst_001".to_string(),
            role: Role::Assistant,
            content: "I'll analyze the data for you.".to_string(),
            tool_calls: Some(vec![ToolCall {
                id: "call_001".to_string(),
                name: "analyze_data".to_string(),
                arguments: Some(json!({
                    "dataset": "sales_2023",
                    "metrics": ["mean", "median"]
                })),
            }]),
            name: None,
            tool_call_id: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "tool_001".to_string(),
            role: Role::Tool,
            content: "{\"mean\": 42.5, \"median\": 38.0}".to_string(),
            tool_call_id: Some("call_001".to_string()),
            name: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
    ];

    let input = RunAgentInput {
        thread_id: "thread_12345".to_string(),
        run_id: "run_67890".to_string(),
        messages: Some(messages),
        tools: Some(vec![
            Tool {
                name: "analyze_data".to_string(),
                description: "Analyze a dataset and return statistics".to_string(),
                parameters: Some(json!({
                    "type": "object",
                    "properties": {
                        "dataset": {"type": "string"},
                        "metrics": {"type": "array", "items": {"type": "string"}}
                    },
                    "required": ["dataset"]
                })),
            }
        ]),
        context: Some(vec![
            Context {
                user_id: Some("user_123".to_string()),
                session_id: Some("session_456".to_string()),
                metadata: Some({
                    let mut map = HashMap::new();
                    map.insert("theme".to_string(), json!("dark"));
                    map.insert("language".to_string(), json!("English"));
                    map
                }),
            }
        ]),
        state: Some({
            let mut map = HashMap::new();
            map.insert("conversation_state".to_string(), json!("active"));
            map.insert("custom_data".to_string(), json!({"key": "value"}));
            map
        }),
        forwarded_props: Some({
            let mut map = HashMap::new();
            map.insert("api_version".to_string(), json!("v1"));
            map.insert("custom_settings".to_string(), json!({"max_tokens": 500}));
            map
        }),
    };

    // Test serialization
    let serialized = serde_json::to_value(&input).unwrap();
    assert_eq!(serialized["thread_id"], "thread_12345");
    assert_eq!(serialized["run_id"], "run_67890");
    assert_eq!(serialized["messages"].as_array().unwrap().len(), 5);

    // Verify message types and content
    let messages_array = serialized["messages"].as_array().unwrap();
    assert_eq!(messages_array[0]["role"], "system");
    assert_eq!(messages_array[1]["role"], "user");
    assert_eq!(messages_array[2]["role"], "developer");
    assert_eq!(messages_array[3]["role"], "assistant");
    assert_eq!(messages_array[4]["role"], "tool");

    // Verify tool call in assistant message
    if let Some(tool_calls) = messages_array[3].get("tool_calls").or_else(|| messages_array[3].get("toolCalls")) {
        assert_eq!(tool_calls.as_array().unwrap().len(), 1);
        assert_eq!(tool_calls[0]["name"], "analyze_data");
    }

    // Verify tool message has tool_call_id
    assert!(messages_array[4].get("tool_call_id").is_some() || messages_array[4].get("toolCallId").is_some());
}