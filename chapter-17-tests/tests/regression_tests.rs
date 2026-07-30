use std::process::Command;

/// Helper to check if a binary compiles
fn binary_compiles(package_name: &str) -> bool {
    let output = Command::new("cargo")
        .args(&["check", "--package", package_name])
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

// --- Ownership and Borrowing Tests (9 tests) ---
#[test]
fn test_01_double_ownership() {
    assert!(binary_compiles("chapter_17_17_01_double_ownership"));
}

#[test]
fn test_02_dangling_references() {
    assert!(binary_compiles("chapter_17_17_02_dangling_references"));
}

#[test]
fn test_03_borrowing_confusion() {
    assert!(binary_compiles("chapter_17_17_03_borrowing_confusion"));
}

#[test]
fn test_04_dangling_references_scope() {
    assert!(binary_compiles("chapter_17_17_04_dangling_references_scope"));
}

#[test]
fn test_05_lifetime_specifiers() {
    assert!(binary_compiles("chapter_17_17_05_lifetime_specifiers"));
}

#[test]
fn test_06_mutable_vs_immutable_lifetimes() {
    assert!(binary_compiles("chapter_17_17_06_mutable_vs_immutable_lifetimes"));
}

#[test]
fn test_07_simultaneous_references() {
    assert!(binary_compiles("chapter_17_17_07_simultaneous_references"));
}

#[test]
fn test_08_multiple_mutable_references() {
    assert!(binary_compiles("chapter_17_17_08_multiple_mutable_references"));
}

#[test]
fn test_09_scope_confusion() {
    assert!(binary_compiles("chapter_17_17_09_scope_confusion"));
}

// --- Error Handling Tests (8 tests) ---
#[test]
fn test_10_overusing_panic() {
    assert!(binary_compiles("chapter_17_17_10_overusing_panic"));
}

#[test]
fn test_11_ignoring_result() {
    assert!(binary_compiles("chapter_17_17_11_ignoring_result"));
}

#[test]
fn test_12_unwrapping_without_care() {
    assert!(binary_compiles("chapter_17_17_12_unwrapping_without_care"));
}

#[test]
fn test_13_overusing_unwrap() {
    assert!(binary_compiles("chapter_17_17_13_overusing_unwrap"));
}

#[test]
fn test_14_uninformative_expect() {
    assert!(binary_compiles("chapter_17_17_14_uninformative_expect"));
}

#[test]
fn test_15_not_using_combinators() {
    assert!(binary_compiles("chapter_17_17_15_not_using_combinators"));
}

#[test]
fn test_16_using_map() {
    assert!(binary_compiles("chapter_17_17_16_using_map"));
}

#[test]
fn test_17_using_unwrap_or_else() {
    assert!(binary_compiles("chapter_17_17_17_using_unwrap_or_else"));
}

// --- Concurrency Tests (9 tests) ---
#[test]
fn test_18_data_races() {
    assert!(binary_compiles("chapter_17_17_18_data_races"));
}

#[test]
fn test_19_deadlocks() {
    assert!(binary_compiles("chapter_17_17_19_deadlocks"));
}

#[test]
fn test_20_atomic_memory_ordering() {
    assert!(binary_compiles("chapter_17_17_20_atomic_memory_ordering"));
}

#[test]
fn test_21_inefficient_scheduling() {
    assert!(binary_compiles("chapter_17_17_21_inefficient_scheduling"));
}

#[test]
fn test_22_inefficient_memory() {
    assert!(binary_compiles("chapter_17_17_22_inefficient_memory"));
}

#[test]
fn test_23_excessive_dynamic_dispatch() {
    assert!(binary_compiles("chapter_17_17_23_excessive_dynamic_dispatch"));
}

#[test]
fn test_24_unoptimized_data_structures() {
    assert!(binary_compiles("chapter_17_17_24_unoptimized_data_structures"));
}

#[test]
fn test_25_excessive_cloning() {
    assert!(binary_compiles("chapter_17_17_25_excessive_cloning"));
}

#[test]
fn test_26_suboptimal_iteration() {
    assert!(binary_compiles("chapter_17_17_26_suboptimal_iteration"));
}

// --- Debugging and Diagnostics Tests (9 tests) ---
#[test]
fn test_27_mismatched_types() {
    assert!(binary_compiles("chapter_17_17_27_mismatched_types"));
}

#[test]
fn test_28_mismatched_return_types() {
    assert!(binary_compiles("chapter_17_17_28_mismatched_return_types"));
}

#[test]
fn test_29_unresolved_imports() {
    assert!(binary_compiles("chapter_17_17_29_unresolved_imports"));
}

#[test]
fn test_30_unused_variables() {
    assert!(binary_compiles("chapter_17_17_30_unused_variables"));
}

#[test]
fn test_31_unclear_error_messages() {
    assert!(binary_compiles("chapter_17_17_31_unclear_error_messages"));
}

#[test]
fn test_32_complex_lifetime_errors() {
    assert!(binary_compiles("chapter_17_17_32_complex_lifetime_errors"));
}

#[test]
fn test_33_ineffective_logging() {
    assert!(binary_compiles("chapter_17_17_33_ineffective_logging"));
}

#[test]
fn test_34_test_coverage_gaps() {
    assert!(binary_compiles("chapter_17_17_34_test_coverage_gaps"));
}

#[test]
fn test_36_debugging_concurrency() {
    assert!(binary_compiles("chapter_17_17_36_debugging_concurrency"));
}

// --- Best Practices Tests (5 tests) ---
#[test]
fn test_37_ownership_model() {
    assert!(binary_compiles("chapter_17_17_37_ownership_model"));
}

#[test]
fn test_38_type_system() {
    assert!(binary_compiles("chapter_17_17_38_type_system"));
}

#[test]
fn test_39_idiomatic_rust() {
    assert!(binary_compiles("chapter_17_17_39_idiomatic_rust"));
}

#[test]
fn test_40_optimize_performance() {
    assert!(binary_compiles("chapter_17_17_40_optimize_performance"));
}

#[test]
fn test_41_leverage_ecosystem() {
    assert!(binary_compiles("chapter_17_17_41_leverage_ecosystem"));
}
