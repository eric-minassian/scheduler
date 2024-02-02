use scheduler::process::interactive_shell;

#[test]
fn test_interactive_shell() {
    interactive_shell("files/sample-input.txt", "files/temp-output.txt").unwrap();

    let output = std::fs::read_to_string("files/temp-output.txt").unwrap();
    let expected_output = std::fs::read_to_string("files/sample-output.txt").unwrap();

    assert_eq!(output, expected_output);
}

#[test]
fn negative_values() {
    interactive_shell("files/negative-input.txt", "files/negative-temp-output.txt").unwrap();

    let output = std::fs::read_to_string("files/negative-temp-output.txt").unwrap();
    let expected_output = std::fs::read_to_string("files/negative-output.txt").unwrap();

    assert_eq!(output, expected_output);
}

#[test]
fn provided_values() {
    interactive_shell("files/provided-input.txt", "files/provided-temp-output.txt").unwrap();

    let output = std::fs::read_to_string("files/provided-temp-output.txt").unwrap();
    let expected_output = std::fs::read_to_string("files/provided-output.txt").unwrap();

    assert_eq!(output, expected_output);
}
