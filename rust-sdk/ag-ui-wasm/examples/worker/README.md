# AG-UI WASM Cloudflare Worker - Pure Rust Implementation

A complete example of the [AG-UI (Agent User Interaction Protocol)](https://github.com/attackordie/ag-ui) Rust SDK running in a Cloudflare Worker via WebAssembly, implemented entirely in Rust with minimal JavaScript wrapper.

**AG-UI** is an open, lightweight, event-based protocol created by **[CopilotKit](https://copilotkit.ai)** that standardizes how AI agents communicate with user interfaces in real-time.

## 🌐 **Live Demo**

**🚀 Try it now:** **[https://ag-ui-worker-example.brianpboynton.workers.dev](https://ag-ui-worker-example.brianpboynton.workers.dev)**

Experience the [AG-UI (Agent User Interaction Protocol)](https://github.com/attackordie/ag-ui) protocol in action with this interactive demonstration of:
- Complete AG-UI event streaming (RUN_STARTED → TEXT_MESSAGE_START → TEXT_MESSAGE_CONTENT → TEXT_MESSAGE_END → RUN_FINISHED)
- Pure Rust implementation (99% Rust, 1% JavaScript) running in production
- Real-time Server-Sent Events with beautiful educational interface
- Full protocol explanation and technical architecture details

## ✨ Features

- **Pure Rust Implementation**: All logic implemented in Rust using wasm-bindgen
- **Built-in Test Interface**: Interactive HTML page served directly from Rust
- **Complete AG-UI Protocol**: Full event streaming with Server-Sent Events
- **Production Ready**: Proper error handling, CORS support, and async initialization

## 🎯 What You'll Experience

When you visit `http://localhost:8787/`, you'll see an **interactive test interface** that demonstrates the full AG-UI workflow:

### 🎨 Beautiful Test Interface
- **Clean, modern UI** with proper styling and responsive design
- **Pre-filled inputs** for thread_id ("rust-test-thread") and run_id ("rust-test-run")
- **Real-time status updates** showing connection progress
- **Live event streaming** display with timestamps and formatted JSON

### 🚀 Complete AG-UI Workflow
When you click "🚀 Run Agent", you'll see the complete event sequence:
1. **`RUN_STARTED`** - Workflow initialization
2. **`TEXT_MESSAGE_START`** - Assistant message begins (role: assistant)
3. **`TEXT_MESSAGE_CONTENT`** - Streaming message: "Hello! I'm an AG-UI agent running in a Cloudflare Worker (Pure Rust implementation)"
4. **`TEXT_MESSAGE_END`** - Message completion
5. **`RUN_FINISHED`** - Workflow completion

All events stream in **real-time** with proper Server-Sent Events encoding, demonstrating the full AG-UI protocol implementation.

## 📋 Prerequisites

Before starting, ensure you have the following installed:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack for building WebAssembly packages
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install wrangler CLI for Cloudflare Workers
npm install -g wrangler
```

## 🚀 Quick Start

```bash
# Navigate to the worker example directory
cd rust-sdk/ag-ui-wasm/examples/worker

# Build the WASM package (this may take a few minutes on first run)
wasm-pack build --target web --out-dir ./pkg

# Start the development server
wrangler dev --local

# Visit http://localhost:8787 for the test interface
# Or use the API directly at http://localhost:8787/awp
```

**Expected output:** You should see "Ready on http://localhost:8787" and be able to access the interactive test interface in your browser.

## 📁 Project Structure

```
worker/
├── src/
│   ├── lib.rs           # Simple module exports
│   └── worker.rs        # Main worker implementation in Rust
├── worker.js            # Minimal JavaScript wrapper for WASM init
├── wrangler.toml        # Cloudflare Workers configuration
├── Cargo.toml           # Rust dependencies
└── pkg/                 # Generated WASM package (after build)
```

## 🔧 Architecture

### 99% Rust, 1% JavaScript
This example showcases how you can build a **complete AG-UI-compatible service** using almost entirely Rust:

- **`worker.rs` (400+ lines)**: Complete HTTP handler, HTML interface, AG-UI events, and streaming logic
- **`worker.js` (26 lines)**: Minimal WASM initialization wrapper

The JavaScript wrapper does only three things:
1. Import the WASM module and binary
2. Initialize WASM on first request  
3. Delegate all requests to the Rust `fetch` function

```javascript
import init, { fetch as wasmFetch } from './pkg/ag_ui_worker_example.js';
import wasmModule from './pkg/ag_ui_worker_example_bg.wasm';

export default {
  async fetch(request, env, ctx) {
    await init(wasmModule);
    return wasmFetch(request);
  }
};
```

### Pure Rust Components

- **HTTP Request/Response Handling**: Complete web server logic in Rust
- **AG-UI Event Generation**: Native Rust event creation with proper types
- **Server-Sent Events**: SSE encoding via `SSEEncoder` from ag-ui-wasm
- **HTML Interface**: Embedded as Rust string constant with full interactivity
- **Streaming**: Web Streams API integration via wasm-bindgen

## 🌐 Endpoints

### `GET /` - Test Interface
Interactive HTML page for testing the AG-UI worker with:
- Input fields for thread_id and run_id
- Real-time event streaming display
- Error handling and status updates

### `POST /awp` - AG-UI API
Compliant AG-UI endpoint that accepts:
```json
{
  "thread_id": "test-thread",
  "run_id": "test-run"
}
```

Returns streaming Server-Sent Events:
```
data: {"type":"RUN_STARTED","thread_id":"test-thread","run_id":"test-run"}
data: {"type":"TEXT_MESSAGE_START","message_id":"uuid","role":"assistant"}
data: {"type":"TEXT_MESSAGE_CONTENT","message_id":"uuid","delta":"Hello!..."}
data: {"type":"TEXT_MESSAGE_END","message_id":"uuid"}
data: {"type":"RUN_FINISHED","thread_id":"test-thread","run_id":"test-run"}
```

### `OPTIONS /awp` - CORS Preflight
Handles CORS preflight requests for browser compatibility.

## 🧪 Testing

### Browser Experience
1. **Open http://localhost:8787** - See the beautiful test interface
2. **Enter thread ID and run ID** (or use the pre-filled defaults)
3. **Click "🚀 Run Agent"** - Watch the button change to "⏳ Running..."
4. **Watch real-time streaming** - See each AG-UI event arrive with timestamps
5. **Status updates** - Connection status changes from "Ready" → "Connecting" → "Connected" → "Streaming"

### Expected Output
You should see a sequence like this in the event display:
```
[timestamp] {"type":"RUN_STARTED","thread_id":"rust-test-thread","run_id":"rust-test-run"}
[timestamp] {"type":"TEXT_MESSAGE_START","message_id":"uuid","role":"assistant"}
[timestamp] {"type":"TEXT_MESSAGE_CONTENT","message_id":"uuid","delta":"Hello! I'm an AG-UI agent..."}
[timestamp] {"type":"TEXT_MESSAGE_END","message_id":"uuid"}
[timestamp] {"type":"RUN_FINISHED","thread_id":"rust-test-thread","run_id":"rust-test-run"}
```

### Command Line Testing
```bash
# Test the HTML interface
curl http://localhost:8787

# Test the API endpoint directly
curl -X POST http://localhost:8787/awp \
  -H "Content-Type: application/json" \
  -d '{"thread_id":"test","run_id":"test"}'
```

## 🔄 Development Workflow

1. **Edit Rust Code**: Modify `src/worker.rs`
2. **Rebuild WASM**: `wasm-pack build --target web --out-dir ./pkg`
3. **Restart Worker**: Restart `wrangler dev --local`

Note: Automatic rebuilding was disabled due to wasm-pack/wrangler compatibility issues.

## 🏗️ Implementation Details

### WASM Initialization
- Uses direct WASM binary import to avoid URL resolution issues
- Lazy initialization on first request for optimal performance
- Proper error handling for WASM load failures

### Event Streaming
- Native Rust `ReadableStream` creation using `wasm-bindgen`
- SSE encoding via `SSEEncoder` from ag-ui-wasm
- Proper stream cleanup and error handling

### Error Handling
- Comprehensive error responses with CORS headers
- JavaScript Promise/Result conversion
- Graceful degradation for unsupported browsers

## 🎯 Benefits of Pure Rust Approach

1. **Type Safety**: Full Rust type checking across the entire stack
2. **Performance**: Minimal JavaScript overhead
3. **Maintainability**: Single language codebase
4. **Protocol Compliance**: Direct use of ag-ui-wasm types
5. **Testing**: Easy unit testing in Rust

## 🚀 Deployment

```bash
# Production build
wasm-pack build --target web --out-dir ./pkg

# Deploy to Cloudflare Workers
wrangler deploy
```

**🌐 Live Example:** This worker is deployed at [https://ag-ui-worker-example.brianpboynton.workers.dev](https://ag-ui-worker-example.brianpboynton.workers.dev)

## 🔍 Comparison with JavaScript Implementation

| Aspect | Pure Rust | JavaScript Wrapper |
|--------|-----------|-------------------|
| Lines of Code | ~350 Rust | ~50 Rust + ~200 JS |
| Type Safety | Full | Partial |
| Debugging | Rust tools | Mixed tools |
| Performance | ~Same | ~Same |
| Maintainability | Higher | Lower |
| Learning Curve | Rust knowledge | Web knowledge |

## 📖 Related

- [AG-UI WASM SDK Documentation](../../README.md)
- [Architecture Guide](../../../ARCHITECTURE.md)
- [Protocol Specification](../../../../protocol/)

## 🎯 Why This Implementation is Impressive

### Technical Excellence
1. **Type Safety**: Full Rust type checking across the entire stack
2. **Performance**: Minimal JavaScript overhead with direct WASM integration
3. **Protocol Compliance**: Uses actual `ag-ui-wasm` types and SSE encoding
4. **Maintainability**: Single language codebase with consistent patterns
5. **Production Ready**: Proper error handling, CORS support, and stream management

### Architectural Benefits
- **No Runtime Dependencies**: Everything bundled in WASM
- **Consistent Error Handling**: Rust's Result types throughout
- **Memory Safety**: Rust's ownership system prevents common web vulnerabilities
- **Easy Testing**: Unit testable in pure Rust environment

### Educational Value
This example demonstrates:
- How to build **complete web services** in Rust via WASM
- **AG-UI protocol implementation** from scratch
- **Server-Sent Events streaming** in a serverless environment
- **Cloudflare Workers** integration with complex Rust applications 