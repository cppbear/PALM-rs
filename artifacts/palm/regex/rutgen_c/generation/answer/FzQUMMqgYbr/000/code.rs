// Answer 0

#[test]
fn test_show_state_ptr_unknown() {
    let state = STATE_UNKNOWN;
    let result = show_state_ptr(state);
    assert_eq!(result, "1 (unknown)");
}

#[test]
fn test_show_state_ptr_dead() {
    let state = STATE_DEAD;
    let result = show_state_ptr(state);
    assert_eq!(result, "2 (dead)");
}

#[test]
fn test_show_state_ptr_quit() {
    let state = STATE_QUIT;
    let result = show_state_ptr(state);
    assert_eq!(result, "3 (quit)");
}

#[test]
fn test_show_state_ptr_start() {
    let state = STATE_START;
    let result = show_state_ptr(state);
    assert_eq!(result, "1073741824 (start)");
}

#[test]
fn test_show_state_ptr_match() {
    let state = STATE_MATCH;
    let result = show_state_ptr(state);
    assert_eq!(result, "536870912 (match)");
}

#[test]
fn test_show_state_ptr_normal() {
    let state = 42; // An arbitrary number within bounds of STATE_MAX
    let result = show_state_ptr(state);
    assert_eq!(result, "42");
}

