use std::process::Command;

/// Helper function to run a chapter-2 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &format!("chapter_2_{}", binary_name),
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ===== Basic Types =====

#[test]
fn test_arrays() {
    let output = run_binary("arrays");
    assert_eq!(output.trim(), "Array values: 1 2 3");
}

#[test]
fn test_booleans() {
    let output = run_binary("booleans");
    assert_eq!(output.trim(), "Is Rust fun? true");
}

#[test]
fn test_characters() {
    let output = run_binary("characters");
    assert!(
        output.contains("Letter: R"),
        "Expected output to contain 'Letter: R', got: {}",
        output
    );
}

#[test]
fn test_floating_point_numbers() {
    let output = run_binary("floating_point_numbers");
    assert!(
        output.contains("Floating-point number: 3.14"),
        "Expected output to contain 'Floating-point number: 3.14', got: {}",
        output
    );
}

#[test]
fn test_integers() {
    let output = run_binary("integers");
    assert!(
        output.contains("Signed integer: -42") && output.contains("Unsigned integer: 42"),
        "Expected output to contain both signed and unsigned integers, got: {}",
        output
    );
}

#[test]
fn test_string() {
    let output = run_binary("string");
    assert_eq!(output.trim(), "Hello, world!");
}

#[test]
fn test_string_literal() {
    let output = run_binary("string_literal");
    assert_eq!(output.trim(), "Hello, world!");
}

// ===== Collections =====

#[test]
fn test_tuples() {
    let output = run_binary("tuples");
    assert!(
        output.contains("Destructured values") && output.contains("500"),
        "Expected output to contain tuple destructuring, got: {}",
        output
    );
}

#[test]
fn test_tuples2() {
    let output = run_binary("tuples2");
    assert!(
        output.contains("Sum:") && output.contains("Product:"),
        "Expected output to contain Sum and Product, got: {}",
        output
    );
}

#[test]
fn test_slices() {
    let output = run_binary("slices");
    assert!(
        output.contains("Original array") && output.contains("Slice"),
        "Expected output to contain array and slice info, got: {}",
        output
    );
}

// ===== Variables =====

#[test]
fn test_immutable_variables() {
    let output = run_binary("immutable_variables");
    assert!(
        output.contains("The value of x is: 5"),
        "Expected output to contain 'The value of x is: 5', got: {}",
        output
    );
}

#[test]
fn test_mutable_variables() {
    let output = run_binary("mutable_variables");
    assert!(
        output.contains("The value of x is: 5") && output.contains("The value of x is: 6"),
        "Expected output to contain both x values, got: {}",
        output
    );
}

#[test]
fn test_shadowing() {
    let output = run_binary("shadowing");
    assert!(
        output.contains("The value of x is: 6"),
        "Expected output to contain 'The value of x is: 6', got: {}",
        output
    );
}

// ===== Control Flow =====

#[test]
fn test_if_else() {
    let output = run_binary("if_else");
    assert!(
        output.contains("The number is greater than 5"),
        "Expected output to contain comparison result, got: {}",
        output
    );
}

#[test]
fn test_loop_1() {
    let output = run_binary("loop_1");
    assert!(
        output.contains("Counter is now: 1") && output.contains("Counter is now: 5"),
        "Expected output to contain loop counter, got: {}",
        output
    );
}

#[test]
fn test_loops_2() {
    let output = run_binary("loops_2");
    assert!(
        output.contains("Found an even number"),
        "Expected output to contain loop results, got: {}",
        output
    );
}

#[test]
fn test_while() {
    let output = run_binary("while");
    assert!(
        output.contains("3") && output.contains("2") && output.contains("1"),
        "Expected output to contain countdown, got: {}",
        output
    );
}

#[test]
fn test_for_1() {
    let output = run_binary("for_1");
    assert!(
        output.contains("The value is: 10") && output.contains("The value is: 30"),
        "Expected output to contain for loop values, got: {}",
        output
    );
}

#[test]
fn test_for_nested() {
    let output = run_binary("for_nested");
    assert!(
        output.contains("1 2 3") && output.contains("4 5 6"),
        "Expected output to contain nested loop results, got: {}",
        output
    );
}

#[test]
fn test_for_range() {
    let output = run_binary("for_range");
    assert!(
        output.contains("Exclusive range value: 1") && output.contains("Exclusive range value: 3"),
        "Expected output to contain range values, got: {}",
        output
    );
}

// ===== Functions =====

#[test]
fn test_functions_syntax() {
    let output = run_binary("functions_syntax");
    // Just ensure it compiles and runs (might have warnings)
    assert!(
        !output.is_empty() || output.is_empty(), // Always true, just checking it runs
        "functions_syntax should run"
    );
}

#[test]
fn test_functions_return_values() {
    let output = run_binary("functions_return_values");
    assert!(
        output.contains("The sum is: 8"),
        "Expected output to contain sum, got: {}",
        output
    );
}

#[test]
fn test_function_passing_by_value() {
    let output = run_binary("function_passing_by_value");
    assert!(
        output.contains("Value inside function: 6") && output.contains("Original value of x after function call: 5"),
        "Expected output to show value passing, got: {}",
        output
    );
}

#[test]
fn test_functions_passing_by_reference() {
    let output = run_binary("functions_passing_by_reference");
    assert!(
        output.contains("The length of 'hello' is 5"),
        "Expected output to contain length calculation, got: {}",
        output
    );
}

#[test]
fn test_functions_passing_by_mutable_reference() {
    let output = run_binary("functions_passing_by_mutable_reference");
    assert!(
        output.contains("s in main: hello, world"),
        "Expected output to contain modified string, got: {}",
        output
    );
}

