# Chapter 15 Regression Tests

This directory contains integration tests for Chapter 15 examples covering advanced Rust concepts including unsafe code, FFI, and CLI applications.

## Running Tests

To run all chapter-15 tests:
```bash
cargo test --package chapter-15-tests
```

To run a specific test:
```bash
cargo test --package chapter-15-tests test_01_zero_cost_abstraction_compiles
```

## Test Coverage

### Core Rust Concepts (4 tests)
These tests validate advanced Rust features:

- `test_01_zero_cost_abstraction_compiles` - Zero-cost abstractions comparison
- `test_02_stack_vs_heap_compiles` - Stack vs heap memory management
- `test_03_raw_pointers_compiles` - Raw pointer usage and safety
- `test_04_unsafe_function_compiles` - Unsafe function declarations and calls

### CLI and File Handling (5 tests)
These tests cover command-line argument parsing and file I/O:

- `test_05_cli_args_compiles` - Manual command-line argument parsing
- `test_06_clap_args_compiles` - Clap framework for CLI parsing (requires clap 4.4)
- `test_07_read_file_lines_compiles` - Reading files line by line
- `test_08_list_dir_contents_compiles` - Directory traversal
- `test_09_build_compiles` - Build script example

### Foreign Function Interface (FFI) (5 tests)
These tests demonstrate interoperability with C code:

- `test_10_c_callbacks_compiles` - C callbacks from Rust
- `test_11_c_struct_layout_compiles` - C structure layout compatibility
- `test_12_safe_ffi_wrapper_compiles` - Safe wrappers around unsafe FFI
- `test_13_ffi_c_lib_compiles` - FFI C library implementation (flexible due to attribute syntax changes)
- `test_handling_standard_input_compiles` - Standard input handling

## Dependencies

Chapter-15 projects require:
- **clap** (4.4) - Command-line parsing with derive macros
- Standard Rust library (no external dependencies for most projects)

## Notes

- Most projects compile and run successfully with basic Rust features
- FFI examples may show deprecation warnings with newer Rust versions
- Projects using unsafe code are intentionally designed to demonstrate Rust's safety features
- Test coverage focuses on compilation validation; behavioral tests should verify actual functionality with sample inputs
