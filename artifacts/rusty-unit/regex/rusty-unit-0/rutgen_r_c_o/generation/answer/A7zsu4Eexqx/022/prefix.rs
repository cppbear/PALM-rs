// Answer 0

#[test]
fn test_is_valid_cap_letter_with_invalid_numeric() {
    let input = b'!';
    is_valid_cap_letter(&input);
}

#[test]
fn test_is_valid_cap_letter_with_invalid_lowercase() {
    let input = b'~';
    is_valid_cap_letter(&input);
}

#[test]
fn test_is_valid_cap_letter_with_invalid_uppercase() {
    let input = b'@';
    is_valid_cap_letter(&input);
}

#[test]
fn test_is_valid_cap_letter_with_valid_underscore() {
    let input = b'_';
    is_valid_cap_letter(&input);
}

#[test]
fn test_is_valid_cap_letter_with_valid_non_alphanumeric() {
    let input = b'\n';
    is_valid_cap_letter(&input);
}

