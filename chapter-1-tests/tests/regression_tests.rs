use std::process::Command;

/// Helper function to run a chapter-1 binary and capture its output
fn run_binary(binary_name: &str) -> String {
    let package_name = if binary_name == "functions_rs" {
        "chapter_1_functions_rs".to_string()
    } else {
        format!("chapter_1_{}", binary_name)
    };

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            &package_name,
            "--quiet",
        ])
        .output()
        .expect(&format!("Failed to run {}", binary_name));

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_hello_world() {
    let output = run_binary("hello_world");
    assert_eq!(output.trim(), "Hello, Rust!");
}

#[test]
fn test_booleans() {
    let output = run_binary("booleans");
    assert_eq!(output.trim(), "Boolean: true");
}

#[test]
fn test_integers() {
    let output = run_binary("integers");
    let expected = "Signed Integer: -42\nUnsigned Integer: 123456";
    assert_eq!(output.trim(), expected);
}

#[test]
fn test_floats() {
    let output = run_binary("floats");
    assert!(
        output.contains("Floating-point: 3.14"),
        "Expected output to contain 'Floating-point: 3.14', got: {}",
        output
    );
}

#[test]
fn test_characters() {
    let output = run_binary("characters");
    assert!(
        output.contains("Character: A"),
        "Expected output to contain 'Character: A', got: {}",
        output
    );
}

#[test]
fn test_constants() {
    let output = run_binary("constants");
    assert_eq!(output.trim(), "The maximum points are: 100000");
}

#[test]
fn test_control_flow_1() {
    let output = run_binary("control_flow_1");
    assert_eq!(output.trim(), "The number is odd");
}

#[test]
fn test_control_flow_2() {
    let output = run_binary("control_flow_2");
    assert!(
        output.contains("Counted to 5"),
        "Expected output to contain 'Counted to 5', got: {}",
        output
    );
}

#[test]
fn test_control_flow_3() {
    let output = run_binary("control_flow_3");
    assert!(
        output.contains("Perfect weather") || output.contains("Espresso"),
        "Expected output to contain weather or Espresso content, got: {}",
        output
    );
}

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
        output.contains("The initial value of y is: 10") && output.contains("The updated value of y is: 15"),
        "Expected output to contain both 'The initial value of y is: 10' and 'The updated value of y is: 15', got: {}",
        output
    );
}

#[test]
fn test_shadowing() {
    let output = run_binary("shadowing");
    assert!(
        output.contains("The value of z after shadowing is: 6"),
        "Expected output to contain 'The value of z after shadowing is: 6', got: {}",
        output
    );
}

#[test]
fn test_functions() {
    let output = run_binary("functions_rs");
    assert!(
        output.contains("Hello") && output.contains("sum"),
        "Expected output to contain 'Hello' and 'sum', got: {}",
        output
    );
}

#[test]
fn test_developer_mood() {
    let output = run_binary("developer_mood");
    assert!(
        output.contains("Productivity mode") || output.contains("mood"),
        "Expected output to mention productivity mode or mood, got: {}",
        output
    );
}

#[test]
fn test_why_learn_rust_fails_to_compile() {
    // why_learn_rust intentionally has a compilation error to demonstrate Rust's safety
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--package",
            "chapter_1_why_learn_rust",
            "--quiet",
        ])
        .output()
        .expect("Failed to check if why_learn_rust compiles");

    assert!(
        !output.status.success(),
        "why_learn_rust should fail to compile due to uninitialized variable"
    );
}

#[test]
fn test_web_server_compiles() {
    // web_server is more complex, just ensure it compiles
    let output = Command::new("cargo")
        .args(&["build", "--package", "chapter_1_web_server", "--quiet"])
        .output()
        .expect("Failed to check if web_server compiles");

    assert!(
        output.status.success(),
        "web_server should compile successfully"
    );
}
