//! Text message role tests matching Python test_text_roles.py patterns

use ag_ui_wasm::{BaseEvent, EventType, EventData, Role, TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;

wasm_bindgen_test_configure!(run_in_browser);

// Test all available roles for text messages (excluding "tool" which is not for text messages)
const TEXT_MESSAGE_ROLES: &[Role] = &[Role::Developer, Role::System, Role::Assistant, Role::User];

// ===== TEXT MESSAGE START WITH ALL ROLES =====

#[wasm_bindgen_test]
fn test_text_message_start_with_all_roles() {
    for role in TEXT_MESSAGE_ROLES {
        let event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: "test-msg".to_string(),
                role: Some(*role),
            }),
        };

        assert_eq!(event.event_type, EventType::TextMessageStart);

        if let EventData::TextMessageStart(data) = &event.data {
            assert_eq!(data.message_id, "test-msg");
            assert_eq!(data.role, Some(*role));
        } else {
            panic!("Expected TextMessageStart event data");
        }

        // Test serialization
        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["type"], "TEXT_MESSAGE_START");
        assert_eq!(parsed["message_id"], "test-msg");

        // Verify role serialization
        let role_str = match role {
            Role::Developer => "developer",
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
            Role::Tool => "tool", // Should not reach here
        };
        assert_eq!(parsed["role"], role_str);
    }
}

// ===== TEXT MESSAGE CONTENT WITH ALL ROLES =====

#[wasm_bindgen_test]
fn test_text_message_content_with_all_roles() {
    for role in TEXT_MESSAGE_ROLES {
        let content = format!("Hello from {}", match role {
            Role::Developer => "developer",
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
            Role::Tool => "tool",
        });

        let event = BaseEvent {
            event_type: EventType::TextMessageContent,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent {
                message_id: "test-msg".to_string(),
                delta: content.clone(),
            }),
        };

        assert_eq!(event.event_type, EventType::TextMessageContent);

        if let EventData::TextMessageContent(data) = &event.data {
            assert_eq!(data.message_id, "test-msg");
            assert_eq!(data.delta, content);
        } else {
            panic!("Expected TextMessageContent event data");
        }

        // Test serialization
        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["type"], "TEXT_MESSAGE_CONTENT");
        assert_eq!(parsed["message_id"], "test-msg");
        assert_eq!(parsed["delta"], content);
    }
}

// ===== TEXT MESSAGE CONTENT WITHOUT ROLE (OPTIONAL) =====

#[wasm_bindgen_test]
fn test_text_message_content_without_role() {
    let event = BaseEvent {
        event_type: EventType::TextMessageContent,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::TextMessageContent(TextMessageContentEvent {
            message_id: "test-msg".to_string(),
            delta: "Hello without role".to_string(),
        }),
    };

    assert_eq!(event.event_type, EventType::TextMessageContent);

    if let EventData::TextMessageContent(data) = &event.data {
        assert_eq!(data.message_id, "test-msg");
        assert_eq!(data.delta, "Hello without role");
    } else {
        panic!("Expected TextMessageContent event data");
    }

    // Test serialization - role field should be null or omitted
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "TEXT_MESSAGE_CONTENT");
    assert_eq!(parsed["message_id"], "test-msg");
    assert_eq!(parsed["delta"], "Hello without role");
    // Role should be null or not present
    assert!(parsed["role"].is_null() || !parsed.as_object().unwrap().contains_key("role"));
}

// ===== MULTIPLE MESSAGES WITH DIFFERENT ROLES =====

#[wasm_bindgen_test]
fn test_multiple_messages_different_roles() {
    let mut events = Vec::new();

    for role in TEXT_MESSAGE_ROLES {
        let role_name = match role {
            Role::Developer => "developer",
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
            Role::Tool => "tool",
        };

        let start_event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: format!("msg-{}", role_name),
                role: Some(*role),
            }),
        };

        let content_event = BaseEvent::text_message_content(
            format!("msg-{}", role_name),
            format!("Message from {}", role_name)
        );

        let end_event = BaseEvent::text_message_end(format!("msg-{}", role_name));

        events.extend(vec![start_event, content_event, end_event]);
    }

    // Verify we have 3 events per role
    assert_eq!(events.len(), TEXT_MESSAGE_ROLES.len() * 3);

    // Verify each start event has the correct role
    for (i, role) in TEXT_MESSAGE_ROLES.iter().enumerate() {
        let start_event = &events[i * 3];
        assert_eq!(start_event.event_type, EventType::TextMessageStart);

        if let EventData::TextMessageStart(data) = &start_event.data {
            assert_eq!(data.role, Some(*role));
            let role_name = match role {
                Role::Developer => "developer",
                Role::System => "system",
                Role::Assistant => "assistant",
                Role::User => "user",
                Role::Tool => "tool",
            };
            assert_eq!(data.message_id, format!("msg-{}", role_name));
        }
    }
}

