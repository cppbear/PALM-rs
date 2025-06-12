// Answer 0

#[test]
fn test_show_state_ptr_unknown() {
    let si: StatePtr = STATE_UNKNOWN;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_dead() {
    let si: StatePtr = STATE_DEAD;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_quit() {
    let si: StatePtr = STATE_QUIT;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_start() {
    let si: StatePtr = STATE_START;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_match() {
    let si: StatePtr = STATE_MATCH;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_start_and_match() {
    let si: StatePtr = STATE_START | STATE_MATCH;
    let result = show_state_ptr(si);
}

