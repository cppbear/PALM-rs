// Answer 0

#[test]
fn test_from_lowercase_valid_header() {
    let hdr = from_lowercase(b"content-length").unwrap();
    assert_eq!(hdr.to_string(), "content-length");
}

#[test]
fn test_from_lowercase_invalid_header_uppercase() {
    let result = from_lowercase(b"Content-Length");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_invalid_header_mixed_case() {
    let result = from_lowercase(b"Content-length");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_invalid_header_special_chars() {
    let result = from_lowercase(b"content-length!");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_invalid_header_empty() {
    let result = from_lowercase(b"");
    assert!(result.is_err());
}

#[test]
fn test_from_lowercase_invalid_header_invalid_utf8() {
    let result = from_lowercase(b"\xFF\xFF");
    assert!(result.is_err());
}

