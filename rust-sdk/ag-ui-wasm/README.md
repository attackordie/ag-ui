# AG-UI Rust SDK for V8 Isolates (WASM)

A Rust implementation of the Agent-User Interaction Protocol (AG-UI) designed specifically for V8 isolates like browsers and Cloudflare Workers.

## Features

- 🌐 **Web-native**: Built on Web APIs (Fetch, Streams, SSE)
- 🚀 **V8 Isolate Compatible**: Works in browsers and Cloudflare Workers
- 🔄 **Streaming Support**: Real-time event streaming using Web Streams
- 📦 **Zero Native Dependencies**: No Tokio, no system calls
- ⚡ **WASM-first Design**: Optimized for WebAssembly execution
- 🛡️ **Type-safe**: Full Rust type safety with WASM bindings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ag-ui-wasm = "0.1.0"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = "0.3"
```

## Quick Start

### Using in a Cloudflare Worker

```rust
use wasm_bindgen::prelude::*;
use ag_ui_wasm::{WebAgent, RunAgentInput};

#[wasm_bindgen]
pub async fn handle_request(request: web_sys::Request) -> Result<web_sys::Response, JsValue> {
    // Create an agent client
    let agent = WebAgent::new("https://your-agent-endpoint.com/awp".to_string());
    
    // Run the agent
    let input = RunAgentInput::new(
        "thread-123".to_string(),
        "run-456".to_string()
    );
    
    let stream = agent.run_agent(input)?;
    
    // Return streaming response
    create_streaming_response(stream)
}
```

### Creating a Custom Agent

```rust
use ag_ui_wasm::{BaseEvent, EventType, SSEEncoder};
use web_sys::{ReadableStream, ReadableStreamDefaultController};

pub fn create_agent_stream() -> Result<ReadableStream, JsValue> {
    let encoder = SSEEncoder::new()?;
    
    let source = js_sys::Object::new();
    let start = Closure::wrap(Box::new(
        move |controller: ReadableStreamDefaultController| -> Result<(), JsValue> {
            // Emit events
            let event = BaseEvent::run_started("thread-1".into(), "run-1".into());
            let encoded = encoder.encode_event(&event)?;
            controller.enqueue_with_array_buffer_view(&encoded)?;
            
            // Continue streaming...
            Ok(())
        }
    ) as Box<dyn FnMut(_) -> Result<(), JsValue>>);
    
    js_sys::Reflect::set(&source, &"start".into(), start.as_ref())?;
    start.forget();
    
    ReadableStream::new_with_underlying_source(&source)
}
```

## Architecture

### V8 Isolate Constraints

This SDK is designed to work within V8 isolate constraints:

- ✅ Uses Web Streams API instead of Tokio streams
- ✅ Fetch API for HTTP requests (no reqwest)
- ✅ JavaScript Promises via wasm-bindgen-futures
- ✅ Single-threaded execution model
- ✅ No file system or network socket access

### Key Components

1. **WebAgent** - HTTP client using Fetch API
2. **EventStream** - Web Streams-based event processing
3. **SSEEncoder** - Server-Sent Events encoding for streaming
4. **Event Types** - Full AG-UI protocol event support

## Building for Production

### For Cloudflare Workers

```bash
# Build the WASM module
wasm-pack build --target web

# Deploy with Wrangler
wrangler publish
```

### For Browser

```bash
# Build for web
wasm-pack build --target web --out-dir pkg

# Include in your web app
import * as ag_ui from './pkg/ag_ui_wasm.js';
```

## API Reference

### Core Types

#### `RunAgentInput`
Configuration for running an agent:
```rust
let input = RunAgentInput::new("thread-id".to_string(), "run-id".to_string());
```

#### `Message`
Represents a conversation message:
```rust
let message = Message::new(Role::User, "Hello!".to_string());
```

#### `BaseEvent`
AG-UI protocol events:
```rust
let event = BaseEvent::text_message_content("msg-1".to_string(), "Hello".to_string());
```

### Client

#### `WebAgent`
Web-based agent client:
```rust
let agent = WebAgent::new("https://api.example.com/awp".to_string());
let result = agent.run_agent_js(input_js_value).await?;
```

### Streaming

#### `SSEEncoder`
Encode events as Server-Sent Events:
```rust
let encoder = SSEEncoder::new()?;
let encoded = encoder.encode_raw("data")?;
```

#### `EventStream`
Process incoming event streams:
```rust
let stream = EventStream::from_readable_stream(response_body)?;
let event = stream.next_js().await?;
```

## Examples

See the `examples/worker` directory for a complete Cloudflare Worker implementation.

### Running the Example

```bash
cd examples/worker
wasm-pack build --target web
wrangler dev
```

## Testing

Run tests in the browser:

```bash
wasm-pack test --headless --chrome
```

## Performance Considerations

- **Chunk Size**: Process in 64KB chunks for optimal performance
- **Backpressure**: Use ReadableStream's built-in backpressure handling
- **CPU Limits**: Be aware of 10-50ms CPU burst limits in Workers
- **Memory**: Stream processing to avoid buffering entire responses

## Error Handling

The SDK uses a custom `AgUiError` type that converts seamlessly between Rust and JavaScript:

```rust
use ag_ui_wasm::{AgUiError, Result};

fn my_function() -> Result<String> {
    // Returns Result<String, AgUiError>
    Ok("success".to_string())
}
```

## V8 Isolate Compatibility

This SDK is specifically designed for V8 isolate environments:

- **Cloudflare Workers**: Full compatibility
- **Browsers**: Full compatibility  
- **Deno**: Compatible (V8-based)
- **Node.js**: Limited compatibility (prefer native Rust for Node.js)

## Contributing

Contributions are welcome! Please ensure any code works within V8 isolate constraints.

### Development Setup

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the project
wasm-pack build

# Run tests
wasm-pack test --headless --chrome
```

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.

## Changelog

### v0.1.0
- Initial release
- Basic AG-UI protocol support
- Cloudflare Workers compatibility
- Web Streams integration
- SSE encoding/decoding 