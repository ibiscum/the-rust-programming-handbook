# Chapter 3 Regression Tests

This test suite provides comprehensive regression testing for all Chapter 3 projects in the Rust Programming Handbook.

## Overview

The `chapter-3-tests` crate contains integration tests that validate the output and behavior of all Chapter 3 examples. This ensures that refactoring or updates to Chapter 3 code maintain backward compatibility and produce expected results.

## Test Coverage

The regression test suite includes 14 tests covering:

### Basic Functions (6 tests)
- **test_basic_function_syntax**: Validates basic function definition and execution
- **test_functons_no_parameters_no_return_value**: Tests functions with no parameters or return values (note: typo in directory name is preserved)
- **test_functions_no_return_value**: Validates functions that perform side effects
- **test_parameters_and_return_values**: Tests parameter passing and return values
- **test_returning_values_and_ownership**: Validates value return and ownership transfer
- **test_function_documented**: Tests documented function behavior

### Borrowing & Ownership (3 tests)
- **test_functions_immutable_borrowing**: Tests immutable reference passing
- **test_functions_mutable_borrowing**: Tests mutable reference passing
- **test_functions_ownership**: Tests ownership transfer in function calls

### Anonymous Functions & Closures (4 tests)
- **test_anonymous_functions**: Validates closure behavior with map and sum operations
- **test_closures_1**: Tests basic closure syntax
- **test_closures_2**: Validates closure capture behavior and mutable closures
- **test_closures_3**: Tests closures with threads

### Higher-Order Functions (1 test)
- **test_high_order_functions**: Tests function composition and advanced functional operations

## Running Tests

Run all regression tests:
```bash
cargo test --package chapter-3-tests
```

Run a specific test:
```bash
cargo test --package chapter-3-tests test_anonymous_functions
```

Run tests with output:
```bash
cargo test --package chapter-3-tests -- --nocapture
```

## Test Strategy

The tests use subprocess execution to run each Chapter 3 binary and validate:
1. **Output correctness**: Program outputs match expected values
2. **Functional behavior**: Operations like closures, ownership transfer, and borrowing work as expected
3. **Parameter and return value handling**: Functions correctly process inputs and outputs

## Adding New Tests

To add a new regression test:

1. Identify the Chapter 3 project (e.g., `chapter-3/new_topic`)
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

- Package names follow the pattern `chapter_3_<project_name>` with underscores replacing hyphens
- The `functons_no_parameters_no_return_value` directory has a typo in its name (missing 'n'), but the package name is correct
- Chapter 3 focuses on advanced function concepts: closures, higher-order functions, and ownership/borrowing patterns
- All tests verify functional correctness through output validation
