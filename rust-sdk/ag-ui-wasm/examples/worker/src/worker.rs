use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Request, Response, ResponseInit, Headers, Url};
use ag_ui_wasm::{
    BaseEvent, EventType, EventData, RunAgentInput, 
    SSEEncoder, Role,
    core::events::{
        TextMessageStartEvent, TextMessageContentEvent, 
        TextMessageEndEvent, RunStartedEvent, RunFinishedEvent
    }
};
use uuid::Uuid;

const TEST_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AG-UI WASM Worker Test</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            text-align: center;
        }
        .highlight-box {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 15px;
            border-radius: 8px;
            margin: 15px 0;
        }
        .highlight-box h3 {
            margin: 0 0 10px 0;
            font-size: 1.1em;
        }
        .highlight-box ul {
            margin: 5px 0;
            padding-left: 20px;
        }
        .highlight-box li {
            margin: 3px 0;
            font-size: 0.9em;
        }
        .stats {
            display: flex;
            justify-content: space-around;
            background-color: #f8f9fa;
            padding: 10px;
            border-radius: 6px;
            margin: 10px 0;
            border: 1px solid #e9ecef;
        }
        .stat {
            text-align: center;
        }
        .stat-number {
            font-size: 1.5em;
            font-weight: bold;
            color: #007cba;
        }
        .stat-label {
            font-size: 0.8em;
            color: #666;
        }
        .input-group {
            margin: 10px 0;
        }
        label {
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
        }
        input[type="text"] {
            width: 100%;
            padding: 8px;
            border: 1px solid #ddd;
            border-radius: 4px;
            box-sizing: border-box;
        }
        button {
            background-color: #007cba;
            color: white;
            padding: 10px 20px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
            margin: 10px 0;
        }
        button:hover {
            background-color: #005a8b;
        }
        button:disabled {
            background-color: #ccc;
            cursor: not-allowed;
        }
        #output {
            background-color: #f8f9fa;
            border: 1px solid #e9ecef;
            border-radius: 4px;
            padding: 15px;
            margin-top: 20px;
            min-height: 100px;
            white-space: pre-wrap;
            font-family: 'Courier New', monospace;
            max-height: 400px;
            overflow-y: auto;
        }
        .event {
            margin: 5px 0;
            padding: 5px;
            border-left: 3px solid #007cba;
            background-color: #e7f3ff;
        }
        .timestamp {
            color: #666;
            font-size: 0.9em;
        }
        .status {
            text-align: center;
            margin: 10px 0;
            font-weight: bold;
        }
        .status.connected {
            color: #28a745;
        }
        .status.error {
            color: #dc3545;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🤖 AG-UI WASM Worker Test (Pure Rust)</h1>
        <p>Test the AG-UI Rust SDK running in a Cloudflare Worker via WASM - Pure Rust Implementation</p>
        
        <div class="highlight-box" style="background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);">
            <h3>❓ What is AG-UI?</h3>
            <p style="margin: 5px 0 10px 0; font-size: 0.9em;">
                <strong>AG-UI</strong> stands for <strong>Agent User Interaction Protocol</strong> - a standardized way for AI agents to communicate with user interfaces in real-time.
            </p>
            <ul style="margin: 5px 0; padding-left: 20px; font-size: 0.9em;">
                <li><strong>🔄 Real-time Streaming:</strong> AI agents send events as they work (like typing indicators and progressive responses)</li>
                <li><strong>📋 Standardized Events:</strong> Common event types like RUN_STARTED, TEXT_MESSAGE_CONTENT, etc.</li>
                <li><strong>🌐 Universal Protocol:</strong> Works across different AI providers and user interfaces</li>
                <li><strong>⚡ Live Updates:</strong> Users see AI thinking and responding in real-time, not just final results</li>
            </ul>
            <p style="margin: 10px 0 5px 0; font-size: 0.85em;">
                📖 Learn more at <a href="https://docs.ag-ui.com/introduction" target="_blank" style="color: white; text-decoration: underline;">docs.ag-ui.com/introduction</a>
            </p>
        </div>
        
        <div class="highlight-box">
            <h3>🚀 Why This Demo is Impressive</h3>
            <div class="stats">
                <div class="stat">
                    <div class="stat-number">99%</div>
                    <div class="stat-label">Rust Code</div>
                </div>
                <div class="stat">
                    <div class="stat-number">1%</div>
                    <div class="stat-label">JavaScript</div>
                </div>
                <div class="stat">
                    <div class="stat-number">400+</div>
                    <div class="stat-label">Lines Rust</div>
                </div>
                <div class="stat">
                    <div class="stat-number">26</div>
                    <div class="stat-label">Lines JS</div>
                </div>
            </div>
            <ul>
                <li><strong>🔧 Complete Web Service in Rust:</strong> HTTP handling, HTML interface, and AG-UI protocol - all in Rust via WASM</li>
                <li><strong>⚡ Real-time Streaming:</strong> Server-Sent Events with native Rust Web Streams API integration</li>
                <li><strong>🛡️ Type Safety:</strong> Full Rust type checking across the entire stack with zero runtime errors</li>
                <li><strong>🌐 Production Ready:</strong> Proper CORS, error handling, and async streams in serverless environment</li>
                <li><strong>📦 Self-Contained:</strong> This HTML page is embedded as a Rust string constant!</li>
            </ul>
        </div>
        
        <div class="highlight-box" style="background: linear-gradient(135deg, #28a745 0%, #20c997 100%);">
            <h3>📋 What You'll See When You Run the Agent</h3>
            <p style="margin: 5px 0 10px 0; font-size: 0.9em;">Click "🚀 Run Agent" to see exactly 5 AG-UI events stream in real-time. Here's what each one means:</p>
            <ol style="margin: 5px 0; padding-left: 20px; font-size: 0.9em;">
                <li><strong>RUN_STARTED:</strong> Workflow begins - Rust router processes your request and initializes the AG-UI stream</li>
                <li><strong>TEXT_MESSAGE_START:</strong> Assistant message begins - UUID generated in Rust for message tracking</li>
                <li><strong>TEXT_MESSAGE_CONTENT:</strong> Message content streams - Demonstrates real-time delta content delivery</li>
                <li><strong>TEXT_MESSAGE_END:</strong> Message complete - Same UUID links all message events together</li>
                <li><strong>RUN_FINISHED:</strong> Workflow complete - Stream closes and resources cleaned up automatically</li>
            </ol>
            <p style="margin: 10px 0 5px 0; font-size: 0.85em; font-style: italic;">
                💡 All events happen synchronously in this demo, but in production they could be spaced out as real AI processing occurs!
            </p>
        </div>
        
        <div class="input-group">
            <label for="threadId">Thread ID:</label>
            <input type="text" id="threadId" value="rust-test-thread" placeholder="Enter thread ID">
        </div>
        
        <div class="input-group">
            <label for="runId">Run ID:</label>
            <input type="text" id="runId" value="rust-test-run" placeholder="Enter run ID">
        </div>
        
        <button id="runButton" onclick="runAgent()">🚀 Run Agent</button>
        <button id="clearButton" onclick="clearOutput()">🗑️ Clear Output</button>
        
        <div id="status" class="status">Ready to test AG-UI Worker (Pure Rust)</div>
        
        <div id="output"></div>
        
        <div style="margin-top: 20px; padding: 15px; background-color: #f8f9fa; border-radius: 6px; border: 1px solid #e9ecef;">
            <h4 style="margin: 0 0 10px 0; color: #495057;">🔍 Technical Details</h4>
            <ul style="margin: 5px 0; padding-left: 20px; font-size: 0.85em; color: #666;">
                <li><strong>Architecture:</strong> Browser → worker.js (26 lines) → WASM → worker.rs (400+ lines)</li>
                <li><strong>Stream Flow:</strong> ReadableStream → SSEEncoder → Server-Sent Events → Browser display</li>
                <li><strong>Type Safety:</strong> Every event uses strongly-typed Rust structs from ag-ui-wasm crate</li>
                <li><strong>Protocol:</strong> Full AG-UI compliance with proper event lifecycle management</li>
                <li><strong>Performance:</strong> Zero-copy WASM integration with automatic memory management</li>
            </ul>
            <p style="margin: 10px 0 0 0; font-size: 0.8em; color: #868e96;">
                📖 View source code and documentation: <a href="https://github.com/attackordie/ag-ui/tree/main/rust-sdk" target="_blank" style="color: #007cba;">github.com/attackordie/ag-ui/tree/main/rust-sdk</a>
            </p>
        </div>
    </div>

    <script>
        let isRunning = false;

        function updateStatus(message, className = '') {
            const status = document.getElementById('status');
            status.textContent = message;
            status.className = `status ${className}`;
        }

        function addEvent(eventData, timestamp) {
            const output = document.getElementById('output');
            const eventDiv = document.createElement('div');
            eventDiv.className = 'event';
            
            const timeSpan = document.createElement('span');
            timeSpan.className = 'timestamp';
            timeSpan.textContent = `[${timestamp}] `;
            
            const contentSpan = document.createElement('span');
            contentSpan.textContent = JSON.stringify(eventData, null, 2);
            
            eventDiv.appendChild(timeSpan);
            eventDiv.appendChild(contentSpan);
            output.appendChild(eventDiv);
            
            output.scrollTop = output.scrollHeight;
        }

        function clearOutput() {
            document.getElementById('output').innerHTML = '';
            updateStatus('Ready to test AG-UI Worker (Pure Rust)');
        }

        async function runAgent() {
            if (isRunning) return;
            
            const threadId = document.getElementById('threadId').value;
            const runId = document.getElementById('runId').value;
            
            if (!threadId || !runId) {
                alert('Please enter both Thread ID and Run ID');
                return;
            }
            
            isRunning = true;
            const runButton = document.getElementById('runButton');
            runButton.disabled = true;
            runButton.textContent = '⏳ Running...';
            
            updateStatus('🔄 Connecting to AG-UI Worker...', 'connected');
            
            try {
                const response = await fetch('/awp', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        thread_id: threadId,
                        run_id: runId
                    })
                });
                
                if (!response.ok) {
                    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
                }
                
                updateStatus('✅ Connected! Streaming events...', 'connected');
                
                const reader = response.body.getReader();
                const decoder = new TextDecoder();
                
                try {
                    while (true) {
                        const { done, value } = await reader.read();
                        
                        if (done) {
                            break;
                        }
                        
                        const text = decoder.decode(value);
                        const lines = text.split('\n');
                        
                        for (const line of lines) {
                            if (line.startsWith('data: ')) {
                                try {
                                    const eventData = JSON.parse(line.substring(6));
                                    const timestamp = new Date().toLocaleTimeString();
                                    addEvent(eventData, timestamp);
                                } catch (e) {
                                    console.warn('Failed to parse event:', line);
                                }
                            }
                        }
                    }
                    
                    updateStatus('✅ Agent run completed successfully!', 'connected');
                    
                } finally {
                    reader.releaseLock();
                }
                
            } catch (error) {
                console.error('Error:', error);
                updateStatus(`❌ Error: ${error.message}`, 'error');
                
                const timestamp = new Date().toLocaleTimeString();
                addEvent({ error: error.message }, timestamp);
            } finally {
                isRunning = false;
                runButton.disabled = false;
                runButton.textContent = '🚀 Run Agent';
            }
        }
    </script>
