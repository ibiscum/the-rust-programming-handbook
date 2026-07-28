use std::process::Command;

/// Helper function to run a chapter-13 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_13_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Helper function to check if a binary fails to compile
fn binary_fails_to_compile(binary_name: &str) -> bool {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_13_{}", binary_name),
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    !output.status.success()
}

// ===== Basic Threading =====

#[test]
fn test_01_spawn_thread() {
    let output = run_binary("01_spawn_thread");
    assert!(
        output.contains("Main thread") && output.contains("New thread"),
        "Expected output to contain main and spawned thread messages, got: {}",
        output
    );
}

#[test]
fn test_02_thread_result() {
    let output = run_binary("02_thread_result");
    assert!(
        output.contains("Main:") && output.contains("Spawning"),
        "Expected output to contain thread result handling, got: {}",
        output
    );
}

// ===== Shared Ownership Across Threads =====

#[test]
fn test_03_arc_sharing() {
    let output = run_binary("03_arc_sharing");
    assert!(
        output.contains("Arc") || output.contains("count") || output.contains("Thread"),
        "Expected output to contain Arc sharing examples, got: {}",
        output
    );
}

#[test]
fn test_04_mutex_counter() {
    let output = run_binary("04_mutex_counter");
    assert!(
        output.contains("counter") || output.contains("Thread") || output.contains("Finished"),
        "Expected output to contain mutex operations, got: {}",
        output
    );
}

#[test]
fn test_05_rwlock_cache() {
    let output = run_binary("05_rwlock_cache");
    assert!(
        output.contains("Reader") || output.contains("Writer") || output.contains("lock"),
        "Expected output to contain RwLock operations, got: {}",
        output
    );
}

// ===== Channels =====

#[test]
fn test_06_channel_basics() {
    let output = run_binary("06_channel_basics");
    assert!(
        output.contains("Channel") || output.contains("Sender") || output.contains("Receiver"),
        "Expected output to contain channel creation, got: {}",
        output
    );
}

#[test]
fn test_07_channel_iteration() {
    let output = run_binary("07_channel_iteration");
    assert!(
        output.contains("Sender") || output.contains("Received") || output.contains("Thread"),
        "Expected output to contain channel iteration, got: {}",
        output
    );
}

#[test]
fn test_08_multiple_producers() {
    let output = run_binary("08_multiple_producers");
    assert!(
        output.contains("Producer") || output.contains("Consumer") || output.contains("Received"),
        "Expected output to contain multiple producer examples, got: {}",
        output
    );
}

#[test]
fn test_09_bidirectional_channel() {
    let output = run_binary("09_bidirectional_channel");
    assert!(
        output.contains("Main Thread") || output.contains("Worker") || output.contains("signal"),
        "Expected output to contain bidirectional channel examples, got: {}",
        output
    );
}

// ===== Async/Await =====

#[test]
fn test_10_async_basics() {
    let output = run_binary("10_async_basics");
    assert!(
        output.contains("Task") || output.contains("fetch") || output.contains("Received"),
        "Expected output to contain async task examples, got: {}",
        output
    );
}

#[test]
fn test_11_tokio_basics_fails_to_compile() {
    assert!(
        binary_fails_to_compile("11_tokio_basics"),
        "Expected chapter_13_11_tokio_basics to fail compilation (missing tokio time feature)"
    );
}

#[test]
fn test_12_mutex_lock_duration() {
    let output = run_binary("12_mutex_lock_duration");
    assert!(
        output.contains("Updated") || output.contains("Thread") || output.contains("value"),
        "Expected output to contain lock duration examples, got: {}",
        output
    );
}
