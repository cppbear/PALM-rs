// Answer 0

#[test]
fn test_is_valid_cap_letter_lowercase_a() {
    let b: u8 = b'a';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_lowercase_m() {
    let b: u8 = b'm';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_lowercase_z() {
    let b: u8 = b'z';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_uppercase_a() {
    let b: u8 = b'A';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_uppercase_m() {
    let b: u8 = b'M';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_uppercase_z() {
    let b: u8 = b'Z';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_numeric_0() {
    let b: u8 = b'0';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_numeric_9() {
    let b: u8 = b'9';
    is_valid_cap_letter(&b);
}

#[test]
fn test_is_valid_cap_letter_underscore() {
    let b: u8 = b'_';
    is_valid_cap_letter(&b);
}

