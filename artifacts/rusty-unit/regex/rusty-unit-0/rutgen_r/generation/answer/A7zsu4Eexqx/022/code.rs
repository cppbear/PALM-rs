// Answer 0

#[test]
fn test_is_valid_cap_letter_zero() {
    let b: u8 = b'0';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_a() {
    let b: u8 = b'a';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_A() {
    let b: u8 = b'A';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_nine() {
    let b: u8 = b'9';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_lower_z() {
    let b: u8 = b'z';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_upper_Z() {
    let b: u8 = b'Z';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let b: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_other_char() {
    let b: u8 = b'#';
    assert_eq!(is_valid_cap_letter(&b), false);
}

