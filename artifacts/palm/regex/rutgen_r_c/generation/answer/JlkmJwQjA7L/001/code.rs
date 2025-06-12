// Answer 0

#[test]
fn test_transitions_row_fmt_with_dead_state() {
    let states: &[StatePtr] = &[STATE_DEAD, STATE_UNKNOWN, 5, 10, 15];
    let transitions_row = TransitionsRow(states);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", transitions_row);
    assert!(result.is_ok());
    assert!(buffer.contains("0: DEAD"));
    assert!(buffer.contains("2: 5"));
    assert!(buffer.contains("3: 10"));
    assert!(buffer.contains("4: 15"));
}

#[test]
fn test_transitions_row_fmt_with_only_unknown_states() {
    let states: &[StatePtr] = &[STATE_UNKNOWN, STATE_UNKNOWN, STATE_UNKNOWN];
    let transitions_row = TransitionsRow(states);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", transitions_row);
    assert!(result.is_ok());
    assert!(buffer.is_empty());
}

#[test]
fn test_transitions_row_fmt_with_mixed_states() {
    let states: &[StatePtr] = &[2, STATE_DEAD, STATE_UNKNOWN, STATE_MATCH, STATE_START];
    let transitions_row = TransitionsRow(states);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", transitions_row);
    assert!(result.is_ok());
    assert!(buffer.contains("0: 2"));
    assert!(buffer.contains("1: DEAD"));
    assert!(buffer.contains("3: STATE_MATCH"));
    assert!(buffer.contains("4: STATE_START"));
}

#[test]
fn test_transitions_row_fmt_with_empty_states() {
    let states: &[StatePtr] = &[];
    let transitions_row = TransitionsRow(states);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", transitions_row);
    assert!(result.is_ok());
    assert!(buffer.is_empty());
}

