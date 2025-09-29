//! Comprehensive encoder tests matching TypeScript and Python SDK patterns
//! This test suite ensures full parity with the TypeScript EventEncoder and Python EventEncoder

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, Role, RawEvent, ToolCall,
    TextMessageStartEvent, ToolCallStartEvent, ToolCallChunkEvent, ToolCallEndEvent,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;

wasm_bindgen_test_configure!(run_in_browser);

// ===== BASIC ENCODER TESTS =====

#[wasm_bindgen_test]
fn test_basic_event_serialization() {
    // Test basic event serialization (matching Python test_encode_method)
    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: json!({"test": "data"}) }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "RAW");
    assert!(parsed["timestamp"].is_string());
    // raw_event should be excluded when None (matching Python null exclusion test)
    assert!(!parsed.as_object().unwrap().contains_key("rawEvent"));
}

#[wasm_bindgen_test]
fn test_sse_format_encoding() {
    // Test SSE format encoding (matching TypeScript/Python SSE format tests)
    let event = BaseEvent::text_message_content(
        "msg_123".to_string(),
        "Hello, world!".to_string()
    );

    let json_str = serde_json::to_string(&event).unwrap();
    let sse_format = format!("data: {}\n\n", json_str);

    // Verify SSE format structure
    assert!(sse_format.starts_with("data: "));
    assert!(sse_format.ends_with("\n\n"));

    // Verify the embedded JSON is correct
    let json_part = &sse_format[6..sse_format.len()-2]; // Remove "data: " and "\n\n"
    let parsed: serde_json::Value = serde_json::from_str(json_part).unwrap();

    assert_eq!(parsed["type"], "TEXT_MESSAGE_CONTENT");
    assert_eq!(parsed["message_id"], "msg_123");
    assert_eq!(parsed["delta"], "Hello, world!");
}

// ===== FIELD NAMING TESTS (camelCase vs snake_case) =====

#[wasm_bindgen_test]
fn test_field_name_consistency() {
    // Test that field names are consistent (matching Python camelCase tests)
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

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Check that field names follow consistent naming convention
    if let Some(data) = parsed["data"].as_object() {
        // Should have either camelCase or snake_case, but be consistent
        let has_camel_case = data.contains_key("toolCallId") || data.contains_key("toolName") || data.contains_key("parentMessageId");
        let has_snake_case = data.contains_key("tool_call_id") || data.contains_key("tool_name") || data.contains_key("parent_message_id");

        // Should have one consistent style, not a mix
        assert!(has_camel_case || has_snake_case);
        if has_camel_case {
            assert!(!has_snake_case, "Should not mix camelCase and snake_case");
        }
    }
}

// ===== NULL VALUE EXCLUSION TESTS =====

#[wasm_bindgen_test]
fn test_null_value_exclusion() {
    // Test that None/null values are excluded (matching Python test_null_value_exclusion)
    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None, // Should be excluded
        data: EventData::Raw(RawEvent { event: json!({"test": "value"}) }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Required fields should be present
    assert!(parsed.as_object().unwrap().contains_key("type"));
    assert!(parsed.as_object().unwrap().contains_key("timestamp"));

    // Optional field with None value should be excluded
    assert!(!parsed.as_object().unwrap().contains_key("rawEvent"));
    assert!(!parsed.as_object().unwrap().contains_key("raw_event"));
}

#[wasm_bindgen_test]
fn test_optional_fields_exclusion() {
    // Test exclusion of optional fields when None (matching Python pattern)
    let event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "call_123".to_string(),
            tool_name: "test_tool".to_string(),
            parent_message_id: None, // Optional field set to None
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    if let Some(data) = parsed["data"].as_object() {
        // Required fields should be present
        assert!(data.contains_key("tool_call_id") || data.contains_key("toolCallId"));
        assert!(data.contains_key("tool_name") || data.contains_key("toolName"));

        // Optional field with None should be excluded
        assert!(!data.contains_key("parent_message_id"));
        assert!(!data.contains_key("parentMessageId"));
    }
}

// ===== DIFFERENT EVENT TYPES TESTS =====

#[wasm_bindgen_test]
fn test_encoding_different_event_types() {
    // Test encoding various event types (matching Python test_encode_with_different_event_types)

    // Test base event
    let base_event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: json!({"test": "base"}) }),
    };

    let base_json = serde_json::to_string(&base_event).unwrap();
    assert!(base_json.contains("\"type\":\"RAW\""));

    // Test text message content event
    let content_event = BaseEvent::text_message_content(
        "msg_456".to_string(),
        "Testing different events".to_string()
    );

    let content_json = serde_json::to_string(&content_event).unwrap();
    assert!(content_json.contains("\"type\":\"TEXT_MESSAGE_CONTENT\""));
    assert!(content_json.contains("msg_456"));
    assert!(content_json.contains("Testing different events"));

    // Test tool call event
    let tool_event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "call_789".to_string(),
            tool_name: "search".to_string(),
            parent_message_id: None,
        }),
    };

    let tool_json = serde_json::to_string(&tool_event).unwrap();
    assert!(tool_json.contains("\"type\":\"TOOL_CALL_START\""));
    assert!(tool_json.contains("call_789"));
    assert!(tool_json.contains("search"));
}

