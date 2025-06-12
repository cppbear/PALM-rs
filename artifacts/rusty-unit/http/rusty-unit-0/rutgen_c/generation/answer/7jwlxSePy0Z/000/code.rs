// Answer 0

#[test]
fn test_from_str_valid() {
    let result = StatusCode::from_str("200");
    assert!(result.is_ok());
    if let Ok(status_code) = result {
        assert_eq!(status_code.as_u16(), 200);
    }
}

#[test]
fn test_from_str_invalid_length() {
    let result = StatusCode::from_str("20");
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_non_digit() {
    let result = StatusCode::from_str("20A");
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_digit() {
    let result = StatusCode::from_str("2-0");
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_zero_prefix() {
    let result = StatusCode::from_str("000");
    assert!(result.is_err());
}

