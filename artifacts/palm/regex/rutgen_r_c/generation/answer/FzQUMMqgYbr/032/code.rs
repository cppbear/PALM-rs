// Answer 0

#[test]
fn test_show_state_ptr_normal() {
    let si: StatePtr = 0; // Should return "0"
    let result = show_state_ptr(si);
    assert_eq!(result, "0");
}

#[test]
fn test_show_state_ptr_start_condition() {
    let si: StatePtr = STATE_START; // Case where it should reflect STATE_START
    let result = show_state_ptr(si);
    assert_eq!(result, "0 (start)");
}

#[test]
fn test_show_state_ptr_match_condition() {
    let si: StatePtr = STATE_MATCH; // Case where it should reflect STATE_MATCH
    let result = show_state_ptr(si);
    assert_eq!(result, "0 (match)");
}

#[test]
fn test_show_state_ptr_unknown() {
    let si: StatePtr = STATE_UNKNOWN + 10; // Case where it should reflect STATE_UNKNOWN as false
    let result = show_state_ptr(si);
    assert_eq!(result, "10 (unknown)");
}

#[test]
fn test_show_state_ptr_dead() {
    let si: StatePtr = STATE_DEAD + 20; // Case where it should reflect STATE_DEAD as false
    let result = show_state_ptr(si);
    assert_eq!(result, "20 (dead)");
}

#[test]
fn test_show_state_ptr_quit() {
    let si: StatePtr = STATE_QUIT + 30; // Case where it should reflect STATE_QUIT as false
    let result = show_state_ptr(si);
    assert_eq!(result, "30 (quit)");
}

