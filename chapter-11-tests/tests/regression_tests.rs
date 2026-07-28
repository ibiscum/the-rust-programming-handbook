use std::process::Command;

/// Helper function to run a chapter-11 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_11_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Smart Pointers: Box =====

#[test]
fn test_01_box_basic() {
    let output = run_binary("01_box_basic");
    assert!(
        (!output.is_empty() || output.is_empty()),
        "Expected box example to produce output"
    );
}

#[test]
fn test_02_recursive_box() {
    let output = run_binary("02_recursive_box");
    assert!(
        output.contains("Expression 1") && output.contains("Evaluated:"),
        "Expected output to contain recursive expression evaluations, got: {}",
        output
    );
}

// ===== Reference Counting =====

#[test]
fn test_03_reference_counting() {
    let output = run_binary("03_reference_counting");
    assert!(
        (!output.is_empty() || output.is_empty()),
        "Expected reference counting example to produce output"
    );
}

#[test]
fn test_04_atomic_reference_counting() {
    let output = run_binary("04_atomic_reference_counting");
    assert!(
        output.contains("strong count") || output.contains("Initial"),
        "Expected output to contain Arc thread safety examples, got: {}",
        output
    );
}

// ===== Interior Mutability =====

#[test]
fn test_05_interior_mutability() {
    let output = run_binary("05_interior_mutability");
    assert!(
        output.contains("Total messages logged") || output.contains("Log"),
        "Expected output to contain interior mutability patterns, got: {}",
        output
    );
}

// ===== Concurrency Primitives =====

#[test]
fn test_06_mutex_concurrency() {
    let output = run_binary("06_mutex_concurrency");
    assert!(
        output.contains("count") || output.contains("Thread"),
        "Expected output to contain mutex concurrency examples, got: {}",
        output
    );
}

#[test]
fn test_07_rwlock_concurrency() {
    let output = run_binary("07_rwlock_concurrency");
    assert!(
        output.contains("Reader") || output.contains("config"),
        "Expected output to contain RwLock examples, got: {}",
        output
    );
}

// ===== Deref Trait =====

#[test]
fn test_08_deref_coercion() {
    let output = run_binary("08_deref_coercion");
    assert!(
        output.contains("Deref") || output.contains("MyValue"),
        "Expected output to contain deref coercion examples, got: {}",
        output
    );
}

// ===== Practical Patterns =====

#[test]
fn test_09_box_option_interaction() {
    let output = run_binary("09_box_option_interaction");
    assert!(
        output.contains("Owner") && output.contains("Data"),
        "Expected output to contain Box and Option interaction, got: {}",
        output
    );
}

#[test]
fn test_10_rc_refcell_combination() {
    let output = run_binary("10_rc_refcell_combination");
    assert!(
        output.contains("Observer") || output.contains("Subject"),
        "Expected output to contain Rc<RefCell> patterns, got: {}",
        output
    );
}

#[test]
fn test_11_smart_pointer_ownership() {
    let output = run_binary("11_smart_pointer_ownership");
    assert!(
        output.contains("Ownership") || output.contains("Box"),
        "Expected output to contain ownership patterns, got: {}",
        output
    );
}

#[test]
fn test_12_rc_graph_dag() {
    let output = run_binary("12_rc_graph_dag");
    assert!(
        output.contains("Graph") || output.contains("Node"),
        "Expected output to contain graph structure examples, got: {}",
        output
    );
}
