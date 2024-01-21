use scheduler::process::interactive_shell;

#[test]
fn test_interactive_shell() {
    interactive_shell("files/sample-input.txt", "files/temp-output.txt").unwrap();

    let output = std::fs::read_to_string("files/temp-output.txt").unwrap();
    let expected_output = std::fs::read_to_string("files/sample-output.txt").unwrap();

    assert_eq!(output, expected_output);
}
