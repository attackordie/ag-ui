# AG-UI WASM Cloudflare Worker - Pure Rust Implementation

A complete example of the AG-UI Rust SDK running in a Cloudflare Worker via WebAssembly, implemented entirely in Rust with minimal JavaScript wrapper.

## ✨ Features

- **Pure Rust Implementation**: All logic implemented in Rust using wasm-bindgen
- **Built-in Test Interface**: Interactive HTML page served directly from Rust
- **Complete AG-UI Protocol**: Full event streaming with Server-Sent Events
- **Production Ready**: Proper error handling, CORS support, and async initialization

## 🚀 Quick Start

```bash
# Build the WASM package
wasm-pack build --target web --out-dir ./pkg

# Start the development server
wrangler dev --local

# Visit http://localhost:8787 for the test interface
# Or use the API directly at http://localhost:8787/awp
```

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

### Pure Rust Components

- **Worker Handler**: Complete HTTP request/response handling in Rust
- **AG-UI Events**: Native Rust event generation and SSE encoding
- **Test Interface**: HTML page embedded as Rust string constant
- **Streaming**: Web Streams API integration via wasm-bindgen

### JavaScript Wrapper

The only JavaScript is a minimal wrapper (`worker.js`) that:
1. Imports the WASM module and binary
2. Initializes WASM on first request
3. Delegates all requests to the Rust `fetch` function

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

### Command Line
```bash
# Test the HTML interface
curl http://localhost:8787

# Test the API endpoint
curl -X POST http://localhost:8787/awp \
  -H "Content-Type: application/json" \
  -d '{"thread_id":"test","run_id":"test"}'
```

### Browser
1. Open http://localhost:8787
2. Enter thread ID and run ID
3. Click "🚀 Run Agent"
4. Watch real-time event streaming

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
wrangler publish
```

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