# Chapter-11-Tests: Regression Test Suite

Comprehensive regression test suite for **Chapter 11: Smart Pointers and Concurrency** of The Rust Programming Handbook.

## Test Coverage

The test suite validates 12 example programs covering smart pointers, reference counting, interior mutability, and concurrency:

### Smart Pointers: Box (2 tests)
- `test_01_box_basic` - Basic Box<T> for heap allocation
- `test_02_recursive_box` - Using Box for recursive data structures

### Reference Counting (2 tests)
- `test_03_reference_counting` - Rc<T> for shared ownership
- `test_04_atomic_reference_counting` - Arc<T> for thread-safe shared ownership

### Interior Mutability (1 test)
- `test_05_interior_mutability` - Interior mutability patterns with RefCell

### Concurrency Primitives (2 tests)
- `test_06_mutex_concurrency` - Mutex for synchronized access
- `test_07_rwlock_concurrency` - RwLock for read-write synchronization

### Deref Trait (1 test)
- `test_08_deref_coercion` - Deref trait and automatic coercion

### Practical Patterns (4 tests)
- `test_09_box_option_interaction` - Combining Box with Option
- `test_10_rc_refcell_combination` - Rc<RefCell<T>> for mutable shared data
- `test_11_smart_pointer_ownership` - Ownership patterns with smart pointers
- `test_12_rc_graph_dag` - Graph structures using Rc for DAG representations

## Running Tests

### Run all chapter-11 tests
```bash
cargo test --package chapter-11-tests
```

### Run a specific test
```bash
cargo test --package chapter-11-tests test_01_box_basic
```

### Run tests with output
```bash
cargo test --package chapter-11-tests -- --nocapture
```

## Test Methodology

Each test follows the integration test pattern:
1. Executes a chapter-11 binary using `cargo run --package chapter_11_*`
2. Captures standard output
3. Validates output contains expected strings demonstrating the concept

## Documentation

Test names follow the chapter structure:
- `test_NN_*` corresponds to directory `NN_*` in chapter-11/
- Tests are organized by concept (Box, Reference Counting, Interior Mutability, Concurrency, Deref, Patterns)

Each test validates that the corresponding program:
- Compiles successfully
- Produces output that demonstrates the smart pointer feature being taught
- Works as intended by the chapter's curriculum

## Integration with Full Test Suite

The chapter-11-tests crate integrates with the workspace test suite. Run all chapter tests:
```bash
cargo test --package chapter-1-tests --package chapter-2-tests --package chapter-3-tests --package chapter-4-tests --package chapter-5-tests --package chapter-6-tests --package chapter-7-tests --package chapter-8-tests --package chapter-9-tests --package chapter-10-tests --package chapter-11-tests
```

Total: 224 tests across 11 chapters (16 + 15 + 49 + 14 + 28 + 27 + 18 + 20 + 14 + 11 + 12)
