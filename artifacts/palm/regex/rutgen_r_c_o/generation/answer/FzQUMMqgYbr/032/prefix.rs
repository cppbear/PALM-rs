// Answer 0

#[test]
fn test_show_state_ptr_zero() {
    let si: StatePtr = 0;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_one() {
    let si: StatePtr = 1;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_small_value() {
    let si: StatePtr = 100;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_large_value() {
    let si: StatePtr = 2147483646;
    let result = show_state_ptr(si);
}

#[test]
fn test_show_state_ptr_mid_range_value() {
    let si: StatePtr = 1073741824;
    let result = show_state_ptr(si);
}

