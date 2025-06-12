// Answer 0

#[test]
fn test_valid_cap_letter_uppercase() {
    let b: u8 = b'A'; 
    assert!(is_valid_cap_letter(&b));
}

#[test]
fn test_valid_cap_letter_lowercase() {
    let b: u8 = b'a';
    assert!(is_valid_cap_letter(&b));
}

#[test]
fn test_valid_cap_letter_digit() {
    let b: u8 = b'0';
    assert!(is_valid_cap_letter(&b));
}

#[test]
fn test_valid_cap_letter_underscore() {
    let b: u8 = b'_'; 
    assert!(is_valid_cap_letter(&b));
}

#[test]
fn test_invalid_cap_letter_special() {
    let b: u8 = b'#'; 
    assert!(!is_valid_cap_letter(&b));
}

#[test]
fn test_invalid_cap_letter_control() {
    let b: u8 = b'\0'; 
    assert!(!is_valid_cap_letter(&b));
}

