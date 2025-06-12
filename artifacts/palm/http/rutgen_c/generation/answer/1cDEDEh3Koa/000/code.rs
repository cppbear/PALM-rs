// Answer 0

#[test]
fn test_from_bytes_valid_code_200() {
    let input = &[b'2', b'0', b'0'];
    let result = StatusCode::from_bytes(input);
    assert!(result.is_ok());
    let status_code = result.unwrap();
    assert_eq!(status_code.as_u16(), 200);
}

#[test]
fn test_from_bytes_valid_code_404() {
    let input = &[b'4', b'0', b'4'];
    let result = StatusCode::from_bytes(input);
    assert!(result.is_ok());
    let status_code = result.unwrap();
    assert_eq!(status_code.as_u16(), 404);
}

#[test]
fn test_from_bytes_invalid_length() {
    let input = &[b'4', b'0'];
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_digit() {
    let input = &[b'1', b'0', b'0'];
    let result = StatusCode::from_bytes(input);
    assert!(result.is_ok());
    let status_code = result.unwrap();
    assert_eq!(status_code.as_u16(), 100);

    let invalid_input = &[b'1', b'0', b'0' + 1]; // '0' + 1 generates a digit > 9
    let invalid_result = StatusCode::from_bytes(invalid_input);
    assert!(invalid_result.is_err());
}

#[test]
fn test_from_bytes_zero_first_digit() {
    let input = &[b'0', b'1', b'0'];
    let result = StatusCode::from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_large_code() {
    let input = &[b'1', b'5', b'0']; // 150 should be valid
    let result = StatusCode::from_bytes(input);
    assert!(result.is_ok());
    let status_code = result.unwrap();
    assert_eq!(status_code.as_u16(), 150);
}

