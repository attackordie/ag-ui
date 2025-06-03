# AG-UI Rust SDK for V8 Isolates (WASM)

A Rust implementation of the Agent-User Interaction Protocol (AG-UI) designed specifically for V8 isolates like browsers and Cloudflare Workers.

> **🤔 Why a separate WASM SDK?** V8 isolates have strict constraints that require a different approach than native Rust. Read our **[📖 Architecture Guide](../ARCHITECTURE.md)** for the complete technical explanation.

## Features

- 🌐 **Web-native**: Built on Web APIs (Fetch, Streams, SSE)
- 🚀 **V8 Isolate Compatible**: Works in browsers and Cloudflare Workers
- 🔄 **Streaming Support**: Real-time event streaming using Web Streams
- 📦 **Zero Native Dependencies**: No Tokio, no system calls
- ⚡ **WASM-first Design**: Optimized for WebAssembly execution
- 🛡️ **Type-safe**: Full Rust type safety with WASM bindings

## 🚀 Installation & Import

### Option 1: Pre-built Package (Recommended)

If you just want to use the SDK without building from source:

```bash
# Clone the repository
git clone https://github.com/attackordie/ag-ui.git
cd ag-ui/rust-sdk/ag-ui-wasm

# Build the package
wasm-pack build --target web

# Copy pkg/ to your project
cp -r pkg/ /path/to/your/project/node_modules/ag-ui-wasm/
```

### Option 2: Git Dependency (Rust Projects)

Add to your `Cargo.toml`:

```toml
[dependencies]
ag-ui-wasm = { git = "https://github.com/attackordie/ag-ui.git", path = "rust-sdk/ag-ui-wasm" }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = "0.3"
js-sys = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
```

### Option 3: Git Submodule

```bash
# Add as submodule
git submodule add https://github.com/attackordie/ag-ui.git deps/ag-ui

# Build WASM package
cd deps/ag-ui/rust-sdk/ag-ui-wasm
wasm-pack build --target web --out-dir ../../../../pkg/ag-ui-wasm

# The package is now available in pkg/ag-ui-wasm/
```

## 📦 Import in Different Environments

### JavaScript/TypeScript (ES Modules)

```javascript
// After copying pkg/ to your project
import init, * as ag_ui from './pkg/ag_ui_wasm.js';

// Initialize the WASM module
await init();

// Create an agent
const agent = new ag_ui.WebAgent('https://your-api.com/awp');
```

### JavaScript (CommonJS)

```javascript
const ag_ui = require('./pkg/ag_ui_wasm.js');

async function main() {
  await ag_ui.default(); // Initialize WASM
  const agent = new ag_ui.WebAgent('https://your-api.com/awp');
}
```

### TypeScript with Types

```typescript
import init, * as ag_ui from './pkg/ag_ui_wasm.js';
// Types are automatically loaded from ag_ui_wasm.d.ts

await init();

const agent = new ag_ui.WebAgent('https://your-api.com/awp');
const input: ag_ui.RunAgentInput = {
  thread_id: 'thread-1',
  run_id: 'run-1'
};
```

### Cloudflare Workers

#### Method 1: Copy Package

```bash
# Build and copy to worker
wasm-pack build --target web
cp -r pkg/ /path/to/worker/node_modules/ag-ui-wasm/
```

```javascript
// worker.js
import * as ag_ui from 'ag-ui-wasm';

export default {
  async fetch(request, env, ctx) {
    const agent = new ag_ui.WebAgent(env.AG_UI_ENDPOINT);
    // Use agent...
  }
};
```

#### Method 2: Wrangler Configuration

Add to your `wrangler.toml`:

```toml
[build]
command = "wasm-pack build --target web ../ag-ui/rust-sdk/ag-ui-wasm"

[[rules]]
type = "ESModule"
globs = ["**/*.wasm"]
fallthrough = true
```

### Browser (Vanilla JS)

```html
<!DOCTYPE html>
<html>
<head>
  <script type="module">
    import init, * as ag_ui from './pkg/ag_ui_wasm.js';
    
    async function run() {
      await init();
      
      const agent = new ag_ui.WebAgent('https://api.example.com/awp');
      const result = await agent.run_agent_js({
        thread_id: 'thread-1',
        run_id: 'run-1'
      });
      
      console.log(result);
    }
    
    run();
  </script>
</head>
<body>
  <h1>AG-UI WASM Example</h1>
</body>
</html>
```

