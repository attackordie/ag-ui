//! Comprehensive raw and custom events tests matching TypeScript and Python SDK patterns
//! Tests RawEvent and CustomEvent with complex nested data and edge cases

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, RawEvent, CustomEvent,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;

wasm_bindgen_test_configure!(run_in_browser);

// ===== RAW EVENT TESTS =====

#[wasm_bindgen_test]
fn test_raw_event_basic() {
    // Test basic RawEvent (matching TypeScript round-trip test)
    let raw_data = json!({
        "type": "user_action",
        "action": "button_click",
        "elementId": "submit-btn",
        "timestamp": 1676480210000i64
    });

    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: raw_data.clone() }),
    };

    assert_eq!(event.event_type, EventType::Raw);

    if let EventData::Raw(data) = &event.data {
        assert_eq!(data.event["type"], "user_action");
        assert_eq!(data.event["action"], "button_click");
        assert_eq!(data.event["elementId"], "submit-btn");
        assert_eq!(data.event["timestamp"], 1676480210000i64);
    } else {
        panic!("Expected Raw event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "RAW");
    assert_eq!(parsed["event"]["type"], "user_action");
    assert_eq!(parsed["event"]["action"], "button_click");
}

#[wasm_bindgen_test]
fn test_raw_event_with_source() {
    // Test RawEvent with source field
    let raw_data = json!({
        "eventId": "evt_123",
        "payload": "test_data"
    });

    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: raw_data.clone() }),
    };

    // Test round-trip serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    assert_eq!(deserialized.event_type, EventType::Raw);
    if let EventData::Raw(data) = &deserialized.data {
        assert_eq!(data.event["eventId"], "evt_123");
        assert_eq!(data.event["payload"], "test_data");
    }
}

#[wasm_bindgen_test]
fn test_raw_event_complex_nested_data() {
    // Test RawEvent with complex nested data (matching TypeScript complex nested test)
    let complex_data = json!({
        "type": "analytics_event",
        "session": {
            "id": "sess-12345",
            "user": {
                "id": "user-456",
                "attributes": {
                    "plan": "premium",
                    "signupDate": "2023-01-15",
                    "preferences": ["feature1", "feature2"]
                }
            },
            "actions": [
                {
                    "type": "page_view",
                    "path": "/home",
                    "timestamp": 1676480210000i64
                },
                {
                    "type": "button_click",
                    "elementId": "cta-1",
                    "timestamp": 1676480215000i64
                },
                {
                    "type": "form_submit",
                    "formId": "signup",
                    "timestamp": 1676480230000i64,
                    "data": {
                        "email": "user@example.com"
                    }
                }
            ]
        },
        "metadata": {
            "source": "web",
            "version": "1.2.3",
            "environment": "production"
        }
    });

    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: complex_data.clone() }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Raw(data) = &deserialized.data {
        // Verify complex nested structure is preserved
        assert_eq!(data.event["type"], "analytics_event");
        assert_eq!(data.event["session"]["id"], "sess-12345");
        assert_eq!(data.event["session"]["user"]["id"], "user-456");
        assert_eq!(data.event["session"]["user"]["attributes"]["plan"], "premium");
        assert_eq!(data.event["session"]["user"]["attributes"]["preferences"][0], "feature1");
        assert_eq!(data.event["session"]["actions"][0]["type"], "page_view");
        assert_eq!(data.event["session"]["actions"][0]["path"], "/home");
        assert_eq!(data.event["session"]["actions"][2]["data"]["email"], "user@example.com");
        assert_eq!(data.event["metadata"]["source"], "web");
        assert_eq!(data.event["metadata"]["version"], "1.2.3");
    }
}

// ===== CUSTOM EVENT TESTS =====

