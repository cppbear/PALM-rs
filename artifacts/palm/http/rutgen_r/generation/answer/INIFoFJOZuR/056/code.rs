// Answer 0

#[test]
fn test_from_bytes_get_method() {
    use crate::http::{from_bytes, Method, InvalidMethod}; // Adjust the path based on your project structure

    let src: &[u8] = b"GET";
    let result = from_bytes(src);
    assert_eq!(result, Ok(Method::Get));
}

#[test]
fn test_from_bytes_empty() {
    use crate::http::{from_bytes, InvalidMethod}; // Adjust the path based on your project structure

    let src: &[u8] = b"";
    let result = from_bytes(src);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidMethod::new());
}

#[test]
fn test_from_bytes_invalid_method() {
    use crate::http::{from_bytes, Method, InvalidMethod}; // Adjust the path based on your project structure

    let src: &[u8] = b"XYZ"; // Invalid method
    let result = from_bytes(src);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidMethod::new());
}

