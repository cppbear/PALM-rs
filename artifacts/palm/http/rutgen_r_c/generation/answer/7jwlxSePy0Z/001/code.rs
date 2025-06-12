// Answer 0

#[test]
fn test_from_str_valid_status_code() {
    let result = StatusCode::from_str("200");
    assert!(result.is_ok());
    if let Ok(code) = result {
        assert_eq!(code.as_u16(), 200);
        assert_eq!(code.as_str(), "OK");
    }
}

#[test]
fn test_from_str_invalid_length() {
    let result = StatusCode::from_str("20");
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_characters() {
    let result = StatusCode::from_str("20A");
    assert!(result.is_err());

    let result = StatusCode::from_str("2000");
    assert!(result.is_err());
}

#[test]
fn test_from_str_zero_first_digit() {
    let result = StatusCode::from_str("000");
    assert!(result.is_err());

    let result = StatusCode::from_str("005");
    assert!(result.is_err());
}

#[test]
fn test_from_str_valid_edge_case() {
    let result = StatusCode::from_str("100");
    assert!(result.is_ok());
    if let Ok(code) = result {
        assert_eq!(code.as_u16(), 100);
        assert_eq!(code.as_str(), "Continue");
    }

    let result = StatusCode::from_str("599");
    assert!(result.is_ok());
    if let Ok(code) = result {
        assert_eq!(code.as_u16(), 599);
    }
}

#[test]
fn test_from_str_invalid_non_numeric() {
    let result = StatusCode::from_str("abc");
    assert!(result.is_err());
}

