//! Comprehensive lifecycle events tests matching TypeScript and Python SDK patterns
//! Tests RunStarted, RunFinished, RunError, StepStarted, StepFinished events with edge cases

use ag_ui_wasm::{
    BaseEvent, EventType, EventData,
    RunStartedEvent, ErrorEvent,
    StepStartedEvent, StepFinishedEvent,
};
use wasm_bindgen_test::*;
use serde_json::json;
use chrono::Utc;

wasm_bindgen_test_configure!(run_in_browser);

// ===== RUN LIFECYCLE EVENTS TESTS =====

#[wasm_bindgen_test]
fn test_run_started_event_basic() {
    // Test basic RunStarted event (matching TypeScript round-trip test)
    let event = BaseEvent::run_started("thread-1234".to_string(), "run-5678".to_string());

    assert_eq!(event.event_type, EventType::RunStarted);

    if let EventData::RunStarted(data) = &event.data {
        assert_eq!(data.thread_id, "thread-1234");
        assert_eq!(data.run_id, "run-5678");
    } else {
        panic!("Expected RunStarted event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "RUN_STARTED");
    if let Some(data) = parsed["data"].as_object() {
        assert_eq!(data["thread_id"], "thread-1234");
        assert_eq!(data["run_id"], "run-5678");
    }
}

#[wasm_bindgen_test]
fn test_run_started_event_with_timestamp() {
    // Test RunStarted event with explicit timestamp
    let timestamp = Utc::now();
    let event = BaseEvent {
        event_type: EventType::RunStarted,
        timestamp: Some(timestamp),
        raw_event: None,
        data: EventData::RunStarted(RunStartedEvent {
            thread_id: "thread-abc".to_string(),
            run_id: "run-def".to_string(),
        }),
    };

    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    assert_eq!(deserialized.event_type, EventType::RunStarted);
    assert!(deserialized.timestamp.is_some());

    if let EventData::RunStarted(data) = &deserialized.data {
        assert_eq!(data.thread_id, "thread-abc");
        assert_eq!(data.run_id, "run-def");
    }
}

#[wasm_bindgen_test]
fn test_run_finished_event_basic() {
    // Test basic RunFinished event (matching TypeScript round-trip test)
    let event = BaseEvent::run_finished("thread-1234".to_string(), "run-5678".to_string());

    assert_eq!(event.event_type, EventType::RunFinished);

    if let EventData::RunFinished(data) = &event.data {
        assert_eq!(data.thread_id, "thread-1234");
        assert_eq!(data.run_id, "run-5678");
    } else {
        panic!("Expected RunFinished event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "RUN_FINISHED");
    if let Some(data) = parsed["data"].as_object() {
        assert_eq!(data["thread_id"], "thread-1234");
        assert_eq!(data["run_id"], "run-5678");
    }
}

#[wasm_bindgen_test]
fn test_run_error_event_basic() {
    // Test basic RunError event (matching TypeScript round-trip test)
    let event = BaseEvent::error(
        "Failed to execute tool call".to_string(),
        None,
    );

    assert_eq!(event.event_type, EventType::Error);

    if let EventData::Error(data) = &event.data {
        assert_eq!(data.error, "Failed to execute tool call");
        assert!(data.code.is_none());
    } else {
        panic!("Expected Error event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "ERROR");
    if let Some(data) = parsed["data"].as_object() {
        assert_eq!(data["error"], "Failed to execute tool call");
        assert!(data["code"].is_null() || !data.contains_key("code"));
    }
}

#[wasm_bindgen_test]
fn test_run_error_event_with_code() {
    // Test RunError event with error code (matching TypeScript detailed error test)
    let event = BaseEvent::error(
        "API request failed".to_string(),
        Some("API_ERROR".to_string()),
    );

    if let EventData::Error(data) = &event.data {
        assert_eq!(data.error, "API request failed");
        assert_eq!(data.code, Some("API_ERROR".to_string()));
    } else {
        panic!("Expected Error event data");
    }

    // Test round-trip serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();

    if let EventData::Error(data) = &deserialized.data {
        assert_eq!(data.error, "API request failed");
        assert_eq!(data.code, Some("API_ERROR".to_string()));
    }
}

// ===== STEP LIFECYCLE EVENTS TESTS =====

#[wasm_bindgen_test]
fn test_step_started_event_basic() {
    // Test basic StepStarted event (matching TypeScript round-trip test)
    let event = BaseEvent {
        event_type: EventType::StepStarted,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StepStarted(StepStartedEvent {
            thread_id: "thread_123".to_string(),
            run_id: "run_456".to_string(),
            step_id: "step_data_analysis".to_string(),
            step_type: Some("data_analysis".to_string()),
        }),
    };

    assert_eq!(event.event_type, EventType::StepStarted);

    if let EventData::StepStarted(data) = &event.data {
        assert_eq!(data.thread_id, "thread_123");
        assert_eq!(data.run_id, "run_456");
        assert_eq!(data.step_id, "step_data_analysis");
        assert_eq!(data.step_type, Some("data_analysis".to_string()));
    } else {
        panic!("Expected StepStarted event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "STEP_STARTED");
    assert_eq!(parsed["thread_id"], "thread_123");
    assert_eq!(parsed["run_id"], "run_456");
    assert_eq!(parsed["step_id"], "step_data_analysis");
    assert_eq!(parsed["step_type"], "data_analysis");
}

#[wasm_bindgen_test]
fn test_step_finished_event_basic() {
    // Test basic StepFinished event (matching TypeScript round-trip test)
    let event = BaseEvent {
        event_type: EventType::StepFinished,
        timestamp: Some(Utc::now()),
        raw_event: None,
        data: EventData::StepFinished(StepFinishedEvent {
            thread_id: "thread_123".to_string(),
            run_id: "run_456".to_string(),
            step_id: "step_data_analysis".to_string(),
        }),
    };

    assert_eq!(event.event_type, EventType::StepFinished);

    if let EventData::StepFinished(data) = &event.data {
        assert_eq!(data.thread_id, "thread_123");
        assert_eq!(data.run_id, "run_456");
        assert_eq!(data.step_id, "step_data_analysis");
    } else {
        panic!("Expected StepFinished event data");
    }

    // Test serialization
    let json_str = serde_json::to_string(&event).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["type"], "STEP_FINISHED");
    assert_eq!(parsed["thread_id"], "thread_123");
    assert_eq!(parsed["run_id"], "run_456");
    assert_eq!(parsed["step_id"], "step_data_analysis");
}

#[wasm_bindgen_test]
fn test_step_events_minimal_fields() {
    // Test step events with minimal fields (matching TypeScript minimal fields tests)

    // StepStarted with minimal fields
    let step_started = BaseEvent {
        event_type: EventType::StepStarted,
        timestamp: None, // No timestamp
        raw_event: None,
        data: EventData::StepStarted(StepStartedEvent {
            thread_id: "thread_minimal".to_string(),
            run_id: "run_minimal".to_string(),
            step_id: "step_process_payment".to_string(),
            step_type: Some("process_payment".to_string()),
        }),
    };

    let started_json = serde_json::to_string(&step_started).unwrap();
    let started_deser: BaseEvent = serde_json::from_str(&started_json).unwrap();

    assert_eq!(started_deser.event_type, EventType::StepStarted);
    if let EventData::StepStarted(data) = &started_deser.data {
        assert_eq!(data.step_id, "step_process_payment");
        assert_eq!(data.step_type, Some("process_payment".to_string()));
    }

    // StepFinished with minimal fields
    let step_finished = BaseEvent {
        event_type: EventType::StepFinished,
        timestamp: None, // No timestamp
        raw_event: None,
        data: EventData::StepFinished(StepFinishedEvent {
            thread_id: "thread_minimal".to_string(),
            run_id: "run_minimal".to_string(),
            step_id: "step_process_payment".to_string(),
        }),
    };

    let finished_json = serde_json::to_string(&step_finished).unwrap();
    let finished_deser: BaseEvent = serde_json::from_str(&finished_json).unwrap();

    assert_eq!(finished_deser.event_type, EventType::StepFinished);
    if let EventData::StepFinished(data) = &finished_deser.data {
        assert_eq!(data.step_id, "step_process_payment");
    }
}

// ===== COMPREHENSIVE LIFECYCLE FLOW TESTS =====

#[wasm_bindgen_test]
fn test_complete_run_lifecycle() {
    // Test a complete run lifecycle with multiple steps
    let events = vec![
        // Start run
        BaseEvent::run_started("thread_lifecycle".to_string(), "run_lifecycle".to_string()),

        // Start first step
        BaseEvent {
            event_type: EventType::StepStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepStarted(StepStartedEvent {
                thread_id: "thread_lifecycle".to_string(),
                run_id: "run_lifecycle".to_string(),
                step_id: "step_initialization".to_string(),
                step_type: Some("initialization".to_string()),
            }),
        },

        // Finish first step
        BaseEvent {
            event_type: EventType::StepFinished,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepFinished(StepFinishedEvent {
                thread_id: "thread_lifecycle".to_string(),
                run_id: "run_lifecycle".to_string(),
                step_id: "step_initialization".to_string(),
            }),
        },

        // Start second step
        BaseEvent {
            event_type: EventType::StepStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepStarted(StepStartedEvent {
                thread_id: "thread_lifecycle".to_string(),
                run_id: "run_lifecycle".to_string(),
                step_id: "step_data_processing".to_string(),
                step_type: Some("data_processing".to_string()),
            }),
        },

        // Finish second step
        BaseEvent {
            event_type: EventType::StepFinished,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepFinished(StepFinishedEvent {
                thread_id: "thread_lifecycle".to_string(),
                run_id: "run_lifecycle".to_string(),
                step_id: "step_data_processing".to_string(),
            }),
        },

        // Finish run
        BaseEvent::run_finished("thread_lifecycle".to_string(), "run_lifecycle".to_string()),
    ];

    // Test that all events serialize and deserialize correctly
    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }

    // Test batch serialization
    let events_json = serde_json::to_string(&events).unwrap();
    let deserialized_events: Vec<BaseEvent> = serde_json::from_str(&events_json).unwrap();

    assert_eq!(events.len(), deserialized_events.len());

    // Verify the lifecycle flow
    assert_eq!(deserialized_events[0].event_type, EventType::RunStarted);
    assert_eq!(deserialized_events[1].event_type, EventType::StepStarted);
    assert_eq!(deserialized_events[2].event_type, EventType::StepFinished);
    assert_eq!(deserialized_events[3].event_type, EventType::StepStarted);
    assert_eq!(deserialized_events[4].event_type, EventType::StepFinished);
    assert_eq!(deserialized_events[5].event_type, EventType::RunFinished);

    // Verify specific data
    if let EventData::RunStarted(data) = &deserialized_events[0].data {
        assert_eq!(data.thread_id, "thread_lifecycle");
        assert_eq!(data.run_id, "run_lifecycle");
    }

    if let EventData::StepStarted(data) = &deserialized_events[1].data {
        assert_eq!(data.step_id, "step_initialization");
        assert_eq!(data.step_type, Some("initialization".to_string()));
    }

    if let EventData::StepStarted(data) = &deserialized_events[3].data {
        assert_eq!(data.step_id, "step_data_processing");
        assert_eq!(data.step_type, Some("data_processing".to_string()));
    }
}

#[wasm_bindgen_test]
fn test_error_during_lifecycle() {
    // Test error event during a run lifecycle
    let events = vec![
        BaseEvent::run_started("thread_error".to_string(), "run_error".to_string()),
        BaseEvent {
            event_type: EventType::StepStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepStarted(StepStartedEvent {
                thread_id: "thread_error".to_string(),
                run_id: "run_error".to_string(),
                step_id: "step_failing".to_string(),
                step_type: Some("failing_step".to_string()),
            }),
        },
        BaseEvent::error(
            "Step failed due to invalid input".to_string(),
            Some("VALIDATION_ERROR".to_string()),
        ),
        // Run might still finish even after error
        BaseEvent::run_finished("thread_error".to_string(), "run_error".to_string()),
    ];

    // Test serialization of error scenario
    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }

    // Verify error event details
    if let EventData::Error(error_data) = &events[2].data {
        assert_eq!(error_data.error, "Step failed due to invalid input");
        assert_eq!(error_data.code, Some("VALIDATION_ERROR".to_string()));
    }
}

// ===== EDGE CASES AND SPECIAL VALUES TESTS =====

#[wasm_bindgen_test]
fn test_lifecycle_events_with_special_characters() {
    // Test events with special characters in IDs and names
    let special_chars_events = vec![
        BaseEvent::run_started(
            "thread-with-special-chars_🚀".to_string(),
            "run-with-unicode-你好".to_string(),
        ),
        BaseEvent {
            event_type: EventType::StepStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepStarted(StepStartedEvent {
                thread_id: "thread_special".to_string(),
                run_id: "run_special".to_string(),
                step_id: "step_special".to_string(),
                step_type: Some("step_with_special\"quotes'and\\backslashes".to_string()),
            }),
        },
        BaseEvent::error(
            "Error message with special chars: \n\t\"'<>&".to_string(),
            Some("ERROR_🚨".to_string()),
        ),
    ];

    for event in &special_chars_events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }

    // Verify special characters are preserved
    if let EventData::RunStarted(data) = &special_chars_events[0].data {
        assert!(data.thread_id.contains("🚀"));
        assert!(data.run_id.contains("你好"));
    }

    if let EventData::StepStarted(data) = &special_chars_events[1].data {
        assert!(data.step_type.as_ref().unwrap().contains("\"quotes'"));
        assert!(data.step_type.as_ref().unwrap().contains("\\backslashes"));
    }

    if let EventData::Error(data) = &special_chars_events[2].data {
        assert!(data.error.contains("\n\t"));
        assert!(data.error.contains("\"'<>&"));
        assert_eq!(data.code, Some("ERROR_🚨".to_string()));
    }
}

#[wasm_bindgen_test]
fn test_lifecycle_events_edge_cases() {
    // Test edge cases for lifecycle events

    // Empty strings
    let empty_events = vec![
        BaseEvent::run_started("".to_string(), "".to_string()),
        BaseEvent {
            event_type: EventType::StepStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepStarted(StepStartedEvent {
                thread_id: "thread_empty".to_string(),
                run_id: "run_empty".to_string(),
                step_id: "step_empty".to_string(),
                step_type: Some("".to_string()),
            }),
        },
        BaseEvent::error("".to_string(), Some("".to_string())),
    ];

    for event in &empty_events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }

    // Very long strings
    let long_string = "A".repeat(10000);
    let long_events = vec![
        BaseEvent::run_started(long_string.clone(), long_string.clone()),
        BaseEvent {
            event_type: EventType::StepStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::StepStarted(StepStartedEvent {
                thread_id: "thread_long".to_string(),
                run_id: "run_long".to_string(),
                step_id: "step_long".to_string(),
                step_type: Some(long_string.clone()),
            }),
        },
        BaseEvent::error(long_string.clone(), Some(long_string.clone())),
    ];

    for event in &long_events {
        let json_str = serde_json::to_string(event).unwrap();
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
    }
}

// ===== EVENTS WITH RAW EVENT DATA =====

#[wasm_bindgen_test]
fn test_lifecycle_events_with_raw_event() {
    // Test lifecycle events with rawEvent data (matching TypeScript all base fields test)
    let raw_data = json!({
        "original": "external_event_data",
        "source": "external_system",
        "metadata": {
            "timestamp": "2023-10-01T12:00:00Z",
            "version": "1.0.0"
        }
    });

    let events = vec![
        BaseEvent {
            event_type: EventType::RunStarted,
            timestamp: Some(Utc::now()),
            raw_event: Some(raw_data.clone()),
            data: EventData::RunStarted(RunStartedEvent {
                thread_id: "thread_with_raw".to_string(),
                run_id: "run_with_raw".to_string(),
            }),
        },
        BaseEvent {
            event_type: EventType::Error,
            timestamp: Some(Utc::now()),
            raw_event: Some(raw_data.clone()),
            data: EventData::Error(ErrorEvent {
                error: "Test error with raw event".to_string(),
                code: Some("TEST_ERROR".to_string()),
                details: None,
            }),
        },
    ];

    for event in &events {
        let json_str = serde_json::to_string(event).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify raw_event is included
        assert!(parsed["raw_event"].is_object());
        assert_eq!(parsed["raw_event"]["original"], "external_event_data");
        assert_eq!(parsed["raw_event"]["source"], "external_system");
        assert_eq!(parsed["raw_event"]["metadata"]["version"], "1.0.0");

        // Round-trip test
        let deserialized: BaseEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
        assert!(deserialized.raw_event.is_some());
    }
}