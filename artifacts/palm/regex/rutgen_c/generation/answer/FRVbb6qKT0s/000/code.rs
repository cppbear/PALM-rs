// Answer 0

#[test]
fn test_eof() {
    let byte_eof = Byte::eof();
    assert_eq!(byte_eof.0, 256);
}

#[test]
fn test_byte_conversion() {
    let byte_value = Byte::byte(5);
    assert_eq!(byte_value.0, 5);
}

#[test]
fn test_eof_is_eof() {
    let byte_eof = Byte::eof();
    assert!(byte_eof.is_eof());
}

#[test]
fn test_byte_is_not_eof() {
    let byte_value = Byte::byte(5);
    assert!(!byte_value.is_eof());
}

