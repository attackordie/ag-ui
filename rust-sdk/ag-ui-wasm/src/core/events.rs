use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::types::{Message, Role, State, ToolCall, ToolResult};
use wasm_bindgen::prelude::*;

/// Event types in the AG-UI protocol
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    // Lifecycle events
    RunStarted,
    RunFinished,
    RunAborted,
    
    // Message events
    TextMessageStart,
    TextMessageContent,
    TextMessageChunk,
    TextMessageEnd,
    MessagesSnapshot,
    
    // Tool events
    ToolCallStart,
    ToolCallChunk,
    ToolCallEnd,
    ToolCallResult,
    
    // State events
    StateSnapshot,
    StateDelta,
    
    // Error events
    Error,
}

/// Base event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    #[serde(rename = "type")]
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_event: Option<serde_json::Value>,
    #[serde(flatten)]
    pub data: EventData,
}

/// Event data variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    RunStarted(RunStartedEvent),
    RunFinished(RunFinishedEvent),
    RunAborted(RunAbortedEvent),
    TextMessageStart(TextMessageStartEvent),
    TextMessageContent(TextMessageContentEvent),
    TextMessageChunk(TextMessageChunkEvent),
    TextMessageEnd(TextMessageEndEvent),
    MessagesSnapshot(MessagesSnapshotEvent),
    ToolCallStart(ToolCallStartEvent),
    ToolCallChunk(ToolCallChunkEvent),
    ToolCallEnd(ToolCallEndEvent),
    ToolCallResult(ToolCallResultEvent),
    StateSnapshot(StateSnapshotEvent),
    StateDelta(StateDeltaEvent),
    Error(ErrorEvent),
}

// Event structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunStartedEvent {
    pub thread_id: String,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunFinishedEvent {
    pub thread_id: String,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAbortedEvent {
    pub thread_id: String,
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessageStartEvent {
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessageContentEvent {
    pub message_id: String,
    pub delta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessageChunkEvent {
    pub message_id: String,
    pub delta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessageEndEvent {
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesSnapshotEvent {
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallStartEvent {
    pub tool_call_id: String,
    pub tool_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallChunkEvent {
    pub tool_call_id: String,
    pub delta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallEndEvent {
    pub tool_call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call: Option<ToolCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResultEvent {
    pub tool_result: ToolResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshotEvent {
    pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDeltaEvent {
    pub delta: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

// Helper implementations
impl BaseEvent {
    pub fn run_started(thread_id: String, run_id: String) -> Self {
        Self {
            event_type: EventType::RunStarted,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::RunStarted(RunStartedEvent { thread_id, run_id }),
        }
    }
    
    pub fn run_finished(thread_id: String, run_id: String) -> Self {
        Self {
            event_type: EventType::RunFinished,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::RunFinished(RunFinishedEvent { thread_id, run_id }),
        }
    }
    
    pub fn text_message_content(message_id: String, delta: String) -> Self {
        Self {
            event_type: EventType::TextMessageContent,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent { message_id, delta }),
        }
    }
    
    pub fn text_message_start(message_id: String, role: Option<Role>) -> Self {
        Self {
            event_type: EventType::TextMessageStart,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent { message_id, role }),
        }
    }
    
    pub fn text_message_end(message_id: String) -> Self {
        Self {
            event_type: EventType::TextMessageEnd,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::TextMessageEnd(TextMessageEndEvent { message_id }),
        }
    }
    
    pub fn error(error: String, code: Option<String>) -> Self {
        Self {
            event_type: EventType::Error,
            timestamp: Some(Utc::now()),
            raw_event: None,
            data: EventData::Error(ErrorEvent { error, code, details: None }),
        }
    }
} 