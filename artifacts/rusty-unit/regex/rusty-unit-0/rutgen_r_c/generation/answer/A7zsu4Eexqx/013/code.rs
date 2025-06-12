// Answer 0

#[test]
fn test_is_valid_cap_letter_with_lowercase_a() {
    let byte: u8 = b'a';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_lowercase_z() {
    let byte: u8 = b'z';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_digit_0() {
    let byte: u8 = b'0';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_digit_9() {
    let byte: u8 = b'9';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_uppercase_A() {
    let byte: u8 = b'A';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_uppercase_Z() {
    let byte: u8 = b'Z';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_underscore() {
    let byte: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_invalid_character() {
    let byte: u8 = b'@';
    assert_eq!(is_valid_cap_letter(&byte), false);
}

