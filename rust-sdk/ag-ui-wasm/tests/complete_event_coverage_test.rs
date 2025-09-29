//! Complete event coverage tests matching TypeScript SDK event test patterns

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, Role,
    TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
    ToolCallStartEvent, ToolCallChunkEvent, ToolCallEndEvent,
    StateSnapshotEvent, StateDeltaEvent, MessagesSnapshotEvent,
    RunStartedEvent, RunFinishedEvent, ErrorEvent,
    Message, ToolCall,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// ===== MESSAGE EVENTS COMPREHENSIVE TESTS =====

#[wasm_bindgen_test]
fn test_text_message_start_event_comprehensive() {
    // Test with all fields
    let event = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({"source": "test", "version": "1.0"})),
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg-comprehensive".to_string(),
            role: Some(Role::Assistant),
        }),
    };

    assert_eq!(event.event_type, EventType::TextMessageStart);

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "TEXT_MESSAGE_START");
    assert_eq!(parsed["message_id"], "msg-comprehensive");
    assert_eq!(parsed["role"], "assistant");
    assert!(parsed["timestamp"].is_number());
    assert_eq!(parsed["raw_event"]["source"], "test");

    // Test deserialization
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized.event_type, EventType::TextMessageStart);

    if let EventData::TextMessageStart(data) = &deserialized.data {
        assert_eq!(data.message_id, "msg-comprehensive");
        assert_eq!(data.role, Some(Role::Assistant));
    }
}

