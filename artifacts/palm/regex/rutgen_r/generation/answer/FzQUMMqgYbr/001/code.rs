// Answer 0

#[test]
fn test_show_state_ptr_state_unknown() {
    const STATE_UNKNOWN: u32 = 0b0001;
    const STATE_MAX: u32 = 0b1111;
    
    let si = STATE_UNKNOWN;
    let result = show_state_ptr(si);
    assert_eq!(result, "1 (unknown)");
}

#[test]
fn test_show_state_ptr_state_dead() {
    const STATE_DEAD: u32 = 0b0010;
    const STATE_MAX: u32 = 0b1111;
    
    let si = STATE_DEAD;
    let result = show_state_ptr(si);
    assert_eq!(result, "2 (dead)");
}

#[test]
fn test_show_state_ptr_state_quit() {
    const STATE_QUIT: u32 = 0b0100;
    const STATE_MAX: u32 = 0b1111;
    
    let si = STATE_QUIT;
    let result = show_state_ptr(si);
    assert_eq!(result, "4 (quit)");
}

#[test]
fn test_show_state_ptr_state_start_and_match() {
    const STATE_START: u32 = 0b1000;
    const STATE_MATCH: u32 = 0b0011;
    const STATE_MAX: u32 = 0b1111;
    
    let si = STATE_START | STATE_MATCH;
    let result = show_state_ptr(si);
    assert_eq!(result, "15 (start) (match)");
}

