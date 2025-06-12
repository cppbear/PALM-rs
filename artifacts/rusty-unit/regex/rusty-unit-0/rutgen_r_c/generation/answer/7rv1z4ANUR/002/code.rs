// Answer 0

#[test]
fn test_as_byte_returns_some_when_not_eof() {
    let byte_value = Byte::byte(100); // A valid byte value (not EOF)
    assert_eq!(byte_value.as_byte(), Some(100));
}

#[test]
fn test_as_byte_handles_lower_boundary() {
    let byte_value = Byte::byte(0); // Lower boundary valid byte value
    assert_eq!(byte_value.as_byte(), Some(0));
}

#[test]
fn test_as_byte_handles_upper_boundary() {
    let byte_value = Byte::byte(255); // Upper boundary valid byte value
    assert_eq!(byte_value.as_byte(), Some(255));
}