#[wasm_bindgen_test]
fn test_text_message_content_event_comprehensive() {
    // Test with special characters and different content types
    let test_cases = vec![
        ("plain text", "Hello, world!"),
        ("unicode", "Hello 你好 こんにちは 안녕하세요 👋 🌍"),
        ("json_string", r#"{"key": "value", "number": 42, "array": [1, 2, 3]}"#),
        ("code_snippet", "function test() {\n  return \"Hello\";\n}"),
        ("markdown", "# Header\n\n**Bold** and *italic* text\n\n- List item 1\n- List item 2"),
        ("empty", ""),
    ];

    for (test_name, content) in test_cases {
        let event = BaseEvent {
            event_type: EventType::TextMessageContent,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent {
                message_id: format!("msg-{}", test_name),
                delta: content.to_string(),
            }),
        };

        // Test serialization and deserialization
        let json_str = serde_json::to_string(&event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

        if let EventData::TextMessageContent(data) = &deserialized.data {
            assert_eq!(data.delta, content);
            assert_eq!(data.message_id, format!("msg-{}", test_name));
        }
    }
}

#[wasm_bindgen_test]
fn test_text_message_end_event_comprehensive() {
    let event = BaseEvent {
        event_type: EventType::TextMessageEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::TextMessageEnd(TextMessageEndEvent {
            message_id: "msg-end-test".to_string(),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "TEXT_MESSAGE_END");
    assert_eq!(parsed["message_id"], "msg-end-test");
}

// ===== TOOL CALL EVENTS COMPREHENSIVE TESTS =====

#[wasm_bindgen_test]
fn test_tool_call_start_event_comprehensive() {
    let event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({
            "original": "event data",
            "from": "source system",
            "trace_id": "trace_123"
        })),
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "call-comprehensive-123".to_string(),
            tool_name: "advanced_data_processor".to_string(),
            parent_message_id: Some("parent-msg-456".to_string()),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "TOOL_CALL_START");
    assert_eq!(parsed["tool_call_id"], "call-comprehensive-123");
    assert_eq!(parsed["tool_name"], "advanced_data_processor");
    assert_eq!(parsed["parent_message_id"], "parent-msg-456");
    assert_eq!(parsed["raw_event"]["trace_id"], "trace_123");

    // Test deserialization
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    if let EventData::ToolCallStart(data) = &deserialized.data {
        assert_eq!(data.tool_call_id, "call-comprehensive-123");
        assert_eq!(data.tool_name, "advanced_data_processor");
        assert_eq!(data.parent_message_id, Some("parent-msg-456".to_string()));
    }
}

#[wasm_bindgen_test]
fn test_tool_call_chunk_event_comprehensive() {
    // Test different types of tool call argument chunks
    let test_cases = vec![
        ("json_start", r#"{"query": "SELECT * FROM"#),
        ("json_middle", r#" users WHERE age > "#),
        ("json_end", r#"18 AND status = 'active'}"#),
        ("partial_json", r#"{"location":"San Fra"#),
        ("complex_args", r#"{"filters": {"age": {"min": 18, "max": 65}, "location": {"country": "US", "states": ["CA", "NY"]}}}"#),
        ("empty_chunk", ""),
    ];

    for (test_name, chunk_content) in test_cases {
        let event = BaseEvent {
            event_type: EventType::ToolCallChunk,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallChunk(ToolCallChunkEvent {
                tool_call_id: format!("call-{}", test_name),
                delta: chunk_content.to_string(),
            }),
        };

        let json_str = serde_json::to_string(&event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

        if let EventData::ToolCallChunk(data) = &deserialized.data {
            assert_eq!(data.delta, chunk_content);
            assert_eq!(data.tool_call_id, format!("call-{}", test_name));
        }
    }
}

#[wasm_bindgen_test]
fn test_tool_call_end_event_comprehensive() {
    // Test with complete tool call
    let tool_call = ToolCall {
        id: "call-end-complete".to_string(),
        name: "calculate_statistics".to_string(),
        arguments: Some(json!({
            "dataset": "sales_2023",
            "metrics": ["mean", "median", "std_dev"],
            "filters": {
                "region": "North America",
                "quarter": "Q4"
            }
        })),
    };

    let event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: "call-end-complete".to_string(),
            tool_call: Some(tool_call),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "TOOL_CALL_END");
    assert_eq!(parsed["tool_call_id"], "call-end-complete");
    assert!(parsed["tool_call"].is_object());
    assert_eq!(parsed["tool_call"]["id"], "call-end-complete");
    assert_eq!(parsed["tool_call"]["name"], "calculate_statistics");
    assert_eq!(parsed["tool_call"]["arguments"]["dataset"], "sales_2023");

    // Test without tool call (minimal)
    let minimal_event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: None,
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: "call-minimal".to_string(),
            tool_call: None,
        }),
    };

    let minimal_json = serde_json::to_string(&minimal_event).unwrap();
    let minimal_parsed: serde_json::Value = serde_json::from_str(&minimal_json).unwrap();

    assert_eq!(minimal_parsed["tool_call_id"], "call-minimal");
    assert!(minimal_parsed["tool_call"].is_null() || !minimal_parsed.as_object().unwrap().contains_key("tool_call"));
}

// ===== TOOL CALL EVENT SEQUENCE TESTS =====

#[wasm_bindgen_test]
fn test_complete_tool_call_sequence() {
    // Test a complete tool call sequence matching TypeScript patterns
    let tool_call_id = "sequence-test-123";

    let start_event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: tool_call_id.to_string(),
            tool_name: "database_query".to_string(),
            parent_message_id: Some("msg-parent".to_string()),
        }),
    };

    let chunk1_event = BaseEvent {
        event_type: EventType::ToolCallChunk,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallChunk(ToolCallChunkEvent {
            tool_call_id: tool_call_id.to_string(),
            delta: r#"{"query": "SELECT * FROM"#.to_string(),
        }),
    };

    let chunk2_event = BaseEvent {
        event_type: EventType::ToolCallChunk,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallChunk(ToolCallChunkEvent {
            tool_call_id: tool_call_id.to_string(),
            delta: r#" users WHERE age > 18"}"#.to_string(),
        }),
    };

    let end_event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: tool_call_id.to_string(),
            tool_call: Some(ToolCall {
                id: tool_call_id.to_string(),
                name: "database_query".to_string(),
                arguments: Some(json!({"query": "SELECT * FROM users WHERE age > 18"})),
            }),
        }),
    };

    let events = vec![start_event, chunk1_event, chunk2_event, end_event];

    // Test that all events serialize and have consistent tool_call_id
    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["tool_call_id"], tool_call_id);

        // Test deserialization
        let _deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    }

    // Verify sequence progression
    let start_json = serde_json::to_string(&events[0]).unwrap();
    let start_parsed: serde_json::Value = serde_json::from_str(&start_json).unwrap();
    assert_eq!(start_parsed["type"], "TOOL_CALL_START");
    assert_eq!(start_parsed["tool_name"], "database_query");

    let end_json = serde_json::to_string(&events[3]).unwrap();
    let end_parsed: serde_json::Value = serde_json::from_str(&end_json).unwrap();
    assert_eq!(end_parsed["type"], "TOOL_CALL_END");
    assert_eq!(end_parsed["tool_call"]["name"], "database_query");
}

// ===== STATE EVENTS COMPREHENSIVE TESTS =====

