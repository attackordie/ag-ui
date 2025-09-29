//! Comprehensive state events tests matching TypeScript and Python SDK patterns
//! Tests StateSnapshot and StateDelta events with complex nested objects and JSON Patch operations

use ag_ui_wasm::{
    BaseEvent, EventType, EventData, State, StateSnapshotEvent, StateDeltaEvent
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// ===== STATE SNAPSHOT TESTS =====

#[wasm_bindgen_test]
fn test_state_snapshot_basic() {
    // Test basic StateSnapshot event (matching TypeScript round-trip test)
    let mut snapshot: State = HashMap::new();
    snapshot.insert("counter".to_string(), json!(42));
    snapshot.insert("items".to_string(), json!(["apple", "banana", "cherry"]));
    snapshot.insert("config".to_string(), json!({
        "enabled": true,
        "maxRetries": 3
    }));

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state: snapshot }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    assert_eq!(deserialized.event_type, EventType::StateSnapshot);

    if let EventData::StateSnapshot(state_data) = &deserialized.data {
        assert_eq!(state_data.state["counter"], 42);
        assert_eq!(state_data.state["items"], json!(["apple", "banana", "cherry"]));
        assert_eq!(state_data.state["config"]["enabled"], true);
        assert_eq!(state_data.state["config"]["maxRetries"], 3);
    } else {
        panic!("Expected StateSnapshot event data");
    }
}

#[wasm_bindgen_test]
fn test_state_snapshot_empty_object() {
    // Test empty snapshot object (matching TypeScript empty snapshot test)
    let snapshot: State = HashMap::new();

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: None,
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state: snapshot }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateSnapshot(state_data) = &deserialized.data {
        assert!(state_data.state.is_empty());
    } else {
        panic!("Expected StateSnapshot event data");
    }
}

#[wasm_bindgen_test]
fn test_state_snapshot_complex_nested_objects() {
    // Test complex nested objects (matching TypeScript complex nested test)
    let mut snapshot: State = HashMap::new();

    snapshot.insert("userProfile".to_string(), json!({
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

    snapshot.insert("serviceConfig".to_string(), json!({
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

    snapshot.insert("stats".to_string(), json!({
        "visits": 1042,
        "conversions": 123,
        "bounceRate": 0.25,
        "dataPoints": [
            {"date": "2023-01-01", "value": 10},
            {"date": "2023-01-02", "value": 15},
            {"date": "2023-01-03", "value": 8}
        ]
    }));

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state: snapshot }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateSnapshot(state_data) = &deserialized.data {
        // Verify deeply nested structure is preserved
        assert_eq!(state_data.state["userProfile"]["name"], "John Doe");
        assert_eq!(state_data.state["userProfile"]["contact"]["address"]["city"], "Anytown");
        assert_eq!(state_data.state["userProfile"]["contact"]["address"]["coordinates"]["lat"], 37.7749);
        assert_eq!(state_data.state["serviceConfig"]["endpoints"][0]["name"], "api1");
        assert_eq!(state_data.state["serviceConfig"]["retryPolicy"]["maxRetries"], 3);
        assert_eq!(state_data.state["stats"]["visits"], 1042);
        assert_eq!(state_data.state["stats"]["dataPoints"][1]["date"], "2023-01-02");
    } else {
        panic!("Expected StateSnapshot event data");
    }
}

#[wasm_bindgen_test]
fn test_state_snapshot_special_values() {
    // Test special values in snapshot (matching TypeScript special values test)
    let mut snapshot: State = HashMap::new();

    snapshot.insert("nullValue".to_string(), json!(null));
    snapshot.insert("emptyString".to_string(), json!(""));
    snapshot.insert("zero".to_string(), json!(0));
    snapshot.insert("negativeNumber".to_string(), json!(-123));
    snapshot.insert("floatNumber".to_string(), json!(3.14159));
    snapshot.insert("emptyArray".to_string(), json!([]));
    snapshot.insert("emptyObject".to_string(), json!({}));
    snapshot.insert("boolValues".to_string(), json!({"true": true, "false": false}));
    // Note: Skipping Infinity/NaN as they don't serialize well in JSON

    let event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state: snapshot }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateSnapshot(state_data) = &deserialized.data {
        assert_eq!(state_data.state["nullValue"], json!(null));
        assert_eq!(state_data.state["emptyString"], "");
        assert_eq!(state_data.state["zero"], 0);
        assert_eq!(state_data.state["negativeNumber"], -123);
        assert_eq!(state_data.state["floatNumber"], 3.14159);
        assert_eq!(state_data.state["emptyArray"], json!([]));
        assert_eq!(state_data.state["emptyObject"], json!({}));
        assert_eq!(state_data.state["boolValues"]["true"], true);
        assert_eq!(state_data.state["boolValues"]["false"], false);
    } else {
        panic!("Expected StateSnapshot event data");
    }
}

// ===== STATE DELTA TESTS =====

#[wasm_bindgen_test]
fn test_state_delta_basic() {
    // Test basic StateDelta event (matching TypeScript round-trip test)
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

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    assert_eq!(deserialized.event_type, EventType::StateDelta);

    if let EventData::StateDelta(delta_data) = &deserialized.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches.len(), 2);
        assert_eq!(patches[0]["op"], "add");
        assert_eq!(patches[0]["path"], "/counter");
        assert_eq!(patches[0]["value"], 42);
        assert_eq!(patches[1]["op"], "add");
        assert_eq!(patches[1]["path"], "/items");
        assert_eq!(patches[1]["value"], json!(["apple", "banana", "cherry"]));
    } else {
        panic!("Expected StateDelta event data");
    }
}

#[wasm_bindgen_test]
fn test_state_delta_all_json_patch_operations() {
    // Test all JSON Patch operation types (matching TypeScript comprehensive test)
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

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateDelta(delta_data) = &deserialized.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches.len(), 6);

        // Verify each operation type
        assert_eq!(patches[0]["op"], "add");
        assert_eq!(patches[1]["op"], "remove");
        assert_eq!(patches[2]["op"], "replace");
        assert_eq!(patches[3]["op"], "move");
        assert_eq!(patches[4]["op"], "copy");
        assert_eq!(patches[5]["op"], "test");

        // Verify operation-specific fields
        assert_eq!(patches[0]["path"], "/users/123");
        assert_eq!(patches[0]["value"]["name"], "John");
        assert_eq!(patches[1]["path"], "/users/456");
        assert_eq!(patches[2]["value"], "Jane Doe");
        assert_eq!(patches[3]["from"], "/users/old");
        assert_eq!(patches[4]["from"], "/templates/default");
        assert_eq!(patches[5]["value"], true);
    } else {
        panic!("Expected StateDelta event data");
    }
}