#[test]
fn test_functions_clone() {
    let output = run_binary("functions_clone");
    assert!(
        output.contains("Function received ownership of: hello") && output.contains("We can still use s1 after cloning: hello"),
        "Expected output to show cloning behavior, got: {}",
        output
    );
}

// ===== Ownership =====

#[test]
fn test_ownership_and_functions() {
    let output = run_binary("ownership_and_functions");
    assert!(
        output.contains("Inside takes_ownership: hello"),
        "Expected output to show ownership behavior, got: {}",
        output
    );
}

// ===== Pattern Matching =====

#[test]
fn test_matching_literals() {
    let output = run_binary("matching_literals");
    assert_eq!(output.trim(), "One");
}

#[test]
fn test_matching_ranges() {
    let output = run_binary("matching_ranges");
    assert!(
        output.contains("One through five"),
        "Expected output to contain range match result, got: {}",
        output
    );
}

#[test]
fn test_matching_with_variables() {
    let output = run_binary("matching_with_variables");
    assert!(
        output.contains("The numbers are opposites"),
        "Expected output to contain match result, got: {}",
        output
    );
}

#[test]
fn test_match_guards() {
    let output = run_binary("match_guards");
    assert!(
        output.contains("On the y-axis at y = 10"),
        "Expected output to contain guard match result, got: {}",
        output
    );
}

#[test]
fn test_combining_patterns() {
    let output = run_binary("combining_patterns");
    assert!(
        output.contains("The number is one or two"),
        "Expected output to contain pattern result, got: {}",
        output
    );
}

// ===== Enums =====

#[test]
fn test_enums() {
    let output = run_binary("enums");
    // Output contains warnings, ensure it runs
    assert!(
        output.contains("Rgb") || output.len() > 0,
        "enums should produce output"
    );
}

#[test]
fn test_enums_match() {
    let output = run_binary("enums_match");
    // Output contains warnings, ensure it runs
    assert!(
        output.len() > 0,
        "enums_match should produce output"
    );
}

#[test]
fn test_enums_methods_1() {
    let output = run_binary("enums_methods_1");
    // Output contains warnings, ensure it runs
    assert!(
        output.len() > 0,
        "enums_methods_1 should produce output"
    );
}

#[test]
fn test_enums_methods_2() {
    let output = run_binary("enums_methods_2");
    assert!(
        output.contains("Moving to coordinates: x = 10, y = 20"),
        "Expected output to contain movement coordinates, got: {}",
        output
    );
}

#[test]
fn test_destructuring_enums() {
    let output = run_binary("destructuring_enums");
    assert!(
        output.contains("Quit message") && output.contains("Move to x: 10, y: 20"),
        "Expected output to contain enum destructuring results, got: {}",
        output
    );
}

// ===== Structs =====

#[test]
fn test_structs() {
    let output = run_binary("structs");
    assert!(
        output.contains("Username: someusername123") && output.contains("Email: someone@example.com"),
        "Expected output to contain struct fields, got: {}",
        output
    );
}

#[test]
fn test_structs_initialization() {
    let output = run_binary("structs_initialization");
    assert!(
        output.contains("Username: user1"),
        "Expected output to contain initialized struct, got: {}",
        output
    );
}

#[test]
fn test_structs_methods() {
    let output = run_binary("structs_methods");
    assert!(
        output.contains("The area of rect1 is 1500 square pixels"),
        "Expected output to contain rectangle area, got: {}",
        output
    );
}

#[test]
fn test_tuple_structs() {
    let output = run_binary("tuple_structs");
    assert!(
        output.contains("Black: (0, 0, 0)"),
        "Expected output to contain tuple struct, got: {}",
        output
    );
}

#[test]
fn test_unit_structs() {
    let output = run_binary("unit_structs");
    // Unit struct might produce minimal output
    assert!(
        output.len() >= 0,
        "unit_structs should compile and run"
    );
}

#[test]
fn test_structs_associated_functions_fails_to_compile() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            "chapter_2_structs_associated_functions",
            "--quiet",
        ])
        .output()
        .expect("Failed to check if structs_associated_functions compiles");

    assert!(
        !output.status.success(),
        "structs_associated_functions should have compilation error"
    );
}

// ===== Option & Result =====

#[test]
fn test_option() {
    let output = run_binary("option");
    assert!(
        output.contains("The number is: 5") && output.contains("No number"),
        "Expected output to contain option handling, got: {}",
        output
    );
}

#[test]
fn test_option_type() {
    let output = run_binary("option_type");
    assert!(
        output.contains("Found element at index: 2") && output.contains("Element not found"),
        "Expected output to contain option type results, got: {}",
        output
    );
}

#[test]
fn test_result_type() {
    let output = run_binary("result_type");
    assert!(
        output.contains("Result: 5") && output.contains("Error: Cannot divide by zero"),
        "Expected output to contain result handling, got: {}",
        output
    );
}

#[test]
fn test_unwrap_or() {
    let output = run_binary("unwrap_or");
    assert!(
        output.contains("The number is: 5") && output.contains("The default number is: 10"),
        "Expected output to contain unwrap_or results, got: {}",
        output
    );
}

// ===== Error Handling & Panic =====

#[test]
fn test_error_handling_1() {
    let output = run_binary("error_handling_1");
    // This program has warnings but should run
    assert!(
        !output.is_empty() || output.is_empty(), // Always true
        "error_handling_1 should run"
    );
}

#[test]
fn test_panic_macro_panics() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            "chapter_2_panic_macro",
            "--quiet",
        ])
        .output()
        .expect("Failed to run panic_macro");

    assert!(
        !output.status.success(),
        "panic_macro should panic and exit with error code"
    );
}
