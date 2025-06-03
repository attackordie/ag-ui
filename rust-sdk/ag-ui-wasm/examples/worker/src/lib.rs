use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Request, Response, ResponseInit, Headers};
use ag_ui_wasm::{
    BaseEvent, EventType, EventData, RunAgentInput, 
    SSEEncoder, Message, Role,
    core::events::{
        TextMessageStartEvent, TextMessageContentEvent, 
        TextMessageEndEvent, RunStartedEvent, RunFinishedEvent
    }
};
use uuid::Uuid;

/// Handle incoming requests
#[wasm_bindgen]
pub fn handle_request(request: Request) -> js_sys::Promise {
    future_to_promise(async move {
        let url = request.url();
        
        if url.contains("/awp") && request.method() == "POST" {
            handle_agent_request(request).await
        } else {
            create_error_response("Not found", 404)
        }
    })
}

async fn handle_agent_request(request: Request) -> Result<Response, JsValue> {
    // Parse request body
    let body = wasm_bindgen_futures::JsFuture::from(request.json()?).await?;
    let input: RunAgentInput = serde_wasm_bindgen::from_value(body)?;
    
    // Create SSE stream
    let stream = create_agent_stream(input)?;
    
    // Create response headers
    let headers = Headers::new()?;
    headers.set("Content-Type", "text/event-stream")?;
    headers.set("Cache-Control", "no-cache")?;
    headers.set("Transfer-Encoding", "chunked")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    
    // Create response
    let mut init = ResponseInit::new();
    init.status(200);
    init.headers(&headers);
    
    Response::new_with_opt_readable_stream_and_init(Some(&stream), &init)
}

fn create_agent_stream(input: RunAgentInput) -> Result<web_sys::ReadableStream, JsValue> {
    let encoder = SSEEncoder::new()?;
    let thread_id = input.thread_id.clone();
    let run_id = input.run_id.clone();
    
    let source = js_sys::Object::new();
    
    // Start function
    let start = Closure::wrap(Box::new(move |controller: web_sys::ReadableStreamDefaultController| -> Result<(), JsValue> {
        // Send RUN_STARTED
        let event = BaseEvent {
            event_type: EventType::RunStarted,
            timestamp: None,
            raw_event: None,
            data: EventData::RunStarted(RunStartedEvent {
                thread_id: thread_id.clone(),
                run_id: run_id.clone(),
            }),
        };
        let encoded = encoder.encode_event(&event)?;
        controller.enqueue_with_array_buffer_view(&encoded)?;
        
        // Simulate message generation
        let message_id = Uuid::new_v4().to_string();
        
        // TEXT_MESSAGE_START
        let event = BaseEvent {
            event_type: EventType::TextMessageStart,
            timestamp: None,
            raw_event: None,
            data: EventData::TextMessageStart(TextMessageStartEvent {
                message_id: message_id.clone(),
                role: Some(Role::Assistant),
            }),
        };
        let encoded = encoder.encode_event(&event)?;
        controller.enqueue_with_array_buffer_view(&encoded)?;
        
        // Send message content
        let content = "Hello! I'm an AG-UI agent running in a Cloudflare Worker.";
        let event = BaseEvent {
            event_type: EventType::TextMessageContent,
            timestamp: None,
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent {
                message_id: message_id.clone(),
                delta: content.to_string(),
            }),
        };
        let encoded = encoder.encode_event(&event)?;
        controller.enqueue_with_array_buffer_view(&encoded)?;
        
        // TEXT_MESSAGE_END
        let event = BaseEvent {
            event_type: EventType::TextMessageEnd,
            timestamp: None,
            raw_event: None,
            data: EventData::TextMessageEnd(TextMessageEndEvent {
                message_id,
            }),
        };
        let encoded = encoder.encode_event(&event)?;
        controller.enqueue_with_array_buffer_view(&encoded)?;
        
        // RUN_FINISHED
        let event = BaseEvent {
            event_type: EventType::RunFinished,
            timestamp: None,
            raw_event: None,
            data: EventData::RunFinished(RunFinishedEvent {
                thread_id,
                run_id,
            }),
        };
        let encoded = encoder.encode_event(&event)?;
        controller.enqueue_with_array_buffer_view(&encoded)?;
        
        // Close stream
        controller.close()?;
        Ok(())
    }) as Box<dyn FnMut(_) -> Result<(), JsValue>>);
    
    js_sys::Reflect::set(&source, &"start".into(), start.as_ref())?;
    start.forget();
    
    web_sys::ReadableStream::new_with_underlying_source(&source)
}

fn create_error_response(message: &str, status: u16) -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    init.status(status);
    Response::new_with_opt_str_and_init(Some(message), &init)
} 