# AG-UI Rust SDK

A comprehensive Rust SDK for the AG-UI (Autonomous Agent User Interface) platform, providing both native Rust and WebAssembly interfaces for building autonomous agent applications.

> **🤔 Why Two SDKs?** Native and web environments have fundamentally different capabilities and constraints. See our **[📖 Architecture Guide](./ARCHITECTURE.md)** for the complete technical explanation.

## 📦 SDK Components

### 🦀 Native Rust SDK (`ag-ui-rust`)
**For: Server-side applications, CLI tools, desktop apps**
- **Full-featured Rust client** with tokio async runtime
- **Multi-threading support** for CPU-intensive tasks
- **Rich ecosystem access** - use any crate from crates.io
- **Direct system integration** - file system, databases, TCP sockets
- **Native HTTP client** with full TLS and connection pooling

### 🌐 WebAssembly SDK (`ag-ui-wasm`)
**For: Browsers, Cloudflare Workers, edge functions, V8 isolates**
- **Browser-compatible** WASM bindings with zero cold start
- **V8 isolate optimized** - no tokio, no threads, no file system
- **Web APIs only** - Fetch API, Web Streams, JavaScript interop
- **Sandboxed security** - runs in isolated environments
- **Modern web frameworks** integration (React, Vue, etc.)

## 🚀 Quick Start

**New to AG-UI Rust SDK?** → **[📋 Integration Guide](./INTEGRATION.md)**

**Need to understand the architecture?** → **[📖 Architecture Guide](./ARCHITECTURE.md)**

## 🌐 **Live Demo**

**🚀 See it in action:** **[https://ag-ui-worker-example.brianpboynton.workers.dev](https://ag-ui-worker-example.brianpboynton.workers.dev)**

Experience our **pure Rust WebAssembly implementation** running live on Cloudflare Workers:
- **Complete AG-UI protocol** streaming (99% Rust, 1% JavaScript)
- **Real-time events** with Server-Sent Events and Web Streams
- **Interactive demonstration** with full technical explanations
- **Production deployment** showcasing WASM performance and capabilities

### For Web/Browser Projects
```bash
cd ag-ui-wasm
wasm-pack build --target web
# Copy pkg/ to your project
```

### For Rust Projects
```toml
# Cargo.toml
[dependencies]
ag-ui-rust = { path = "../ag-ui-rust" }
tokio = { version = "1.0", features = ["full"] }
```

## 📚 Documentation

- **[🌐 WASM Integration Guide](./INTEGRATION.md)** - 5-minute setup for web projects
- **[🏗️ Architecture Guide](./ARCHITECTURE.md)** - Why native vs. web targets differ
- **[📖 WASM Documentation](./ag-ui-wasm/README.md)** - Complete WebAssembly SDK docs
- **[🦀 Native Rust Documentation](./ag-ui-rust/README.md)** - Native SDK documentation
- **[⚡ Examples](./ag-ui-wasm/examples/)** - Real-world implementation examples

## 🎯 Use Cases

### 🌐 WebAssembly SDK
**V8 Isolate Environments** (Single-threaded, sandboxed)
- **Single Page Applications** (React, Vue, Angular)
- **Progressive Web Apps** (PWAs)
- **Browser Extensions**
- **Cloudflare Workers** and edge functions
- **Serverless platforms** (Vercel Edge, Deno Deploy)
- **Static Site Generators** (Next.js, Nuxt, etc.)

### 🦀 Native Rust SDK
**System Environments** (Multi-threaded, full system access)
- **Backend Services** and APIs
- **Command Line Tools**
- **Desktop Applications** (Tauri, egui)
- **Microservices** and serverless functions
- **Data Processing** pipelines
- **System Integration** tools

## 🏗️ Architecture

```
ag-ui/
├── rust-sdk/
│   ├── ARCHITECTURE.md         # 🏗️ Technical deep-dive
│   ├── INTEGRATION.md          # 🚀 Quick start guide
│   ├── ag-ui-rust/             # 🦀 Native Rust SDK
│   │   ├── src/
│   │   ├── examples/
│   │   └── README.md
│   └── ag-ui-wasm/             # 🌐 WebAssembly SDK
│       ├── src/
│       ├── examples/
│       ├── pkg/                # Generated WASM output
│       └── README.md
```

## 🔄 **Key Architectural Differences**

| Aspect | 🦀 Native | 🌐 Web/WASM |
|--------|-----------|-------------|
| **Runtime** | Tokio (multi-threaded) | V8 Event Loop (single-threaded) |
| **HTTP Client** | `reqwest` | Web Fetch API |
| **Streaming** | `tokio::stream` | Web Streams API |
| **File System** | ✅ Full access | ❌ Sandboxed |
| **Threading** | ✅ Multi-core | ❌ Single-threaded |
| **Cold Start** | ~100ms | ~1ms |
| **Bundle Size** | N/A | ~280KB WASM |
| **Dependencies** | Full ecosystem | WASM-compatible only |

> **📖 Learn More**: Read the complete [Architecture Guide](./ARCHITECTURE.md) to understand why these differences matter.

## 🔄 Development Workflow

### 1. Building Both SDKs
```bash
# Native Rust SDK
cd ag-ui-rust && cargo build

# WebAssembly SDK
cd ag-ui-wasm && wasm-pack build --target web
```

### 2. Running Examples
```bash
# Test native SDK
cd ag-ui-rust && cargo run --example basic

# Test WASM in browser
cd ag-ui-wasm/examples/worker && npm install && npm run dev
```

### 3. Running Tests
```bash
# Test both SDKs
cd ag-ui-rust && cargo test
cd ag-ui-wasm && cargo test && wasm-pack test --headless --firefox
```

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** your changes in either `ag-ui-rust/` or `ag-ui-wasm/`
4. **Test** your changes (`cargo test` and `wasm-pack test`)
5. **Commit** your changes (`git commit -m 'Add amazing feature'`)
6. **Push** to the branch (`git push origin feature/amazing-feature`)
7. **Open** a Pull Request

### Development Guidelines
- **Follow Rust conventions** (rustfmt, clippy)
- **Add tests** for new functionality
- **Update documentation** for API changes
- **Ensure WASM builds** successfully with `wasm-pack`
- **Consider V8 isolate constraints** for web target changes

## 📄 License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🔗 Links

- **[AG-UI Platform](https://github.com/attackordie/ag-ui)** - Main repository
- **[Documentation](https://docs.ag-ui.com)** - Complete platform docs
- **[Examples](./ag-ui-wasm/examples/)** - Implementation examples
- **[Integration Guide](./INTEGRATION.md)** - Quick setup guide
- **[Architecture Guide](./ARCHITECTURE.md)** - Technical deep-dive

---

**Ready to integrate?** → **[Start with the Integration Guide](./INTEGRATION.md)** 🚀 