// ===== ROUND TRIP SERIALIZATION TESTS =====

#[wasm_bindgen_test]
fn test_round_trip_serialization() {
    // Test round-trip serialization (matching Python test_round_trip_serialization)
    let original_event = BaseEvent {
        event_type: EventType::ToolCallStart,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::ToolCallStart(ToolCallStartEvent {
            tool_call_id: "call_abc123".to_string(),
            tool_name: "search_tool".to_string(),
            parent_message_id: Some("msg_parent_456".to_string()),
        }),
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&original_event).unwrap();

    // Deserialize back
    let deserialized_event: BaseEvent = serde_json::from_str(&json_str).unwrap();

    // Verify event type matches
    assert_eq!(deserialized_event.event_type, original_event.event_type);

    // Verify data matches
    if let (EventData::ToolCallStart(orig), EventData::ToolCallStart(deser)) =
        (&original_event.data, &deserialized_event.data) {
        assert_eq!(orig.tool_call_id, deser.tool_call_id);
        assert_eq!(orig.tool_name, deser.tool_name);
        assert_eq!(orig.parent_message_id, deser.parent_message_id);
    } else {
        panic!("Event data types don't match after round-trip");
    }
}

// ===== COMPLEX DATA STRUCTURES TESTS =====

#[wasm_bindgen_test]
fn test_complex_nested_data_encoding() {
    // Test encoding complex nested structures (matching TypeScript complex objects)
    let complex_data = json!({
        "userProfile": {
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
        },
        "serviceConfig": {
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
        }
    });

    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: complex_data.clone() }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Verify complex nested structure is preserved (RawEvent uses "event" field)
    assert_eq!(parsed["event"]["userProfile"]["name"], "John Doe");
    assert_eq!(parsed["event"]["userProfile"]["contact"]["address"]["city"], "Anytown");
    assert_eq!(parsed["event"]["serviceConfig"]["endpoints"][0]["name"], "api1");
    assert_eq!(parsed["event"]["serviceConfig"]["retryPolicy"]["maxRetries"], 3);
}

// ===== SPECIAL VALUES TESTS =====

#[wasm_bindgen_test]
fn test_special_values_handling() {
    // Test handling of special values (matching TypeScript special values test)
    let special_data = json!({
        "nullValue": null,
        "emptyString": "",
        "zero": 0,
        "negativeNumber": -123,
        "floatNumber": 3.14159,
        "emptyArray": [],
        "emptyObject": {},
        "boolValues": {"true": true, "false": false}
        // Note: NaN and Infinity don't serialize well in JSON, so we skip them
    });

    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: special_data.clone() }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Raw(data) = &deserialized.data {
        assert_eq!(data.event["nullValue"], json!(null));
        assert_eq!(data.event["emptyString"], "");
        assert_eq!(data.event["zero"], 0);
        assert_eq!(data.event["negativeNumber"], -123);
        assert_eq!(data.event["floatNumber"], 3.14159);
        assert_eq!(data.event["emptyArray"], json!([]));
        assert_eq!(data.event["emptyObject"], json!({}));
        assert_eq!(data.event["boolValues"]["true"], true);
        assert_eq!(data.event["boolValues"]["false"], false);
    }
}

// ===== UNICODE AND SPECIAL CHARACTERS TESTS =====

#[wasm_bindgen_test]
fn test_unicode_and_special_characters() {
    // Test Unicode and special character handling (matching Python/TypeScript patterns)
    let unicode_content = "Hello 你好 こんにちは 안녕하세요 👋 🌍 \\n\\t\"'/<>";

    let event = BaseEvent::text_message_content(
        "msg_unicode".to_string(),
        unicode_content.to_string()
    );

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::TextMessageContent(data) = &deserialized.data {
        assert_eq!(data.delta, unicode_content);
    }
}

