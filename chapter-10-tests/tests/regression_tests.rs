use std::process::Command;

/// Helper function to run tests for a chapter-10 package and capture output
fn run_package_tests(package_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&["test", "--package", package_name, "--quiet"])
        .output()
        .expect(&format!("Failed to run tests for {}", package_name));

    String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr).to_string()
}

/// Helper to check if tests pass (exit code 0)
fn tests_pass(package_name: &str) -> bool {
    let output = Command::new("cargo")
        .args(&["test", "--package", package_name])
        .output()
        .expect(&format!("Failed to run tests for {}", package_name));

    output.status.success()
}

// ===== Basic Testing =====

#[test]
fn test_01_basic_unit_tests() {
    assert!(
        tests_pass("chapter_10_01_basic_unit_tests"),
        "chapter_10_01_basic_unit_tests should pass"
    );
}

#[test]
fn test_02_assertion_macros() {
    assert!(
        tests_pass("chapter_10_02_assertion_macros"),
        "chapter_10_02_assertion_macros should pass"
    );
}

#[test]
fn test_03_testing_results_and_options() {
    assert!(
        tests_pass("chapter_10_03_testing_results_and_options"),
        "chapter_10_03_testing_results_and_options should pass"
    );
}

#[test]
fn test_04_testing_panics() {
    assert!(
        tests_pass("chapter_10_04_testing_panics"),
        "chapter_10_04_testing_panics should pass"
    );
}

#[test]
fn test_05_ignoring_tests() {
    assert!(
        tests_pass("chapter_10_05_ignoring_tests"),
        "chapter_10_05_ignoring_tests should pass"
    );
}

#[test]
fn test_06_library_structure() {
    assert!(
        tests_pass("chapter_10_06_library_structure"),
        "chapter_10_06_library_structure should pass"
    );
}

// ===== Integration Testing =====

#[test]
fn test_07_integration_tests() {
    assert!(
        tests_pass("chapter_10_07_integration_tests"),
        "chapter_10_07_integration_tests should pass"
    );
}

#[test]
fn test_08_test_helpers() {
    assert!(
        tests_pass("chapter_10_08_test_helpers"),
        "chapter_10_08_test_helpers should pass"
    );
}

// ===== Documentation Testing =====

#[test]
fn test_09_documentation_tests() {
    assert!(
        tests_pass("chapter_10_09_documentation_tests"),
        "chapter_10_09_documentation_tests should pass"
    );
}

#[test]
fn test_10_controlling_doc_tests() {
    assert!(
        tests_pass("chapter_10_10_controlling_doc_tests"),
        "chapter_10_10_controlling_doc_tests should pass"
    );
}

// ===== Test Driven Development =====

#[test]
fn test_11_test_driven_development_has_expected_failures() {
    // This project intentionally demonstrates the "Red Phase" of TDD
    // where tests fail before implementation is complete
    let output = run_package_tests("chapter_10_11_test_driven_development");
    assert!(
        output.contains("FAILED"),
        "chapter_10_11_test_driven_development should demonstrate test failures (Red Phase)"
    );
}

#[test]
fn test_12_tdd_green_phase() {
    assert!(
        tests_pass("chapter_10_12_tdd_green_phase"),
        "chapter_10_12_tdd_green_phase should pass (Green Phase)"
    );
}

#[test]
fn test_13_tdd_refactor_phase() {
    assert!(
        tests_pass("chapter_10_13_tdd_refactor_phase"),
        "chapter_10_13_tdd_refactor_phase should pass (Refactor Phase)"
    );
}

#[test]
fn test_14_tdd_api_handler() {
    assert!(
        tests_pass("chapter_10_14_tdd_api_handler"),
        "chapter_10_14_tdd_api_handler should pass"
    );
}

// ===== Test Doubles and Stubs =====

#[test]
fn test_15_test_doubles_and_stubs() {
    assert!(
        tests_pass("chapter_10_15_test_doubles_and_stubs"),
        "chapter_10_15_test_doubles_and_stubs should pass"
    );
}
