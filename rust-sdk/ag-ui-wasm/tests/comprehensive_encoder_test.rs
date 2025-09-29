//! Additional comprehensive encoder tests to complement existing encoder tests

use ag_ui_wasm::{
    BaseEvent, SSEEncoder, Role, EventType, EventData,
    TextMessageStartEvent, StateSnapshotEvent,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// ===== ADDITIONAL ENCODER EDGE CASE TESTS =====

#[wasm_bindgen_test]
fn test_encoder_with_complex_state_snapshot() {
    // Test encoding complex state snapshot similar to TypeScript/Python patterns
    let mut state = HashMap::new();
    state.insert("session".to_string(), json!({
        "user": {
            "id": "user_123",
            "preferences": {
                "theme": "dark",
                "notifications": true,
                "filters": ["news", "social", "tech"]
            }
        },
        "stats": {
            "messages": 42,
            "interactions": {
                "clicks": 18,
                "searches": 7
            }
        }
    }));
    state.insert("active_tools".to_string(), json!(["search", "calculator", "weather"]));
    state.insert("settings".to_string(), json!({
        "language": "en",
        "timezone": "UTC-5"
    }));

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };

    let result = SSEEncoder::encode_event_string(&event);
    assert!(result.is_ok());

    let encoded_str = result.unwrap();
    assert!(encoded_str.contains("STATE_SNAPSHOT"));
    assert!(encoded_str.contains("user_123"));
    assert!(encoded_str.contains("dark"));
    assert!(encoded_str.contains("calculator"));
    assert!(encoded_str.contains("UTC-5"));

    // Verify complex nested structure preservation
    let json_part = encoded_str.trim_start_matches("data: ").trim_end_matches("\n\n");
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    assert_eq!(parsed["state"]["session"]["user"]["id"], "user_123");
    assert_eq!(parsed["state"]["session"]["user"]["preferences"]["theme"], "dark");
    assert_eq!(parsed["state"]["session"]["stats"]["interactions"]["searches"], 7);
    assert_eq!(parsed["state"]["active_tools"][1], "calculator");
    assert_eq!(parsed["state"]["settings"]["timezone"], "UTC-5");
}

#[wasm_bindgen_test]
fn test_encoder_with_raw_event_metadata() {
    // Test encoding events with complex raw_event metadata
    let event = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({
            "original": "event data",
            "source": "test system",
            "metadata": {
                "version": "1.0",
                "priority": "high",
                "nested_data": {
                    "key1": "value1",
                    "key2": [1, 2, 3]
                }
            }
        })),
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg_with_metadata".to_string(),
            role: Some(Role::Assistant),
        }),
    };

    let result = SSEEncoder::encode_event_string(&event);
    assert!(result.is_ok());

    let encoded_str = result.unwrap();
    assert!(encoded_str.contains("TEXT_MESSAGE_START"));
    assert!(encoded_str.contains("msg_with_metadata"));

    // Verify raw_event metadata is included
    let json_part = encoded_str.trim_start_matches("data: ").trim_end_matches("\n\n");
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    assert!(parsed["raw_event"].is_object());
    assert_eq!(parsed["raw_event"]["original"], "event data");
    assert_eq!(parsed["raw_event"]["source"], "test system");
    assert_eq!(parsed["raw_event"]["metadata"]["version"], "1.0");
    assert_eq!(parsed["raw_event"]["metadata"]["priority"], "high");
    assert_eq!(parsed["raw_event"]["metadata"]["nested_data"]["key1"], "value1");
    assert_eq!(parsed["raw_event"]["metadata"]["nested_data"]["key2"][1], 2);
}

#[wasm_bindgen_test]
fn test_encoder_performance_with_many_events() {
    // Create a large number of events to test performance
    let mut events = Vec::new();

    for i in 0..100 {
        events.push(BaseEvent::text_message_content(
            format!("msg_{}", i),
            format!("Content for message {}", i)
        ));
    }

    let start_time = web_sys::js_sys::Date::now();
    let result = SSEEncoder::encode_events_string(&events);
    let end_time = web_sys::js_sys::Date::now();

    assert!(result.is_ok());

    let encoded_str = result.unwrap();
    let event_count = encoded_str.matches("data: ").count();
    assert_eq!(event_count, 100);

    // Should complete reasonably quickly (less than 1 second)
    let duration = end_time - start_time;
    assert!(duration < 1000.0, "Encoding took too long: {}ms", duration);
}

