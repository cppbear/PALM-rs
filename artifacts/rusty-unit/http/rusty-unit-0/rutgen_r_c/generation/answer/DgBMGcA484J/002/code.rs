// Answer 0

#[test]
fn test_from_u16_below_lower_bound() {
    let result = StatusCode::from_u16(99);
    assert!(result.is_err());
}

#[test]
fn test_from_u16_at_lower_bound() {
    let result = StatusCode::from_u16(100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StatusCode::OK);
}

#[test]
fn test_from_u16_above_upper_bound() {
    let result = StatusCode::from_u16(1000);
    assert!(result.is_err());
}

#[test]
fn test_from_u16_at_upper_bound_minus_one() {
    let result = StatusCode::from_u16(999);
    assert!(result.is_ok());
}

#[test]
fn test_from_u16_negative_number() {
    let result = StatusCode::from_u16(u16::MAX);
    assert!(result.is_err());
}

#[test]
fn test_from_u16_boundary_values() {
    for value in [100, 999].iter() {
        let result = StatusCode::from_u16(*value);
        assert!(result.is_ok());
    }

    for value in [99, 1000].iter() {
        let result = StatusCode::from_u16(*value);
        assert!(result.is_err());
    }
}

