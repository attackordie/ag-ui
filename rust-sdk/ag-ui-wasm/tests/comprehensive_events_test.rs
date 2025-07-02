//! Comprehensive event testing suite that matches the actual Rust SDK implementation

use ag_ui_wasm::{
    BaseEvent, EventType, EventData,
    ToolCallStartEvent, ToolCallChunkEvent, ToolCallEndEvent,
    StateSnapshotEvent, StateDeltaEvent, MessagesSnapshotEvent,
    Message, Role, State, ToolCall,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// Test BaseEvent creation using helper methods
#[wasm_bindgen_test]
fn test_base_event_helpers() {
    let run_started = BaseEvent::run_started("thread_123".to_string(), "run_456".to_string());
    assert_eq!(run_started.event_type, EventType::RunStarted);
    
    let run_finished = BaseEvent::run_finished("thread_123".to_string(), "run_456".to_string());
    assert_eq!(run_finished.event_type, EventType::RunFinished);
    
    let text_start = BaseEvent::text_message_start("msg_001".to_string(), Some(Role::Assistant));
    assert_eq!(text_start.event_type, EventType::TextMessageStart);
    
    let text_content = BaseEvent::text_message_content("msg_001".to_string(), "Hello!".to_string());
    assert_eq!(text_content.event_type, EventType::TextMessageContent);
    
    let text_end = BaseEvent::text_message_end("msg_001".to_string());
    assert_eq!(text_end.event_type, EventType::TextMessageEnd);
    
    let error_event = BaseEvent::error("Something went wrong".to_string(), Some("ERR_001".to_string()));
    assert_eq!(error_event.event_type, EventType::Error);
}

// Test event serialization
#[wasm_bindgen_test]
fn test_event_serialization() {
    let event = BaseEvent::text_message_content("msg_123".to_string(), "Hello, world!".to_string());
    
    let json_result = serde_json::to_string(&event);
    assert!(json_result.is_ok());
    
    let json_str = json_result.unwrap();
    assert!(json_str.contains("TEXT_MESSAGE_CONTENT"));
    assert!(json_str.contains("msg_123"));
    assert!(json_str.contains("Hello, world!"));
}

// Test event deserialization round trip
#[wasm_bindgen_test]
fn test_event_round_trip() {
    let original = BaseEvent::text_message_start("msg_456".to_string(), Some(Role::Assistant));
    
    let json_str = serde_json::to_string(&original).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(original.event_type, deserialized.event_type);
    
    if let (EventData::TextMessageStart(orig), EventData::TextMessageStart(deser)) = 
        (&original.data, &deserialized.data) {
        assert_eq!(orig.message_id, deser.message_id);
        assert_eq!(orig.role, deser.role);
    }
}

// Test ToolCallStart event creation
#[wasm_bindgen_test]
fn test_tool_call_start() {
    let event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "call_123".to_string(),
            tool_name: "search_tool".to_string(),
            parent_message_id: Some("msg_parent".to_string()),
        }),
    };
    
    assert_eq!(event.event_type, EventType::ToolCallStart);
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("TOOL_CALL_START"));
    assert!(json_str.contains("call_123"));
    assert!(json_str.contains("search_tool"));
}

// Test StateSnapshot event
#[wasm_bindgen_test]
fn test_state_snapshot() {
    let mut state: State = HashMap::new();
    state.insert("session_id".to_string(), json!("sess_123"));
    state.insert("user_data".to_string(), json!({
        "name": "Alice",
        "preferences": {
            "theme": "dark",
            "language": "en"
        }
    }));
    
    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };
    
    assert_eq!(event.event_type, EventType::StateSnapshot);
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("STATE_SNAPSHOT"));
    assert!(json_str.contains("sess_123"));
    assert!(json_str.contains("Alice"));
}