#[wasm_bindgen_test]
fn test_state_delta_complex_values() {
    // Test complex values in add operations (matching TypeScript complex values test)
    let delta = json!([
        {
            "op": "add",
            "path": "/data",
            "value": {
                "nested": {
                    "array": [1, 2, 3],
                    "object": {"key": "value"}
                },
                "boolean": true,
                "number": 42
            }
        }
    ]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateDelta(delta_data) = &deserialized.data {
        let patches = delta_data.delta.as_array().unwrap();
        let value = &patches[0]["value"];

        assert_eq!(value["nested"]["array"], json!([1, 2, 3]));
        assert_eq!(value["nested"]["object"]["key"], "value");
        assert_eq!(value["boolean"], true);
        assert_eq!(value["number"], 42);
    } else {
        panic!("Expected StateDelta event data");
    }
}

#[wasm_bindgen_test]
fn test_state_delta_array_operations() {
    // Test array operations (matching TypeScript array operations test)
    let delta = json!([
        {"op": "add", "path": "/items", "value": []},
        {"op": "add", "path": "/items/0", "value": "first"},
        {"op": "add", "path": "/items/-", "value": "last"},
        {"op": "replace", "path": "/items/0", "value": "updated first"},
        {"op": "remove", "path": "/items/1"}
    ]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateDelta(delta_data) = &deserialized.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches.len(), 5);

        // Verify array-specific paths
        assert_eq!(patches[0]["path"], "/items");
        assert_eq!(patches[1]["path"], "/items/0");
        assert_eq!(patches[2]["path"], "/items/-"); // Append to end
        assert_eq!(patches[3]["path"], "/items/0");
        assert_eq!(patches[4]["path"], "/items/1");

        // Verify values
        assert_eq!(patches[0]["value"], json!([]));
        assert_eq!(patches[1]["value"], "first");
        assert_eq!(patches[2]["value"], "last");
        assert_eq!(patches[3]["value"], "updated first");
    } else {
        panic!("Expected StateDelta event data");
    }
}

#[wasm_bindgen_test]
fn test_state_delta_special_characters_in_paths() {
    // Test special characters in paths (matching TypeScript special chars test)
    let delta = json!([
        {"op": "add", "path": "/special~0field", "value": "value with tilde"},
        {"op": "add", "path": "/special~1field", "value": "value with slash"},
        {"op": "add", "path": "/special/field", "value": "value with actual slash"},
        {"op": "add", "path": "/special\"field", "value": "value with quote"},
        {"op": "add", "path": "/emoji🚀field", "value": "value with emoji"}
    ]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateDelta(delta_data) = &deserialized.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches.len(), 5);

        // Verify special character paths are preserved
        assert_eq!(patches[0]["path"], "/special~0field");
        assert_eq!(patches[1]["path"], "/special~1field");
        assert_eq!(patches[2]["path"], "/special/field");
        assert_eq!(patches[3]["path"], "/special\"field");
        assert_eq!(patches[4]["path"], "/emoji🚀field");

        // Verify values are preserved
        assert_eq!(patches[0]["value"], "value with tilde");
        assert_eq!(patches[1]["value"], "value with slash");
        assert_eq!(patches[4]["value"], "value with emoji");
    } else {
        panic!("Expected StateDelta event data");
    }
}

