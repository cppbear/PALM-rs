// Answer 0

#[test]
fn test_is_hex_with_digit_zero() {
    let result = is_hex('0');
    assert!(result);
}

#[test]
fn test_is_hex_with_digit_nine() {
    let result = is_hex('9');
    assert!(result);
}

#[test]
fn test_is_hex_with_lowercase_a() {
    let result = is_hex('a');
    assert!(result);
}

#[test]
fn test_is_hex_with_lowercase_f() {
    let result = is_hex('f');
    assert!(result);
}

#[test]
fn test_is_hex_with_uppercase_a() {
    let result = is_hex('A');
    assert!(result);
}

#[test]
fn test_is_hex_with_uppercase_f() {
    let result = is_hex('F');
    assert!(result);
}

#[test]
fn test_is_hex_with_non_hex_character() {
    let result = is_hex('g');
    assert!(!result);
}

#[test]
fn test_is_hex_with_special_character() {
    let result = is_hex('#');
    assert!(!result);
}

#[test]
fn test_is_hex_with_space_character() {
    let result = is_hex(' ');
    assert!(!result);
}

