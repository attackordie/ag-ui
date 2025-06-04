# AG-UI WASM SDK Test Suite

This directory contains a comprehensive test suite for the AG-UI WASM SDK, ensuring the protocol implementation is correct, performant, and reliable.

## Test Structure

### 1. Integration Tests (`tests/integration_test.rs`)
Complete end-to-end tests of the AG-UI protocol flow:
- Full conversation simulation with all event types
- Event sequencing validation
- SSE encoding verification
- Message construction and role handling
- State management
- Tool definitions and execution

### 2. Test Utilities (`tests/test_utils.rs`)
Reusable testing infrastructure:
- `TestFixtures`: Pre-configured test data
- `EventBuilder`: Fluent API for creating test events
- `SSEValidator`: Validation helpers for SSE format
- `MockAgent`: Simplified agent for testing
- `PerformanceTimer`: Browser-based performance measurement
- Assertion macros for common validations

### 3. Example Agent (`tests/example_agent.rs`)
A complete weather assistant implementation demonstrating:
- Agent initialization with tools
- Message processing and streaming responses
- Tool execution (weather and forecast)
- State management and updates
- Proper event sequencing
- Real-world usage patterns

### 4. Performance Tests (`tests/performance_test.rs`)
Comprehensive performance and stress testing:
- SSE encoding benchmarks (small and large events)
- Event creation performance
- Large conversation simulation (50+ messages)
- Rapid event generation (1000 events/sec)
- Concurrent agent stress test (100 agents)
- JSON serialization benchmarks
- Maximum event size handling (up to 1MB)

## Running Tests

### Prerequisites
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Chrome/Firefox for browser testing
```

### Run All Tests
```bash
# Using the test runner script
./test_runner.sh

# Or manually with wasm-pack
wasm-pack test --chrome --headless
wasm-pack test --firefox --headless
wasm-pack test --node
```

### Run Specific Test Suites
```bash
# Integration tests only
./test_runner.sh --integration

# Performance tests only
./test_runner.sh --performance

# Example agent tests only
./test_runner.sh --example
```

### Run Individual Test Files
```bash
# Run a specific test file
wasm-pack test --chrome --headless -- --test integration_test

# Run with console output
wasm-pack test --chrome -- --nocapture
```

## Test Coverage

The test suite covers:

1. **Protocol Compliance**
   - All event types (RUN_STARTED, TEXT_MESSAGE_*, TOOL_*, STATE_UPDATE, etc.)
   - Proper event sequencing
   - SSE format validation

2. **Core Functionality**
   - Message creation and handling
   - Tool definition and execution
   - State management
   - Error handling

3. **Performance**
   - Event encoding speed
   - Memory usage under load
   - Concurrent operation handling
   - Large payload processing

4. **Real-World Scenarios**
   - Complete conversation flows
   - Multi-turn interactions
   - Tool-based responses
   - State persistence

## Writing New Tests

When adding new tests:

1. Use the provided test utilities for consistency
2. Follow the naming convention: `test_*` for unit tests, `bench_*` for benchmarks
3. Include both positive and negative test cases
4. Add performance tests for any new features that might impact speed
5. Document complex test scenarios

Example test structure:
```rust
#[wasm_bindgen_test]
async fn test_new_feature() {
    // Arrange
    let builder = EventBuilder::new("test-run");
    
    // Act
    let event = builder.custom_event();
    let encoded = SSEEncoder::new().encode(&event);
    
    // Assert
    assert_sse_valid!(encoded);
    assert_event_type!(event, EventType::Custom);
}
```

## Debugging Tests

To debug failing tests:

1. Run without headless mode: `wasm-pack test --chrome`
2. Add console logging: `web_sys::console::log_1(&"Debug info".into());`
3. Use `--nocapture` flag to see all output
4. Check browser console for JavaScript errors

## Performance Baselines

Current performance targets:
- Small event encoding: < 0.1ms per event
- Large event encoding (10KB): < 5ms per event
- 1000 event generation: < 100ms total
- 50-message conversation: < 1000ms total

## CI Integration

These tests are designed to run in CI environments:
```yaml
# Example GitHub Actions configuration
- name: Run WASM tests
  run: |
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    wasm-pack test --chrome --headless
```