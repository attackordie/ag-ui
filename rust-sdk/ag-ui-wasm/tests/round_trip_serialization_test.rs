//! Round-trip serialization tests matching TypeScript and Python patterns

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, Role,
    ToolCallStartEvent, ToolCallChunkEvent, ToolCallEndEvent,
    StateSnapshotEvent, StateDeltaEvent, MessagesSnapshotEvent,
    TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
    ErrorEvent, Message, ToolCall,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to perform round-trip serialization test
fn expect_round_trip_equality(event: &BaseEvent) {
    // Serialize to JSON
    let json_str = serde_json::to_string(event).expect("Failed to serialize event");

    // Deserialize back to event
    let deserialized: BaseEvent = serde_json::from_str(&json_str).expect("Failed to deserialize event");

    // Verify type matches
    assert_eq!(event.event_type, deserialized.event_type);

    // Verify timestamps are preserved (allowing for None timestamps)
    match (&event.timestamp, &deserialized.timestamp) {
        (Some(orig), Some(deser)) => {
            // Allow small timestamp differences due to serialization
            let diff = (*orig - *deser).num_milliseconds().abs();
            assert!(diff < 1000, "Timestamp difference too large: {}ms", diff);
        }
        (None, None) => {} // Both None is fine
        _ => panic!("Timestamp preservation failed"),
    }

    // Verify event-specific data matches based on event type
    match (&event.data, &deserialized.data) {
        (EventData::TextMessageStart(orig), EventData::TextMessageStart(deser)) => {
            assert_eq!(orig.message_id, deser.message_id);
            assert_eq!(orig.role, deser.role);
        }
        (EventData::TextMessageContent(orig), EventData::TextMessageContent(deser)) => {
            assert_eq!(orig.message_id, deser.message_id);
            assert_eq!(orig.delta, deser.delta);
        }
        (EventData::TextMessageEnd(orig), EventData::TextMessageEnd(deser)) => {
            assert_eq!(orig.message_id, deser.message_id);
        }
        (EventData::ToolCallStart(orig), EventData::ToolCallStart(deser)) => {
            assert_eq!(orig.tool_call_id, deser.tool_call_id);
            assert_eq!(orig.tool_name, deser.tool_name);
            assert_eq!(orig.parent_message_id, deser.parent_message_id);
        }
        (EventData::ToolCallChunk(orig), EventData::ToolCallChunk(deser)) => {
            assert_eq!(orig.tool_call_id, deser.tool_call_id);
            assert_eq!(orig.delta, deser.delta);
        }
        (EventData::ToolCallEnd(orig), EventData::ToolCallEnd(deser)) => {
            assert_eq!(orig.tool_call_id, deser.tool_call_id);
            // Compare tool_call if present
            match (&orig.tool_call, &deser.tool_call) {
                (Some(orig_call), Some(deser_call)) => {
                    assert_eq!(orig_call.id, deser_call.id);
                    assert_eq!(orig_call.name, deser_call.name);
                    assert_eq!(orig_call.arguments, deser_call.arguments);
                }
                (None, None) => {}
                _ => panic!("ToolCall preservation failed"),
            }
        }
        (EventData::StateSnapshot(orig), EventData::StateSnapshot(deser)) => {
            // Compare state objects as JSON values for deep equality
            let orig_json = serde_json::to_value(&orig.state).unwrap();
            let deser_json = serde_json::to_value(&deser.state).unwrap();
            assert_eq!(orig_json, deser_json);
        }
        (EventData::StateDelta(orig), EventData::StateDelta(deser)) => {
            assert_eq!(orig.delta, deser.delta);
        }
        (EventData::MessagesSnapshot(orig), EventData::MessagesSnapshot(deser)) => {
            assert_eq!(orig.messages.len(), deser.messages.len());
            for (orig_msg, deser_msg) in orig.messages.iter().zip(deser.messages.iter()) {
                assert_eq!(orig_msg.id, deser_msg.id);
                assert_eq!(orig_msg.role, deser_msg.role);
                assert_eq!(orig_msg.content, deser_msg.content);
                assert_eq!(orig_msg.tool_call_id, deser_msg.tool_call_id);
                // Compare tool_calls if present
                match (&orig_msg.tool_calls, &deser_msg.tool_calls) {
                    (Some(orig_calls), Some(deser_calls)) => {
                        assert_eq!(orig_calls.len(), deser_calls.len());
                        for (orig_call, deser_call) in orig_calls.iter().zip(deser_calls.iter()) {
                            assert_eq!(orig_call.id, deser_call.id);
                            assert_eq!(orig_call.name, deser_call.name);
                            assert_eq!(orig_call.arguments, deser_call.arguments);
                        }
                    }
                    (None, None) => {}
                    _ => panic!("Tool calls preservation failed"),
                }
            }
        }
        (EventData::Error(orig), EventData::Error(deser)) => {
            assert_eq!(orig.error, deser.error);
            assert_eq!(orig.code, deser.code);
            assert_eq!(orig.details, deser.details);
        }
        (EventData::RunStarted(orig), EventData::RunStarted(deser)) => {
            assert_eq!(orig.thread_id, deser.thread_id);
            assert_eq!(orig.run_id, deser.run_id);
        }
        (EventData::RunFinished(orig), EventData::RunFinished(deser)) => {
            assert_eq!(orig.thread_id, deser.thread_id);
            assert_eq!(orig.run_id, deser.run_id);
        }
        _ => panic!("Event data types don't match: {:?} vs {:?}", event.event_type, deserialized.event_type),
    }
}

