// Answer 0

#[test]
fn test_from_bytes_invalid_length() {
    let input = b"12"; // Length is not 3
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_char_first_byte() {
    let input = b"0A1"; // First byte is non-digit
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_char_second_byte() {
    let input = b"12Z"; // Second byte is non-digit
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_invalid_char_third_byte() {
    let input = b"34*"; // Third byte is non-digit
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_first_byte_zero() {
    let input = b"012"; // First byte is 0
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_first_byte_ten() {
    let input = b"10A"; // First byte exceeds 9
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_second_byte_ten() {
    let input = b"3A4"; // Second byte exceeds 9
    let result = from_bytes(input);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_third_byte_ten() {
    let input = b"56A"; // Third byte exceeds 9
    let result = from_bytes(input);
    assert!(result.is_err());
}

