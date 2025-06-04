#!/bin/bash

# AG-UI WASM SDK Test Runner
# This script runs all tests for the AG-UI WASM SDK

echo "🧪 AG-UI WASM SDK Test Suite"
echo "============================"
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ Error: wasm-pack is not installed"
    echo "Please install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WASM package first
echo "📦 Building WASM package..."
wasm-pack build --target web --out-dir pkg

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"
echo ""

# Run tests in different browsers
echo "🌐 Running tests in browsers..."
echo ""

# Chrome tests
echo "🔵 Testing in Chrome..."
wasm-pack test --chrome --headless

# Firefox tests
echo "🦊 Testing in Firefox..."
wasm-pack test --firefox --headless

# Node tests (for non-browser specific functionality)
echo "📗 Testing in Node..."
wasm-pack test --node

echo ""
echo "✅ All tests completed!"
echo ""

# Optional: Run specific test files
if [ "$1" = "--integration" ]; then
    echo "🔄 Running integration tests only..."
    wasm-pack test --chrome --headless -- --test integration_test
elif [ "$1" = "--performance" ]; then
    echo "⚡ Running performance tests only..."
    wasm-pack test --chrome --headless -- --test performance_test
elif [ "$1" = "--example" ]; then
    echo "📚 Running example agent tests only..."
    wasm-pack test --chrome --headless -- --test example_agent
fi

# Generate test coverage report (if grcov is installed)
if command -v grcov &> /dev/null; then
    echo ""
    echo "📊 Generating coverage report..."
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/
    echo "Coverage report generated at: target/coverage/index.html"
fi