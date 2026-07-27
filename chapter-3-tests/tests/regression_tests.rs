use std::process::Command;

/// Helper function to run a chapter-3 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_3_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Basic Functions =====

#[test]
fn test_basic_function_syntax() {
    let output = run_binary("basic_function_syntax");
    assert!(
        output.contains("The area of the rectangle is: 50"),
        "Expected output to contain rectangle area, got: {}",
        output
    );
}

#[test]
fn test_functons_no_parameters_no_return_value() {
    let output = run_binary("functons_no_parameters_no_return_value");
    assert_eq!(output.trim(), "Hello, world!");
}

#[test]
fn test_functions_no_return_value() {
    let output = run_binary("functions_no_return_value");
    assert!(
        output.contains("[LOG] System online") && output.contains("[LOG] Task finished"),
        "Expected output to contain log messages, got: {}",
        output
    );
}

#[test]
fn test_parameters_and_return_values() {
    let output = run_binary("parameters_and_return_values");
    assert!(
        output.contains("Hello, Alice!") && output.contains("30 years old"),
        "Expected output to contain greeting and age, got: {}",
        output
    );
}

#[test]
fn test_returning_values_and_ownership() {
    let output = run_binary("returning_values_and_ownership");
    assert!(
        output.contains("s1: hello") && output.contains("s3: hello"),
        "Expected output to contain string ownership results, got: {}",
        output
    );
}

#[test]
fn test_function_documented() {
    let output = run_binary("function_documented");
    assert!(
        output.contains("The calculated area is: 200"),
        "Expected output to contain calculated area, got: {}",
        output
    );
}

// ===== Borrowing & Ownership =====

#[test]
fn test_functions_immutable_borrowing() {
    let output = run_binary("functions_immutable_borrowing");
    assert!(
        output.contains("hello"),
        "Expected output to contain 'hello', got: {}",
        output
    );
}

#[test]
fn test_functions_mutable_borrowing() {
    let output = run_binary("functions_mutable_borrowing");
    assert!(
        output.contains("hello, world"),
        "Expected output to contain 'hello, world', got: {}",
        output
    );
}

#[test]
fn test_functions_ownership() {
    let output = run_binary("functions_ownership");
    assert!(
        output.contains("hello"),
        "Expected output to contain 'hello', got: {}",
        output
    );
}

// ===== Anonymous Functions & Closures =====

#[test]
fn test_anonymous_functions() {
    let output = run_binary("anonymous_functions");
    assert!(
        output.contains("Doubled numbers:") && output.contains("Sum of numbers: 15"),
        "Expected output to contain doubled numbers and sum, got: {}",
        output
    );
}

#[test]
fn test_closures_1() {
    let output = run_binary("closures_1");
    assert!(
        output.contains("5 + 1 = 6") && output.contains("3 * 4 = 12"),
        "Expected output to contain closure calculations, got: {}",
        output
    );
}

#[test]
fn test_closures_2() {
    let output = run_binary("closures_2");
    assert!(
        output.contains("Hello, Alice!") && output.contains("can still be used") && output.contains("Counter is now: 1"),
        "Expected output to contain closure capture examples, got: {}",
        output
    );
}

#[test]
fn test_closures_3() {
    let output = run_binary("closures_3");
    assert!(
        output.contains("Thread received: Data for the new thread"),
        "Expected output to contain thread message, got: {}",
        output
    );
}

// ===== Higher-Order Functions =====

#[test]
fn test_high_order_functions() {
    let output = run_binary("high_order_functions");
    assert!(
        output.contains("The sum of the squares of even numbers greater than 3 is: 216"),
        "Expected output to contain sum of squares calculation, got: {}",
        output
    );
}
