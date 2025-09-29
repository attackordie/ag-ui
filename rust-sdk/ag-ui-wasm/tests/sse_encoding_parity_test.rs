//! Comprehensive SSE encoding tests matching TypeScript encoder patterns
//! Tests SSE format encoding, binary conversion, and JavaScript interop

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, SSEEncoder, RawEvent,
    TextMessageStartEvent, TextMessageContentEvent, Role,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;

wasm_bindgen_test_configure!(run_in_browser);

// ===== SSE ENCODER BASIC TESTS =====

#[wasm_bindgen_test]
fn test_sse_encoder_creation() {
    // Test SSE encoder initialization (matching Python encoder initialization test)
    let encoder = SSEEncoder::new();

    // Should create successfully
    // No specific assertions needed, just verifies constructor works
}

#[wasm_bindgen_test]
fn test_sse_encode_event_string() {
    // Test encoding event to SSE string format (matching Python encode_sse_method test)
    let event = BaseEvent {
        event_type: EventType::TextMessageContent,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::TextMessageContent(TextMessageContentEvent {
            message_id: "msg_123".to_string(),
            delta: "Hello, world!".to_string(),
        }),
    };

    let sse_string = SSEEncoder::encode_event_string(&event).unwrap();

    // Verify SSE format structure
    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));

    // Extract and verify JSON content
    let json_part = &sse_string[6..sse_string.len()-2]; // Remove "data: " and "\n\n"
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    assert_eq!(parsed["type"], "TEXT_MESSAGE_CONTENT");
    assert_eq!(parsed["message_id"], "msg_123");
    assert_eq!(parsed["delta"], "Hello, world!");
}

#[wasm_bindgen_test]
fn test_sse_encode_multiple_events() {
    // Test encoding multiple events (batch encoding)
    let events = vec![
        BaseEvent::run_started("thread_123".to_string(), "run_456".to_string()),
        BaseEvent::text_message_start("msg_001".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg_001".to_string(), "Hello!".to_string()),
        BaseEvent::text_message_end("msg_001".to_string()),
        BaseEvent::run_finished("thread_123".to_string(), "run_456".to_string()),
    ];

    let sse_string = SSEEncoder::encode_events_string(&events).unwrap();

    // Should contain all events in SSE format
    let event_count = sse_string.matches("data: ").count();
    assert_eq!(event_count, 5);

    // Should contain all expected event types
    assert!(sse_string.contains("RUN_STARTED"));
    assert!(sse_string.contains("TEXT_MESSAGE_START"));
    assert!(sse_string.contains("TEXT_MESSAGE_CONTENT"));
    assert!(sse_string.contains("TEXT_MESSAGE_END"));
    assert!(sse_string.contains("RUN_FINISHED"));

    // Should have proper SSE formatting
    assert!(sse_string.ends_with("\n\n"));
}

#[wasm_bindgen_test]
fn test_sse_encode_message() {
    // Test encoding plain message (matching Python/TypeScript message encoding)
    let encoder = SSEEncoder::new();
    let message = "Plain text message";

    let result = encoder.encode_message(message).unwrap();

    // Convert Uint8Array back to string for verification
    let bytes: Vec<u8> = result.to_vec();
    let sse_string = String::from_utf8(bytes).unwrap();

    assert_eq!(sse_string, "data: Plain text message\n\n");
}

#[wasm_bindgen_test]
fn test_sse_encode_comment() {
    // Test encoding SSE comment
    let encoder = SSEEncoder::new();
    let comment = "This is a comment";

    let result = encoder.encode_comment(comment).unwrap();

    // Convert Uint8Array back to string for verification
    let bytes: Vec<u8> = result.to_vec();
    let sse_string = String::from_utf8(bytes).unwrap();

    assert_eq!(sse_string, ": This is a comment\n");
}

#[wasm_bindgen_test]
fn test_sse_encode_ping() {
    // Test encoding SSE ping (keep-alive)
    let encoder = SSEEncoder::new();

    let result = encoder.encode_ping().unwrap();

    // Convert Uint8Array back to string for verification
    let bytes: Vec<u8> = result.to_vec();
    let sse_string = String::from_utf8(bytes).unwrap();

    assert_eq!(sse_string, ": ping\n\n");
}

// ===== BINARY ENCODING TESTS =====

#[wasm_bindgen_test]
fn test_sse_binary_encoding() {
    // Test binary encoding functionality (similar to TypeScript encodeBinary tests)
    let event = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg123".to_string(),
            role: Some(Role::Assistant),
        }),
    };

    let encoder = SSEEncoder::new();
    let result = encoder.encode_event(&event).unwrap();

    // Verify it's a Uint8Array
    assert!(result.length() > 0);

    // Convert back to string and verify SSE format
    let bytes: Vec<u8> = result.to_vec();
    let sse_string = String::from_utf8(bytes).unwrap();

    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));
    assert!(sse_string.contains("TEXT_MESSAGE_START"));
    assert!(sse_string.contains("msg123"));
}

