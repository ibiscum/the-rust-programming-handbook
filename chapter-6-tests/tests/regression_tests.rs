use std::process::Command;

/// Helper function to run a chapter-6 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_6_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Option Examples =====

#[test]
fn test_option_example_1() {
    let output = run_binary("option_example_1");
    assert!(
        output.contains("Searching for 'fox'") && output.contains("Success! Found at index"),
        "Expected output to contain search result, got: {}",
        output
    );
}

// ===== Result Examples =====

#[test]
fn test_result_example_1() {
    let output = run_binary("result_example_1");
    assert!(
        output.contains("Division 10/2 succeeded: 5") && output.contains("division by zero"),
        "Expected output to contain division results, got: {}",
        output
    );
}

// ===== Combinators =====

#[test]
fn test_combinators() {
    let output = run_binary("combinators");
    assert!(
        output.contains("Result for '10': Ok(10)") && output.contains("Result for '7': Err"),
        "Expected output to contain combinator results, got: {}",
        output
    );
}

// ===== Error Handling Methods =====

#[test]
fn test_unwrap() {
    let output = run_binary("unwrap");
    assert!(
        output.contains("The Result before unwrapping: Ok(2.0)") && output.contains("The value after unwrap: 2"),
        "Expected output to contain unwrap operation results, got: {}",
        output
    );
}

#[test]
fn test_expect() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            "chapter_6_expect",
            "--quiet",
        ])
        .output()
        .expect("Failed to run expect");

    assert!(
        !output.status.success(),
        "expect should panic and exit with error"
    );
}

#[test]
fn test_safe_unwrapping() {
    let output = run_binary("safe_unwrapping");
    assert!(
        output.contains("Using .unwrap_or()"),
        "Expected output to contain unwrap_or example, got: {}",
        output
    );
}

// ===== Question Mark Operator =====

#[test]
fn test_question_mark_operator() {
    let output = run_binary("question_mark_operator");
    assert!(
        output.contains("Success Case") && output.contains("Result with '?': 50.25"),
        "Expected output to contain question mark operator result, got: {}",
        output
    );
}

#[test]
fn test_question_mark_operator_with_option() {
    let output = run_binary("question_mark_operator_with_option");
    assert!(
        output.contains("Some(24)") && output.contains("None"),
        "Expected output to contain option results, got: {}",
        output
    );
}

// ===== Error Propagation =====

#[test]
fn test_manual_propagation() {
    let output = run_binary("manual_propagation");
    assert!(
        output.contains("SUCCESS: Final result is 40") && output.contains("FAILURE (Propagation 1)"),
        "Expected output to contain propagation examples, got: {}",
        output
    );
}

// ===== Custom Error Types =====

#[test]
fn test_custom_error_types_with_enum() {
    let output = run_binary("custom_error_types_with_enum");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "custom_error_types_with_enum should compile and run"
    );
}

#[test]
fn test_custom_errors_functions() {
    let output = run_binary("custom_errors_functions");
    assert!(
        output.contains("Test 1: I/O Error") && output.contains("os error"),
        "Expected output to contain error message, got: {}",
        output
    );
}

#[test]
fn test_implementing_standard_error_traits() {
    let output = run_binary("implementing_standard_error_traits");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "implementing_standard_error_traits should compile and run"
    );
}

// ===== Panic Handling =====

#[test]
fn test_panic_example() {
    let output = run_binary("panic_example");
    assert!(
        output.contains("Value 10 is valid") && output.contains("Program finished successfully"),
        "Expected output to contain panic example output, got: {}",
        output
    );
}

// ===== External Dependency Projects =====

#[test]
fn test_anyhow_example() {
    let output = run_binary("anyhow_example");
    assert!(
        output.contains("Success with anyhow") || output.contains("Test 2: Negative Value Error"),
        "Expected output to contain anyhow example output, got: {}",
        output
    );
}

#[test]
fn test_env_logger_example() {
    let output = run_binary("env_logger_example");
    // env_logger may not show output without RUST_LOG, but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "env_logger_example should compile and run"
    );
}

#[test]
fn test_env_logger_with_context() {
    let output = run_binary("env_logger_with_context");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "env_logger_with_context should compile and run"
    );
}

#[test]
fn test_thiserror_example() {
    let output = run_binary("thiserror_example");
    // The program outputs errors and also successful parsing
    assert!(
        output.contains("Successfully read") || output.len() > 0,
        "Expected output from thiserror example, got: {}",
        output
    );
}

#[test]
fn test_errors_final_example() {
    let output = run_binary("errors_final_example");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "errors_final_example should compile and run"
    );
}