#[wasm_bindgen_test]
fn test_state_snapshot_event_comprehensive() {
    // Test with complex nested state (matching TypeScript patterns)
    let mut state = HashMap::new();

    // Add different types of state data
    state.insert("user_profile".to_string(), json!({
        "name": "John Doe",
        "age": 30,
        "contact": {
            "email": "john@example.com",
            "phone": "+1234567890",
            "address": {
                "street": "123 Main St",
                "city": "Anytown",
                "country": "USA",
                "coordinates": {
                    "lat": 37.7749,
                    "lng": -122.4194
                }
            }
        },
        "preferences": {
            "theme": "dark",
            "notifications": true,
            "privateProfile": false
        }
    }));

    state.insert("service_config".to_string(), json!({
        "endpoints": [
            {
                "name": "api1",
                "url": "https://api1.example.com",
                "methods": ["GET", "POST"]
            },
            {
                "name": "api2",
                "url": "https://api2.example.com",
                "methods": ["GET"]
            }
        ],
        "retry_policy": {
            "max_retries": 3,
            "backoff": "exponential",
            "timeouts": [1000, 2000, 4000]
        }
    }));

    // Test edge case values
    state.insert("edge_cases".to_string(), json!({
        "null_value": null,
        "empty_string": "",
        "zero": 0,
        "negative_number": -123,
        "float_number": 3.14159,
        "empty_array": [],
        "empty_object": {},
        "bool_values": {"true": true, "false": false},
        "unicode_text": "Special chars: 你好 こんにちは 안녕하세요 👋 🌍",
        "date_string": "2023-10-01T12:00:00Z"
    }));

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Verify structure preservation
    assert_eq!(parsed["type"], "STATE_SNAPSHOT");
    assert_eq!(parsed["state"]["user_profile"]["name"], "John Doe");
    assert_eq!(parsed["state"]["user_profile"]["contact"]["address"]["coordinates"]["lat"], 37.7749);
    assert_eq!(parsed["state"]["service_config"]["endpoints"][0]["name"], "api1");
    assert_eq!(parsed["state"]["service_config"]["retry_policy"]["timeouts"][2], 4000);

    // Verify edge cases
    assert!(parsed["state"]["edge_cases"]["null_value"].is_null());
    assert_eq!(parsed["state"]["edge_cases"]["empty_string"], "");
    assert_eq!(parsed["state"]["edge_cases"]["zero"], 0);
    assert_eq!(parsed["state"]["edge_cases"]["negative_number"], -123);
    assert_eq!(parsed["state"]["edge_cases"]["float_number"], 3.14159);

    // Test deserialization
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    if let EventData::StateSnapshot(data) = &deserialized.data {
        let state_json = serde_json::to_value(&data.state).unwrap();
        assert_eq!(state_json["user_profile"]["preferences"]["theme"], "dark");
        assert_eq!(state_json["edge_cases"]["unicode_text"], "Special chars: 你好 こんにちは 안녕하세요 👋 🌍");
    }
}

#[wasm_bindgen_test]
fn test_state_delta_event_comprehensive() {
    // Test all JSON Patch operations (matching TypeScript patterns)
    let delta = json!([
        {"op": "add", "path": "/users/123", "value": {"name": "John", "age": 30}},
        {"op": "remove", "path": "/users/456"},
        {"op": "replace", "path": "/users/789/name", "value": "Jane Doe"},
        {"op": "move", "from": "/users/old_location", "path": "/users/new_location"},
        {"op": "copy", "from": "/templates/default", "path": "/users/123/template"},
        {"op": "test", "path": "/users/123/active", "value": true}
    ]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "STATE_DELTA");
    let delta_array = parsed["delta"].as_array().unwrap();
    assert_eq!(delta_array.len(), 6);

    // Verify each operation
    assert_eq!(delta_array[0]["op"], "add");
    assert_eq!(delta_array[0]["path"], "/users/123");
    assert_eq!(delta_array[0]["value"]["name"], "John");

    assert_eq!(delta_array[1]["op"], "remove");
    assert_eq!(delta_array[1]["path"], "/users/456");

    assert_eq!(delta_array[2]["op"], "replace");
    assert_eq!(delta_array[2]["value"], "Jane Doe");

    assert_eq!(delta_array[3]["op"], "move");
    assert_eq!(delta_array[3]["from"], "/users/old_location");

    assert_eq!(delta_array[4]["op"], "copy");
    assert_eq!(delta_array[5]["op"], "test");
}

// ===== MESSAGES SNAPSHOT COMPREHENSIVE TESTS =====