#[wasm_bindgen_test]
fn test_to_uint8_array_conversion() {
    // Test converting SSE string to Uint8Array
    let sse_data = "data: {\"test\": \"value\"}\n\n";

    let result = SSEEncoder::to_uint8_array(sse_data).unwrap();

    // Verify conversion
    assert!(result.length() > 0);
    assert_eq!(result.length() as usize, sse_data.len());

    // Convert back and verify content
    let bytes: Vec<u8> = result.to_vec();
    let converted_string = String::from_utf8(bytes).unwrap();
    assert_eq!(converted_string, sse_data);
}

// ===== COMPLEX DATA ENCODING TESTS =====

#[wasm_bindgen_test]
fn test_sse_encode_complex_event() {
    // Test encoding complex event with nested data
    let complex_event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({
            "source": "external_system",
            "metadata": {
                "version": "1.0.0",
                "environment": "test"
            }
        })),
        data: EventData::Raw(RawEvent { event: json!({
            "userProfile": {
                "name": "John Doe",
                "preferences": {
                    "theme": "dark",
                    "notifications": true
                }
            },
            "analytics": {
                "events": [
                    {"type": "click", "element": "button"},
                    {"type": "scroll", "position": 100}
                ]
            }
        }) }),
    };

    let sse_string = SSEEncoder::encode_event_string(&complex_event).unwrap();

    // Verify SSE format
    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));

    // Extract and verify JSON content
    let json_part = &sse_string[6..sse_string.len()-2];
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    assert_eq!(parsed["type"], "RAW");
    assert_eq!(parsed["event"]["userProfile"]["name"], "John Doe");
    assert_eq!(parsed["event"]["analytics"]["events"][0]["type"], "click");
    assert_eq!(parsed["raw_event"]["source"], "external_system");
}

// ===== SPECIAL CHARACTER AND UNICODE TESTS =====

#[wasm_bindgen_test]
fn test_sse_encode_unicode_content() {
    // Test encoding Unicode and special characters (matching encoder Unicode tests)
    let unicode_text = "Hello 你好 こんにちは 안녕하세요 👋 🌍 \\n\\t\"'/<>";

    let event = BaseEvent::text_message_content(
        "msg_unicode".to_string(),
        unicode_text.to_string()
    );

    let sse_string = SSEEncoder::encode_event_string(&event).unwrap();

    // Verify Unicode is preserved in SSE format
    assert!(sse_string.contains("你好"));
    assert!(sse_string.contains("こんにちは"));
    assert!(sse_string.contains("👋"));
    assert!(sse_string.contains("🌍"));

    // Test binary encoding preserves Unicode
    let encoder = SSEEncoder::new();
    let binary_result = encoder.encode_event(&event).unwrap();
    let bytes: Vec<u8> = binary_result.to_vec();
    let decoded_string = String::from_utf8(bytes).unwrap();

    assert!(decoded_string.contains("你好"));
    assert!(decoded_string.contains("👋"));
}

#[wasm_bindgen_test]
fn test_sse_encode_special_characters_in_message() {
    // Test special characters in plain message encoding
    let encoder = SSEEncoder::new();
    let special_message = "Message with \"quotes\", 'apostrophes', \n newlines, \t tabs, and \\ backslashes";

    let result = encoder.encode_message(special_message).unwrap();
    let bytes: Vec<u8> = result.to_vec();
    let sse_string = String::from_utf8(bytes).unwrap();

    // Should properly escape or preserve special characters
    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));
    assert!(sse_string.contains("quotes"));
    assert!(sse_string.contains("apostrophes"));
}

// ===== LARGE DATA TESTS =====

#[wasm_bindgen_test]
fn test_sse_encode_large_event() {
    // Test encoding large event data (matching large content tests)
    let large_content = "A".repeat(10000);

    let event = BaseEvent::text_message_content(
        "msg_large".to_string(),
        large_content.clone()
    );

    let sse_string = SSEEncoder::encode_event_string(&event).unwrap();

    // Should handle large content without issues
    assert!(sse_string.len() > 10000);
    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));

    // Verify content is preserved
    assert!(sse_string.contains(&large_content));

    // Test binary encoding of large data
    let encoder = SSEEncoder::new();
    let binary_result = encoder.encode_event(&event).unwrap();
    assert!(binary_result.length() > 10000);
}

// ===== ERROR HANDLING TESTS =====

