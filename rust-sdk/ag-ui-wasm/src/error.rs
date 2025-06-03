use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum AgUiError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Stream error: {0}")]
    StreamError(String),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Agent error: {0}")]
    AgentError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Encoding error: {0}")]
    EncodingError(String),
    
    #[error("WASM bindgen error: {0}")]
    WasmBindgenError(String),
}

impl From<JsValue> for AgUiError {
    fn from(value: JsValue) -> Self {
        if let Some(s) = value.as_string() {
            AgUiError::WasmBindgenError(s)
        } else {
            AgUiError::WasmBindgenError("Unknown JS error".to_string())
        }
    }
}

impl From<AgUiError> for JsValue {
    fn from(error: AgUiError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl From<serde_wasm_bindgen::Error> for AgUiError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        AgUiError::WasmBindgenError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AgUiError>; 