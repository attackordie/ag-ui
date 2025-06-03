//! Test suite for the Web and Node.js/V8 environments.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use ag_ui_wasm::*;

#[wasm_bindgen_test]
fn test_version() {
    assert!(!VERSION.is_empty());
}

#[wasm_bindgen_test]
fn test_message_creation() {
    let message = Message::new(Role::User, "Hello, world!".to_string());
    assert_eq!(message.role, Role::User);
    assert_eq!(message.content, "Hello, world!");
    assert!(!message.id.is_empty());
}

#[wasm_bindgen_test]
fn test_run_agent_input() {
    let input = RunAgentInput::new("thread-123".to_string(), "run-456".to_string());
    assert_eq!(input.thread_id, "thread-123");
    assert_eq!(input.run_id, "run-456");
    assert!(input.messages.is_none());
}

#[wasm_bindgen_test]
fn test_base_event_creation() {
    let event = BaseEvent::run_started("thread-1".to_string(), "run-1".to_string());
    assert_eq!(event.event_type, EventType::RunStarted);
}

#[wasm_bindgen_test]
fn test_sse_encoder() {
    let encoder = SSEEncoder::new().unwrap();
    let data = encoder.encode_raw("test data").unwrap();
    assert!(data.length() > 0);
} 