#[wasm_bindgen_test]
fn test_state_delta_empty_array() {
    // Test empty delta array (matching TypeScript empty delta test)
    let delta = json!([]);

    let event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::StateDelta(delta_data) = &deserialized.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches.len(), 0);
    } else {
        panic!("Expected StateDelta event data");
    }
}

// ===== COMPREHENSIVE STATE FLOW TESTS =====

#[wasm_bindgen_test]
fn test_comprehensive_state_flow() {
    // Test a comprehensive state management flow

    // 1. Initial state snapshot
    let mut initial_state: State = HashMap::new();
    initial_state.insert("session".to_string(), json!({
        "id": "sess_123",
        "user": {"id": "user_456", "name": "Alice"},
        "status": "active"
    }));

    let snapshot_event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state: initial_state }),
    };

    // 2. State delta updates
    let delta_update = json!([
        {"op": "replace", "path": "/session/status", "value": "paused"},
        {"op": "add", "path": "/session/pausedAt", "value": "2023-10-01T12:00:00Z"},
        {"op": "add", "path": "/session/user/lastAction", "value": "button_click"}
    ]);

    let delta_event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta: delta_update }),
    };

    // Test serialization of both events
    let snapshot_json = serde_json::to_string(&snapshot_event).unwrap();
    let delta_json = serde_json::to_string(&delta_event).unwrap();

    // Test deserialization
    let deser_snapshot: BaseEvent = serde_json::from_str(&snapshot_json).unwrap();
    let deser_delta: BaseEvent = serde_json::from_str(&delta_json).unwrap();

    // Verify snapshot
    if let EventData::StateSnapshot(state_data) = &deser_snapshot.data {
        assert_eq!(state_data.state["session"]["id"], "sess_123");
        assert_eq!(state_data.state["session"]["user"]["name"], "Alice");
        assert_eq!(state_data.state["session"]["status"], "active");
    } else {
        panic!("Expected StateSnapshot event data");
    }

    // Verify delta
    if let EventData::StateDelta(delta_data) = &deser_delta.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches[0]["op"], "replace");
        assert_eq!(patches[0]["value"], "paused");
        assert_eq!(patches[1]["op"], "add");
        assert_eq!(patches[1]["path"], "/session/pausedAt");
        assert_eq!(patches[2]["path"], "/session/user/lastAction");
    } else {
        panic!("Expected StateDelta event data");
    }
}

// ===== EDGE CASES AND ERROR HANDLING =====

#[wasm_bindgen_test]
fn test_state_edge_cases() {
    // Test various edge cases for state management

    // Very large state object
    let mut large_state: State = HashMap::new();
    for i in 0..1000 {
        large_state.insert(format!("item_{}", i), json!({
            "id": i,
            "data": format!("content_{}", i),
            "nested": {
                "values": vec![i, i+1, i+2]
            }
        }));
    }

    let large_event = BaseEvent {
        event_type: EventType::StateSnapshot,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateSnapshot(StateSnapshotEvent { state: large_state }),
    };

    // Should handle large state without issues
    let large_json = serde_json::to_string(&large_event).unwrap();
    let large_deser: BaseEvent = serde_json::from_str(&large_json).unwrap();

    if let EventData::StateSnapshot(state_data) = &large_deser.data {
        assert_eq!(state_data.state.len(), 1000);
        assert_eq!(state_data.state["item_500"]["id"], 500);
    }

    // Complex delta with many operations
    let complex_delta = json!((0..100).map(|i| {
        json!({"op": "add", "path": format!("/dynamic_{}", i), "value": i})
    }).collect::<Vec<_>>());

    let complex_delta_event = BaseEvent {
        event_type: EventType::StateDelta,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StateDelta(StateDeltaEvent { delta: complex_delta }),
    };

    let complex_json = serde_json::to_string(&complex_delta_event).unwrap();
    let complex_deser: BaseEvent = serde_json::from_str(&complex_json).unwrap();

    if let EventData::StateDelta(delta_data) = &complex_deser.data {
        let patches = delta_data.delta.as_array().unwrap();
        assert_eq!(patches.len(), 100);
        assert_eq!(patches[50]["path"], "/dynamic_50");
        assert_eq!(patches[50]["value"], 50);
    }
}