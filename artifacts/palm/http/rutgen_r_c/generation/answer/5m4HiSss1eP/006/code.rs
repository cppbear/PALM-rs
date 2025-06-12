// Answer 0

#[test]
fn test_from_bytes_valid_standard_header() {
    // Given valid standard header bytes for "accept"
    let header_bytes: &[u8] = b"accept";

    // When calling from_bytes
    let result = HeaderName::from_bytes(header_bytes);

    // Then it should return the expected HeaderName corresponding to StandardHeader::Accept
    assert!(result.is_ok());
    let header_name = result.unwrap();
    assert_eq!(header_name.as_str(), "accept");
}

#[test]
fn test_from_bytes_valid_standard_header_max_length() {
    // Given valid standard header bytes for "content-type" (including max length case)
    let header_bytes: &[u8] = b"content-type";

    // When calling from_bytes
    let result = HeaderName::from_bytes(header_bytes);

    // Then it should return the expected HeaderName corresponding to StandardHeader::ContentType
    assert!(result.is_ok());
    let header_name = result.unwrap();
    assert_eq!(header_name.as_str(), "content-type");
}

#[test]
fn test_from_bytes_valid_standard_header_with_uppercase() {
    // Given valid standard header bytes for "Accept-Language" with uppercase
    let header_bytes: &[u8] = b"ACCEPT-LANGUAGE";

    // When calling from_bytes
    let result = HeaderName::from_bytes(header_bytes);

    // Then it should return the expected HeaderName corresponding to StandardHeader::AcceptLanguage
    assert!(result.is_ok());
    let header_name = result.unwrap();
    assert_eq!(header_name.as_str(), "accept-language");
}

#[test]
fn test_from_bytes_valid_standard_header_with_special_chars() {
    // Given valid standard header bytes for "content-security-policy" 
    let header_bytes: &[u8] = b"content-security-policy";

    // When calling from_bytes
    let result = HeaderName::from_bytes(header_bytes);

    // Then it should return the expected HeaderName corresponding to StandardHeader::ContentSecurityPolicy
    assert!(result.is_ok());
    let header_name = result.unwrap();
    assert_eq!(header_name.as_str(), "content-security-policy");
}