#[wasm_bindgen_test]
fn test_custom_event_basic() {
    // Test basic CustomEvent (matching TypeScript round-trip test)
    let event = BaseEvent {
        event_type: EventType::Custom,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Custom(CustomEvent {
            event_type: "user_preference_updated".to_string(),
            data: json!({
                "theme": "dark",
                "fontSize": "medium",
                "notifications": true
            }),
        }),
    };

    assert_eq!(event.event_type, EventType::Custom);

    if let EventData::Custom(data) = &event.data {
        assert_eq!(data.event_type, "user_preference_updated");
        assert_eq!(data.data["theme"], "dark");
        assert_eq!(data.data["fontSize"], "medium");
        assert_eq!(data.data["notifications"], true);
    } else {
        panic!("Expected Custom event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "CUSTOM");
    assert_eq!(parsed["event_type"], "user_preference_updated");
    assert_eq!(parsed["theme"], "dark");
    assert_eq!(parsed["fontSize"], "medium");
}

#[wasm_bindgen_test]
fn test_custom_event_without_value() {
    // Test CustomEvent without value (matching TypeScript no value test)
    let event = BaseEvent {
        event_type: EventType::Custom,
        timestamp: None,
        raw_event: None,
        data: EventData::Custom(CustomEvent {
            event_type: "heartbeat".to_string(),
            data: json!({}),
        }),
    };

    if let EventData::Custom(data) = &event.data {
        assert_eq!(data.event_type, "heartbeat");
    } else {
        panic!("Expected Custom event data");
    }

    // Test round-trip serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Custom(data) = &deserialized.data {
        assert_eq!(data.event_type, "heartbeat");
        // Value should be None or excluded from JSON
    }
}

#[wasm_bindgen_test]
fn test_custom_event_complex_values() {
    // Test CustomEvent with complex values (matching TypeScript complex values test)
    let complex_value = json!({
        "metrics": {
            "active_users": 12345,
            "conversion_rate": 0.0354,
            "revenue": 98765.43
        },
        "segments": [
            {
                "name": "new_users",
                "count": 543,
                "growth": 0.12
            },
            {
                "name": "returning_users",
                "count": 876,
                "growth": -0.05
            },
            {
                "name": "power_users",
                "count": 234,
                "growth": 0.08
            }
        ],
        "period": {
            "start": "2023-01-01",
            "end": "2023-01-31",
            "duration_days": 31
        },
        "trends": {
            "daily": [10, 12, 15, 14, 18, 20, 22],
            "weekly": [70, 85, 92, 105],
            "monthly": [320, 370]
        }
    });

    let event = BaseEvent {
        event_type: EventType::Custom,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Custom(CustomEvent {
            event_type: "analytics_update".to_string(),
            data: complex_value.clone(),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Custom(data) = &deserialized.data {
        assert_eq!(data.event_type, "analytics_update");
        let value = &data.data;
            // Verify complex nested structure
            assert_eq!(value["metrics"]["active_users"], 12345);
            assert_eq!(value["metrics"]["conversion_rate"], 0.0354);
            assert_eq!(value["segments"][0]["name"], "new_users");
            assert_eq!(value["segments"][0]["growth"], 0.12);
            assert_eq!(value["segments"][1]["growth"], -0.05);
            assert_eq!(value["period"]["start"], "2023-01-01");
            assert_eq!(value["period"]["duration_days"], 31);
            assert_eq!(value["trends"]["daily"][2], 15);
            assert_eq!(value["trends"]["weekly"][3], 105);
            assert_eq!(value["trends"]["monthly"][1], 370);
    }
}

// ===== EDGE CASES AND SPECIAL VALUES =====

#[wasm_bindgen_test]
fn test_raw_event_special_values() {
    // Test RawEvent with special values (matching encoder special values patterns)
    let special_data = json!({
        "nullValue": null,
        "emptyString": "",
        "zero": 0,
        "negativeNumber": -123,
        "floatNumber": 3.14159,
        "emptyArray": [],
        "emptyObject": {},
        "boolValues": {
            "true": true,
            "false": false
        },
        "largeNumber": 9223372036854775807i64,
        "scientificNotation": 1.23e-4
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
        assert_eq!(data.event["largeNumber"], 9223372036854775807i64);
        // Scientific notation should be preserved as a number
        assert!((data.event["scientificNotation"].as_f64().unwrap() - 1.23e-4).abs() < 1e-10);
    }
}

#[wasm_bindgen_test]
fn test_custom_event_unicode_and_special_chars() {
    // Test CustomEvent with Unicode and special characters
    let unicode_name = "event_with_unicode_你好_🚀";
    let unicode_value = json!({
        "message": "Hello 你好 こんにちは 안녕하세요 👋 🌍",
        "specialChars": "Special chars: \\n\\t\"'/<>{}[]",
        "emoji": "🎉🎊🔥💯",
        "mathematical": "∑∆∇∞≈≠≤≥"
    });

    let event = BaseEvent {
        event_type: EventType::Custom,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Custom(CustomEvent {
            event_type: unicode_name.to_string(),
            data: unicode_value.clone(),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Custom(data) = &deserialized.data {
        assert_eq!(data.event_type, unicode_name);
        let value = &data.data;
            assert!(value["message"].as_str().unwrap().contains("你好"));
            assert!(value["message"].as_str().unwrap().contains("🌍"));
            assert!(value["specialChars"].as_str().unwrap().contains("\\n\\t"));
            assert!(value["emoji"].as_str().unwrap().contains("🎉"));
            assert!(value["mathematical"].as_str().unwrap().contains("∑"));
    }
}

#[wasm_bindgen_test]
fn test_raw_event_large_data() {
    // Test RawEvent with large data structures
    let large_array: Vec<serde_json::Value> = (0..1000)
        .map(|i| json!({
            "id": i,
            "name": format!("item_{}", i),
            "data": format!("content_{}", i),
            "nested": {
                "values": [i, i+1, i+2],
                "metadata": {
                    "created": "2023-10-01",
                    "modified": "2023-10-02"
                }
            }
        }))
        .collect();

    let large_data = json!({
        "type": "bulk_data",
        "items": large_array,
        "metadata": {
            "total_count": 1000,
            "processed_at": "2023-10-01T12:00:00Z"
        }
    });

    let event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::Raw(RawEvent { event: large_data.clone() }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Raw(data) = &deserialized.data {
        assert_eq!(data.event["type"], "bulk_data");
        assert_eq!(data.event["items"].as_array().unwrap().len(), 1000);
        assert_eq!(data.event["items"][500]["id"], 500);
        assert_eq!(data.event["items"][500]["name"], "item_500");
        assert_eq!(data.event["metadata"]["total_count"], 1000);
    }
}

// ===== COMPREHENSIVE MIXED EVENT TESTS =====

#[wasm_bindgen_test]
fn test_mixed_raw_and_custom_events() {
    // Test a mixture of raw and custom events
    let events = vec![
        // Raw event
        BaseEvent {
            event_type: EventType::Raw,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Raw(RawEvent { event: json!({
                "source": "api",
                "event": "user_login",
                "userId": "user_123",
                "timestamp": 1676480210000i64
            }) }),
        },

        // Custom event
        BaseEvent {
            event_type: EventType::Custom,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Custom(CustomEvent {
                event_type: "feature_flag_toggled".to_string(),
                data: json!({
                    "flag": "new_ui",
                    "enabled": true,
                    "userId": "user_123"
                }),
            }),
        },

        // Raw event with complex structure
        BaseEvent {
            event_type: EventType::Raw,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Raw(RawEvent { event: json!({
                "type": "performance_metrics",
                "metrics": {
                    "pageLoadTime": 1.5,
                    "renderTime": 0.8,
                    "jsExecutionTime": 0.3
                },
                "userAgent": "Mozilla/5.0..."
            }) }),
        },

        // Custom event without value
        BaseEvent {
            event_type: EventType::Custom,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Custom(CustomEvent {
                event_type: "session_ping".to_string(),
                data: json!({}),
            }),
        },
    ];

    // Test serialization of all events
    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }

    // Test batch serialization
    let events_json = serde_json::to_string(&events).unwrap();
    let deserialized_events: Vec<BaseEvent> = serde_json::from_str(&events_json).unwrap();

    assert_eq!(events.len(), deserialized_events.len());

    // Verify specific event data
    if let EventData::Raw(data) = &deserialized_events[0].data {
        assert_eq!(data.event["source"], "api");
        assert_eq!(data.event["event"], "user_login");
    }

    if let EventData::Custom(data) = &deserialized_events[1].data {
        assert_eq!(data.event_type, "feature_flag_toggled");
        assert_eq!(data.data["flag"], "new_ui");
        assert_eq!(data.data["enabled"], true);
    }

    if let EventData::Raw(data) = &deserialized_events[2].data {
        assert_eq!(data.event["type"], "performance_metrics");
        assert_eq!(data.event["metrics"]["pageLoadTime"], 1.5);
    }

    if let EventData::Custom(data) = &deserialized_events[3].data {
        assert_eq!(data.event_type, "session_ping");
    }
}

// ===== BASIC FIELD VALIDATION TESTS =====

#[wasm_bindgen_test]
fn test_basic_fields_validation() {
    // Test basic fields for each event type (matching TypeScript basic fields test)
    let raw_event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: None,
        raw_event: None,
        data: EventData::Raw(RawEvent { event: json!({"test": "value"}) }),
    };

    let custom_event = BaseEvent {
        event_type: EventType::Custom,
        timestamp: None,
        raw_event: None,
        data: EventData::Custom(CustomEvent {
            event_type: "empty".to_string(),
            data: json!({}),
        }),
    };

    let events = vec![raw_event, custom_event];

    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify basic structure
        assert!(parsed["type"].is_string());

        // Check appropriate data field based on event type
        match parsed["type"].as_str().unwrap() {
            "RAW" => {
                assert!(parsed["event"].is_object()); // RawEvent uses "event" field
            },
            "CUSTOM" => {
                assert!(parsed["event_type"].is_string()); // CustomEvent has "event_type" field directly flattened
            },
            _ => panic!("Unexpected event type"),
        }

        // Verify deserialization
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }
}

#[wasm_bindgen_test]
fn test_events_with_all_base_fields() {
    // Test events with all base fields (matching TypeScript all base fields test)
    let raw_event_data = json!({"original": "data", "from": "external_system"});

    let raw_event = BaseEvent {
        event_type: EventType::Raw,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({"original": "raw_data", "from": "external_system"})),
        data: EventData::Raw(RawEvent { event: raw_event_data.clone() }),
    };

    let custom_event = BaseEvent {
        event_type: EventType::Custom,
        timestamp: Some(Utc::now()),
        raw_event: Some(json!({"original": "raw_data", "from": "external_system"})),
        data: EventData::Custom(CustomEvent {
            event_type: "full_event".to_string(),
            data: json!({"custom": "data"}),
        }),
    };

    let events = vec![raw_event, custom_event];

    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify all fields are present
        assert!(parsed["type"].is_string());
        assert!(parsed["timestamp"].is_string());
        assert!(parsed["raw_event"].is_object());

        // Verify event-specific data field
        match parsed["type"].as_str().unwrap() {
            "RAW" => {
                assert!(parsed["event"].is_object()); // RawEvent uses "event" field
            },
            "CUSTOM" => {
                assert!(parsed["event_type"].is_string()); // CustomEvent has "event_type" field
                // CustomEvent data is flattened, so additional properties are at the top level
            },
            _ => panic!("Unexpected event type"),
        }

        // Verify raw_event content
        assert_eq!(parsed["raw_event"]["original"], "raw_data");
        assert_eq!(parsed["raw_event"]["from"], "external_system");

        // Test round-trip
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
        assert!(deserialized.timestamp.is_some());
        assert!(deserialized.raw_event.is_some());
    }
}