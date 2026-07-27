use std::process::Command;

/// Helper function to run a chapter-8 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_8_{}", binary_name),
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
            &format!("chapter_8_{}", binary_name),
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    !output.status.success()
}

// ===== Trait Basics and Advanced Usage =====

#[test]
fn test_01_averaged_collection() {
    let output = run_binary("01_averaged_collection");
    assert!(
        output.contains("Average after adds: 20") && output.contains("Average after remove: 15"),
        "Expected output to contain averaged collection results, got: {}",
        output
    );
}

#[test]
fn test_02_trait_summarizable() {
    let output = run_binary("02_trait_summarizable");
    assert!(
        output.contains("Tweet summary") && output.contains("Article summary"),
        "Expected output to contain trait summaries, got: {}",
        output
    );
}

#[test]
fn test_03_impl_trait_notify_fails_to_compile() {
    assert!(
        binary_fails_to_compile("03_impl_trait_notify"),
        "Expected chapter_8_03_impl_trait_notify to fail compilation (intentional error example)"
    );
}

#[test]
fn test_04_trait_bounds_generics() {
    let output = run_binary("04_trait_bounds_generics");
    assert!(
        output.contains("Breaking news (generic)") && output.contains("Summary:"),
        "Expected output to contain trait bounds and generics examples, got: {}",
        output
    );
}

#[test]
fn test_05_monomorphization() {
    let output = run_binary("05_monomorphization");
    assert!(
        output.contains("Generic Function") && output.contains("compiler created"),
        "Expected output to contain monomorphization explanation, got: {}",
        output
    );
}

#[test]
fn test_06_trait_objects() {
    let output = run_binary("06_trait_objects");
    assert!(
        output.contains("Daily Feed") && output.contains("Summary:"),
        "Expected output to contain trait object examples, got: {}",
        output
    );
}

#[test]
fn test_07_object_safety() {
    let output = run_binary("07_object_safety");
    assert!(
        output.contains("Cloned tweet") || output.contains("Object safety"),
        "Expected output to contain object safety demonstration, got: {}",
        output
    );
}

#[test]
fn test_08_default_methods() {
    let output = run_binary("08_default_methods");
    assert!(
        output.contains("Button") && output.contains("clicked"),
        "Expected output to contain default method examples, got: {}",
        output
    );
}

#[test]
fn test_09_supertraits() {
    let output = run_binary("09_supertraits");
    assert!(
        output.contains("Summary") && output.contains("Report:"),
        "Expected output to contain supertrait examples, got: {}",
        output
    );
}

#[test]
fn test_10_multiple_trait_bounds() {
    let output = run_binary("10_multiple_trait_bounds");
    assert!(
        output.contains("Processing item") && output.contains("clone"),
        "Expected output to contain multiple trait bounds examples, got: {}",
        output
    );
}

#[test]
fn test_11_associated_types() {
    let output = run_binary("11_associated_types");
    assert!(
        output.contains("Some(0)") || output.contains("associated"),
        "Expected output to contain associated types examples, got: {}",
        output
    );
}

#[test]
fn test_12_builder_pattern() {
    let output = run_binary("12_builder_pattern");
    assert!(
        output.contains("Basic Window") && output.contains("Custom Window"),
        "Expected output to contain builder pattern examples, got: {}",
        output
    );
}

#[test]
fn test_13_state_pattern_enum() {
    let output = run_binary("13_state_pattern_enum");
    assert!(
        output.contains("Draft") && output.contains("Published"),
        "Expected output to contain state pattern examples, got: {}",
        output
    );
}

#[test]
fn test_14_observer_pattern() {
    let output = run_binary("14_observer_pattern");
    assert!(
        output.contains("State changed") || output.contains("Logger"),
        "Expected output to contain observer pattern examples, got: {}",
        output
    );
}