// Test MessagesSnapshot event
#[wasm_bindgen_test]
fn test_messages_snapshot() {
    let messages = vec![
        Message {
            id: "msg_1".to_string(),
            role: Role::User,
            content: "Hello AI".to_string(),
            name: None,
            tool_call_id: None,
            tool_calls: None,
            function_call: None,
            metadata: None,
            created_at: Some(Utc::now()),
        },
        Message {
            id: "msg_2".to_string(),
            role: Role::Assistant,
            content: "Hello! How can I help?".to_string(),
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
    
    assert_eq!(event.event_type, EventType::MessagesSnapshot);
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("MESSAGES_SNAPSHOT"));
    assert!(json_str.contains("Hello AI"));
    assert!(json_str.contains("Hello! How can I help?"));
}

// Test error event
#[wasm_bindgen_test]
fn test_error_event() {
    let event = BaseEvent::error("Connection failed".to_string(), Some("CONN_ERROR".to_string()));
    
    if let EventData::Error(data) = &event.data {
        assert_eq!(data.error, "Connection failed");
        assert_eq!(data.code, Some("CONN_ERROR".to_string()));
    }
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("ERROR"));
    assert!(json_str.contains("Connection failed"));
    assert!(json_str.contains("CONN_ERROR"));
}

// Test complex event sequence
#[wasm_bindgen_test]
fn test_event_sequence() {
    let events = vec![
        BaseEvent::run_started("thread_123".to_string(), "run_456".to_string()),
        BaseEvent::text_message_start("msg_001".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg_001".to_string(), "Processing your request...".to_string()),
        BaseEvent::text_message_end("msg_001".to_string()),
        BaseEvent::run_finished("thread_123".to_string(), "run_456".to_string()),
    ];
    
    // All events should serialize successfully
    for event in &events {
        let json_result = serde_json::to_string(event);
        assert!(json_result.is_ok());
    }
    
    // Verify event types
    assert_eq!(events[0].event_type, EventType::RunStarted);
    assert_eq!(events[1].event_type, EventType::TextMessageStart);
    assert_eq!(events[2].event_type, EventType::TextMessageContent);
    assert_eq!(events[3].event_type, EventType::TextMessageEnd);
    assert_eq!(events[4].event_type, EventType::RunFinished);
}

// Test Unicode and special characters
#[wasm_bindgen_test]
fn test_unicode_content() {
    let unicode_text = "Hello 你好 こんにちは 안녕하세요 👋 🌍 \\n\\t\"'/<>";
    let event = BaseEvent::text_message_content("msg_unicode".to_string(), unicode_text.to_string());
    
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    
    if let EventData::TextMessageContent(data) = &deserialized.data {
        assert_eq!(data.delta, unicode_text);
    }
}

// Test large content
#[wasm_bindgen_test]
fn test_large_content() {
    let large_content = "A".repeat(5000);
    let event = BaseEvent::text_message_content("msg_large".to_string(), large_content.clone());
    
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
    
    if let EventData::TextMessageContent(data) = &deserialized.data {
        assert_eq!(data.delta.len(), 5000);
        assert_eq!(data.delta, large_content);
    }
}

// Test StateDelta event
#[wasm_bindgen_test]
fn test_state_delta() {
    let delta = json!([
        {"op": "replace", "path": "/status", "value": "active"},
        {"op": "add", "path": "/new_field", "value": 42}
    ]);
    
    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };
    
    assert_eq!(event.event_type, EventType::StateDelta);
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("STATE_DELTA"));
    assert!(json_str.contains("replace"));
    assert!(json_str.contains("active"));
}

// Test ToolCallChunk event
#[wasm_bindgen_test]
fn test_tool_call_chunk() {
    let event = BaseEvent {
        event_type: EventType::ToolCallChunk,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallChunk(ToolCallChunkEvent {
            tool_call_id: "call_123".to_string(),
            delta: "partial content".to_string(),
        }),
    };
    
    assert_eq!(event.event_type, EventType::ToolCallChunk);
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("TOOL_CALL_CHUNK"));
    assert!(json_str.contains("call_123"));
    assert!(json_str.contains("partial content"));
}

// Test ToolCallEnd event with ToolCall
#[wasm_bindgen_test]
fn test_tool_call_end_with_tool_call() {
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        name: "search".to_string(),
        arguments: Some(json!({"query": "test"})),
    };
    
    let event = BaseEvent {
        event_type: EventType::ToolCallEnd,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallEnd(ToolCallEndEvent {
            tool_call_id: "call_123".to_string(),
            tool_call: Some(tool_call),
        }),
    };
    
    assert_eq!(event.event_type, EventType::ToolCallEnd);
    
    let json_str = serde_json::to_string(&event).unwrap();
    assert!(json_str.contains("TOOL_CALL_END"));
    assert!(json_str.contains("call_123"));
    assert!(json_str.contains("search"));
}