### React/Next.js

```typescript
// hooks/useAgUi.ts
import { useEffect, useState } from 'react';
import type * as ag_ui from '../pkg/ag_ui_wasm.js';

export function useAgUi() {
  const [agUi, setAgUi] = useState<typeof ag_ui | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadWasm() {
      try {
        const ag_ui = await import('../pkg/ag_ui_wasm.js');
        await ag_ui.default(); // Initialize WASM
        setAgUi(ag_ui);
      } catch (error) {
        console.error('Failed to load AG-UI WASM:', error);
      } finally {
        setIsLoading(false);
      }
    }

    loadWasm();
  }, []);

  return { agUi, isLoading };
}
```

```tsx
// components/AgentRunner.tsx
import { useAgUi } from '../hooks/useAgUi';

export function AgentRunner() {
  const { agUi, isLoading } = useAgUi();

  const runAgent = async () => {
    if (!agUi) return;

    const agent = new agUi.WebAgent('https://api.example.com/awp');
    const result = await agent.run_agent_js({
      thread_id: 'thread-1',
      run_id: 'run-1'
    });
    
    console.log(result);
  };

  if (isLoading) return <div>Loading AG-UI...</div>;

  return (
    <button onClick={runAgent}>
      Run Agent
    </button>
  );
}
```

### Vite

```typescript
// vite.config.ts
import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    fs: {
      allow: ['..'] // Allow access to parent directories for pkg/
    }
  },
  optimizeDeps: {
    exclude: ['ag_ui_wasm'] // Don't pre-bundle WASM
  }
});
```

```typescript
// main.ts
import init, * as ag_ui from '../rust-sdk/ag-ui-wasm/pkg/ag_ui_wasm.js';

async function main() {
  await init();
  
  const agent = new ag_ui.WebAgent('https://api.example.com/awp');
  // Use agent...
}

main();
```

### Webpack

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/async',
      },
    ],
  },
};
```

## 🔧 Build Instructions

### Prerequisites

- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **wasm-pack**: [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building the Package

```bash
# Clone the repository
git clone https://github.com/attackordie/ag-ui.git
cd ag-ui/rust-sdk/ag-ui-wasm

# Build for web (browsers/workers)
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler

# The built package will be in pkg/
```

### Generated Files

After building, you'll find these files in `pkg/`:

```
pkg/
├── ag_ui_wasm.js          # JavaScript bindings
├── ag_ui_wasm_bg.wasm     # WebAssembly binary
├── ag_ui_wasm.d.ts        # TypeScript definitions
├── package.json           # NPM package metadata
└── README.md              # Package documentation
```

## 📋 Package Contents

The generated `pkg/` directory contains everything you need:

- **`ag_ui_wasm.js`** - Main JavaScript module with bindings
- **`ag_ui_wasm_bg.wasm`** - The compiled WebAssembly binary
- **`ag_ui_wasm.d.ts`** - TypeScript type definitions
- **`package.json`** - NPM package configuration
- **`.gitignore`** - Recommended git ignore patterns

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

> **📖 Learn More**: Read the [Architecture Guide](../ARCHITECTURE.md) for a complete technical deep-dive into these constraints and why they matter.

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

> **📖 Deep Dive**: See the [Architecture Guide](../ARCHITECTURE.md) for detailed performance optimization strategies for V8 isolates.

## Troubleshooting

### Common Issues

#### "Cannot resolve module" errors

Make sure you've built the WASM package:
```bash
wasm-pack build --target web
```

#### TypeScript compilation errors

Ensure the `.d.ts` file is in your TypeScript include path:
```json
{
  "compilerOptions": {
    "typeRoots": ["./pkg", "./node_modules/@types"]
  }
}
```

#### Cloudflare Worker deployment failures

- Use `--target web` not `--target nodejs`
- Check your wrangler.toml configuration
- Ensure WASM file size is under limits

#### Browser CORS issues

WASM modules must be served over HTTP/HTTPS, not file:// protocol.

### Debug Mode

Build with debug info for development:
```bash
wasm-pack build --dev --target web
```

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

> **📖 Why these differences?** Read the [Architecture Guide](../ARCHITECTURE.md) to understand the fundamental differences between V8 isolates and native environments.

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