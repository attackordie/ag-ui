# AG-UI WASM SDK Quick Reference

**This is a Rust project** that compiles to WASM. Development happens in Rust, JavaScript is only for loading the WASM binary.

## Rust Development

### Build WASM Package
```bash
# Primary build command
wasm-pack build --target web

# For different environments
wasm-pack build --target web      # Browsers, Cloudflare Workers  
wasm-pack build --target nodejs   # Node.js
wasm-pack build --target bundler  # Webpack, Rollup
```

### Basic Rust Structure
```rust
// src/lib.rs
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct WebAgent {
    endpoint: String,
}

#[wasm_bindgen]
impl WebAgent {
    #[wasm_bindgen(constructor)]
    pub fn new(endpoint: String) -> WebAgent {
        WebAgent { endpoint }
    }

    #[wasm_bindgen]
    pub async fn run_agent_js(&self, input: JsValue) -> Result<ReadableStream, JsValue> {
        // All logic implemented in Rust
        self.run_agent_internal(input).await
    }
}
```

### Using Web APIs from Rust
```rust
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::JsFuture;

async fn make_request(url: &str) -> Result<Response, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
    Ok(resp_value.dyn_into()?)
}
```

### Streaming in Rust
```rust
use web_sys::{ReadableStream, ReadableStreamDefaultController};
use wasm_bindgen::closure::Closure;

pub fn create_stream() -> Result<ReadableStream, JsValue> {
    let source = js_sys::Object::new();
    
    let start = Closure::wrap(Box::new(
        move |controller: ReadableStreamDefaultController| -> Result<(), JsValue> {
            // Rust streaming logic
            let data = create_sse_data()?;
            controller.enqueue_with_array_buffer_view(&data)?;
            Ok(())
        }
    ) as Box<dyn FnMut(_) -> Result<(), JsValue>>);
    
    js_sys::Reflect::set(&source, &"start".into(), start.as_ref())?;
    start.forget();
    
    ReadableStream::new_with_underlying_source(&source)
}
```

### Error Handling in Rust
```rust
#[derive(Debug)]
#[wasm_bindgen]
pub struct AgUiError {
    message: String,
}

#[wasm_bindgen]
impl AgUiError {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> AgUiError {
        AgUiError { message }
    }
}

impl From<JsValue> for AgUiError {
    fn from(js_val: JsValue) -> Self {
        AgUiError::new(format!("{:?}", js_val))
    }
}
```

### Essential Cargo.toml Dependencies
```toml
[dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
  "console", "Request", "Response", "ReadableStream", 
  "Headers", "Fetch", "Window", "WorkerGlobalScope"
]}
js-sys = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
```

## JavaScript Shim (Minimal Usage Only)

### WASM Loading Only
```javascript
// Minimal shim - ONLY for loading WASM
import init, * as ag_ui from './pkg/ag_ui_wasm.js';

// Initialize WASM (required once)
await init();

// All functionality is Rust-compiled WASM
const agent = new ag_ui.WebAgent('https://your-api.com/awp');
```

### Cloudflare Worker (Minimal)
```javascript
// worker.js - minimal loading shim
import * as ag_ui from './pkg/ag_ui_wasm.js';

export default {
  async fetch(request, env) {
    const agent = new ag_ui.WebAgent(env.AG_UI_ENDPOINT);
    return await agent.run_agent_js(await request.json());
  }
};
```

### Browser (Minimal)
```html
<script type="module">
  import init, * as ag_ui from './pkg/ag_ui_wasm.js';
  await init();
  window.agUi = ag_ui;
</script>
```

## Rust Testing
```bash
# Test in browser environment
wasm-pack test --headless --chrome

# Build and test
wasm-pack build && wasm-pack test --headless --chrome
```

## Debugging from Rust
```rust
use web_sys::console;

// Debug output from Rust
console::log_1(&"Debug from Rust".into());

// Complex debugging
let obj = js_sys::Object::new();
js_sys::Reflect::set(&obj, &"key".into(), &"value".into()).unwrap();
console::log_1(&obj);
```

## Essential Rules

1. **This is a Rust project** - JavaScript is only for WASM loading
2. **Use web-sys** for all Web API access from Rust  
3. **No Tokio** - use Web Streams and Fetch API from Rust
4. **Build with wasm-pack** - not regular cargo build
5. **All business logic in Rust** - never in JavaScript

## Common Rust Patterns

### Async in WASM
```rust
use wasm_bindgen_futures::JsFuture;

async fn async_rust_function() -> Result<String, JsValue> {
    let promise = some_web_api();
    let result = JsFuture::from(promise).await?;
    Ok(result.as_string().unwrap_or_default())
}
```

### Type Conversion
```rust
// Rust to JS
impl From<MyRustType> for JsValue {
    fn from(val: MyRustType) -> Self {
        serde_wasm_bindgen::to_value(&val).unwrap()
    }
}

// JS to Rust  
impl TryFrom<JsValue> for MyRustType {
    type Error = JsValue;
    
    fn try_from(val: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(val)
    }
}
```

## Build Optimization
```toml
# Cargo.toml
[profile.release]
opt-level = "s"  # Optimize for size
lto = true       # Link-time optimization

[features]
default = []
wee_alloc = ["wee_alloc"]  # Smaller allocator
```

## Project Structure
```
rust-sdk/ag-ui-wasm/
├── src/
│   ├── lib.rs          # Main Rust entry point
│   ├── client.rs       # WebAgent in Rust
│   ├── events.rs       # Event types in Rust  
│   └── streaming.rs    # Stream handling in Rust
├── Cargo.toml          # Rust dependencies
└── pkg/               # Generated WASM (after build)
```

**Remember**: This is a Rust WebAssembly project. All development happens in Rust using web-sys bindings. JavaScript/TypeScript is only a minimal shim to load the WASM binary. 