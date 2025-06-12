// Answer 0

#[test]
fn test_is_valid_cap_letter_with_valid_underscore() {
    let byte: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_invalid_character() {
    let byte: u8 = b'!';
    assert_eq!(is_valid_cap_letter(&byte), false);
}

#[test]
fn test_is_valid_cap_letter_with_valid_numeric() {
    let byte: u8 = b'5';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_valid_lowercase() {
    let byte: u8 = b'a';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_valid_uppercase() {
    let byte: u8 = b'A';
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_with_invalid_non_alphanumeric() {
    let byte: u8 = b'-';
    assert_eq!(is_valid_cap_letter(&byte), false);
}

