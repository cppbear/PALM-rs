// Answer 0

#[test]
fn test_valid_cap_letter_uppercase_A() {
    let byte: u8 = b'A';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_valid_cap_letter_uppercase_Z() {
    let byte: u8 = b'Z';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_valid_cap_letter_underscore() {
    let byte: u8 = b'_';
    is_valid_cap_letter(&byte);
}

#[test]
fn test_valid_cap_letter_range() {
    for byte in b'A'..=b'Z' {
        is_valid_cap_letter(&byte);
    }
}

#[test]
fn test_valid_cap_letter_inclusive() {
    let bytes: [u8; 3] = [b'A', b'Z', b'_'];
    for &byte in &bytes {
        is_valid_cap_letter(&byte);
    }
}

