//! AG-UI SDK for V8 Isolates (WASM)
//! 
//! This SDK implements the Agent-User Interaction Protocol for browser
//! and Cloudflare Workers environments.

use wasm_bindgen::prelude::*;

pub mod client;
pub mod core;
pub mod encoder;
pub mod stream;
pub mod error;

// Re-export commonly used types
pub use client::web_agent::WebAgent;
pub use core::{
    events::{
        BaseEvent, EventType, EventData,
        TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent, TextMessageChunkEvent,
        ToolCallStartEvent, ToolCallChunkEvent, ToolCallEndEvent, ToolCallArgsEvent, ToolCallResultEvent,
        StateSnapshotEvent, StateDeltaEvent, MessagesSnapshotEvent,
        RunStartedEvent, RunFinishedEvent, RunAbortedEvent, RunErrorEvent,
        StepStartedEvent, StepFinishedEvent,
        ThinkingStartEvent, ThinkingEndEvent, 
        ThinkingTextMessageStartEvent, ThinkingTextMessageContentEvent, ThinkingTextMessageEndEvent,
        ErrorEvent, RawEvent, CustomEvent
    },
    types::{Message, RunAgentInput, State, Role, Tool, Context, ToolCall, ToolResult, FunctionCall},
};
pub use encoder::SseEncoder as SSEEncoder;
pub use error::{AgUiError, Result};
pub use stream::EventStream;

// Set panic hook for better error messages in browser
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
} 