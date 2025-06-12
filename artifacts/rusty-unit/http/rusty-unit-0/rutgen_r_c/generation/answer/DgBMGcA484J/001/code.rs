// Answer 0

#[test]
fn test_from_u16_valid_lower_bound() {
    let status_code = StatusCode::from_u16(100).unwrap();
    assert_eq!(status_code, StatusCode::CONTINUE);
}

#[test]
fn test_from_u16_valid_success() {
    let status_code = StatusCode::from_u16(200).unwrap();
    assert_eq!(status_code, StatusCode::OK);
}

#[test]
fn test_from_u16_valid_upper_bound() {
    let status_code = StatusCode::from_u16(999).unwrap();
    // Since there is no specific constant for 999, we can check the raw value.
    assert_eq!(status_code.as_u16(), 999);
}

#[test]
fn test_from_u16_invalid_too_low() {
    let result = StatusCode::from_u16(99);
    assert!(result.is_err());
}

#[test]
fn test_from_u16_invalid_too_high() {
    let result = StatusCode::from_u16(1000);
    assert!(result.is_err());
}

