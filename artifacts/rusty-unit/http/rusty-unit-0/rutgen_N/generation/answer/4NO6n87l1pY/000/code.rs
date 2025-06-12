// Answer 0

#[test]
fn test_from_utf8_valid_utf8() {
    use std::str;
    use crate::http::{from_utf8, ByteStr}; 
    use bytes::Bytes;

    let valid_bytes = Bytes::from("valid utf8 string");
    let result = from_utf8(valid_bytes);
    assert!(result.is_ok());
    
    let byte_str = result.unwrap();
    assert_eq!(byte_str.bytes, Bytes::from("valid utf8 string"));
}

#[test]
fn test_from_utf8_invalid_utf8() {
    use crate::http::{from_utf8, ByteStr}; 
    use bytes::Bytes;

    let invalid_bytes = Bytes::from(vec![0, 159, 146, 150]); // Invalid UTF-8 bytes
    let result = from_utf8(invalid_bytes);
    assert!(result.is_err());
} 

#[test]
#[should_panic]
fn test_from_utf8_empty_bytes() {
    use crate::http::{from_utf8, ByteStr}; 
    use bytes::Bytes;

    let empty_bytes = Bytes::from("");
    let result = from_utf8(empty_bytes).unwrap();
    assert_eq!(result.bytes.len(), 0);
}

