// Answer 0

#[test]
fn test_from_bytes_too_short() {
    let input = b"12"; // length is 2
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_too_long() {
    let input = b"1234"; // length is 4
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_empty() {
    let input = b""; // length is 0
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_character_high() {
    let input = b"1A1"; // contains non-numeric character
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_character_low() {
    let input = b"-1*"; // contains non-numeric character
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_leading_zero() {
    let input = b"012"; // first character is zero
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_character_over_nine() {
    let input = b"1:2"; // second character is greater than 9
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_all_valid_digits() {
    let input = b"200"; // valid input
    let result = StatusCode::from_bytes(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 200);
}

