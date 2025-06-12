// Answer 0

#[test]
fn test_from_bytes_valid() {
    let result = StatusCode::from_bytes(b"200");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 200);
}

#[test]
fn test_from_bytes_length_not_3() {
    let result = StatusCode::from_bytes(b"20");
    assert!(result.is_err());

    let result = StatusCode::from_bytes(b"2000");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_leading_zero_a() {
    let result = StatusCode::from_bytes(b"000");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_a_greater_than_9() {
    let result = StatusCode::from_bytes(b"9A0");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_b_greater_than_9() {
    let result = StatusCode::from_bytes(b"2A0");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_c_greater_than_9() {
    let result = StatusCode::from_bytes(b"20A");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_valid_upper_boundary() {
    let result = StatusCode::from_bytes(b"999");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 999);
}

