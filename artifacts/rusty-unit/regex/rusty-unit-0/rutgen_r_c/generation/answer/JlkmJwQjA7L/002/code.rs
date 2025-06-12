// Answer 0

#[test]
fn test_transitions_row_debug_empty() {
    let transition_row = TransitionsRow(&[]);
    let result = format!("{:?}", transition_row);
    assert_eq!(result, "{}");
}

#[test]
fn test_transitions_row_debug_dead_state() {
    let transition_row = TransitionsRow(&[STATE_DEAD]);
    let result = format!("{:?}", transition_row);
    assert_eq!(result, "{EOF: \"DEAD\"}");
}

#[test]
fn test_transitions_row_debug_unknown_state() {
    let transition_row = TransitionsRow(&[STATE_UNKNOWN]);
    let result = format!("{:?}", transition_row);
    assert_eq!(result, "{}");
}

#[test]
fn test_transitions_row_debug_mixed_states() {
    let transition_row = TransitionsRow(&[STATE_UNKNOWN, STATE_DEAD, STATE_MAX]);
    let result = format!("{:?}", transition_row);
    assert_eq!(result, "{EOF: \"DEAD\", EOF: \"1048575\"}");
}

#[test]
fn test_transitions_row_debug_multiple_dead_states() {
    let transition_row = TransitionsRow(&[STATE_DEAD, STATE_DEAD]);
    let result = format!("{:?}", transition_row);
    assert_eq!(result, "{EOF: \"DEAD\", EOF: \"DEAD\"}");
}