#[wasm_bindgen_test]
fn test_encoder_with_special_json_characters() {
    // Test encoding events with complex JSON structures that might cause issues
    let complex_data = json!({
        "nested": {
            "array": [1, 2, 3, {"inner": "value"}],
            "object": {
                "key": "value with \"quotes\" and \\backslashes\\",
                "special_chars": "Special: 你好 €€€ 🚀",
                "numbers": [1.5, -42, 0, 999999999],
                "boolean": true,
                "null_value": null
            }
        },
        "emoji_array": ["😀", "😃", "😄", "😁"],
        "unicode_key_测试": "unicode value",
        "json_string": "{\"inner_json\": \"value\"}"
    });

    let mut state = HashMap::new();
    state.insert("complex_data".to_string(), complex_data);

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state }),
    };

    let result = SSEEncoder::encode_event_string(&event);
    assert!(result.is_ok());

    let encoded_str = result.unwrap();

    // Verify complex content is preserved
    assert!(encoded_str.contains("你好"));
    assert!(encoded_str.contains("🚀"));
    assert!(encoded_str.contains("😀"));
    assert!(encoded_str.contains("inner_json"));

    // Verify JSON is valid
    let json_part = encoded_str.trim_start_matches("data: ").trim_end_matches("\n\n");
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    assert_eq!(parsed["state"]["complex_data"]["nested"]["array"][3]["inner"], "value");
    assert_eq!(parsed["state"]["complex_data"]["emoji_array"][0], "😀");
}

#[wasm_bindgen_test]
fn test_encoder_timestamp_serialization() {
    // Test that timestamps are properly serialized across different events
    let timestamp = Utc::now();

    let event_with_timestamp = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: Some(timestamp),
        raw_event: None,
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg_timestamp".to_string(),
            role: Some(Role::User),
        }),
    };

    let event_without_timestamp = BaseEvent {
        event_type: EventType::TextMessageStart,
        timestamp: None,
        raw_event: None,
        data: EventData::TextMessageStart(TextMessageStartEvent {
            message_id: "msg_no_timestamp".to_string(),
            role: Some(Role::User),
        }),
    };

    // Test event with timestamp
    let result1 = SSEEncoder::encode_event_string(&event_with_timestamp);
    assert!(result1.is_ok());

    let encoded1 = result1.unwrap();
    let json1: serde_json::Value = serde_json::from_str(
        encoded1.trim_start_matches("data: ").trim_end_matches("\n\n")
    ).unwrap();

    assert!(json1["timestamp"].is_number());

    // Test event without timestamp
    let result2 = SSEEncoder::encode_event_string(&event_without_timestamp);
    assert!(result2.is_ok());

    let encoded2 = result2.unwrap();
    let json2: serde_json::Value = serde_json::from_str(
        encoded2.trim_start_matches("data: ").trim_end_matches("\n\n")
    ).unwrap();

    // Timestamp should be null or not present
    assert!(json2["timestamp"].is_null() || !json2.as_object().unwrap().contains_key("timestamp"));
}

#[wasm_bindgen_test]
fn test_encoder_error_handling() {
    // Test that encoder handles various edge cases gracefully
    let empty_message_event = BaseEvent::text_message_content("".to_string(), "".to_string());
    let result = SSEEncoder::encode_event_string(&empty_message_event);
    assert!(result.is_ok());

    let encoded = result.unwrap();
    assert!(encoded.contains("TEXT_MESSAGE_CONTENT"));
    assert!(encoded.contains(r#""message_id":"""#));
    assert!(encoded.contains(r#""delta":"""#));
}

#[wasm_bindgen_test]
fn test_encoder_role_serialization() {
    // Test that all roles serialize correctly in events
    let roles = vec![Role::Developer, Role::System, Role::Assistant, Role::User, Role::Tool];

    for role in roles {
        let event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: format!("msg_{:?}", role),
                role: Some(role),
            }),
        };

        let result = SSEEncoder::encode_event_string(&event);
        assert!(result.is_ok());

        let encoded = result.unwrap();
        let json: serde_json::Value = serde_json::from_str(
            encoded.trim_start_matches("data: ").trim_end_matches("\n\n")
        ).unwrap();

        let role_str = match role {
            Role::Developer => "developer",
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
            Role::Tool => "tool",
        };

        assert_eq!(json["role"], role_str);
    }
}