#[wasm_bindgen_test]
fn test_sse_encode_empty_content() {
    // Test encoding events with empty content
    let empty_event = BaseEvent::text_message_content(
        "msg_empty".to_string(),
        "".to_string()
    );

    let sse_string = SSEEncoder::encode_event_string(&empty_event).unwrap();

    // Should handle empty content gracefully
    assert!(sse_string.starts_with("data: "));
    assert!(sse_string.ends_with("\n\n"));

    // Extract JSON and verify
    let json_part = &sse_string[6..sse_string.len()-2];
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    if let Some(data) = parsed["data"].as_object() {
        assert_eq!(data["delta"], "");
    }
}

#[wasm_bindgen_test]
fn test_sse_encode_null_fields() {
    // Test encoding events with null/None fields (matching null exclusion tests)
    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: None, // None field
        raw_event: None, // None field
        data: EventData::Raw(RawEvent { event: json!({"test": "value"}) }),
    };

    let sse_string = SSEEncoder::encode_event_string(&event).unwrap();

    // Should exclude None fields or set them to null
    assert!(sse_string.starts_with("data: "));

    let json_part = &sse_string[6..sse_string.len()-2];
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    // Required fields should be present
    assert!(parsed["type"].is_string());
    assert!(parsed["event"].is_object()); // RawEvent has "event" field, flattened by EventData

    // None fields should be excluded when skip_serializing_if is used
    assert!(!parsed.as_object().unwrap().contains_key("timestamp"));
    assert!(!parsed.as_object().unwrap().contains_key("raw_event"));
}

// ===== ROUND TRIP TESTS =====

#[wasm_bindgen_test]
fn test_sse_round_trip_encoding() {
    // Test round-trip encoding/decoding (matching round-trip tests)
    let original_event = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({"source": "test"})),
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg_round_trip".to_string(),
            role: Some(Role::Assistant),
        }),
    };

    // Encode to SSE
    let sse_string = SSEEncoder::encode_event_string(&original_event).unwrap();

    // Extract JSON from SSE format
    let json_part = &sse_string[6..sse_string.len()-2];

    // Deserialize back to event
    let deserialized_event: BaseEvent = serde_json::from_str(json_part).unwrap();

    // Verify round-trip preservation
    assert_eq!(original_event.event_type, deserialized_event.event_type);

    if let (EventData::TextMessageStart(orig), EventData::TextMessageStart(deser)) =
        (&original_event.data, &deserialized_event.data) {
        assert_eq!(orig.message_id, deser.message_id);
        assert_eq!(orig.role, deser.role);
    }
}

// ===== BATCH PROCESSING TESTS =====

#[wasm_bindgen_test]
fn test_sse_batch_encoding() {
    // Test batch encoding of multiple events
    let events = vec![
        BaseEvent {
            event_type: EventType::Raw,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Raw(RawEvent { event: json!({"event": 1}) }),
        },
        BaseEvent {
            event_type: EventType::Raw,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Raw(RawEvent { event: json!({"event": 2}) }),
        },
        BaseEvent {
            event_type: EventType::Raw,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Raw(RawEvent { event: json!({"event": 3}) }),
        },
    ];

    let batch_sse = SSEEncoder::encode_events_string(&events).unwrap();

    // Should contain all events
    let data_count = batch_sse.matches("data: ").count();
    assert_eq!(data_count, 3);

    // Should be valid SSE format
    assert!(batch_sse.contains("\"event\":1"));
    assert!(batch_sse.contains("\"event\":2"));
    assert!(batch_sse.contains("\"event\":3"));

    // Should end with proper SSE termination
    assert!(batch_sse.ends_with("\n\n"));
}

// ===== PERFORMANCE AND EDGE CASE TESTS =====

#[wasm_bindgen_test]
fn test_sse_encode_many_events() {
    // Test encoding many events (performance test)
    let events: Vec<BaseEvent> = (0..100)
        .map(|i| BaseEvent {
            event_type: EventType::Raw,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Raw(RawEvent { event: json!({"index": i, "data": format!("event_{}", i)}) }),
        })
        .collect();

    let batch_sse = SSEEncoder::encode_events_string(&events).unwrap();

    // Should handle 100 events
    let data_count = batch_sse.matches("data: ").count();
    assert_eq!(data_count, 100);

    // Should contain first and last events
    assert!(batch_sse.contains("\"index\":0"));
    assert!(batch_sse.contains("\"index\":99"));
    assert!(batch_sse.contains("event_0"));
    assert!(batch_sse.contains("event_99"));
}

#[wasm_bindgen_test]
fn test_sse_encode_empty_event_list() {
    // Test encoding empty event list
    let events: Vec<BaseEvent> = vec![];

    let batch_sse = SSEEncoder::encode_events_string(&events).unwrap();

    // Should return empty string
    assert!(batch_sse.is_empty());
}