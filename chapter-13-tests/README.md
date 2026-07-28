# Chapter-13-Tests: Regression Test Suite

Comprehensive regression test suite for **Chapter 13: Concurrency and Async/Await** of The Rust Programming Handbook.

## Test Coverage

The test suite validates 12 example programs covering threading, channels, and async/await patterns:

### Basic Threading (2 tests)
- `test_01_spawn_thread` - Creating and spawning new threads
- `test_02_thread_result` - Handling thread results and panics

### Shared Ownership Across Threads (4 tests)
- `test_03_arc_sharing` - Arc<T> for atomic reference counting across threads
- `test_04_mutex_counter` - Mutex for synchronized access to shared data
- `test_05_rwlock_cache` - RwLock for reader-writer synchronization

### Channels for Message Passing (4 tests)
- `test_06_channel_basics` - Basic channel creation and usage
- `test_07_channel_iteration` - Iterating over messages from channels
- `test_08_multiple_producers` - Multiple producer patterns
- `test_09_bidirectional_channel` - Two-way communication via channels

### Async/Await (2 tests)
- `test_10_async_basics` - Basic async task execution
- `test_11_tokio_basics_fails_to_compile` - Intentional compilation error (missing tokio time feature)

### Practical Concurrency (1 test)
- `test_12_mutex_lock_duration` - Managing mutex lock scope duration

## Running Tests

### Run all chapter-13 tests
```bash
cargo test --package chapter-13-tests
```

### Run a specific test
```bash
cargo test --package chapter-13-tests test_01_spawn_thread
```

### Run tests with output
```bash
cargo test --package chapter-13-tests -- --nocapture
```

## Test Methodology

Each test follows the integration test pattern:
1. Executes a chapter-13 binary using `cargo run --package chapter_13_*`
2. Captures standard output
3. Validates output contains expected strings demonstrating the concept

**Special Case**: Test `test_11_tokio_basics_fails_to_compile` verifies that a project intentionally fails to compile (missing tokio time feature as educational example).

## Documentation

Test names follow the chapter structure:
- `test_NN_*` corresponds to directory `NN_*` in chapter-13/
- Tests are organized by concept (Threading, Shared Ownership, Channels, Async, Practical)

Each test validates that the corresponding program:
- Compiles successfully (except test_11 which intentionally fails)
- Produces output that demonstrates the concurrency feature being taught
- Works as intended by the chapter's curriculum

## Integration with Full Test Suite

The chapter-13-tests crate integrates with the workspace test suite. Run all chapter tests:
```bash
cargo test --package chapter-1-tests --package chapter-2-tests --package chapter-3-tests --package chapter-4-tests --package chapter-5-tests --package chapter-6-tests --package chapter-7-tests --package chapter-8-tests --package chapter-9-tests --package chapter-10-tests --package chapter-11-tests --package chapter-13-tests
```

Total: 248 tests across 12 chapters (16 + 15 + 12 + 49 + 14 + 28 + 27 + 18 + 20 + 14 + 11 + 12)

Note: Chapter-12 regression tests are being finalized separately.
