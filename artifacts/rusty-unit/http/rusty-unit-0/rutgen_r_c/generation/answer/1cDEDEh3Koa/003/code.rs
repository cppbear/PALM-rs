// Answer 0

#[test]
fn test_from_bytes_length_not_three() {
    let input = &[1, 2]; // Length is not 3
    assert!(matches!(StatusCode::from_bytes(input), Err(_)));
}

#[test]
fn test_from_bytes_first_digit_zero() {
    let input = b"012"; // First digit is 0
    assert!(matches!(StatusCode::from_bytes(input), Err(_)));
}

#[test]
fn test_from_bytes_first_digit_greater_than_nine() {
    let input = b"10A"; // First digit 'A' is out of the valid range (0-9)
    assert!(matches!(StatusCode::from_bytes(input), Err(_)));
}

#[test]
fn test_from_bytes_second_digit_greater_than_nine() {
    let input = b"0A1"; // Second digit 'A' is out of the valid range (0-9)
    assert!(matches!(StatusCode::from_bytes(input), Err(_)));
}

#[test]
fn test_from_bytes_third_digit_greater_than_nine() {
    let input = b"01A"; // Third digit 'A' is out of the valid range (0-9)
    assert!(matches!(StatusCode::from_bytes(input), Err(_)));
}

