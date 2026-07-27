# Chapter 4 Regression Tests

This test suite provides comprehensive regression testing for all Chapter 4 projects in the Rust Programming Handbook.

## Overview

The `chapter-4-tests` crate contains integration tests that validate the output and behavior of all Chapter 4 examples. This ensures that refactoring or updates to Chapter 4 code maintain backward compatibility and produce expected results.

## Test Coverage

The regression test suite includes 28 tests covering:

### Ownership Basics (3 tests)
- **test_ownership_1**: Validates basic ownership concepts
- **test_ownership_key_rules**: Tests ownership key rules with multiple values
- **test_ownership_cleaning_up**: Verifies automatic cleanup behavior

### Moving Ownership (6 tests)
- **test_move_ownership**: Tests moving ownership of values
- **test_moving_ownership**: Validates ownership transfer
- **test_moving_ownership_2**: Tests large data structure moves
- **test_moving_in_function_calls**: Validates move in function calls
- **test_ownership_transfer_function_calls**: Tests ownership transfer in function parameters
- **test_returning_ownership**: Validates ownership return from functions

### Borrowing - Immutable (3 tests)
- **test_immutable_borrowing**: Tests immutable references
- **test_borrowing_read_only_access**: Validates read-only access patterns
- **test_borrowing_functions_1**: Tests borrowing in function parameters

### Borrowing - Mutable (3 tests)
- **test_mutable_borrowing**: Tests mutable references
- **test_mutable_borrowing_function**: Validates mutable borrowing in functions
- **test_borrowing_functions_2**: Tests mutable borrowing patterns

### Borrowing Rules (1 test)
- **test_borrowing_rules**: Validates Rust borrowing rules

### Traits (2 tests)
- **test_copy_trait**: Tests Copy trait behavior
- **test_clone_trait**: Tests Clone trait behavior

### Library & Pitfalls (2 tests)
- **test_managing_library**: Tests managing library patterns
- **test_pitfall_1**: Validates pitfall demonstration #1
- **test_pitfall_2**: Validates pitfall demonstration #2

### Test Copy Variants (5 tests)
- **test_test_copy**: Tests basic copy behavior
- **test_test_copy_4**: Tests copy variant #4
- **test_test_copy_13**: Tests copy variant #13
- **test_test_copy_14**: Tests copy variant #14
- **test_test_copy_15**: Tests copy variant #15

### Compilation Errors (Intentional) (2 tests)
- **test_lifetime_error_fails_to_compile**: Verifies missing lifetime specifier error
- **test_returning_reference_fails_to_compile**: Verifies dangling reference error

## Running Tests

Run all regression tests:
```bash
cargo test --package chapter-4-tests
```

Run a specific test:
```bash
cargo test --package chapter-4-tests test_immutable_borrowing
```

Run tests with output:
```bash
cargo test --package chapter-4-tests -- --nocapture
```

## Test Strategy

The tests use subprocess execution to run each Chapter 4 binary and validate:
1. **Output correctness**: Program outputs match expected values
2. **Ownership behavior**: Ownership transfer and cleanup work as expected
3. **Borrowing patterns**: Immutable and mutable borrowing behave correctly
4. **Trait implementations**: Copy and Clone work as expected
5. **Expected failures**: Programs like `lifetime_error` and `returning_reference` intentionally demonstrate compilation errors

## Adding New Tests

To add a new regression test:

1. Identify the Chapter 4 project (e.g., `chapter-4/new_topic`)
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

- Package names follow the pattern `chapter_4_<project_name>` with underscores replacing hyphens and spaces
- Some directories have spaces in their names (e.g., "test copy") which is handled correctly
- Chapter 4 focuses on ownership, borrowing, and lifetime concepts
- Some projects intentionally demonstrate compilation errors to teach Rust's safety features
- All tests verify concepts through output validation or compilation behavior
