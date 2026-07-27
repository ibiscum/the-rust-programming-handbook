# Chapter 2 Regression Tests

This test suite provides comprehensive regression testing for all Chapter 2 projects in the Rust Programming Handbook.

## Overview

The `chapter-2-tests` crate contains integration tests that validate the output and behavior of all Chapter 2 examples. This ensures that refactoring or updates to Chapter 2 code maintain backward compatibility and produce expected results.

## Test Coverage

The regression test suite includes 49 tests covering:

### Basic Types (7 tests)
- **test_arrays**: Validates array indexing and output
- **test_booleans**: Checks boolean value output
- **test_characters**: Confirms character and emoji handling
- **test_floating_point_numbers**: Validates float formatting
- **test_integers**: Verifies signed and unsigned integer output
- **test_string**: Tests String type manipulation
- **test_string_literal**: Validates string literal behavior

### Collections (3 tests)
- **test_tuples**: Checks tuple destructuring
- **test_tuples2**: Validates tuple math operations
- **test_slices**: Tests array slicing and views

### Variables (3 tests)
- **test_immutable_variables**: Verifies immutable variable behavior
- **test_mutable_variables**: Confirms mutable variable changes
- **test_shadowing**: Validates variable shadowing behavior

### Control Flow (8 tests)
- **test_if_else**: Tests conditional expressions
- **test_loop_1**: Validates loop behavior
- **test_loops_2**: Tests loop with break/continue
- **test_while**: Confirms while loop behavior
- **test_for_1**: Validates for loop over arrays
- **test_for_nested**: Tests nested loops
- **test_for_range**: Validates range iteration

### Functions (6 tests)
- **test_functions_syntax**: Validates function definitions
- **test_functions_return_values**: Tests return value behavior
- **test_function_passing_by_value**: Confirms value passing
- **test_functions_passing_by_reference**: Tests borrowing
- **test_functions_passing_by_mutable_reference**: Tests mutable borrowing
- **test_functions_clone**: Validates cloning behavior

### Ownership (1 test)
- **test_ownership_and_functions**: Tests ownership transfer in functions

### Pattern Matching (5 tests)
- **test_matching_literals**: Validates literal pattern matching
- **test_matching_ranges**: Tests range patterns
- **test_matching_with_variables**: Confirms variable capture in patterns
- **test_match_guards**: Tests guard conditions
- **test_combining_patterns**: Validates pattern combinations

### Enums (5 tests)
- **test_enums**: Validates enum type definition
- **test_enums_match**: Tests enum pattern matching
- **test_enums_methods_1**: Confirms enum with methods
- **test_enums_methods_2**: Tests complex enum methods
- **test_destructuring_enums**: Validates enum field destructuring

### Structs (5 tests)
- **test_structs**: Validates struct definition and usage
- **test_structs_initialization**: Tests initialization syntax
- **test_structs_methods**: Confirms struct methods
- **test_tuple_structs**: Tests tuple struct syntax
- **test_unit_structs**: Validates unit struct behavior
- **test_structs_associated_functions_fails_to_compile**: Verifies intentional compilation error

### Option & Result (4 tests)
- **test_option**: Tests Option enum usage
- **test_option_type**: Validates Option type handling
- **test_result_type**: Tests Result type and error handling
- **test_unwrap_or**: Confirms unwrap_or behavior

### Error Handling & Panic (2 tests)
- **test_error_handling_1**: Tests error handling patterns
- **test_panic_macro_panics**: Verifies intentional panic behavior

## Running Tests

Run all regression tests:
```bash
cargo test --package chapter-2-tests
```

Run a specific test:
```bash
cargo test --package chapter-2-tests test_arrays
```

Run tests with output:
```bash
cargo test --package chapter-2-tests -- --nocapture
```

## Test Strategy

The tests use subprocess execution to run each Chapter 2 binary and validate:
1. **Output correctness**: Program outputs match expected values
2. **Compilation success**: Programs compile without critical errors
3. **Expected failures**: Programs like `structs_associated_functions` intentionally demonstrate compilation errors
4. **Panic behavior**: Programs like `panic_macro` intentionally panic to demonstrate error handling

## Adding New Tests

To add a new regression test:

1. Identify the Chapter 2 project (e.g., `chapter-2/new_topic`)
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

- Package names follow the pattern `chapter_2_<project_name>` with underscores replacing dots and hyphens
- Some tests check that programs compile with warnings but still run successfully
- Tests for panic and compilation errors verify the intended failure behavior
- Interactive programs that require user input are excluded from the test suite
