# Chapter 6 Regression Tests

This test suite provides comprehensive regression testing for all Chapter 6 projects in the Rust Programming Handbook.

## Overview

The `chapter-6-tests` crate contains integration tests that validate the output and behavior of all Chapter 6 examples. This ensures that refactoring or updates to Chapter 6 code maintain backward compatibility and produce expected results.

## Test Coverage

The regression test suite includes 18 tests covering:

### Option Type (1 test)
- **test_option_example_1**: Tests Option enum usage and searching

### Result Type (1 test)
- **test_result_example_1**: Tests Result enum for division operations

### Combinators (1 test)
- **test_combinators**: Tests result combinators and transformations

### Error Handling Methods (3 tests)
- **test_unwrap**: Tests unwrap() method on Results
- **test_expect**: Tests expect() method with custom panic messages
- **test_safe_unwrapping**: Tests safe unwrapping patterns like unwrap_or()

### Question Mark Operator (2 tests)
- **test_question_mark_operator**: Tests the ? operator for error propagation
- **test_question_mark_operator_with_option**: Tests ? operator with Option types

### Error Propagation (1 test)
- **test_manual_propagation**: Tests manual error propagation patterns

### Custom Error Types (3 tests)
- **test_custom_error_types_with_enum**: Tests enum-based custom errors
- **test_custom_errors_functions**: Tests custom error implementations in functions
- **test_implementing_standard_error_traits**: Tests implementing std::error::Error trait

### Panic Handling (1 test)
- **test_panic_example**: Tests panic! macro and panic handling

### External Dependency Projects (5 tests)
These projects demonstrate using external crates for error handling:
- **test_anyhow_example**: Tests the anyhow crate for error handling
- **test_env_logger_example**: Tests the env_logger crate for structured logging
- **test_env_logger_with_context**: Tests env_logger with contextual information
- **test_thiserror_example**: Tests the thiserror crate for custom error types
- **test_errors_final_example**: Tests comprehensive error handling with external crates

## Running Tests

Run all regression tests:
```bash
cargo test --package chapter-6-tests
```

Run a specific test:
```bash
cargo test --package chapter-6-tests test_question_mark_operator
```

Run tests with output:
```bash
cargo test --package chapter-6-tests -- --nocapture
```

## Test Strategy

The tests use subprocess execution to run each Chapter 6 binary and validate:
1. **Output correctness**: Program outputs match expected values
2. **Error handling**: Error handling patterns work correctly
3. **Option/Result behavior**: Option and Result types behave as expected
4. **Propagation**: Error propagation with ? operator works correctly
5. **Custom errors**: Custom error types implement expected behavior
6. **Expected failures**: External dependency projects verify compilation errors

## Adding New Tests

To add a new regression test:

1. Identify the Chapter 6 project (e.g., `chapter-6/new_topic`)
2. Determine its package name from the Cargo.toml (uses underscores)
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

- Package names follow the pattern `chapter_6_<project_name>` with underscores replacing hyphens
- Chapter 6 focuses on error handling: Option, Result, custom errors, and panic behavior
- External dependency projects (anyhow, env_logger, thiserror) have their dependencies added and compile successfully
- Tests verify error handling concepts through output validation
- Several projects show error propagation patterns essential for idiomatic Rust error handling
