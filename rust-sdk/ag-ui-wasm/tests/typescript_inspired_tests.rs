//! Tests inspired by TypeScript SDK functionality

use ag_ui_wasm::{
    BaseEvent, EventType, EventData,
    ToolCallStartEvent, ToolCallChunkEvent, ToolCallEndEvent, StateDeltaEvent, MessagesSnapshotEvent, ErrorEvent,
    SSEEncoder, Message, Role,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// ===== PROTOBUF-INSPIRED TESTS (TypeScript has protobuf encoding) =====

#[wasm_bindgen_test]
fn test_event_encoding_different_formats() {
    // TypeScript SDK supports both SSE and protobuf encoding
    // We test SSE encoding with binary output (similar concept)
    let event = BaseEvent::text_message_start("msg123".to_string(), Some(Role::Assistant));
    
    // Test SSE string encoding
    let sse_result = SSEEncoder::encode_event_string(&event);
    assert!(sse_result.is_ok());
    
    let sse_string = sse_result.unwrap();
    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));
    assert!(sse_string.contains("TEXT_MESSAGE_START"));
    assert!(sse_string.contains("msg123"));
    
    // Test that JSON is well-formed
    let json_part = sse_string.trim_start_matches("data: ").trim_end();
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();
    assert_eq!(parsed["type"], "TEXT_MESSAGE_START");
    assert_eq!(parsed["message_id"], "msg123");
}

// ===== EVENT VALIDATION TESTS (TypeScript has comprehensive validation) =====

#[wasm_bindgen_test]
fn test_event_id_consistency_validation() {
    // Simulate validation that message IDs are consistent across related events
    let message_id = "msg_123".to_string();
    
    let start_event = BaseEvent::text_message_start(message_id.clone(), Some(Role::Assistant));
    let content_event = BaseEvent::text_message_content(message_id.clone(), "Hello".to_string());
    let end_event = BaseEvent::text_message_end(message_id.clone());
    
    // Extract message IDs and verify they match
    if let EventData::TextMessageStart(data) = &start_event.data {
        assert_eq!(data.message_id, message_id);
    }
    
    if let EventData::TextMessageContent(data) = &content_event.data {
        assert_eq!(data.message_id, message_id);
    }
    
    if let EventData::TextMessageEnd(data) = &end_event.data {
        assert_eq!(data.message_id, message_id);
    }
}

#[wasm_bindgen_test]
fn test_tool_call_id_consistency_validation() {
    // Simulate validation that tool call IDs are consistent
    let tool_call_id = "call_123".to_string();
    
    let start_event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: tool_call_id.clone(),
            tool_name: "search_tool".to_string(),
            parent_message_id: None,
        }),
    };
    
    let chunk_event = BaseEvent {
        event_type: EventType::ToolCallChunk,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallChunk(ToolCallChunkEvent {
            tool_call_id: tool_call_id.clone(),
            delta: "partial args".to_string(),
        }),
    };
    
    let end_event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: tool_call_id.clone(),
            tool_call: None,
        }),
    };
    
    // Verify all have the same tool call ID
    if let EventData::ToolCallStart(data) = &start_event.data {
        assert_eq!(data.tool_call_id, tool_call_id);
    }
    
    if let EventData::ToolCallChunk(data) = &chunk_event.data {
        assert_eq!(data.tool_call_id, tool_call_id);
    }
    
    if let EventData::ToolCallEnd(data) = &end_event.data {
        assert_eq!(data.tool_call_id, tool_call_id);
    }
}

// ===== COMPLEX EVENT SEQUENCES (TypeScript tests complex workflows) =====

#[wasm_bindgen_test]
fn test_complex_conversation_flow() {
    // Test a complex conversation flow similar to TypeScript tests
    let events = vec![
        BaseEvent::run_started("thread_123".to_string(), "run_456".to_string()),
        BaseEvent::text_message_start("msg1".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg1".to_string(), "I'll help you with that. ".to_string()),
        BaseEvent::text_message_end("msg1".to_string()),
        
        // Tool call sequence
        BaseEvent {
            event_type: EventType::ToolCallStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallStart(ToolCallStartEvent {
                tool_call_id: "t1".to_string(),
                tool_name: "search_tool".to_string(),
                parent_message_id: Some("msg1".to_string()),
            }),
        },
        BaseEvent {
            event_type: EventType::ToolCallChunk,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallChunk(ToolCallChunkEvent {
                tool_call_id: "t1".to_string(),
                delta: "search args".to_string(),
            }),
        },
        BaseEvent {
            event_type: EventType::ToolCallEnd,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallEnd(ToolCallEndEvent {
                tool_call_id: "t1".to_string(),
                tool_call: None,
            }),
        },
        
        // Another message
        BaseEvent::text_message_start("msg2".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg2".to_string(), "Based on the search results...".to_string()),
        BaseEvent::text_message_end("msg2".to_string()),
        BaseEvent::run_finished("thread_123".to_string(), "run_456".to_string()),
    ];
    
    // Verify all events serialize correctly
    for (i, event) in events.iter().enumerate() {
        let json_result = serde_json::to_string(event);
        assert!(json_result.is_ok(), "Event {} serialization failed", i);
        
        // Verify SSE encoding works
        let sse_result = SSEEncoder::encode_event_string(event);
        assert!(sse_result.is_ok(), "Event {} SSE encoding failed", i);
    }
    
    // Test encoding multiple events
    let multi_encoded = SSEEncoder::encode_events_string(&events);
    assert!(multi_encoded.is_ok());
    
    let full_stream = multi_encoded.unwrap();
    assert!(full_stream.contains("RUN_STARTED"));
    assert!(full_stream.contains("TOOL_CALL_START"));
    assert!(full_stream.contains("TOOL_CALL_CHUNK"));
    assert!(full_stream.contains("TOOL_CALL_END"));
    assert!(full_stream.contains("RUN_FINISHED"));
}

// ===== STATE MANAGEMENT TESTS (TypeScript has sophisticated state handling) =====

#[wasm_bindgen_test]
fn test_state_delta_operations() {
    // Test JSON Patch-style operations like TypeScript
    let delta_operations = vec![
        json!({"op": "add", "path": "/user/preferences/theme", "value": "dark"}),
        json!({"op": "replace", "path": "/session/status", "value": "active"}),
        json!({"op": "remove", "path": "/temporary_data"}),
        json!({"op": "test", "path": "/version", "value": "1.0"}),
        json!({"op": "move", "from": "/old_location", "path": "/new_location"}),
        json!({"op": "copy", "from": "/template", "path": "/instance"})
    ];
    
    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent {
            delta: json!(delta_operations),
        }),
    };
    
    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    
    if let EventData::StateDelta(data) = &deserialized.data {
        let operations = data.delta.as_array().unwrap();
        assert_eq!(operations.len(), 6);
        assert_eq!(operations[0]["op"], "add");
        assert_eq!(operations[1]["op"], "replace");
        assert_eq!(operations[2]["op"], "remove");
        assert_eq!(operations[3]["op"], "test");
        assert_eq!(operations[4]["op"], "move");
        assert_eq!(operations[5]["op"], "copy");
    }
}

