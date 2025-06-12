// Answer 0

#[test]
fn test_is_valid_cap_letter_digit() {
    let b: u8 = b'5';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_lowercase() {
    let b: u8 = b'l';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_uppercase() {
    let b: u8 = b'R';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let b: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_invalid() {
    let b: u8 = b'@';
    assert_eq!(is_valid_cap_letter(&b), false);
}

