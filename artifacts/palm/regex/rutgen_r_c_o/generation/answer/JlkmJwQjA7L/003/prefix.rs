// Answer 0

#[test]
fn test_transitions_row_empty() {
    let transitions = TransitionsRow(&[]);
    let _ = format!("{:?}", transitions);
}

#[test]
fn test_transitions_row_state_unknown() {
    let transitions = TransitionsRow(&[STATE_UNKNOWN; 32]);
    let _ = format!("{:?}", transitions);
}

#[test]
fn test_transitions_row_state_dead() {
    let transitions = TransitionsRow(&[STATE_DEAD; 32]);
    let _ = format!("{:?}", transitions);
}

#[test]
fn test_transitions_row_mixed_states() {
    let transitions = TransitionsRow(&[
        STATE_UNKNOWN,
        STATE_DEAD,
        STATE_START,
        STATE_MATCH,
        STATE_DEAD,
        STATE_UNKNOWN,
    ]);
    let _ = format!("{:?}", transitions);
}

#[test]
fn test_transitions_row_non_state_unknown() {
    let transitions = TransitionsRow(&[
        STATE_DEAD, 
        STATE_UNKNOWN, 
        STATE_DEAD, 
        STATE_QUIT, 
        STATE_START,
        STATE_MATCH,
        STATE_UNKNOWN
    ]);
    let _ = format!("{:?}", transitions);
}

