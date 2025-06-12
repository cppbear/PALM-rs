// Answer 0

#[test]
fn test_from_bytes_valid() {
    from_bytes(b"200");
}

#[test]
fn test_from_bytes_leading_zero() {
    let result = from_bytes(b"012");
}

#[test]
fn test_from_bytes_too_long() {
    let result = from_bytes(b"1001");
}

#[test]
fn test_from_bytes_too_short() {
    let result = from_bytes(b"10");
}

#[test]
fn test_from_bytes_invalid_first_digit_zero() {
    let result = from_bytes(b"000");
}

#[test]
fn test_from_bytes_invalid_first_digit_greater_than_nine() {
    let result = from_bytes(b"100");
}

#[test]
fn test_from_bytes_invalid_second_digit_greater_than_nine() {
    let result = from_bytes(b"1a0");
}

#[test]
fn test_from_bytes_invalid_third_digit_greater_than_nine() {
    let result = from_bytes(b"12a");
}