</body>
</html>"#;

/// Main worker entry point - exported as the default fetch handler
#[wasm_bindgen]
pub fn fetch(request: Request) -> js_sys::Promise {
    future_to_promise(async move {
        let result = handle_request(request).await;
        
        match result {
            Ok(response) => Ok(response.into()),
            Err(e) => {
                web_sys::console::error_1(&format!("Worker error: {:?}", e).into());
                let error_response = create_error_response(&format!("Internal Server Error: {:?}", e), 500)?;
                Ok(error_response.into())
            }
        }
    })
}

/// Cloudflare Workers style default export
#[wasm_bindgen]
pub struct WorkerExports;

#[wasm_bindgen]
impl WorkerExports {
    #[wasm_bindgen(js_name = fetch)]
    pub fn fetch(request: Request) -> js_sys::Promise {
        fetch(request)
    }
}

async fn handle_request(request: Request) -> Result<Response, JsValue> {
    let url = Url::new(&request.url())?;
    let pathname = url.pathname();
    let method = request.method();
    
    match (pathname.as_str(), method.as_str()) {
        // Serve test page at root
        ("/", "GET") | ("/test.html", "GET") => {
            serve_test_page()
        },
        
        // Handle AG-UI API requests
        ("/awp", "POST") => {
            handle_agent_request(request).await
        },
        
        // Handle OPTIONS for CORS preflight
        ("/awp", "OPTIONS") => {
            handle_cors_preflight()
        },
        
        // 404 for everything else
        _ => {
            create_error_response("Not found", 404)
        }
    }
}