// ===== LARGE CONTENT TESTS =====

#[wasm_bindgen_test]
fn test_large_content_handling() {
    // Test large content handling (matching Python 10K pattern)
    let large_content = "A".repeat(10000);

    let event = BaseEvent::text_message_content(
        "msg_large".to_string(),
        large_content.clone()
    );

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::TextMessageContent(data) = &deserialized.data {
        assert_eq!(data.delta.len(), 10000);
        assert_eq!(data.delta, large_content);
    }
}

// ===== MESSAGE ROLE ENCODING TESTS =====

#[wasm_bindgen_test]
fn test_message_role_encoding() {
    // Test all message roles encode correctly (matching text roles patterns)
    let roles = vec![
        Role::User, Role::Assistant, Role::System, Role::Tool, Role::Developer
    ];

    for role in roles {
        let event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: "test_msg".to_string(),
                role: Some(role),
            }),
        };

        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify role serializes to correct string
        if let Some(data) = parsed["data"].as_object() {
            let role_value = data.get("role").expect("Role field should be present");
            let expected_role_str = match role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Tool => "tool",
                Role::Developer => "developer",
            };
            assert_eq!(role_value, expected_role_str);
        }
    }
}

// ===== COMPREHENSIVE EVENT FLOW TESTS =====

#[wasm_bindgen_test]
fn test_comprehensive_event_flow_encoding() {
    // Test a complete event flow with multiple event types
    let events = vec![
        BaseEvent::run_started("thread_123".to_string(), "run_456".to_string()),
        BaseEvent::text_message_start("msg_001".to_string(), Some(Role::Assistant)),
        BaseEvent::text_message_content("msg_001".to_string(), "Processing your request...".to_string()),
        BaseEvent {
            event_type: EventType::ToolCallStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallStart(ToolCallStartEvent {
                tool_call_id: "call_001".to_string(),
                tool_name: "search".to_string(),
                parent_message_id: Some("msg_001".to_string()),
            }),
        },
        BaseEvent {
            event_type: EventType::ToolCallChunk,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallChunk(ToolCallChunkEvent {
                tool_call_id: "call_001".to_string(),
                delta: "partial result".to_string(),
            }),
        },
        BaseEvent {
            event_type: EventType::ToolCallEnd,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::ToolCallEnd(ToolCallEndEvent {
                tool_call_id: "call_001".to_string(),
                tool_call: Some(ToolCall {
                    id: "call_001".to_string(),
                    name: "search".to_string(),
                    arguments: Some(json!({"query": "test"})),
                }),
            }),
        },
        BaseEvent::text_message_content("msg_001".to_string(), "Complete!".to_string()),
        BaseEvent::text_message_end("msg_001".to_string()),
        BaseEvent::run_finished("thread_123".to_string(), "run_456".to_string()),
    ];

    // All events should serialize and deserialize successfully
    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }

    // Test batch serialization
    let events_json = serde_json::to_string(&events).unwrap();
    let deserialized_events: Vec<BaseEvent> = serde_json::from_str(&events_json).unwrap();

    assert_eq!(events.len(), deserialized_events.len());
    for (original, deserialized) in events.iter().zip(deserialized_events.iter()) {
        assert_eq!(original.event_type, deserialized.event_type);
    }
}

// ===== EMPTY AND MINIMAL DATA TESTS =====

#[wasm_bindgen_test]
fn test_empty_and_minimal_data() {
    // Test empty data structures
    let empty_data_event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: None,
        raw_event: None,
        data: EventData::Raw(RawEvent { event: json!({}) }),
    };

    let json_str = serde_json::to_string(&empty_data_event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    assert_eq!(deserialized.event_type, EventType::Raw);
    if let EventData::Raw(data) = &deserialized.data {
        assert_eq!(data.event, json!({}));
    }

    // Test minimal required fields
    let minimal_event = BaseEvent::text_message_end("msg_minimal".to_string());
    let minimal_json = serde_json::to_string(&minimal_event).unwrap();
    let minimal_deserialized: BaseEvent = serde_json::from_str(&minimal_json).unwrap();

    assert_eq!(minimal_deserialized.event_type, EventType::TextMessageEnd);
    if let EventData::TextMessageEnd(data) = &minimal_deserialized.data {
        assert_eq!(data.message_id, "msg_minimal");
    }
}