// Answer 0

#[test]
fn test_from_lowercase_valid_header() {
    let hdr = from_lowercase(b"content-length").unwrap();
    assert_eq!(hdr, CONTENT_LENGTH);
}

#[test]
fn test_from_lowercase_invalid_header_with_uppercase() {
    assert!(from_lowercase(b"Content-Length").is_err());
}

#[test]
fn test_from_lowercase_invalid_utf8() {
    assert!(from_lowercase(b"content-\xFFlength").is_err());
}

#[test]
fn test_from_lowercase_valid_custom_header() {
    let valid_custom_header = from_lowercase(b"x-custom-header").unwrap();
    assert!(valid_custom_header.is_custom());
}

#[test]
#[should_panic]
fn test_from_lowercase_invalid_characters() {
    // This test assumes that characters that result in HEADER_CHARS_H2[b as usize] == 0 cause panic
    from_lowercase(b"header\x1Fvalue").unwrap();
}

