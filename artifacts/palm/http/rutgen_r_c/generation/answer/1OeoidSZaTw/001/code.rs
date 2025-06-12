// Answer 0

#[test]
fn test_is_valid_b_equals_32() {
    let b: u8 = 32;
    assert_eq!(is_valid(b), true);
}

#[test]
fn test_is_valid_b_not_equals_127() {
    let b: u8 = 126; // Less than 127, valid
    assert_eq!(is_valid(b), true);
}

#[test]
fn test_is_valid_b_equals_127() {
    let b: u8 = 127; // Equals 127, not valid
    assert_eq!(is_valid(b), false);
}

#[test]
fn test_is_valid_b_equals_tab() {
    let b: u8 = b'\t'; // Tab character, valid
    assert_eq!(is_valid(b), true);
}

#[test]
fn test_is_valid_b_greater_than_127() {
    let b: u8 = 128; // Greater than 127, valid
    assert_eq!(is_valid(b), true);
}

