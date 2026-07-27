use std::process::Command;

/// Helper function to run a chapter-7 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_7_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Traits Basics =====

#[test]
fn test_01_defining_traits() {
    let output = run_binary("01_defining_traits");
    assert!(
        output.contains("Bubbling on the stove") && output.contains("6 cups of coffee"),
        "Expected output to contain coffee brewing message, got: {}",
        output
    );
}

#[test]
fn test_02_default_implementations() {
    let output = run_binary("02_default_implementations");
    assert!(
        output.contains("Brewing") && output.contains("hot water rinse"),
        "Expected output to contain brewing and cleaning methods, got: {}",
        output
    );
}

#[test]
fn test_03_implementing_on_custom_types() {
    let output = run_binary("03_implementing_on_custom_types");
    assert!(
        output.contains("Bubbling on the stove") && output.contains("rich coffee"),
        "Expected output to contain custom type implementation, got: {}",
        output
    );
}

#[test]
fn test_04_orphan_rule() {
    let output = run_binary("04_orphan_rule");
    assert!(
        output.contains("Pour-over coffee") && output.contains("Moka pot"),
        "Expected output to contain orphan rule examples, got: {}",
        output
    );
}

#[test]
fn test_05_trait_objects() {
    let output = run_binary("05_trait_objects");
    assert!(
        output.contains("Brewing: Moka pot") && output.contains("Espresso machine"),
        "Expected output to contain trait objects examples, got: {}",
        output
    );
}

// ===== Generics =====

#[test]
fn test_06_duplication_problem() {
    let output = run_binary("06_duplication_problem");
    assert!(
        output.contains("The largest number is 100") && output.contains("largest char is y"),
        "Expected output to contain largest value results, got: {}",
        output
    );
}

#[test]
fn test_07_generics_syntax_error() {
    let output = run_binary("07_generics_syntax_error");
    assert!(
        output.contains("demonstrates a compiler error"),
        "Expected output to demonstrate error concept, got: {}",
        output
    );
}

#[test]
fn test_08_generics_trait_bounds() {
    let output = run_binary("08_generics_trait_bounds");
    assert!(
        output.contains("The largest number is 100") && output.contains("largest char is y"),
        "Expected output to contain largest value results, got: {}",
        output
    );
}

#[test]
fn test_09_where_clauses() {
    let output = run_binary("09_where_clauses");
    assert!(
        output.contains("Messy:") && output.contains("Clean:"),
        "Expected output to contain where clause examples, got: {}",
        output
    );
}

#[test]
fn test_10_returning_impl_trait() {
    let output = run_binary("10_returning_impl_trait");
    assert!(
        output.contains("Current Status") && output.contains("All Green"),
        "Expected output to contain impl trait return example, got: {}",
        output
    );
}

// ===== Lifetimes =====

#[test]
fn test_11_dangling_reference() {
    let output = run_binary("11_dangling_reference");
    assert!(
        output.contains("demonstrates a safety feature"),
        "Expected output to demonstrate safety concept, got: {}",
        output
    );
}

#[test]
fn test_12_borrow_checker_scopes() {
    let output = run_binary("12_borrow_checker_scopes");
    assert!(
        output.contains("demonstrates lifetime scopes"),
        "Expected output to demonstrate borrow checker scopes, got: {}",
        output
    );
}

#[test]
fn test_13_lifetime_syntax() {
    let output = run_binary("13_lifetime_syntax");
    assert!(
        output.contains("x: 10, y: 20, z: 30"),
        "Expected output to contain lifetime syntax example, got: {}",
        output
    );
}

#[test]
fn test_14_annotating_functions() {
    let output = run_binary("14_annotating_functions");
    assert!(
        output.contains("The longest string is long string") && output.contains("The longest inner string is long string"),
        "Expected output to contain annotated function results, got: {}",
        output
    );
}

#[test]
fn test_15_structs_with_references() {
    let output = run_binary("15_structs_with_references");
    assert!(
        output.contains("First sentence: Call me Ishmael"),
        "Expected output to contain struct with references example, got: {}",
        output
    );
}

#[test]
fn test_16_impl_blocks_with_lifetimes() {
    let output = run_binary("16_impl_blocks_with_lifetimes");
    assert!(
        output.contains("Attention to:") && output.contains("Length of segment"),
        "Expected output to contain impl blocks with lifetimes, got: {}",
        output
    );
}

#[test]
fn test_17_lifetime_elision() {
    let output = run_binary("17_lifetime_elision");
    assert!(
        output.contains("Word 1: Rust") && output.contains("Word 2: Rust"),
        "Expected output to contain lifetime elision example, got: {}",
        output
    );
}

#[test]
fn test_18_static_lifetime() {
    let output = run_binary("18_static_lifetime");
    assert!(
        output.contains("static lifetime") && output.contains("Welcome to the Static Void"),
        "Expected output to contain static lifetime examples, got: {}",
        output
    );
}

// ===== Practical Examples =====

#[test]
fn test_20_zero_copy_parser() {
    let output = run_binary("20_zero_copy_parser");
    assert!(
        output.contains("Loading configuration") && output.contains("192.168.1.50"),
        "Expected output to contain parser results, got: {}",
        output
    );
}

#[test]
fn test_21_validating_references_impl() {
    let output = run_binary("21_validating_references_impl");
    assert!(
        output.contains("Server Configuration") && output.contains("127.0.0.1"),
        "Expected output to contain server configuration, got: {}",
        output
    );
}