#[wasm_bindgen_test]
fn test_messages_snapshot_event_comprehensive() {
    // Create messages with various complexities
    let messages = vec![
        // System message
        Message {
            id: "sys-001".to_string(),
            role: Role::System,
            content: "You are a helpful assistant with access to various tools.".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("system_version".to_string(), json!("v2.1"));
                map.insert("capabilities".to_string(), json!(["search", "calculate", "analyze"]));
                map
            }),
            created_at: Some(Utc::now()),
        },

        // User message with metadata
        Message {
            id: "user-001".to_string(),
            role: Role::User,
            content: "Can you analyze the sales data and provide insights?".to_string(),
            name: Some("Alice Johnson".to_string()),
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("client_info".to_string(), json!({
                    "browser": "Chrome",
                    "version": "120.0",
                    "platform": "Windows",
                    "session_id": "sess_abc123"
                }));
                map.insert("request_context".to_string(), json!({
                    "urgency": "high",
                    "department": "sales"
                }));
                map
            }),
            created_at: Some(Utc::now()),
        },

        // Assistant message with complex tool calls
        Message {
            id: "asst-001".to_string(),
            role: Role::Assistant,
            content: "I'll analyze the sales data for you. Let me fetch and process the information.".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: Some(vec![
                ToolCall {
                    id: "call-data-fetch".to_string(),
                    name: "fetch_sales_data".to_string(),
                    arguments: Some(json!({
                        "time_period": "2023-Q4",
                        "regions": ["North America", "Europe", "Asia"],
                        "metrics": ["revenue", "units_sold", "conversion_rate"],
                        "filters": {
                            "product_categories": ["electronics", "software"],
                            "min_transaction_value": 100
                        }
                    })),
                },
                ToolCall {
                    id: "call-analysis".to_string(),
                    name: "analyze_trends".to_string(),
                    arguments: Some(json!({
                        "analysis_type": "comprehensive",
                        "include_forecasting": true,
                        "comparison_periods": ["2023-Q3", "2022-Q4"]
                    })),
                }
            ]),
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("model_info".to_string(), json!({
                    "model": "gpt-4-turbo",
                    "temperature": 0.7,
                    "max_tokens": 2000
                }));
                map.insert("processing_stats".to_string(), json!({
                    "reasoning_time": 1.2,
                    "token_usage": 450
                }));
                map
            }),
            created_at: Some(Utc::now()),
        },

        // Tool message with results
        Message {
            id: "tool-001".to_string(),
            role: Role::Tool,
            content: json!({
                "sales_data": {
                    "total_revenue": 2_450_000,
                    "units_sold": 15_600,
                    "conversion_rate": 0.078,
                    "regional_breakdown": {
                        "North America": {"revenue": 1_200_000, "units": 7_800},
                        "Europe": {"revenue": 850_000, "units": 5_200},
                        "Asia": {"revenue": 400_000, "units": 2_600}
                    }
                },
                "trends": {
                    "quarter_over_quarter_growth": 0.15,
                    "year_over_year_growth": 0.23,
                    "forecast_next_quarter": 2_800_000
                }
            }).to_string(),
            name: None,
            tool_call_id: Some("call-data-fetch".to_string()),
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("execution_time".to_string(), json!(2.5));
                map.insert("data_sources".to_string(), json!(["CRM", "Analytics DB", "External API"]));
                map.insert("cache_status".to_string(), json!("miss"));
                map
            }),
            created_at: Some(Utc::now()),
        },

        // Developer message
        Message {
            id: "dev-001".to_string(),
            role: Role::Developer,
            content: "System note: Analysis completed successfully. All data sources were accessible and current.".to_string(),
            name: Some("System Monitor".to_string()),
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("log_level".to_string(), json!("INFO"));
                map.insert("component".to_string(), json!("data_pipeline"));
                map.insert("timestamp_utc".to_string(), json!(Utc::now().to_rfc3339()));
                map
            }),
            created_at: Some(Utc::now()),
        },
    ];

    let event = BaseEvent {
        event_type: EventType::MessagesSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::MessagesSnapshot(MessagesSnapshotEvent { messages }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Verify basic structure
    assert_eq!(parsed["type"], "MESSAGES_SNAPSHOT");
    let messages_array = parsed["messages"].as_array().unwrap();
    assert_eq!(messages_array.len(), 5);

    // Verify system message
    assert_eq!(messages_array[0]["role"], "system");
    assert_eq!(messages_array[0]["id"], "sys-001");
    assert_eq!(messages_array[0]["metadata"]["system_version"], "v2.1");

    // Verify user message
    assert_eq!(messages_array[1]["role"], "user");
    assert_eq!(messages_array[1]["name"], "Alice Johnson");
    assert_eq!(messages_array[1]["metadata"]["client_info"]["browser"], "Chrome");

    // Verify assistant message with tool calls
    assert_eq!(messages_array[2]["role"], "assistant");
    let tool_calls = messages_array[2]["tool_calls"].as_array().unwrap();
    assert_eq!(tool_calls.len(), 2);
    assert_eq!(tool_calls[0]["name"], "fetch_sales_data");
    assert_eq!(tool_calls[0]["arguments"]["metrics"][0], "revenue");
    assert_eq!(tool_calls[1]["name"], "analyze_trends");

    // Verify tool message
    assert_eq!(messages_array[3]["role"], "tool");
    assert_eq!(messages_array[3]["tool_call_id"], "call-data-fetch");
    let tool_content: serde_json::Value = serde_json::from_str(messages_array[3]["content"].as_str().unwrap()).unwrap();
    assert_eq!(tool_content["sales_data"]["total_revenue"], 2_450_000);

    // Verify developer message
    assert_eq!(messages_array[4]["role"], "developer");
    assert_eq!(messages_array[4]["name"], "System Monitor");
    assert_eq!(messages_array[4]["metadata"]["log_level"], "INFO");

    // Test deserialization
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    if let EventData::MessagesSnapshot(data) = &deserialized.data {
        assert_eq!(data.messages.len(), 5);
        assert_eq!(data.messages[2].tool_calls.as_ref().unwrap().len(), 2);
        assert_eq!(data.messages[3].tool_call_id, Some("call-data-fetch".to_string()));
    }
}

// ===== ERROR AND RUN LIFECYCLE EVENTS =====

#[wasm_bindgen_test]
fn test_error_event_comprehensive() {
    // Test with detailed error information
    let event = BaseEvent {
        event_type: EventType::Error,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({
            "stack_trace": ["function1", "function2", "function3"],
            "request_id": "req_abc123"
        })),
        data: EventData::Error(ErrorEvent {
            error: "Database connection timeout after multiple retries".to_string(),
            code: Some("DB_TIMEOUT_ERROR".to_string()),
            details: Some(json!({
                "timeout_duration": 30000,
                "retry_count": 3,
                "last_attempt": "2023-10-01T12:00:00Z",
                "database_host": "db-primary.example.com",
                "affected_operations": ["fetch_sales_data", "analyze_trends"],
                "recovery_suggestions": [
                    "Check database connectivity",
                    "Verify network latency",
                    "Consider using read replica"
                ]
            })),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "ERROR");
    assert_eq!(parsed["error"], "Database connection timeout after multiple retries");
    assert_eq!(parsed["code"], "DB_TIMEOUT_ERROR");
    assert_eq!(parsed["details"]["timeout_duration"], 30000);
    assert_eq!(parsed["details"]["retry_count"], 3);
    assert_eq!(parsed["details"]["affected_operations"][0], "fetch_sales_data");
    assert_eq!(parsed["raw_event"]["stack_trace"][1], "function2");
}

#[wasm_bindgen_test]
fn test_run_lifecycle_events_comprehensive() {
    let thread_id = "thread-comprehensive-test";
    let run_id = "run-comprehensive-test";

    // Test run started
    let start_event = BaseEvent::run_started(thread_id.to_string(), run_id.to_string());
    let start_json = serde_json::to_string(&start_event).unwrap();
    let start_parsed: serde_json::Value = serde_json::from_str(&start_json).unwrap();

    assert_eq!(start_parsed["type"], "RUN_STARTED");
    assert_eq!(start_parsed["thread_id"], thread_id);
    assert_eq!(start_parsed["run_id"], run_id);

    // Test run finished
    let finish_event = BaseEvent::run_finished(thread_id.to_string(), run_id.to_string());
    let finish_json = serde_json::to_string(&finish_event).unwrap();
    let finish_parsed: serde_json::Value = serde_json::from_str(&finish_json).unwrap();

    assert_eq!(finish_parsed["type"], "RUN_FINISHED");
    assert_eq!(finish_parsed["thread_id"], thread_id);
    assert_eq!(finish_parsed["run_id"], run_id);

    // Test deserialization
    let start_deserialized: BaseEvent = serde_json::from_str(&start_json).unwrap();
    let finish_deserialized: BaseEvent = serde_json::from_str(&finish_json).unwrap();

    assert_eq!(start_deserialized.event_type, EventType::RunStarted);
    assert_eq!(finish_deserialized.event_type, EventType::RunFinished);
}