# Chapter 1 Regression Tests

This test suite provides comprehensive regression testing for all Chapter 1 projects in the Rust Programming Handbook.

## Overview

The `chapter-1-tests` crate contains integration tests that validate the output and behavior of all Chapter 1 examples. This ensures that refactoring or updates to Chapter 1 code maintain backward compatibility and produce expected results.

## Test Coverage

The regression test suite includes 16 tests covering:

### Basic Data Types
- **test_hello_world**: Validates the "Hello, Rust!" greeting
- **test_booleans**: Checks boolean value output
- **test_integers**: Verifies signed and unsigned integer outputs
- **test_floats**: Confirms floating-point number formatting
- **test_characters**: Validates character output

### Variables
- **test_constants**: Tests constant declarations and values
- **test_immutable_variables**: Verifies immutable variable behavior
- **test_mutable_variables**: Confirms mutable variable changes
- **test_shadowing**: Validates variable shadowing behavior

### Control Flow
- **test_control_flow_1**: Tests if/else expression (even/odd check)
- **test_control_flow_2**: Validates loop counting behavior
- **test_control_flow_3**: Tests complex control flow with iterators

### Functions & Other Topics
- **test_functions**: Validates function definitions and calls
- **test_developer_mood**: Tests mixed content output
- **test_web_server_compiles**: Ensures web_server compiles successfully
- **test_why_learn_rust_fails_to_compile**: Verifies intentional compilation error (demonstrates Rust's safety)

## Running Tests

Run all regression tests:
```bash
cargo test --package chapter-1-tests
```

Run a specific test:
```bash
cargo test --package chapter-1-tests test_hello_world
```

Run tests with output:
```bash
cargo test --package chapter-1-tests -- --nocapture
```

## Test Strategy

The tests use subprocess execution to run each Chapter 1 binary and validate:
1. **Output correctness**: Program outputs match expected values
2. **Compilation success**: Complex programs like `web_server` compile without errors
3. **Expected failures**: `why_learn_rust` intentionally demonstrates compilation errors

## Adding New Tests

To add a new regression test:

1. Identify the Chapter 1 project (e.g., `chapter-1/new_topic`)
2. Determine its package name from the Cargo.toml
3. Run the binary manually to observe expected output
4. Add a test function to `tests/regression_tests.rs`:

```rust
#[test]
fn test_new_topic() {
    let output = run_binary("new_topic");
    assert!(
        output.contains("expected output"),
        "Expected output to contain 'expected output', got: {}",
        output
    );
}
```

## Notes

- Tests that run interactive programs (like `cli_calculator`) are excluded since they require user input
- Package names follow the pattern `chapter_1_<project_name>` with underscores replacing dots and hyphens
- The special case of `functions.rs` is mapped to `chapter_1_functions_rs` package name
