// Answer 0

#[test]
fn test_is_valid_cap_letter_digit_false() {
    let byte: u8 = b'5'; // Matches b'0' ... b'9'
    assert_eq!(is_valid_cap_letter(&byte), false);
}

#[test]
fn test_is_valid_cap_letter_lowercase_false() {
    let byte: u8 = b'g'; // Matches b'a' ... b'z'
    assert_eq!(is_valid_cap_letter(&byte), false);
}

#[test]
fn test_is_valid_cap_letter_uppercase_false() {
    let byte: u8 = b'G'; // Matches b'A' ... b'Z'
    assert_eq!(is_valid_cap_letter(&byte), false);
}

#[test]
fn test_is_valid_cap_letter_underscore_true() {
    let byte: u8 = b'_'; // Matches b'_' 
    assert_eq!(is_valid_cap_letter(&byte), true);
}

#[test]
fn test_is_valid_cap_letter_invalid_true() {
    let byte: u8 = b'@'; // Matches _ (non-specific)
    assert_eq!(is_valid_cap_letter(&byte), true);
}

