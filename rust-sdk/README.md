# AG-UI Rust SDK

This directory contains Rust implementations of the Agent-User Interaction Protocol (AG-UI).

## Available SDKs

### `ag-ui-wasm/` - V8 Isolate SDK (WASM)

A WebAssembly-based SDK specifically designed for V8 isolate environments like browsers and Cloudflare Workers.

**Key Features:**
- Web Streams API for streaming
- Fetch API for HTTP requests
- No native dependencies (Tokio-free)
- Optimized for V8 isolates
- Real-time SSE streaming

**Target Environments:**
- Cloudflare Workers
- Browser environments
- Deno
- Any V8-based runtime

[View Documentation →](./ag-ui-wasm/README.md)

## Getting Started

Choose the appropriate SDK based on your deployment target:

- **Cloudflare Workers/Browser**: Use `ag-ui-wasm`
- **Native Rust applications**: Use `ag-ui-wasm` (future: dedicated native SDK)

## Project Structure

```
rust-sdk/
├── ag-ui-wasm/           # WASM SDK for V8 isolates
│   ├── src/              # Source code
│   ├── examples/         # Usage examples
│   └── tests/            # Test suite
└── README.md             # This file
```

## Development

Each SDK has its own development workflow. See the individual README files for specific instructions.

### Prerequisites

- Rust 1.70+
- wasm-pack (for WASM builds)
- Node.js (for testing)

### Building All SDKs

```bash
# Build WASM SDK
cd ag-ui-wasm
wasm-pack build --target web
```

## Contributing

We welcome contributions to any of the Rust SDKs. Please ensure your changes:

1. Work within the constraints of the target environment
2. Include appropriate tests
3. Follow Rust best practices
4. Update documentation as needed

## License

All Rust SDKs are licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option. 