# AG-UI Rust SDK Architecture Guide

> **Understanding Web vs. Native Targets in Rust**

This document explains the fundamental architectural differences between Rust's **native** and **web** targets, and why the AG-UI SDK provides two distinct implementations.

## 🏗️ **Core Architectural Differences**

### 🦀 **Native Rust Target** (`ag-ui-rust`)
**Environment**: Server-side applications, CLI tools, desktop apps
```rust
// Native Rust uses system-level APIs
use tokio::net::TcpStream;     // Direct TCP access
use std::fs::File;             // File system access
use reqwest::Client;           // Full HTTP client with native TLS
use std::thread;               // Multi-threading support
```

**Key Characteristics:**
- ✅ **Full system access** - File system, network sockets, threads
- ✅ **Multi-threading** with tokio's work-stealing scheduler
- ✅ **Native dependencies** - OpenSSL, system libraries
- ✅ **Direct OS integration** - Process spawning, signal handling
- ✅ **Rich ecosystem** - Full access to crates.io ecosystem

---

### 🌐 **Web Target** (`ag-ui-wasm`)
**Environment**: Browsers, Cloudflare Workers, Deno, edge computing
```rust
// Web target uses browser/V8 APIs only
use web_sys::fetch;           // Browser Fetch API
use web_sys::ReadableStream;  // Web Streams API
use js_sys::Promise;          // JavaScript Promise integration
// NO: tokio, std::fs, std::thread, reqwest
```

**Key Characteristics:**
- ❌ **No system access** - Sandboxed environment
- ❌ **Single-threaded** - No threads, only async/await
- ❌ **No native dependencies** - Pure WASM + Web APIs
- ✅ **V8 isolate compatible** - Works in browsers and Workers
- ✅ **Zero-latency cold starts** - No runtime initialization

## 🔄 **Why Two Different Implementations?**

### **The V8 Isolate Constraint**

V8 isolates (used by browsers, Cloudflare Workers, Deno) impose strict limitations:

```rust
// ❌ This won't work in WASM/V8 isolates:
use tokio::time::sleep;        // No tokio runtime
use std::thread::spawn;        // No threads
use std::fs::read_to_string;   // No file system
use reqwest::get;              // Native HTTP client won't compile

// ✅ This works in WASM/V8 isolates:
use web_sys::window;           // Browser window object
use js_sys::Promise;           // JavaScript promises
use wasm_bindgen_futures::JsFuture; // Async JS integration
use web_sys::fetch;            // Fetch API
```

### **Streaming HTTP: The Key Difference**

This is where the architectural differences become critical:

#### **Native Rust Streaming**
```rust
// Native: Uses tokio streams with system TCP
use tokio_stream::StreamExt;
use reqwest::Client;

let response = Client::new()
    .get("https://api.example.com/stream")
    .send()
    .await?;

let mut stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    // Process chunk with full system resources
    process_chunk(chunk?).await;
}
```

#### **Web Target Streaming**
```rust
// Web: Uses Web Streams API through WASM bindings
use web_sys::{ReadableStream, ReadableStreamDefaultReader};
use wasm_bindgen::JsCast;

let response = fetch_with_request(&request).await?;
let body = response.body().unwrap();
let reader = body.get_reader().dyn_into::<ReadableStreamDefaultReader>()?;

loop {
    let result = JsFuture::from(reader.read()).await?;
    // Process using V8's memory management
    if result.is_done() { break; }
    process_chunk_js(result).await?;
}
```

## 🎯 **Use Case Matrix**

| Environment | Target | Runtime | HTTP Client | Streaming | Threading |
|-------------|--------|---------|-------------|-----------|-----------|
| **Server Applications** | Native | Tokio | reqwest | tokio::stream | Multi-threaded |
| **CLI Tools** | Native | Tokio | reqwest | tokio::stream | Multi-threaded |
| **Desktop Apps** | Native | Tokio | reqwest | tokio::stream | Multi-threaded |
| **Browsers** | Web | V8 | fetch API | Web Streams | Single-threaded |
| **Cloudflare Workers** | Web | V8 | fetch API | Web Streams | Single-threaded |
| **Deno** | Web | V8 | fetch API | Web Streams | Single-threaded |
| **Edge Functions** | Web | V8 | fetch API | Web Streams | Single-threaded |

## 🚀 **Performance Implications**

### **Native Target Performance**
```rust
// Can utilize all CPU cores
#[tokio::main]
async fn main() {
    let handles: Vec<_> = (0..num_cpus::get())
        .map(|_| tokio::spawn(process_data()))
        .collect();
    
    // Parallel processing across threads
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### **Web Target Performance**
```rust
// Single-threaded, but optimized for V8
#[wasm_bindgen]
pub async fn process_stream() -> Result<JsValue, JsValue> {
    // Must yield control to event loop frequently
    let mut count = 0;
    for item in data {
        process(item).await;
        
        count += 1;
        if count % 1000 == 0 {
            // Yield to prevent blocking V8 event loop
            JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await?;
        }
    }
    Ok(JsValue::NULL)
}
```

## 💾 **Memory Management Differences**

### **Native Rust**
```rust
// Direct control over memory allocation
let mut buffer = Vec::with_capacity(1024 * 1024); // 1MB
buffer.extend_from_slice(&data);
// Rust's ownership system manages deallocation
```

### **Web Target**
```rust
// Memory managed by V8's garbage collector
#[wasm_bindgen]
pub struct StreamProcessor {
    // Must be careful with memory retention
    buffer: js_sys::Uint8Array,
}

