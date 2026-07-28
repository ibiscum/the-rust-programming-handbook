# Chapter-10-Tests: Regression Test Suite

Comprehensive regression test suite for **Chapter 10: Testing** of The Rust Programming Handbook.

## Test Coverage

The test suite validates 15 testing modules covering unit tests, integration tests, documentation tests, and test-driven development:

### Basic Testing (6 tests)
- `test_01_basic_unit_tests` - Basic unit test structure and organization
- `test_02_assertion_macros` - Common assertion macros (assert!, assert_eq!, assert_ne!)
- `test_03_testing_results_and_options` - Testing functions that return Results and Options
- `test_04_testing_panics` - Testing code that panics
- `test_05_ignoring_tests` - Ignored tests and running only specific tests
- `test_06_library_structure` - Library organization and public vs private functions

### Integration Testing (2 tests)
- `test_07_integration_tests` - Integration tests for multiple modules
- `test_08_test_helpers` - Helper functions and common test utilities

### Documentation Testing (2 tests)
- `test_09_documentation_tests` - Documentation tests with code examples
- `test_10_controlling_doc_tests` - Controlling doc test execution

### Test Driven Development (5 tests)
- `test_11_test_driven_development_has_expected_failures` - Red Phase: Intentional test failures demonstrating TDD methodology
- `test_12_tdd_green_phase` - Green Phase: All tests passing with working implementation
- `test_13_tdd_refactor_phase` - Refactor Phase: Improved code while maintaining test success
- `test_14_tdd_api_handler` - TDD applied to API handler implementation
- `test_15_test_doubles_and_stubs` - Test doubles, stubs, and mocks for testing

## Running Tests

### Run all chapter-10 tests
```bash
cargo test --package chapter-10-tests
```

### Run a specific test
```bash
cargo test --package chapter-10-tests test_01_basic_unit_tests
```

### Run tests with output
```bash
cargo test --package chapter-10-tests -- --nocapture
```

## Test Methodology

Each regression test verifies that the corresponding chapter-10 test module:
1. Compiles successfully
2. Runs the internal test suite via `cargo test`
3. Produces expected results (pass or intentional failure)

**Special Case**: `test_11_test_driven_development_has_expected_failures` verifies that the TDD Red Phase example intentionally has failing tests to demonstrate the test-first development methodology.

## Documentation

Test names follow the chapter structure:
- `test_NN_*` corresponds to directory `NN_*` in chapter-10/
- Tests are organized by concept (Basic Testing, Integration, Documentation, TDD, Test Doubles)

Each test validates that the corresponding testing module:
- Compiles and runs without errors (except test_11 which intentionally fails)
- Demonstrates the testing pattern or technique being taught
- Works as intended by the chapter's curriculum

## Integration with Full Test Suite

The chapter-10-tests crate integrates with the workspace test suite. Run all chapter tests:
```bash
cargo test --package chapter-1-tests --package chapter-2-tests --package chapter-3-tests --package chapter-4-tests --package chapter-5-tests --package chapter-6-tests --package chapter-7-tests --package chapter-8-tests --package chapter-9-tests --package chapter-10-tests
```

Total: 212 tests across 10 chapters (16 + 49 + 14 + 28 + 27 + 18 + 20 + 14 + 11 + 15)
