// Answer 0

#[test]
fn test_is_valid_cap_letter_digits() {
    let byte: u8 = b'0';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_is_valid_cap_letter_uppercase_A() {
    let byte: u8 = b'A';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_is_valid_cap_letter_uppercase_Z() {
    let byte: u8 = b'Z';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_is_valid_cap_letter_lowercase_a() {
    let byte: u8 = b'a';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_is_valid_cap_letter_lowercase_z() {
    let byte: u8 = b'z';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let byte: u8 = b'_';
    is_valid_cap_letter(&byte);
}