#[wasm_bindgen_test]
fn test_messages_snapshot_with_complex_metadata() {
    // Test messages with complex metadata like TypeScript
    let messages = vec![
        Message {
            id: "user_1".to_string(),
            role: Role::User,
            content: "Hello AI".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("client_info".to_string(), json!({
                    "browser": "Chrome",
                    "version": "91.0",
                    "platform": "Windows"
                }));
                map
            }),
            created_at: Some(Utc::now()),
        },
        Message {
            id: "asst_1".to_string(),
            role: Role::Assistant,
            content: "Hello! How can I help?".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("model_info".to_string(), json!({
                    "model": "gpt-4",
                    "temperature": 0.7,
                    "max_tokens": 1000
                }));
                map.insert("tool_calls".to_string(), json!([
                    {
                        "id": "call_1",
                        "type": "function",
                        "function": {
                            "name": "get_weather",
                            "arguments": "{\"location\": \"New York\"}"
                        }
                    }
                ]));
                map
            }),
            created_at: Some(Utc::now()),
        },
        Message {
            id: "tool_1".to_string(),
            role: Role::Tool,
            content: "{\"temperature\": 72, \"condition\": \"sunny\"}".to_string(),
            name: None,
            tool_call_id: Some("call_1".to_string()),
            tool_calls: None,
            function_call: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("execution_time".to_string(), json!(0.5));
                map.insert("api_version".to_string(), json!("v1.2"));
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
    
    // Test serialization and deserialization
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    
    if let EventData::MessagesSnapshot(data) = &deserialized.data {
        assert_eq!(data.messages.len(), 3);
        
        // Verify user message metadata
        assert_eq!(data.messages[0].metadata.as_ref().unwrap()["client_info"]["browser"], "Chrome");
        
        // Verify assistant message metadata
        assert_eq!(data.messages[1].metadata.as_ref().unwrap()["model_info"]["model"], "gpt-4");
        assert_eq!(data.messages[1].metadata.as_ref().unwrap()["tool_calls"][0]["function"]["name"], "get_weather");
        
        // Verify tool message metadata
        assert_eq!(data.messages[2].tool_call_id, Some("call_1".to_string()));
        assert_eq!(data.messages[2].metadata.as_ref().unwrap()["execution_time"], 0.5);
    }
}

// ===== ERROR HANDLING TESTS (TypeScript has comprehensive error handling) =====

#[wasm_bindgen_test]
fn test_error_event_with_details() {
    let event = BaseEvent {
        event_type: EventType::Error,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Error(ErrorEvent {
            error: "Connection timeout".to_string(),
            code: Some("TIMEOUT_ERROR".to_string()),
            details: Some(json!({
                "timeout_duration": 30000,
                "retry_count": 3,
                "last_attempt": "2023-10-01T12:00:00Z",
                "stack_trace": ["function1", "function2", "function3"]
            })),
        }),
    };
    
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["error"], "Connection timeout");
    assert_eq!(parsed["code"], "TIMEOUT_ERROR");
    assert_eq!(parsed["details"]["timeout_duration"], 30000);
    assert_eq!(parsed["details"]["retry_count"], 3);
    assert_eq!(parsed["details"]["stack_trace"][1], "function2");
}

// ===== TIMESTAMP CONSISTENCY TESTS =====

#[wasm_bindgen_test]
fn test_timestamp_consistency() {
    let before_time = Utc::now();
    
    let event1 = BaseEvent::text_message_start("msg1".to_string(), Some(Role::Assistant));
    let event2 = BaseEvent::text_message_content("msg1".to_string(), "Hello".to_string());
    
    let after_time = Utc::now();
    
    // Timestamps should be reasonable (within the test execution time)
    assert!(event1.timestamp.unwrap() >= before_time);
    assert!(event1.timestamp.unwrap() <= after_time);
    assert!(event2.timestamp.unwrap() >= before_time);
    assert!(event2.timestamp.unwrap() <= after_time);
    
    // Events created close together should have close timestamps
    let time_diff = (event2.timestamp.unwrap() - event1.timestamp.unwrap()).num_milliseconds();
    assert!(time_diff.abs() < 1000); // Less than 1 second difference
}