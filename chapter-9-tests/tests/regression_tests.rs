use std::process::Command;

/// Helper function to run a chapter-9 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_9_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Iterator Basics =====

#[test]
fn test_01_manual_iteration() {
    let output = run_binary("01_manual_iteration");
    assert!(
        output.contains("First call") && output.contains("Some") && output.contains("None"),
        "Expected output to contain iterator progression, got: {}",
        output
    );
}

#[test]
fn test_02_three_types_of_iterators() {
    let output = run_binary("02_three_types_of_iterators");
    assert!(
        output.contains("Hello") && output.contains("still available"),
        "Expected output to contain iterator type examples, got: {}",
        output
    );
}

#[test]
fn test_03_iterator_consumers() {
    let output = run_binary("03_iterator_consumers");
    assert!(
        output.contains("greater than 2") && output.contains("sum"),
        "Expected output to contain consumer operations, got: {}",
        output
    );
}

#[test]
fn test_04_map_iterator_adapter() {
    let output = run_binary("04_map_iterator_adapter");
    assert!(
        output.contains("Squares") && output.contains("Upper names"),
        "Expected output to contain map transformation results, got: {}",
        output
    );
}

#[test]
fn test_05_filter_and_chaining() {
    let output = run_binary("05_filter_and_chaining");
    assert!(
        output.contains("Evens:") && output.contains("Adjusted high scores"),
        "Expected output to contain filter and chaining results, got: {}",
        output
    );
}

#[test]
fn test_06_fold_and_find_consumers() {
    let output = run_binary("06_fold_and_find_consumers");
    assert!(
        output.contains("Product:") && output.contains("First even"),
        "Expected output to contain fold and find operations, got: {}",
        output
    );
}

// ===== Closures =====

#[test]
fn test_07_basic_closures() {
    let output = run_binary("07_basic_closures");
    assert!(
        output.contains("6") && output.contains("12"),
        "Expected output to contain closure results, got: {}",
        output
    );
}

#[test]
fn test_08_closure_capture_modes() {
    let output = run_binary("08_closure_capture_modes");
    assert!(
        output.contains("times factor") && output.contains("Processing item"),
        "Expected output to contain closure capture examples, got: {}",
        output
    );
}

#[test]
fn test_09_closure_trait_bounds() {
    let output = run_binary("09_closure_trait_bounds");
    assert!(
        output.contains("Report") && output.contains("Counter"),
        "Expected output to contain closure trait bounds, got: {}",
        output
    );
}

// ===== Pattern Matching and Destructuring =====

#[test]
fn test_10_match_destructuring() {
    let output = run_binary("10_match_destructuring");
    assert!(
        output.contains("Message Processing") && output.contains("User Matching"),
        "Expected output to contain destructuring examples, got: {}",
        output
    );
}

#[test]
fn test_11_if_let_and_while_let() {
    let output = run_binary("11_if_let_and_while_let");
    assert!(
        output.contains("if let") && output.contains("Processing value"),
        "Expected output to contain if let and while let examples, got: {}",
        output
    );
}
