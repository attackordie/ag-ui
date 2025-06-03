use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStream, ReadableStreamDefaultReader, TextDecoder, TransformStream, TransformStreamDefaultController};
use js_sys::Uint8Array;
use crate::core::events::BaseEvent;
use crate::error::{AgUiError, Result};

/// A stream of server-sent events
#[wasm_bindgen]
pub struct EventStream {
    reader: ReadableStreamDefaultReader,
    decoder: TextDecoder,
}

#[wasm_bindgen]
impl EventStream {
    /// Convert EventStream to JsValue for JavaScript interop
    #[wasm_bindgen(js_name = "toJsValue")]
    pub fn to_js_value(&self) -> JsValue {
        // Return a placeholder for now
        JsValue::NULL
    }
}

impl EventStream {
    /// Create a new EventStream from a ReadableStream
    pub fn from_readable_stream(stream: ReadableStream) -> Result<Self> {
        let reader = stream.get_reader()
            .dyn_into::<ReadableStreamDefaultReader>()
            .map_err(|e| AgUiError::StreamError(format!("Failed to get reader: {:?}", e)))?;
        
        let decoder = TextDecoder::new()
            .map_err(|e| AgUiError::StreamError(format!("Failed to create decoder: {:?}", e)))?;
        
        Ok(Self { reader, decoder })
    }
    
    /// Read the next event from the stream
    pub async fn next_event(&self) -> Result<Option<BaseEvent>> {
        let result = JsFuture::from(self.reader.read()).await?;
        let chunk = js_sys::Reflect::get(&result, &JsValue::from_str("value"))
            .map_err(|e| AgUiError::StreamError(format!("Failed to get chunk value: {:?}", e)))?;
        
        let done = js_sys::Reflect::get(&result, &JsValue::from_str("done"))
            .map_err(|e| AgUiError::StreamError(format!("Failed to get done flag: {:?}", e)))?
            .as_bool()
            .unwrap_or(false);
        
        if done {
            return Ok(None);
        }
        
        let chunk = chunk.dyn_into::<Uint8Array>()
            .map_err(|e| AgUiError::StreamError(format!("Failed to convert to Uint8Array: {:?}", e)))?;
        
        // Convert Uint8Array to Vec<u8> and then back to slice for decoding
        let chunk_vec = chunk.to_vec();
        
        // Use the decode method with a slice
        let text = self.decoder.decode_with_u8_array(&chunk_vec)
            .map_err(|e| AgUiError::StreamError(format!("Failed to decode text: {:?}", e)))?;
        
        // Parse the SSE data
        if text.starts_with("data: ") {
            let json_str = &text[6..]; // Remove "data: " prefix
            let event: BaseEvent = serde_json::from_str(json_str)
                .map_err(|e| AgUiError::JsonError(e))?;
            Ok(Some(event))
        } else {
            // Skip non-data lines (comments, event types, etc.)
            Ok(None)
        }
    }
    
    /// Create a TransformStream for processing events
    pub fn create_transform_stream() -> Result<TransformStream> {
        let decoder = TextDecoder::new()
            .map_err(|e| AgUiError::StreamError(format!("Failed to create decoder: {:?}", e)))?;
        
        let transformer = js_sys::Object::new();
        
        let transform_fn = Closure::wrap(Box::new(move |chunk: JsValue, controller: TransformStreamDefaultController| -> std::result::Result<(), JsValue> {
            let uint8_array = chunk.dyn_into::<Uint8Array>()?;
            let chunk_vec = uint8_array.to_vec();
            
            let text = decoder.decode_with_u8_array(&chunk_vec)?;
            
            // Parse SSE data and extract events
            for line in text.lines() {
                if line.starts_with("data: ") {
                    let json_str = &line[6..];
                    if let Ok(event) = serde_json::from_str::<BaseEvent>(json_str) {
                        let js_event = serde_wasm_bindgen::to_value(&event)
                            .map_err(|e| JsValue::from_str(&e.to_string()))?;
                        controller.enqueue_with_chunk(&js_event)?;
                    }
                }
            }
            
            Ok(())
        }) as Box<dyn FnMut(JsValue, TransformStreamDefaultController) -> std::result::Result<(), JsValue>>);
        
        js_sys::Reflect::set(
            &transformer,
            &JsValue::from_str("transform"),
            transform_fn.as_ref().unchecked_ref(),
        )?;
        
        transform_fn.forget();
        
        let transform_stream = TransformStream::new_with_transformer(&transformer)
            .map_err(|e| AgUiError::StreamError(format!("Failed to create transform stream: {:?}", e)))?;
        
        Ok(transform_stream)
    }
    
    /// Create an EventStream from an async function
    pub fn from_async_fn<F, Fut>(_f: F) -> Result<Self>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        // This would need more complex implementation for creating streams from async functions
        // For now, we'll just return an error
        Err(AgUiError::StreamError("from_async_fn not implemented".to_string()))
    }
} 