// Answer 0

#[test]
fn test_from_bytes_length_not_three() {
    let result = from_bytes(b"12");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_first_digit_zero() {
    let result = from_bytes(b"001");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_first_digit_nine() {
    let result = from_bytes(b"991");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_second_digit_nine() {
    let result = from_bytes(b"199");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_third_digit_ten() {
    let result = from_bytes(b"99A");
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_third_digit_greater_than_nine() {
    let result = from_bytes(b"999");
    assert!(result.is_err());
}

