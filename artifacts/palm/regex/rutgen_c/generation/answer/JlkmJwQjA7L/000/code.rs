// Answer 0

#[test]
fn test_transitions_row_debug_empty() {
    let row = TransitionsRow(&[]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{}");
}

#[test]
fn test_transitions_row_debug_with_dead_state() {
    let row = TransitionsRow(&[STATE_DEAD]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{EOF: \"DEAD\"}");
}

#[test]
fn test_transitions_row_debug_with_unknown_state() {
    let row = TransitionsRow(&[STATE_UNKNOWN]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{}");
}

#[test]
fn test_transitions_row_debug_with_regular_states() {
    let row = TransitionsRow(&[0, 1, 2]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{EOF: \"0\", EOF: \"1\", EOF: \"2\"}");
}

#[test]
fn test_transitions_row_debug_mixed_states() {
    let row = TransitionsRow(&[STATE_UNKNOWN, STATE_DEAD, 2, STATE_UNKNOWN]);
    let result = format!("{:?}", row);
    assert_eq!(result, "{EOF: \"1\", EOF: \"DEAD\", EOF: \"2\"}");
}

