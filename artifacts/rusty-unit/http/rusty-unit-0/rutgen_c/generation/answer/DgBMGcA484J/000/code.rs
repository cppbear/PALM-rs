// Answer 0

#[test]
fn test_from_u16_valid_code() {
    assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
}

#[test]
fn test_from_u16_lower_bound() {
    let result = StatusCode::from_u16(100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 100);
}

#[test]
fn test_from_u16_upper_bound() {
    let result = StatusCode::from_u16(999);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 999);
}

#[test]
fn test_from_u16_below_lower_bound() {
    let result = StatusCode::from_u16(99);
    assert!(result.is_err());
}

#[test]
fn test_from_u16_above_upper_bound() {
    let result = StatusCode::from_u16(1000);
    assert!(result.is_err());
}

#[test]
fn test_from_u16_non_zero() {
    assert!(StatusCode::from_u16(0).is_err());
}

