// Answer 0

#[test]
fn test_from_bytes_invalid_length() {
    let result = StatusCode::from_bytes(&[1, 2]); // Length is not equal to 3
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_first_digit_zero() {
    let result = StatusCode::from_bytes(&[0, 1, 1]); // First digit is 0
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_first_digit_high() {
    let result = StatusCode::from_bytes(&[9, 1, 10]); // Valid case where c > 9
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_second_digit_high() {
    let result = StatusCode::from_bytes(&[1, 10, 1]); // Second digit is > 9
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_third_digit_high() {
    let result = StatusCode::from_bytes(&[1, 1, 10]); // Third digit is > 9
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_valid_values() {
    let result = StatusCode::from_bytes(&[2, 0, 1]); // Valid status code "201"
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_u16(), 201);
}

