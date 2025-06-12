// Answer 0

#[test]
fn test_show_state_ptr_unknown() {
    const STATE_UNKNOWN: u32 = 0xFFFFFFFF;
    const STATE_MAX: u32 = 0xFFFFFFFE;
    let si = STATE_UNKNOWN;
    let result = show_state_ptr(si);
    assert_eq!(result, "4294967295 (unknown)");
}

#[test]
fn test_show_state_ptr_dead() {
    const STATE_DEAD: u32 = 0xFFFFFFFD;
    const STATE_MAX: u32 = 0xFFFFFFFE;
    let si = STATE_DEAD;
    let result = show_state_ptr(si);
    assert_eq!(result, "4294967293 (dead)");
}

#[test]
fn test_show_state_ptr_quit() {
    const STATE_QUIT: u32 = 0xFFFFFFFC;
    const STATE_MAX: u32 = 0xFFFFFFFE;
    let si = STATE_QUIT;
    let result = show_state_ptr(si);
    assert_eq!(result, "4294967292 (quit)");
}

#[test]
fn test_show_state_ptr_start() {
    const STATE_START: u32 = 0x00000001;
    const STATE_MAX: u32 = 0xFFFFFFFE;
    let si = STATE_START;
    let result = show_state_ptr(si);
    assert_eq!(result, "1 (start)");
}

#[test]
fn test_show_state_ptr_match() {
    const STATE_MATCH: u32 = 0x00000002;
    const STATE_MAX: u32 = 0xFFFFFFFE;
    let si = STATE_MATCH;
    let result = show_state_ptr(si);
    assert_eq!(result, "2 (match)");
}

#[test]
fn test_show_state_ptr_combined() {
    const STATE_UNKNOWN: u32 = 0xFFFFFFFF;
    const STATE_START: u32 = 0x00000001;
    const STATE_MATCH: u32 = 0x00000002;
    const STATE_MAX: u32 = 0xFFFFFFFE;
    
    let si = STATE_UNKNOWN | STATE_START | STATE_MATCH;
    let result = show_state_ptr(si);
    assert_eq!(result, "4294967295 (unknown) (start) (match)");
}

