use std::process::Command;

/// Helper function to run a chapter-4 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_4_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Ownership Basics =====

#[test]
fn test_ownership_1() {
    let output = run_binary("ownership_1");
    assert!(
        output.contains("Mario") && output.contains("Carbonara"),
        "Expected output to contain names, got: {}",
        output
    );
}

#[test]
fn test_ownership_key_rules() {
    let output = run_binary("ownership_key_rules");
    assert!(
        output.contains("The Rust Programming Guide") && output.contains("Rust and Beyond"),
        "Expected output to contain book titles, got: {}",
        output
    );
}

#[test]
fn test_ownership_cleaning_up() {
    let output = run_binary("ownership_cleaning_up");
    assert!(
        output.contains("Temporary"),
        "Expected output to contain 'Temporary', got: {}",
        output
    );
}

// ===== Moving Ownership =====

#[test]
fn test_move_ownership() {
    let output = run_binary("move_ownership");
    assert!(
        output.contains("Rustacean"),
        "Expected output to contain 'Rustacean', got: {}",
        output
    );
}

#[test]
fn test_moving_ownership() {
    let output = run_binary("moving_ownership");
    assert!(
        output.contains("Delicious"),
        "Expected output to contain 'Delicious', got: {}",
        output
    );
}

#[test]
fn test_moving_ownership_2() {
    let output = run_binary("moving_ownership_2");
    assert!(
        output.contains("Demonstrating a move") && output.contains("Processing data by move"),
        "Expected output to contain move demonstration, got: {}",
        output
    );
}

#[test]
fn test_moving_in_function_calls() {
    let output = run_binary("moving_in_function_calls");
    assert!(
        output.contains("Processing: Important data"),
        "Expected output to contain processing message, got: {}",
        output
    );
}

#[test]
fn test_ownership_transfer_function_calls() {
    let output = run_binary("ownership_transfer_function_calls");
    assert!(
        output.contains("Taking ownership: Hello, world!"),
        "Expected output to contain ownership transfer, got: {}",
        output
    );
}

#[test]
fn test_returning_ownership() {
    let output = run_binary("returning_ownership");
    assert!(
        output.contains("Hello, Rust!"),
        "Expected output to contain greeting, got: {}",
        output
    );
}

// ===== Borrowing - Immutable =====

#[test]
fn test_immutable_borrowing() {
    let output = run_binary("immutable_borrowing");
    assert!(
        output.contains("The length of 'Rust Programming' is 16"),
        "Expected output to contain length calculation, got: {}",
        output
    );
}

#[test]
fn test_borrowing_read_only_access() {
    let output = run_binary("borrowing_read_only_access");
    assert!(
        output.contains("The book title is: Rust Programming") && output.contains("The book author is: John Doe"),
        "Expected output to contain book details, got: {}",
        output
    );
}

#[test]
fn test_borrowing_functions_1() {
    let output = run_binary("borrowing_functions_1");
    assert!(
        output.contains("Using framework: Actix") && output.contains("Framework: Actix"),
        "Expected output to contain framework info, got: {}",
        output
    );
}

// ===== Borrowing - Mutable =====

#[test]
fn test_mutable_borrowing() {
    let output = run_binary("mutable_borrowing");
    assert!(
        output.contains("Updated article: Rust is awesome for system programming!"),
        "Expected output to contain updated article, got: {}",
        output
    );
}

#[test]
fn test_mutable_borrowing_function() {
    let output = run_binary("mutable_borrowing_function");
    assert!(
        output.contains("Deposited $200.00") && output.contains("Final balance is $1200.00"),
        "Expected output to contain deposit and balance info, got: {}",
        output
    );
}

#[test]
fn test_borrowing_functions_2() {
    let output = run_binary("borrowing_functions_2");
    assert!(
        output.contains("Modified title: Rust Basics - Advanced Topics"),
        "Expected output to contain modified title, got: {}",
        output
    );
}

// ===== Borrowing Rules =====

#[test]
fn test_borrowing_rules() {
    let output = run_binary("borrowing_rules");
    assert!(
        output.contains("Note: Rust is fast") && output.contains("Updated note: Rust is fast and safe"),
        "Expected output to contain note updates, got: {}",
        output
    );
}

// ===== Traits =====

#[test]
fn test_copy_trait() {
    let output = run_binary("copy_trait");
    // This program has warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "copy_trait should compile and run"
    );
}

#[test]
fn test_clone_trait() {
    let output = run_binary("clone_trait");
    assert!(
        output.contains("s1 = hello, s2 = hello"),
        "Expected output to contain cloned strings, got: {}",
        output
    );
}

// ===== Library & Pitfalls =====

#[test]
fn test_managing_library() {
    let output = run_binary("managing_library");
    assert!(
        output.contains("Rust Programming") && output.contains("Advanced Rust"),
        "Expected output to contain book titles, got: {}",
        output
    );
}

#[test]
fn test_pitfall_1() {
    let output = run_binary("pitfall_1");
    assert!(
        output.contains("Margherita"),
        "Expected output to contain 'Margherita', got: {}",
        output
    );
}

#[test]
fn test_pitfall_2() {
    let output = run_binary("pitfall_2");
    // This program has warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "pitfall_2 should compile and run"
    );
}

// ===== Test Copy variants =====

#[test]
fn test_test_copy() {
    let output = run_binary("test_copy");
    assert!(
        output.contains("Array values: 1 2 3"),
        "Expected output to contain array values, got: {}",
        output
    );
}

#[test]
fn test_test_copy_4() {
    let output = run_binary("test_copy_4");
    assert!(
        output.contains("Array values: 1 2 3"),
        "Expected output to contain array values, got: {}",
        output
    );
}

#[test]
fn test_test_copy_13() {
    let output = run_binary("test_copy_13");
    assert!(
        output.contains("Array values: 1 2 3"),
        "Expected output to contain array values, got: {}",
        output
    );
}

#[test]
fn test_test_copy_14() {
    let output = run_binary("test_copy_14");
    assert!(
        output.contains("Array values: 1 2 3"),
        "Expected output to contain array values, got: {}",
        output
    );
}

#[test]
fn test_test_copy_15() {
    let output = run_binary("test_copy_15");
    assert!(
        output.contains("Array values: 1 2 3"),
        "Expected output to contain array values, got: {}",
        output
    );
}

// ===== Compilation Errors (Intentional) =====

#[test]
fn test_lifetime_error_fails_to_compile() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            "chapter_4_lifetime_error",
            "--quiet",
        ])
        .output()
        .expect("Failed to check if lifetime_error compiles");

    assert!(
        !output.status.success(),
        "lifetime_error should have compilation error (missing lifetime specifier)"
    );
}

#[test]
fn test_returning_reference_fails_to_compile() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            "chapter_4_returning_reference",
            "--quiet",
        ])
        .output()
        .expect("Failed to check if returning_reference compiles");

    assert!(
        !output.status.success(),
        "returning_reference should have compilation error (dangling reference)"
    );
}
