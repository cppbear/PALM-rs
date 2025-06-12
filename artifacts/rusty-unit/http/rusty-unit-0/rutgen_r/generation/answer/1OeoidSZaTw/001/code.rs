// Answer 0

#[test]
fn test_is_valid_lower_boundary() {
    let input: u8 = 32; // Lower boundary where b >= 32
    assert_eq!(is_valid(input), true); // Expected: true since 32 >= 32 && 32 != 127
}

#[test]
fn test_is_valid_tab_character() {
    let input: u8 = b'\t'; // Testing the tab character
    assert_eq!(is_valid(input), true); // Expected: true since b == b'\t'
}

#[test]
fn test_is_valid_non_printable_high() {
    let input: u8 = 127; // Testing the value that should return false
    assert_eq!(is_valid(input), false); // Expected: false since 127 is not valid
}

#[test]
fn test_is_valid_above_lower_boundary() {
    let input: u8 = 50; // A value between 32 and 127
    assert_eq!(is_valid(input), true); // Expected: true since 50 >= 32 && 50 != 127
}

#[test]
fn test_is_valid_below_lower_boundary() {
    let input: u8 = 31; // A value below 32
    assert_eq!(is_valid(input), false); // Expected: false since 31 < 32
}

