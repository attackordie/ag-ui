//! Simple SSE encoder test that works with the actual Rust SDK implementation

use ag_ui_wasm::{BaseEvent, SSEEncoder, Role};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Test SSE encoder creation
#[wasm_bindgen_test]
fn test_sse_encoder_creation() {
    let encoder = SSEEncoder::new();
    
    // Test basic methods exist and don't panic
    let result = encoder.encode_message("test message");
    assert!(result.is_ok());
    
    let comment_result = encoder.encode_comment("test comment");
    assert!(result.is_ok());
    
    let ping_result = encoder.encode_ping();
    assert!(ping_result.is_ok());
}

// Test event string encoding (static method)
#[wasm_bindgen_test]
fn test_event_string_encoding() {
    let event = BaseEvent::text_message_content("msg_123".to_string(), "Hello, world!".to_string());
    
    let encoded = SSEEncoder::encode_event_string(&event);
    assert!(encoded.is_ok());
    
    let encoded_str = encoded.unwrap();
    assert!(encoded_str.starts_with("data: "));
    assert!(encoded_str.ends_with("\n\n"));
    assert!(encoded_str.contains("TEXT_MESSAGE_CONTENT"));
    assert!(encoded_str.contains("msg_123"));
    assert!(encoded_str.contains("Hello, world!"));
}

// Test multiple events encoding
#[wasm_bindgen_test]
fn test_multiple_events_encoding() {
    let events = vec![
        BaseEvent::run_started("thread_123".to_string(), "run_456".to_string()),
        BaseEvent::text_message_start("msg_001".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg_001".to_string(), "Hello!".to_string()),
        BaseEvent::text_message_end("msg_001".to_string()),
        BaseEvent::run_finished("thread_123".to_string(), "run_456".to_string()),
    ];
    
    let encoded = SSEEncoder::encode_events_string(&events);
    assert!(encoded.is_ok());
    
    let encoded_str = encoded.unwrap();
    assert!(encoded_str.contains("RUN_STARTED"));
    assert!(encoded_str.contains("TEXT_MESSAGE_START"));
    assert!(encoded_str.contains("TEXT_MESSAGE_CONTENT"));
    assert!(encoded_str.contains("TEXT_MESSAGE_END"));
    assert!(encoded_str.contains("RUN_FINISHED"));
    
    // Count events (each ends with \n\n)
    let event_count = encoded_str.matches("\n\n").count();
    assert_eq!(event_count, 5);
}

// Test Unicode encoding
#[wasm_bindgen_test]
fn test_unicode_event_encoding() {
    let unicode_text = "Hello 你好 こんにちは 안녕하세요 👋 🌍";
    let event = BaseEvent::text_message_content("msg_unicode".to_string(), unicode_text.to_string());
    
    let encoded = SSEEncoder::encode_event_string(&event);
    assert!(encoded.is_ok());
    
    let encoded_str = encoded.unwrap();
    assert!(encoded_str.contains(unicode_text));
}

// Test empty content encoding
#[wasm_bindgen_test]
fn test_empty_content_encoding() {
    let event = BaseEvent::text_message_content("msg_empty".to_string(), "".to_string());
    
    let encoded = SSEEncoder::encode_event_string(&event);
    assert!(encoded.is_ok());
    
    let encoded_str = encoded.unwrap();
    assert!(encoded_str.contains("\"delta\":\"\""));
}

// Test large content encoding
#[wasm_bindgen_test]
fn test_large_content_encoding() {
    let large_content = "A".repeat(1000);
    let event = BaseEvent::text_message_content("msg_large".to_string(), large_content.clone());
    
    let encoded = SSEEncoder::encode_event_string(&event);
    assert!(encoded.is_ok());
    
    let encoded_str = encoded.unwrap();
    assert!(encoded_str.contains(&large_content));
}

// Test error event encoding
#[wasm_bindgen_test]
fn test_error_event_encoding() {
    let event = BaseEvent::error("Something went wrong".to_string(), Some("ERR_001".to_string()));
    
    let encoded = SSEEncoder::encode_event_string(&event);
    assert!(encoded.is_ok());
    
    let encoded_str = encoded.unwrap();
    assert!(encoded_str.contains("ERROR"));
    assert!(encoded_str.contains("Something went wrong"));
    assert!(encoded_str.contains("ERR_001"));
}