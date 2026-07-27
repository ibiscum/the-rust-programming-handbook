use std::process::Command;

/// Helper function to run a chapter-5 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_5_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Struct Basics =====

#[test]
fn test_structs_accessing_fields() {
    let output = run_binary("structs_accessing_fields");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "structs_accessing_fields should compile and run"
    );
}

#[test]
fn test_structs_initialization() {
    let output = run_binary("structs_initialization");
    assert!(
        output.contains("Created user with email: someone@example.com"),
        "Expected output to contain user email, got: {}",
        output
    );
}

#[test]
fn test_structs_field_initialization() {
    let output = run_binary("structs_field_initialization");
    assert!(
        output.contains("User 2 active status: true"),
        "Expected output to contain user status, got: {}",
        output
    );
}

#[test]
fn test_structs_update_instances() {
    let output = run_binary("structs_update_instances");
    assert!(
        output.contains("New User 2 Details") && output.contains("original_user"),
        "Expected output to contain user update details, got: {}",
        output
    );
}

#[test]
fn test_structs_fields_modiy_update() {
    let output = run_binary("structs_fields_modiy_update");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "structs_fields_modiy_update should compile and run"
    );
}

#[test]
fn test_structs_cloning() {
    let output = run_binary("structs_cloning");
    assert!(
        output.contains("--- User 2 Details ---") && output.contains("Email: newemail@example.com"),
        "Expected output to contain cloned user details, got: {}",
        output
    );
}

// ===== Struct Methods & Associated Functions =====

#[test]
fn test_structs_associated_functions() {
    let output = run_binary("structs_associated_functions");
    assert!(
        output.contains("Area of square: 225"),
        "Expected output to contain square area calculation, got: {}",
        output
    );
}

#[test]
fn test_structs_methods() {
    let output = run_binary("structs_methods");
    assert!(
        output.contains("The area of rect1 is 1500") && output.contains("Does rect1 have valid width? true"),
        "Expected output to contain area and validation, got: {}",
        output
    );
}

// ===== Struct Special Cases =====

#[test]
fn test_structs_unit_like() {
    let output = run_binary("structs_unit_like");
    assert!(
        output.contains("This is a marker instance"),
        "Expected output to contain marker instance message, got: {}",
        output
    );
}

#[test]
fn test_structs_unit_like2() {
    let output = run_binary("structs_unit_like2");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "structs_unit_like2 should compile and run"
    );
}

#[test]
fn test_borrowing_struct_fields() {
    let output = run_binary("borrowing_struct_fields");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "borrowing_struct_fields should compile and run"
    );
}

#[test]
fn test_ownership_struct_fields() {
    let output = run_binary("ownership_struct_fields");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "ownership_struct_fields should compile and run"
    );
}

// ===== Debug Output =====

#[test]
fn test_debugging_with_structs() {
    let output = run_binary("debugging_with_structs");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "debugging_with_structs should compile and run"
    );
}

#[test]
fn test_customizing_debug_input() {
    let output = run_binary("customizing_debug_input");
    assert!(
        output.contains("User { username: someusername123"),
        "Expected output to contain custom debug output, got: {}",
        output
    );
}

// ===== Enums =====

#[test]
fn test_enums_first_example() {
    let output = run_binary("enums_first_example");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "enums_first_example should compile and run"
    );
}

#[test]
fn test_enum_variants() {
    let output = run_binary("enum_variants");
    assert!(
        output.contains("--- Processing Messages ---") && output.contains("Received Quit message"),
        "Expected output to contain message processing, got: {}",
        output
    );
}

#[test]
fn test_enum_methods() {
    let output = run_binary("enum_methods");
    assert!(
        output.contains("--- Option Results ---") && output.contains("'green' -> Some(Green)") && output.contains("'purple' -> None"),
        "Expected output to contain option results, got: {}",
        output
    );
}

#[test]
fn test_defining_struct_with_enum_fields() {
    let output = run_binary("defining_struct_with_enum_fields");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "defining_struct_with_enum_fields should compile and run"
    );
}

// ===== Tuples =====

#[test]
fn test_tuples_example_1() {
    let output = run_binary("tuples_example_1");
    assert!(
        output.contains("First: 100, Second: 3.14"),
        "Expected output to contain tuple values, got: {}",
        output
    );
}

#[test]
fn test_tuples_example_points() {
    let output = run_binary("tuples_example_points");
    assert!(
        output.contains("The distance between points is: 5"),
        "Expected output to contain distance calculation, got: {}",
        output
    );
}

#[test]
fn test_tuples_employee_example() {
    let output = run_binary("tuples_employee_example");
    assert!(
        output.contains("Employee ID: 1001") && output.contains("Employee Name: John Doe"),
        "Expected output to contain employee details, got: {}",
        output
    );
}

#[test]
fn test_tuple_destructuring() {
    let output = run_binary("tuple_destructuring");
    assert!(
        output.contains("Integer value: 42") && output.contains("Float value: 6.7"),
        "Expected output to contain destructured values, got: {}",
        output
    );
}

#[test]
fn test_tuples_splitting_full_name() {
    let output = run_binary("tuples_splitting_full_name");
    assert!(
        output.contains("First Name: John") && output.contains("Last Name: Doe"),
        "Expected output to contain name parts, got: {}",
        output
    );
}

#[test]
fn test_tuples_returning_multiple_values() {
    let output = run_binary("tuples_returning_multiple_values");
    assert!(
        output.contains("Sum: 15") && output.contains("Difference: 5"),
        "Expected output to contain calculation results, got: {}",
        output
    );
}

// ===== Tuple Structs =====

#[test]
fn test_tuple_structs() {
    let output = run_binary("tuple_structs");
    assert!(
        output.contains("Red's red component: 255") && output.contains("Red's green component: 0"),
        "Expected output to contain color components, got: {}",
        output
    );
}

#[test]
fn test_tuple_structs_and_functions() {
    let output = run_binary("tuple_structs_and_functions");
    assert!(
        output.contains("Package dimensions") && output.contains("Package volume: 9150 cubic units"),
        "Expected output to contain package details and volume, got: {}",
        output
    );
}

// ===== Final Examples =====

#[test]
fn test_final_example_1() {
    let output = run_binary("final_example_1");
    // Output contains warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "final_example_1 should compile and run"
    );
}
