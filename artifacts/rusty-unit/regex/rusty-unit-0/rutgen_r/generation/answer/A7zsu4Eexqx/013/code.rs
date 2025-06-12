// Answer 0

#[test]
fn test_is_valid_cap_letter_valid_lowercase() {
    let b: u8 = b'a';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_valid_multiple_lowercase() {
    let b: u8 = b'z';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_invalid_digit() {
    let b: u8 = b'3';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_valid_underscore() {
    let b: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_valid_uppercase() {
    let b: u8 = b'A';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_invalid_char() {
    let b: u8 = b'!';
    assert_eq!(is_valid_cap_letter(&b), false);
}

