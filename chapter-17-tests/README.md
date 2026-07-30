# Chapter 17 Regression Tests

This directory contains integration tests for Chapter 17 examples covering common Rust pitfalls, debugging techniques, and best practices.

## Running Tests

To run all chapter-17 tests:
```bash
cargo test --package chapter-17-tests
```

To run a specific test:
```bash
cargo test --package chapter-17-tests test_01_double_ownership
```

## Test Coverage

Chapter 17 contains 40 comprehensive examples organized into 5 major categories:

### Ownership and Borrowing (9 tests)
Demonstrates common pitfalls with Rust's ownership and borrowing system:

- `test_01_double_ownership` - Multiple ownership attempts
- `test_02_dangling_references` - Dangling reference patterns
- `test_03_borrowing_confusion` - Borrowing conflicts and confusion
- `test_04_dangling_references_scope` - Scope-related dangling references
- `test_05_lifetime_specifiers` - Lifetime annotations and their purpose
- `test_06_mutable_vs_immutable_lifetimes` - Immutability requirements in lifetimes
- `test_07_simultaneous_references` - Multiple simultaneous references
- `test_08_multiple_mutable_references` - Mutable reference restrictions
- `test_09_scope_confusion` - Variable scope understanding issues

### Error Handling (8 tests)
Best practices and pitfalls in error handling:

- `test_10_overusing_panic` - When to use panic vs Result
- `test_11_ignoring_result` - Handling discarded Results properly
- `test_12_unwrapping_without_care` - Safe unwrap patterns
- `test_13_overusing_unwrap` - Alternatives to unwrap
- `test_14_uninformative_expect` - Informative error messages
- `test_15_not_using_combinators` - Option/Result combinators
- `test_16_using_map` - Using map for Result/Option
- `test_17_using_unwrap_or_else` - Safe alternatives to unwrap

### Concurrency and Performance (9 tests)
Common concurrency issues and optimization pitfalls:

- `test_18_data_races` - Thread-safe data access
- `test_19_deadlocks` - Deadlock prevention
- `test_20_atomic_memory_ordering` - Atomic memory operations
- `test_21_inefficient_scheduling` - Thread pool optimization (requires rayon)
- `test_22_inefficient_memory` - Memory efficiency patterns
- `test_23_excessive_dynamic_dispatch` - Static vs dynamic dispatch
- `test_24_unoptimized_data_structures` - Choosing efficient data structures
- `test_25_excessive_cloning` - Clone vs reference patterns
- `test_26_suboptimal_iteration` - Iterator efficiency

### Debugging and Diagnostics (9 tests)
Techniques for understanding compiler errors and debugging:

- `test_27_mismatched_types` - Type mismatch resolution
- `test_28_mismatched_return_types` - Return type errors
- `test_29_unresolved_imports` - Import resolution issues
- `test_30_unused_variables` - Handling unused variables
- `test_31_unclear_error_messages` - Interpreting error messages
- `test_32_complex_lifetime_errors` - Complex lifetime diagnostics
- `test_33_ineffective_logging` - Structured logging (requires log, env_logger)
- `test_34_test_coverage_gaps` - Test writing strategies
- `test_36_debugging_concurrency` - Debugging multithreaded code (requires log, env_logger)

### Best Practices (5 tests)
Idiomatic Rust and ecosystem usage:

- `test_37_ownership_model` - Ownership model mastery
- `test_38_type_system` - Effective type system usage
- `test_39_idiomatic_rust` - Writing idiomatic Rust code
- `test_40_optimize_performance` - Performance optimization strategies
- `test_41_leverage_ecosystem` - Using ecosystem crates effectively (requires serde, serde_json)

## Dependencies

Chapter-17 projects have selective dependencies:

- **rayon** (1.10) - Parallel iterators (project 21)
- **log** (0.4) - Logging facade (projects 33, 36)
- **env_logger** (0.10) - Environment-based logging (projects 33, 36)
- **serde** (1.0) - Serialization (project 41)
- **serde_json** (1.0) - JSON support (project 41)

Most projects use only the Rust standard library.

## Notes

- All 40 tests verify successful compilation of examples
- Examples demonstrate both anti-patterns and best practices
- Chapter 17 progresses from basic concepts to advanced debugging
- Tests run quickly as they focus on compilation validation
- Use `cargo run --package <package_name>` to execute examples and see their output
