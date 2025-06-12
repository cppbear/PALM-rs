// Answer 0

#[test]
fn test_transitions_row_empty() {
    let transitions_row = TransitionsRow(&[]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", transitions_row));
    assert!(result.is_ok());
    assert!(output.is_empty());
}

#[test]
fn test_transitions_row_dead_state() {
    let transitions_row = TransitionsRow(&[STATE_DEAD]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", transitions_row));
    assert!(result.is_ok());
    assert!(output.contains("DEAD"));
}

#[test]
fn test_transitions_row_multiple_states() {
    let transitions_row = TransitionsRow(&[STATE_UNKNOWN, STATE_DEAD, 3, 5]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", transitions_row));
    assert!(result.is_ok());
    assert!(output.contains("3"));
    assert!(output.contains("5"));
    assert!(output.contains("DEAD"));
}

#[test]
fn test_transitions_row_escaped_characters() {
    let transitions_row = TransitionsRow(&[10, 32, 127, 255]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", transitions_row));
    assert!(result.is_ok());
    assert!(output.contains("\\n")); // Newline character
    assert!(output.contains(" "));   // Space character
    assert!(output.contains("\\x7f")); // Delete character
    assert!(output.contains("\\xff")); // Escaped 255
}

#[test]
fn test_transitions_row_unknown_state() {
    let transitions_row = TransitionsRow(&[STATE_UNKNOWN, STATE_UNKNOWN]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{:?}", transitions_row));
    assert!(result.is_ok());
    assert!(output.is_empty());
}

