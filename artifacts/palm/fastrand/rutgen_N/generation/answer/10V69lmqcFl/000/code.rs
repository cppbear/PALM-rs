// Answer 0

#[test]
fn test_alphabetic_lowercase() {
    let result = fastrand::alphabetic();
    assert!(result.is_ascii_lowercase());
}

#[test]
fn test_alphabetic_uppercase() {
    let result = fastrand::alphabetic();
    assert!(result.is_ascii_uppercase());
}

#[test]
fn test_alphabetic_not_numeric() {
    let result = fastrand::alphabetic();
    assert!(!result.is_digit(10));
}

#[test]
fn test_alphabetic_is_alphabetic() {
    let result = fastrand::alphabetic();
    assert!(result.is_alphabetic());
}

