// Answer 0

#[test]
fn test_from_bytes_valid() {
    let valid_input: &[u8] = b"200";
    let result = StatusCode::from_bytes(valid_input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 200);
}

#[test]
fn test_from_bytes_invalid_length() {
    let invalid_length: &[u8] = b"20"; // Length is 2, should return Err
    let result = StatusCode::from_bytes(invalid_length);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_zero_first_digit() {
    let zero_first_digit: &[u8] = b"000"; // First digit is 0, should return Err
    let result = StatusCode::from_bytes(zero_first_digit);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_digit() {
    let invalid_digit: &[u8] = b"2A0"; // Second digit is not a valid digit, should return Err
    let result = StatusCode::from_bytes(invalid_digit);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_exceed_digit() {
    let exceed_digit: &[u8] = b"256"; // The first digit exceeds 9, should return Err
    let result = StatusCode::from_bytes(exceed_digit);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_out_of_bounds() {
    let out_of_bounds: &[u8] = b"12"; // Length is 2, should return Err
    let result = StatusCode::from_bytes(out_of_bounds);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_non_numeric() {
    let non_numeric: &[u8] = b"2!9"; // Contains non-numeric character, should return Err
    let result = StatusCode::from_bytes(non_numeric);
    assert!(result.is_err());
}