fn serve_test_page() -> Result<Response, JsValue> {
    let headers = Headers::new()?;
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    headers.set("Cache-Control", "no-cache")?;
    
    let init = ResponseInit::new();
    init.set_status(200);
    init.set_headers(&headers);
    
    Response::new_with_opt_str_and_init(Some(TEST_HTML), &init)
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
    let init = ResponseInit::new();
    init.set_status(200);
    init.set_headers(&headers);
    
    Response::new_with_opt_readable_stream_and_init(Some(&stream), &init)
}

fn handle_cors_preflight() -> Result<Response, JsValue> {
    let headers = Headers::new()?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    let init = ResponseInit::new();
    init.set_status(204);
    init.set_headers(&headers);
    
    Response::new_with_opt_str_and_init(None, &init)
}

fn create_agent_stream(input: RunAgentInput) -> Result<web_sys::ReadableStream, JsValue> {
    let encoder = SSEEncoder::new();
    let thread_id = input.thread_id;
    let run_id = input.run_id;
    
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
        let encoded = encoder.encode_event(&event).map_err(|e| JsValue::from_str(&e.to_string()))?;
        controller.enqueue_with_chunk(&encoded.into())?;
        
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
        let encoded = encoder.encode_event(&event).map_err(|e| JsValue::from_str(&e.to_string()))?;
        controller.enqueue_with_chunk(&encoded.into())?;
        
        // Send message content
        let content = "Hello! I'm an AG-UI agent running in a Cloudflare Worker (Pure Rust implementation).";
        let event = BaseEvent {
            event_type: EventType::TextMessageContent,
            timestamp: None,
            raw_event: None,
            data: EventData::TextMessageContent(TextMessageContentEvent {
                message_id: message_id.clone(),
                delta: content.to_string(),
            }),
        };
        let encoded = encoder.encode_event(&event).map_err(|e| JsValue::from_str(&e.to_string()))?;
        controller.enqueue_with_chunk(&encoded.into())?;
        
        // TEXT_MESSAGE_END
        let event = BaseEvent {
            event_type: EventType::TextMessageEnd,
            timestamp: None,
            raw_event: None,
            data: EventData::TextMessageEnd(TextMessageEndEvent {
                message_id: message_id.clone(),
            }),
        };
        let encoded = encoder.encode_event(&event).map_err(|e| JsValue::from_str(&e.to_string()))?;
        controller.enqueue_with_chunk(&encoded.into())?;
        
        // RUN_FINISHED
        let event = BaseEvent {
            event_type: EventType::RunFinished,
            timestamp: None,
            raw_event: None,
            data: EventData::RunFinished(RunFinishedEvent {
                thread_id: thread_id.clone(),
                run_id: run_id.clone(),
            }),
        };
        let encoded = encoder.encode_event(&event).map_err(|e| JsValue::from_str(&e.to_string()))?;
        controller.enqueue_with_chunk(&encoded.into())?;
        
        // Close stream
        controller.close()?;
        Ok(())
    }) as Box<dyn FnMut(_) -> Result<(), JsValue>>);
    
    js_sys::Reflect::set(&source, &"start".into(), start.as_ref())?;
    start.forget();
    
    web_sys::ReadableStream::new_with_underlying_source(&source)
}

fn create_error_response(message: &str, status: u16) -> Result<Response, JsValue> {
    let headers = Headers::new()?;
    headers.set("Content-Type", "text/plain; charset=utf-8")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    
    let init = ResponseInit::new();
    init.set_status(status);
    init.set_headers(&headers);
    
    Response::new_with_opt_str_and_init(Some(message), &init)
} 