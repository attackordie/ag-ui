use ag_ui_wasm::{
    BaseEvent, EventType, EventData,
    TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
    RunStartedEvent, RunFinishedEvent,
    Message, RunAgentInput, State, Role,
    SSEEncoder,
};
use wasm_bindgen_test::*;
use std::collections::HashMap;
use serde_json::json;
use chrono::Utc;

wasm_bindgen_test_configure!(run_in_browser);

// Test the complete AG-UI protocol flow
#[wasm_bindgen_test]
async fn test_complete_agent_conversation_flow() {
    // Initialize agent input
    let input = RunAgentInput {
        thread_id: "test-thread-123".to_string(),
        run_id: "test-run-123".to_string(),
        messages: Some(vec![
            Message {
                id: "msg-user-001".to_string(),
                role: Role::User,
                content: "What's the weather today?".to_string(),
                tool_call_id: None,
                metadata: None,
                created_at: Some(Utc::now()),
            }
        ]),
        tools: None,
        context: None,
        state: Some(HashMap::new()),
    };

    // Test event sequence
    let events = vec![
        // 1. Run started
        BaseEvent::run_started("test-thread-123".to_string(), "run-123".to_string()),
        
        // 2. Text message start
        BaseEvent::text_message_start("msg-001".to_string(), Some(Role::Assistant)),
        
        // 3. Text message content
        BaseEvent::text_message_content("msg-001".to_string(), "I'll help you check the weather.".to_string()),
        
        // 4. Text message end
        BaseEvent::text_message_end("msg-001".to_string()),
        
        // 5. Run finished
        BaseEvent::run_finished("test-thread-123".to_string(), "run-123".to_string()),
    ];

    // Test SSE encoding
    let encoder = SSEEncoder::new();
    for event in &events {
        let encoded_result = encoder.encode_event(event);
        assert!(encoded_result.is_ok());
        
        let encoded_str = SSEEncoder::encode_event_string(event).unwrap();
        assert!(!encoded_str.is_empty());
        assert!(encoded_str.starts_with("data: "));
        assert!(encoded_str.ends_with("\n\n"));
        
        // Verify JSON is valid
        let json_part = encoded_str.trim_start_matches("data: ").trim_end();
        let parsed: serde_json::Value = serde_json::from_str(json_part)
            .expect("Should produce valid JSON");
        assert!(parsed["type"].is_string());
    }
}

// Test message construction
#[wasm_bindgen_test]
fn test_message_construction() {
    // Test user message
    let user_msg = Message {
        id: "msg-1".to_string(),
        role: Role::User,
        content: "Hello, AI!".to_string(),
        tool_call_id: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };
    assert_eq!(user_msg.role, Role::User);
    assert_eq!(user_msg.content, "Hello, AI!");
    
    // Test assistant message
    let assistant_msg = Message {
        id: "msg-2".to_string(),
        role: Role::Assistant,
        content: "Hello! How can I help you today?".to_string(),
        tool_call_id: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };
    assert_eq!(assistant_msg.role, Role::Assistant);
    
    // Test system message
    let system_msg = Message {
        id: "msg-3".to_string(),
        role: Role::System,
        content: "You are a helpful assistant.".to_string(),
        tool_call_id: None,
        metadata: None,
        created_at: Some(Utc::now()),
    };
    assert_eq!(system_msg.role, Role::System);
}

// Test state management
#[wasm_bindgen_test]
fn test_state_updates() {
    let mut state: State = HashMap::new();
    
    // Add some values
    state.insert("user_preference".to_string(), json!({
        "language": "en",
        "timezone": "PST"
    }));
    
    state.insert("session_data".to_string(), json!({
        "start_time": 1234567890,
        "interactions": 1
    }));
    
    assert_eq!(state.len(), 2);
    assert!(state.contains_key("user_preference"));
    assert!(state.contains_key("session_data"));
}

// Test event creation and serialization
#[wasm_bindgen_test]
fn test_event_creation() {
    let event = BaseEvent::text_message_content("test-msg".to_string(), "Hello, world!".to_string());
    
    // Test serialization
    let json_result = serde_json::to_string(&event);
    assert!(json_result.is_ok());
    
    let json_str = json_result.unwrap();
    assert!(json_str.contains("TEXT_MESSAGE_CONTENT"));
    assert!(json_str.contains("Hello, world!"));
    assert!(json_str.contains("test-msg"));
}

// Test SSE encoding formats
#[wasm_bindgen_test]
fn test_sse_encoding_formats() {
    let encoder = SSEEncoder::new();
    let event = BaseEvent::text_message_content("msg-001".to_string(), "Test content".to_string());
    
    // Test string encoding
    let encoded_str = SSEEncoder::encode_event_string(&event).unwrap();
    assert!(encoded_str.starts_with("data: "));
    assert!(encoded_str.ends_with("\n\n"));
    assert!(encoded_str.contains("Test content"));
    
    // Test message encoding
    let message_result = encoder.encode_message("Simple message");
    assert!(message_result.is_ok());
    
    // Test comment encoding
    let comment_result = encoder.encode_comment("This is a comment");
    assert!(comment_result.is_ok());
    
    // Test ping encoding
    let ping_result = encoder.encode_ping();
    assert!(ping_result.is_ok());
}

// Test error event creation
#[wasm_bindgen_test]
fn test_error_events() {
    let error_event = BaseEvent::error("Connection failed".to_string(), Some("CONN_ERROR".to_string()));
    
    let encoded_str = SSEEncoder::encode_event_string(&error_event).unwrap();
    assert!(encoded_str.contains("ERROR"));
    assert!(encoded_str.contains("Connection failed"));
}

// Test streaming text chunks
#[wasm_bindgen_test]
fn test_streaming_text_chunks() {
    let message = "This is a longer message that would be streamed in chunks to provide a better user experience.";
    let chunks = message.split_whitespace().collect::<Vec<&str>>();
    
    let encoder = SSEEncoder::new();
    let mut accumulated = String::new();
    
    for chunk in chunks.iter() {
        let event = BaseEvent::text_message_content("stream-msg-001".to_string(), format!("{} ", chunk));
        
        let encoded_str = SSEEncoder::encode_event_string(&event).unwrap();
        assert!(encoded_str.contains(chunk));
        accumulated.push_str(chunk);
        accumulated.push(' ');
    }
    
    assert_eq!(accumulated.trim(), message);
}