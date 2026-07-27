# Chapter 5 Regression Tests

This test suite provides comprehensive regression testing for all Chapter 5 projects in the Rust Programming Handbook.

## Overview

The `chapter-5-tests` crate contains integration tests that validate the output and behavior of all Chapter 5 examples. This ensures that refactoring or updates to Chapter 5 code maintain backward compatibility and produce expected results.

## Test Coverage

The regression test suite includes 27 tests covering:

### Struct Basics (6 tests)
- **test_structs_accessing_fields**: Tests struct field access
- **test_structs_initialization**: Validates struct initialization patterns
- **test_structs_field_initialization**: Tests field-specific initialization
- **test_structs_update_instances**: Validates struct update syntax
- **test_structs_fields_modiy_update**: Tests field modification and updates
- **test_structs_cloning**: Validates cloning behavior for structs

### Struct Methods & Associated Functions (2 tests)
- **test_structs_associated_functions**: Tests associated function calls
- **test_structs_methods**: Validates struct method implementation

### Struct Special Cases (4 tests)
- **test_structs_unit_like**: Tests unit-like struct behavior
- **test_structs_unit_like2**: Tests unit-like struct variant
- **test_borrowing_struct_fields**: Tests borrowing in struct fields
- **test_ownership_struct_fields**: Tests ownership in struct fields

### Debug Output (2 tests)
- **test_debugging_with_structs**: Tests struct debugging output
- **test_customizing_debug_input**: Tests custom debug implementations

### Enums (4 tests)
- **test_enums_first_example**: Tests basic enum example
- **test_enum_variants**: Tests enum variants with data
- **test_enum_methods**: Tests enum methods
- **test_defining_struct_with_enum_fields**: Tests structs with enum fields

### Tuples (6 tests)
- **test_tuples_example_1**: Tests basic tuple operations
- **test_tuples_example_points**: Tests tuples for point coordinates
- **test_tuples_employee_example**: Tests tuples for employee data
- **test_tuple_destructuring**: Tests tuple destructuring
- **test_tuples_splitting_full_name**: Tests name splitting with tuples
- **test_tuples_returning_multiple_values**: Tests returning tuples from functions

### Tuple Structs (2 tests)
- **test_tuple_structs**: Tests tuple struct syntax
- **test_tuple_structs_and_functions**: Tests tuple structs with functions

### Final Examples (1 test)
- **test_final_example_1**: Tests comprehensive final example

## Running Tests

Run all regression tests:
```bash
cargo test --package chapter-5-tests
```

Run a specific test:
```bash
cargo test --package chapter-5-tests test_structs_methods
```

Run tests with output:
```bash
cargo test --package chapter-5-tests -- --nocapture
```

## Test Strategy

The tests use subprocess execution to run each Chapter 5 binary and validate:
1. **Output correctness**: Program outputs match expected values
2. **Struct behavior**: Struct initialization, methods, and updates work correctly
3. **Enum functionality**: Enum variants and methods behave as expected
4. **Tuple operations**: Tuple destructuring and multiple return values work properly
5. **Debug output**: Custom debug implementations produce expected output

## Adding New Tests

To add a new regression test:

1. Identify the Chapter 5 project (e.g., `chapter-5/new_topic`)
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

- Package names follow the pattern `chapter_5_<project_name>` with underscores replacing hyphens and spaces
- Some directories have spaces in their names (e.g., "structs_final example") which are converted to underscores in package names
- Chapter 5 focuses on complex data structures: structs, enums, and tuples
- Many projects demonstrate warnings about unused fields - these are intentional for teaching purposes
- All tests verify functionality through output validation