// ===== TEXT MESSAGE EVENTS ROUND-TRIP TESTS =====

#[wasm_bindgen_test]
fn test_text_message_start_round_trip() {
    let event = BaseEvent::text_message_start("msg-123".to_string(), Some(Role::Assistant));
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_text_message_start_missing_optional_fields() {
    let event = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: None, // Missing timestamp
        raw_event: None,
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg-456".to_string(),
            role: Some(Role::User),
        }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_text_message_content_round_trip() {
    let event = BaseEvent::text_message_content("msg-789".to_string(), "Hello, how can I help you today?".to_string());
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_text_message_content_special_characters() {
    let special_text = "Special chars: 🚀 ñ € 😊 \n\t\"'\\`";
    let event = BaseEvent::text_message_content("msg-special".to_string(), special_text.to_string());
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_text_message_end_round_trip() {
    let event = BaseEvent::text_message_end("msg-end".to_string());
    expect_round_trip_equality(&event);
}

// ===== TOOL CALL EVENTS ROUND-TRIP TESTS =====

#[wasm_bindgen_test]
fn test_tool_call_start_round_trip() {
    let event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "tool-123".to_string(),
            tool_name: "get_weather".to_string(),
            parent_message_id: Some("msg-parent".to_string()),
        }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_tool_call_start_with_all_optional_fields() {
    let event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({"original": "event data", "from": "source system"})),
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "tool-call-id-123".to_string(),
            tool_name: "very_long_tool_name_with_underscores".to_string(),
            parent_message_id: Some("parent-message-id-456".to_string()),
        }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_tool_call_chunk_round_trip() {
    let event = BaseEvent {
        event_type: EventType::ToolCallChunk,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallChunk(ToolCallChunkEvent {
            tool_call_id: "tool-456".to_string(),
            delta: "{\"location\":\"San Francisco\"}".to_string(),
        }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_tool_call_chunk_complex_json() {
    let complex_json = json!({
        "query": "SELECT * FROM users",
        "filters": {
            "age": {"min": 18, "max": 65},
            "status": ["active", "pending"],
            "location": {
                "country": "US",
                "states": ["CA", "NY", "TX"]
            }
        },
        "options": {
            "limit": 100,
            "offset": 0,
            "sort": {"field": "created_at", "order": "desc"}
        }
    });

    let event = BaseEvent {
        event_type: EventType::ToolCallChunk,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallChunk(ToolCallChunkEvent {
            tool_call_id: "db-query-tool-123".to_string(),
            delta: complex_json.to_string(),
        }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_tool_call_end_round_trip() {
    let event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: "tool-end-123".to_string(),
            tool_call: None,
        }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_tool_call_end_with_tool_call() {
    let tool_call = ToolCall {
        id: "call-with-result".to_string(),
        name: "calculate".to_string(),
        arguments: Some(json!({"operation": "sum", "values": [1, 2, 3, 4, 5]})),
    };

    let event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: "call-with-result".to_string(),
            tool_call: Some(tool_call),
        }),
    };
    expect_round_trip_equality(&event);
}

// ===== STATE EVENTS ROUND-TRIP TESTS =====

#[wasm_bindgen_test]
fn test_state_snapshot_round_trip() {
    let mut state = HashMap::new();
    state.insert("counter".to_string(), json!(42));
    state.insert("items".to_string(), json!(["apple", "banana", "cherry"]));
    state.insert("config".to_string(), json!({
        "enabled": true,
        "maxRetries": 3
    }));

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_state_snapshot_empty() {
    let state = HashMap::new();

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: None,
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_state_snapshot_complex_nested() {
    let mut state = HashMap::new();
    state.insert("userProfile".to_string(), json!({
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

    state.insert("serviceConfig".to_string(), json!({
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
        "retryPolicy": {
            "maxRetries": 3,
            "backoff": "exponential",
            "timeouts": [1000, 2000, 4000]
        }
    }));

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_state_delta_round_trip() {
    let delta = json!([
        {"op": "add", "path": "/counter", "value": 42},
        {"op": "add", "path": "/items", "value": ["apple", "banana", "cherry"]}
    ]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_state_delta_all_operations() {
    let delta = json!([
        {"op": "add", "path": "/users/123", "value": {"name": "John", "age": 30}},
        {"op": "remove", "path": "/users/456"},
        {"op": "replace", "path": "/users/789/name", "value": "Jane Doe"},
        {"op": "move", "from": "/users/old", "path": "/users/new"},
        {"op": "copy", "from": "/templates/default", "path": "/users/123/template"},
        {"op": "test", "path": "/users/123/active", "value": true}
    ]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };
    expect_round_trip_equality(&event);
}

// ===== MESSAGES SNAPSHOT ROUND-TRIP TESTS =====

#[wasm_bindgen_test]
fn test_messages_snapshot_multiple_messages() {
    let messages = vec![
        Message {
            id: "msg-1".to_string(),
            role: Role::User,
            content: "Can you help me with my task?".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "msg-2".to_string(),
            role: Role::Assistant,
            content: "I'd be happy to help! What task do you need assistance with?".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
    ];

    let event = BaseEvent {
        event_type: EventType::MessagesSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::MessagesSnapshot(MessagesSnapshotEvent { messages }),
    };
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_messages_snapshot_with_tool_calls() {
    let tool_calls = vec![
        ToolCall {
            id: "tool-1".to_string(),
            name: "get_weather".to_string(),
            arguments: Some(json!({"location": "San Francisco"})),
        }
    ];

    let messages = vec![
        Message {
            id: "msg-1".to_string(),
            role: Role::User,
            content: "What's the weather in San Francisco?".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "msg-2".to_string(),
            role: Role::Assistant,
            content: "Let me check the weather for you.".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: Some(tool_calls),
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
    ];

    let event = BaseEvent {
        event_type: EventType::MessagesSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::MessagesSnapshot(MessagesSnapshotEvent { messages }),
    };
    expect_round_trip_equality(&event);
}

// ===== ERROR EVENTS ROUND-TRIP TESTS =====

#[wasm_bindgen_test]
fn test_error_event_round_trip() {
    let event = BaseEvent::error("Something went wrong".to_string(), Some("ERR_001".to_string()));
    expect_round_trip_equality(&event);
}

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
    expect_round_trip_equality(&event);
}

// ===== RUN LIFECYCLE EVENTS ROUND-TRIP TESTS =====

#[wasm_bindgen_test]
fn test_run_started_round_trip() {
    let event = BaseEvent::run_started("thread-abc".to_string(), "run-def".to_string());
    expect_round_trip_equality(&event);
}

#[wasm_bindgen_test]
fn test_run_finished_round_trip() {
    let event = BaseEvent::run_finished("thread-ghi".to_string(), "run-jkl".to_string());
    expect_round_trip_equality(&event);
}

// ===== COMPLEX EVENT SEQUENCE ROUND-TRIP TEST =====

#[wasm_bindgen_test]
fn test_complex_event_sequence_round_trip() {
    let events = vec![
        BaseEvent::run_started("thread_complex".to_string(), "run_complex".to_string()),
        BaseEvent::text_message_start("msg-1".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg-1".to_string(), "Processing your request...".to_string()),
        BaseEvent {
            event_type: EventType::ToolCallStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallStart(ToolCallStartEvent {
                tool_call_id: "tool-1".to_string(),
                tool_name: "search_database".to_string(),
                parent_message_id: Some("msg-1".to_string()),
            }),
        },
        BaseEvent {
            event_type: EventType::ToolCallChunk,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallChunk(ToolCallChunkEvent {
                tool_call_id: "tool-1".to_string(),
                delta: r#"{"query":"SELECT *"#.to_string(),
            }),
        },
        BaseEvent {
            event_type: EventType::ToolCallEnd,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallEnd(ToolCallEndEvent {
                tool_call_id: "tool-1".to_string(),
                tool_call: None,
            }),
        },
        BaseEvent::text_message_content("msg-1".to_string(), r#" FROM users"}"#.to_string()),
        BaseEvent::text_message_end("msg-1".to_string()),
        BaseEvent::run_finished("thread_complex".to_string(), "run_complex".to_string()),
    ];

    // Test each event in the sequence
    for (i, event) in events.iter().enumerate() {
        expect_round_trip_equality(event);
    }
}