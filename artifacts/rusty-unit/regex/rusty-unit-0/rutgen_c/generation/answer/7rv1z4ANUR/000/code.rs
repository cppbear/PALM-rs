// Answer 0

#[test]
fn test_as_byte_returns_none_for_eof() {
    let byte = Byte::eof();
    assert_eq!(byte.as_byte(), None);
}

#[test]
fn test_as_byte_returns_some_for_valid_byte() {
    let byte = Byte::byte(5);
    assert_eq!(byte.as_byte(), Some(5));
}

#[test]
fn test_as_byte_returns_some_for_valid_ascii() {
    let byte = Byte::byte(127);
    assert_eq!(byte.as_byte(), Some(127));
}