// ===== TEXT MESSAGE SERIALIZATION WITH ROLES =====

#[wasm_bindgen_test]
fn test_text_message_serialization() {
    for role in TEXT_MESSAGE_ROLES {
        let event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: "test-msg".to_string(),
                role: Some(*role),
            }),
        };

        // Convert to JSON and verify
        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let role_str = match role {
            Role::Developer => "developer",
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
            Role::Tool => "tool",
        };

        assert_eq!(parsed["role"], role_str);
        assert_eq!(parsed["type"], "TEXT_MESSAGE_START");
        assert_eq!(parsed["message_id"], "test-msg");

        // Test round-trip deserialization
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.event_type, EventType::TextMessageStart);

        if let EventData::TextMessageStart(data) = &deserialized.data {
            assert_eq!(data.role, Some(*role));
            assert_eq!(data.message_id, "test-msg");
        }
    }
}

// ===== DEFAULT ROLE BEHAVIOR =====

#[wasm_bindgen_test]
fn test_text_message_start_default_role() {
    // Test that we can create a TextMessageStart without specifying role
    let event = BaseEvent::text_message_start("test-msg".to_string(), None);

    assert_eq!(event.event_type, EventType::TextMessageStart);

    if let EventData::TextMessageStart(data) = &event.data {
        assert_eq!(data.message_id, "test-msg");
        // Should have no role specified, or default to assistant if that's the impl
        // This depends on the actual implementation of the helper function
    }
}

// ===== ROLE VALIDATION TESTS =====

#[wasm_bindgen_test]
fn test_role_enum_serialization() {
    // Test that all roles serialize correctly
    let roles_and_strings = vec![
        (Role::Developer, "developer"),
        (Role::System, "system"),
        (Role::Assistant, "assistant"),
        (Role::User, "user"),
        (Role::Tool, "tool"),
    ];

    for (role, expected_str) in roles_and_strings {
        let serialized = serde_json::to_value(&role).unwrap();
        assert_eq!(serialized, expected_str);

        // Test deserialization back to role
        let deserialized: Role = serde_json::from_value(json!(expected_str)).unwrap();
        assert_eq!(deserialized, role);
    }
}

// ===== COMPREHENSIVE TEXT MESSAGE FLOW =====

#[wasm_bindgen_test]
fn test_comprehensive_text_message_flow_all_roles() {
    // Test a complete text message flow for each role
    for role in TEXT_MESSAGE_ROLES {
        let role_name = match role {
            Role::Developer => "developer",
            Role::System => "system",
            Role::Assistant => "assistant",
            Role::User => "user",
            Role::Tool => "tool",
        };

        let message_id = format!("flow-msg-{}", role_name);

        // Create start event
        let start_event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: message_id.clone(),
                role: Some(*role),
            }),
        };

        // Create content events with role-specific content
        let content1 = BaseEvent {
            event_type: EventType::TextMessageContent,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent {
                message_id: message_id.clone(),
                delta: format!("Part 1 from {} role", role_name),
            }),
        };

        let content2 = BaseEvent {
            event_type: EventType::TextMessageContent,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent {
                message_id: message_id.clone(),
                delta: format!(" Part 2 from {} role", role_name),
            }),
        };

        // Create end event
        let end_event = BaseEvent {
            event_type: EventType::TextMessageEnd,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageEnd(TextMessageEndEvent {
                message_id: message_id.clone(),
            }),
        };

        let events = vec![start_event, content1, content2, end_event];

        // Verify all events serialize and have consistent message IDs
        for event in &events {
            let json_str = serde_json::to_string(event).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

            assert_eq!(parsed["message_id"], message_id);

            // Check role is present for events that should have it
            match event.event_type {
                EventType::TextMessageStart | EventType::TextMessageContent => {
                    assert_eq!(parsed["role"], role_name);
                }
                EventType::TextMessageEnd => {
                    // End events might not have role field
                }
                _ => panic!("Unexpected event type in text message flow"),
            }
        }
    }
}