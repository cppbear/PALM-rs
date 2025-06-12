// Answer 0

#[test]
fn test_is_hex_lower_bound_false() {
    // Character 'G' is greater than 'F'
    let result = is_hex('G');
    assert_eq!(result, false);
}

#[test]
fn test_is_hex_upper_bound_true() {
    // Character 'A' is equal to 'A' and within the valid range
    let result = is_hex('A');
    assert_eq!(result, true);
}

#[test]
fn test_is_hex_valid_digit() {
    // Character '9' is a valid hexadecimal digit
    let result = is_hex('9');
    assert_eq!(result, true);
}

#[test]
fn test_is_hex_valid_lowercase() {
    // Character 'f' is a valid hexadecimal digit
    let result = is_hex('f');
    assert_eq!(result, true);
}

#[test]
fn test_is_hex_invalid_lowercase() {
    // Character 'g' is out of the valid range for hex digits
    let result = is_hex('g');
    assert_eq!(result, false);
}

