use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use crate::core::events::BaseEvent;
use crate::error::{AgUiError, Result};

/// Server-Sent Events encoder for WASM
#[wasm_bindgen]
pub struct SseEncoder {
    encoder: web_sys::TextEncoder,
}

#[wasm_bindgen]
impl SseEncoder {
    /// Create a new SSE encoder for JavaScript
    #[wasm_bindgen(constructor)]
    pub fn new() -> SseEncoder {
        let encoder = web_sys::TextEncoder::new().unwrap();
        Self { encoder }
    }
    
    /// Encode an event from JavaScript
    #[wasm_bindgen(js_name = "encodeEvent")]
    pub fn encode_event_js_export(&self, event_js: JsValue) -> std::result::Result<Uint8Array, JsValue> {
        let event: BaseEvent = serde_wasm_bindgen::from_value(event_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let json_data = serde_json::to_string(&event)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let sse_data = format!("data: {}\n\n", json_data);
        
        let encoded = self.encoder.encode_with_input(&sse_data);
        Ok(Uint8Array::from(encoded.as_slice()))
    }
    
    /// Encode a message from JavaScript
    #[wasm_bindgen(js_name = "encodeMessage")]
    pub fn encode_message_js(&self, message: &str) -> Uint8Array {
        let sse_data = format!("data: {}\n\n", message);
        let encoded = self.encoder.encode_with_input(&sse_data);
        Uint8Array::from(encoded.as_slice())
    }
    
    /// Encode a comment from JavaScript
    #[wasm_bindgen(js_name = "encodeComment")]
    pub fn encode_comment_js(&self, comment: &str) -> Uint8Array {
        let sse_data = format!(": {}\n", comment);
        let encoded = self.encoder.encode_with_input(&sse_data);
        Uint8Array::from(encoded.as_slice())
    }
    
    /// Encode a ping from JavaScript
    #[wasm_bindgen(js_name = "encodePing")]
    pub fn encode_ping_js(&self) -> Uint8Array {
        let sse_data = ": ping\n\n";
        let encoded = self.encoder.encode_with_input(sse_data);
        Uint8Array::from(encoded.as_slice())
    }
}

impl SseEncoder {
    /// Create a new SSE encoder (internal Rust API)
    pub fn new_internal() -> Result<Self> {
        let encoder = web_sys::TextEncoder::new()
            .map_err(|e| AgUiError::EncodingError(format!("Failed to create TextEncoder: {:?}", e)))?;
        
        Ok(Self { encoder })
    }
    
    /// Encode an event as SSE data (internal API)
    pub fn encode_event(&self, event: &BaseEvent) -> Result<Uint8Array> {
        let json_data = serde_json::to_string(event)?;
        let sse_data = format!("data: {}\n\n", json_data);
        
        let encoded = self.encoder.encode_with_input(&sse_data);
        Ok(Uint8Array::from(encoded.as_slice()))
    }
    
    /// Encode a message as SSE format (internal API)
    pub fn encode_message(&self, message: &str) -> Result<Uint8Array> {
        let sse_data = format!("data: {}\n\n", message);
        let encoded = self.encoder.encode_with_input(&sse_data);
        Ok(Uint8Array::from(encoded.as_slice()))
    }
    
    /// Encode a comment line (internal API)
    pub fn encode_comment(&self, comment: &str) -> Result<Uint8Array> {
        let sse_data = format!(": {}\n", comment);
        let encoded = self.encoder.encode_with_input(&sse_data);
        Ok(Uint8Array::from(encoded.as_slice()))
    }
    
    /// Encode a keep-alive ping (internal API)
    pub fn encode_ping(&self) -> Result<Uint8Array> {
        let sse_data = ": ping\n\n";
        let encoded = self.encoder.encode_with_input(sse_data);
        Ok(Uint8Array::from(encoded.as_slice()))
    }
    
    /// Encode an event as SSE string format
    pub fn encode_event_string(event: &BaseEvent) -> Result<String> {
        let json = serde_json::to_string(event)?;
        Ok(format!("data: {}\n\n", json))
    }
    
    /// Encode multiple events as SSE string format
    pub fn encode_events_string(events: &[BaseEvent]) -> Result<String> {
        let mut result = String::new();
        for event in events {
            result.push_str(&Self::encode_event_string(event)?);
        }
        Ok(result)
    }
    
    /// Convert SSE string to Uint8Array for JavaScript
    pub fn to_uint8_array(sse_data: &str) -> std::result::Result<Uint8Array, JsValue> {
        let bytes = sse_data.as_bytes();
        Ok(Uint8Array::from(bytes))
    }
}

impl Default for SseEncoder {
    fn default() -> Self {
        Self::new()
    }
} 