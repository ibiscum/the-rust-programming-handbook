# Chapter-7-Tests: Regression Test Suite

Comprehensive regression test suite for **Chapter 7: Traits, Generics, and Lifetimes** of The Rust Programming Handbook.

## Test Coverage

The test suite validates 20 example programs covering traits, generics, and lifetime concepts:

### Traits Basics (5 tests)
- `test_01_defining_traits` - Basic trait definitions and implementation
- `test_02_default_implementations` - Default method implementations in traits
- `test_03_implementing_on_custom_types` - Implementing traits for custom types
- `test_04_orphan_rule` - Orphan rule constraints for trait implementation
- `test_05_trait_objects` - Dynamic dispatch with trait objects

### Generics (5 tests)
- `test_06_duplication_problem` - Why generics solve code duplication
- `test_07_generics_syntax_error` - Compiler errors without proper trait bounds
- `test_08_generics_trait_bounds` - Generic types with trait bound constraints
- `test_09_where_clauses` - Alternative syntax for generic constraints
- `test_10_returning_impl_trait` - Returning trait types with impl Trait

### Lifetimes (9 tests)
- `test_11_dangling_reference` - Why lifetimes prevent dangling references
- `test_12_borrow_checker_scopes` - Lifetime scope demonstration
- `test_13_lifetime_syntax` - Basic lifetime annotations
- `test_14_annotating_functions` - Lifetime annotations in function signatures
- `test_15_structs_with_references` - Structs containing references with lifetimes
- `test_16_impl_blocks_with_lifetimes` - Impl blocks with lifetime parameters
- `test_17_lifetime_elision` - Automatic lifetime inference rules
- `test_18_static_lifetime` - Static lifetime semantics
- (Note: Projects 19 is a standalone .rs file, not a runnable binary)

### Practical Examples (2 tests)
- `test_20_zero_copy_parser` - Zero-copy parsing with lifetimes
- `test_21_validating_references_impl` - Server configuration validation with references

## Running Tests

### Run all chapter-7 tests
```bash
cargo test --package chapter-7-tests
```

### Run a specific test
```bash
cargo test --package chapter-7-tests test_01_defining_traits
```

### Run tests with output
```bash
cargo test --package chapter-7-tests -- --nocapture
```

## Test Methodology

Each test follows the integration test pattern:
1. Executes a chapter-7 binary using `cargo run --package chapter_7_*`
2. Captures standard output
3. Validates output contains expected strings demonstrating the concept

## Documentation

Test names follow the chapter structure:
- `test_NN_*` corresponds to directory `NN_*` in chapter-7/
- Tests are organized by concept (Traits, Generics, Lifetimes, Examples)

Each test validates that the corresponding program:
- Compiles successfully
- Produces output that demonstrates the Rust feature being taught
- Works as intended by the chapter's curriculum

## Integration with Full Test Suite

The chapter-7-tests crate integrates with the workspace test suite. Run all chapter tests:
```bash
cargo test --package chapter-1-tests --package chapter-2-tests --package chapter-3-tests --package chapter-4-tests --package chapter-5-tests --package chapter-6-tests --package chapter-7-tests
```

Total: 172 tests across 7 chapters (16 + 49 + 14 + 28 + 27 + 18 + 20)
