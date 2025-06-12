// Answer 0

#[test]
fn test_is_valid_cap_letter_with_uppercase_A() {
    let b = b'A';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_uppercase_Z() {
    let b = b'Z';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_lowercase_a() {
    let b = b'a';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_lowercase_z() {
    let b = b'z';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_digit_0() {
    let b = b'0';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_digit_9() {
    let b = b'9';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_underscore() {
    let b = b'_';
    assert_eq!(is_valid_cap_letter(&b), true);
}

#[test]
fn test_is_valid_cap_letter_with_special_character() {
    let b = b'@';
    assert_eq!(is_valid_cap_letter(&b), false);
}

#[test]
fn test_is_valid_cap_letter_with_non_alphanumeric_character() {
    let b = b'&';
    assert_eq!(is_valid_cap_letter(&b), false);
}

