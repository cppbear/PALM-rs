// Answer 0

#[test]
fn test_show_state_ptr_unknown() {
    let si = STATE_UNKNOWN;
    let expected_output = "1 (unknown)";
    let output = show_state_ptr(si);
    assert_eq!(output, expected_output);
}

#[test]
fn test_show_state_ptr_dead() {
    let si = STATE_DEAD;
    let expected_output = "2 (dead)";
    let output = show_state_ptr(si);
    assert_eq!(output, expected_output);
}

#[test]
fn test_show_state_ptr_quit() {
    let si = STATE_QUIT;
    let expected_output = "3 (quit)";
    let output = show_state_ptr(si);
    assert_eq!(output, expected_output);
}

#[test]
fn test_show_state_ptr_start() {
    let si = STATE_START;
    let expected_output = "1073741824 (start)";
    let output = show_state_ptr(si);
    assert_eq!(output, expected_output);
}

#[test]
fn test_show_state_ptr_match() {
    let si = STATE_MATCH;
    let expected_output = "536870911 (match)";
    let output = show_state_ptr(si);
    assert_eq!(output, expected_output);
}

#[test]
fn test_show_state_ptr_combined_start_match() {
    let si = STATE_START | STATE_MATCH;
    let expected_output = "1073741824 (start) (match)";
    let output = show_state_ptr(si);
    assert_eq!(output, expected_output);
}

