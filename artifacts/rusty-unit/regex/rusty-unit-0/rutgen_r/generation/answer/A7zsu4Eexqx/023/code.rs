// Answer 0

#[test]
fn test_is_valid_cap_letter_with_digit() {
    let b: u8 = b'0';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_lowercase() {
    let b: u8 = b'a';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_uppercase() {
    let b: u8 = b'A';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_underscore() {
    let b: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_non_alphanumeric() {
    let b: u8 = b'$';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_uppercase_A_to_Z() {
    let b: u8 = b'Z';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_lowercase_a_to_z() {
    let b: u8 = b'z';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_digit_9() {
    let b: u8 = b'9';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_underscore_edge_case() {
    let b: u8 = b'_';
    assert_eq!(is_valid_cap_letter(&b), true);
}

