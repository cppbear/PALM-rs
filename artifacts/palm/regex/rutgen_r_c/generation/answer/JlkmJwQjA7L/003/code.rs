// Answer 0

#[test]
fn test_transitions_row_empty() {
    let row = TransitionsRow(&[]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{}");
}

#[test]
fn test_transitions_row_with_unknown_state() {
    let row = TransitionsRow(&[STATE_UNKNOWN, STATE_DEAD]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{}"); // STATE_UNKNOWN is ignored
}

#[test]
fn test_transitions_row_with_dead_state() {
    let row = TransitionsRow(&[STATE_DEAD]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{EOF: \"DEAD\"}"); // Expecting "DEAD" for EOF (0)
}

#[test]
fn test_transitions_row_with_states() {
    let row = TransitionsRow(&[2, 3, STATE_DEAD, STATE_UNKNOWN]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{\\u{2}: \"2\", \\u{3}: \"3\", EOF: \"DEAD\"}"); // Expecting proper output for 2 and 3, and "DEAD" for EOF
}

