use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};
use crate::client::agent::Agent;
use crate::core::types::{Message, RunAgentInput, State};
use crate::stream::EventStream;
use crate::error::{AgUiError, Result};
use js_sys::Promise;

/// Web-based AG-UI agent client using Fetch API
#[wasm_bindgen]
pub struct WebAgent {
    url: String,
    agent_id: Option<String>,
    thread_id: Option<String>,
    messages: Vec<Message>,
    state: State,
}

#[wasm_bindgen]
impl WebAgent {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Self {
        Self {
            url,
            agent_id: None,
            thread_id: None,
            messages: Vec::new(),
            state: State::new(),
        }
    }
    
    #[wasm_bindgen(js_name = "setAgentId")]
    pub fn set_agent_id(&mut self, agent_id: String) {
        self.agent_id = Some(agent_id);
    }
    
    #[wasm_bindgen(js_name = "setThreadId")]
    pub fn set_thread_id(&mut self, thread_id: String) {
        self.thread_id = Some(thread_id);
    }
    
    #[wasm_bindgen(js_name = "runAgent")]
    pub fn run_agent_js(&self, input_js: JsValue) -> Promise {
        let input: RunAgentInput = match serde_wasm_bindgen::from_value(input_js) {
            Ok(input) => input,
            Err(e) => return Promise::reject(&JsValue::from_str(&format!("Invalid RunAgentInput: {}", e))),
        };
        
        let url = self.url.clone();
        let messages = self.messages.clone();
        let state = self.state.clone();
        
        wasm_bindgen_futures::future_to_promise(async move {
            let stream = run_agent_internal(url, input, messages, state).await
                .map_err(|e| JsValue::from(e))?;
            Ok(stream.into())
        })
    }
}

// Internal implementation
async fn run_agent_internal(
    url: String,
    mut input: RunAgentInput,
    messages: Vec<Message>,
    state: State,
) -> Result<EventStream> {
    // Add current messages if not provided
    if input.messages.is_none() && !messages.is_empty() {
        input.messages = Some(messages);
    }
    
    // Add current state if not provided
    if input.state.is_none() && !state.is_empty() {
        input.state = Some(state);
    }
    
    // Create request options
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    // Set headers
    let headers = Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    headers.set("Accept", "text/event-stream")?;
    opts.set_headers(&headers);
    
    // Set body
    let body = serde_json::to_string(&input)?;
    let body_js = JsValue::from_str(&body);
    opts.set_body(&body_js);
    
    // Create and send request
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let response_promise = window.fetch_with_request(&request);
    let response = JsFuture::from(response_promise).await?;
    let response: Response = response.dyn_into()?;
    
    if !response.ok() {
        return Err(AgUiError::ConnectionError(format!(
            "HTTP {}: {}",
            response.status(),
            response.status_text()
        )));
    }
    
    // Get the response body as a ReadableStream
    let body = response.body()
        .ok_or_else(|| AgUiError::StreamError("No response body".to_string()))?;
    
    // Create EventStream from ReadableStream
    EventStream::from_readable_stream(body)
}

impl Agent for WebAgent {
    fn run_agent(&self, _input: RunAgentInput) -> Result<EventStream> {
        // For the sync trait, we need to create a stream that can be populated later
        // This is a limitation of the current design - in practice you'd use the async JS method
        Err(AgUiError::AgentError(
            "Use run_agent_js() for async operation in WASM environment".to_string()
        ))
    }
    
    fn messages(&self) -> Vec<Message> {
        self.messages.clone()
    }
    
    fn set_messages(&mut self, messages: Vec<Message>) {
        self.messages = messages;
    }
    
    fn state(&self) -> State {
        self.state.clone()
    }
    
    fn set_state(&mut self, state: State) {
        self.state = state;
    }
    
    fn agent_id(&self) -> Option<String> {
        self.agent_id.clone()
    }
    
    fn thread_id(&self) -> Option<String> {
        self.thread_id.clone()
    }
} 