#[wasm_bindgen]
impl StreamProcessor {
    pub fn process(&mut self, data: &[u8]) -> Result<(), JsValue> {
        // V8 GC handles JavaScript objects
        let js_array = js_sys::Uint8Array::from(data);
        self.buffer = js_array; // Previous buffer eligible for GC
        Ok(())
    }
}
```

## 🔌 **API Integration Patterns**

### **Native: Rich Ecosystem Integration**
```rust
// Can use any crate from crates.io
use sqlx::PgPool;              // Database integration
use redis::AsyncCommands;      // Redis client
use aws_sdk_s3::Client;        // AWS SDK
use openssl::ssl::SslContext;  // Native TLS

async fn rich_integration() -> Result<(), Box<dyn std::error::Error>> {
    let db = PgPool::connect("postgresql://...").await?;
    let redis = redis::Client::open("redis://...")?;
    let s3 = aws_sdk_s3::Client::new(&aws_config::load_from_env().await);
    
    // Full ecosystem available
    Ok(())
}
```

### **Web: Browser/Worker API Integration**
```rust
// Limited to Web APIs and WASM-compatible crates
use web_sys::{Request, Response, Headers};
use js_sys::{Object, Reflect};

#[wasm_bindgen]
pub async fn worker_integration(env: &Object) -> Result<Response, JsValue> {
    // Access Cloudflare Worker bindings
    let kv = Reflect::get(env, &"MY_KV".into())?;
    let db = Reflect::get(env, &"DB".into())?;
    
    // Use Worker APIs through bindings
    let stored_value = kv.get("key").await?;
    
    Ok(Response::new_with_opt_str_and_init(
        Some("success"),
        &ResponseInit::new().status(200)
    )?)
}
```

## 🛠️ **Build Process Differences**

### **Native Build**
```bash
# Native: Standard Rust compilation
cargo build --release
# Produces: target/release/ag-ui-app (native binary)

# Can link against system libraries
# Full optimization with LLVM
# Platform-specific optimizations
```

### **Web Build**
```bash
# Web: Specialized WASM compilation
wasm-pack build --target web
# Produces: pkg/ag_ui_wasm.js + ag_ui_wasm_bg.wasm

# WASM-specific optimizations
# JavaScript binding generation
# Size optimization for network transfer
```

## 📊 **When to Choose Each Target**

### **✅ Choose Native Target When:**
- Building server-side applications
- Need access to file system or databases
- Require multi-threading for CPU-intensive tasks
- Using existing Rust ecosystem extensively
- Building CLI tools or desktop applications
- Need maximum performance for data processing

### **✅ Choose Web Target When:**
- Deploying to browsers or Progressive Web Apps
- Using Cloudflare Workers or edge functions
- Building serverless functions with instant cold starts
- Need cross-platform client-side compatibility
- Working within V8 isolate environments
- Prioritizing security through sandboxing

## 🔄 **Migration Patterns**

### **From Native to Web**
```rust
// Before (Native)
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn read_config() -> Result<String, std::io::Error> {
    let mut file = File::open("config.json").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

// After (Web) - External data source
use web_sys::fetch;

async fn read_config() -> Result<String, JsValue> {
    let response = fetch_with_str("/api/config").await?;
    let text = JsFuture::from(response.text()?).await?;
    Ok(text.as_string().unwrap())
}
```

### **From Web to Native**
```rust
// Before (Web)
use web_sys::fetch;

async fn api_call() -> Result<JsValue, JsValue> {
    let response = fetch_with_str("https://api.example.com").await?;
    JsFuture::from(response.json()?).await
}

// After (Native) - Rich HTTP client
use reqwest::Client;

async fn api_call() -> Result<serde_json::Value, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://api.example.com")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
```

## 📚 **Summary**

The AG-UI Rust SDK provides two targets because **web** and **native** environments have fundamentally different capabilities:

| Aspect | Native Target | Web Target |
|--------|---------------|------------|
| **Environment** | Server/Desktop | Browser/V8 Isolates |
| **System Access** | Full | Sandboxed |
| **Threading** | Multi-threaded | Single-threaded |
| **HTTP Client** | reqwest | Fetch API |
| **Streaming** | tokio::stream | Web Streams |
| **Dependencies** | Full ecosystem | WASM-compatible only |
| **Performance** | Multi-core | Single-core optimized |
| **Cold Start** | Slower | Instant |
| **Security** | Trust-based | Isolation-based |

Choose the target that matches your deployment environment and constraints! 🚀 