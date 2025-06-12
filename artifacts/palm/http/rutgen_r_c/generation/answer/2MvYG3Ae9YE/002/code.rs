// Answer 0

#[test]
fn test_from_lowercase_valid_header() {
    let hdr = HeaderName::from_lowercase(b"accept").unwrap();
    assert_eq!(hdr.as_str(), "accept");
}

#[test]
fn test_from_lowercase_valid_standard_header() {
    let hdr = HeaderName::from_lowercase(b"accept-charset").unwrap();
    assert_eq!(hdr.as_str(), "accept-charset");
}

#[test]
fn test_from_lowercase_valid_long_header() {
    let hdr = HeaderName::from_lowercase(b"content-encoding").unwrap();
    assert_eq!(hdr.as_str(), "content-encoding");
}

#[test]
fn test_from_lowercase_invalid_uppercase_header() {
    assert!(HeaderName::from_lowercase(b"Accept").is_err());
    assert!(HeaderName::from_lowercase(b"Content-Type").is_err());
}

#[test]
fn test_from_lowercase_empty_bytes() {
    assert!(HeaderName::from_lowercase(b"").is_err());
}

#[test]
fn test_from_lowercase_invalid_bytes() {
    assert!(HeaderName::from_lowercase(b"invalid\xFFheader").is_err());
}

#[test]
fn test_from_lowercase_mixed_case_header() {
    assert!(HeaderName::from_lowercase(b"Content-Encoding").is_err());
}

#[test]
fn test_from_lowercase_boundary_condition_valid() {
    let hdr = HeaderName::from_lowercase(b"vary").unwrap();
    assert_eq!(hdr.as_str(), "vary");
}

#[test]
fn test_from_lowercase_boundary_condition_invalid() {
    assert!(HeaderName::from_lowercase(b"Vary").is_err());
}

