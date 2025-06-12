// Answer 0

#[test]
fn test_empty_transitions_debug() {
    let transitions = Transitions::new(0);
    let result = format!("{:?}", transitions);
    assert_eq!(result, "{}");
}

#[test]
fn test_single_state_transitions_debug() {
    let mut transitions = Transitions::new(1);
    transitions.table.push(STATE_DEAD); // One state with dead transition
    let result = format!("{:?}", transitions);
    assert_eq!(result, "{0: TransitionsRow([4294967297])}");
}

#[test]
fn test_multiple_states_transitions_debug() {
    let mut transitions = Transitions::new(2);
    transitions.table.extend_from_slice(&[STATE_START, STATE_MATCH]); // Adding two states
    let result = format!("{:?}", transitions);
    assert_eq!(result, "{0: TransitionsRow([2147483648, 1073741823])}");
}

#[test]
fn test_transitions_class_stride() {
    let mut transitions = Transitions::new(3);
    transitions.table.extend_from_slice(&[STATE_UNKNOWN, STATE_DEAD, STATE_QUIT]); // Three classes
    let result = format!("{:?}", transitions);
    assert_eq!(result, "{0: TransitionsRow([2147483648, 4294967297, 4294967298])}");
}

