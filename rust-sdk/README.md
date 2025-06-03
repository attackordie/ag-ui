# AG-UI Rust SDK

A comprehensive Rust SDK for the AG-UI (Autonomous Agent User Interface) platform, providing both native Rust and WebAssembly interfaces for building autonomous agent applications.

## 📦 SDK Components

### 🦀 Native Rust SDK (`ag-ui-rust`)
- **Full-featured Rust client** for server-side applications
- **Async/await support** with tokio
- **Type-safe** API with comprehensive error handling
- **HTTP client** built on reqwest

### 🌐 WebAssembly SDK (`ag-ui-wasm`)
- **Browser-compatible** WASM bindings
- **JavaScript/TypeScript** interop
- **Cloudflare Workers** support
- **Modern web frameworks** integration (React, Vue, etc.)

## 🚀 Quick Start

**New to AG-UI Rust SDK?** → **[📋 Integration Guide](./INTEGRATION.md)**

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
- **[📖 WASM Documentation](./ag-ui-wasm/README.md)** - Complete WebAssembly SDK docs
- **[🦀 Native Rust Documentation](./ag-ui-rust/README.md)** - Native SDK documentation
- **[⚡ Examples](./ag-ui-wasm/examples/)** - Real-world implementation examples

## 🎯 Use Cases

### 🌐 WebAssembly SDK
- **Single Page Applications** (React, Vue, Angular)
- **Progressive Web Apps** (PWAs)
- **Browser Extensions**
- **Cloudflare Workers**
- **Edge Computing** platforms
- **Static Site Generators** (Next.js, Nuxt, etc.)

### 🦀 Native Rust SDK
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

---

**Ready to integrate?** → **[Start with the Integration Guide](./INTEGRATION.md)** 🚀 