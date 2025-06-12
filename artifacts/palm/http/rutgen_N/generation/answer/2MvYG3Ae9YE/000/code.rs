// Answer 0

#[test]
fn test_from_lowercase_valid_header() {
    let hdr = http::header::from_lowercase(b"content-length").unwrap();
    assert_eq!(hdr, http::header::CONTENT_LENGTH);
}

#[test]
fn test_from_lowercase_invalid_header_with_uppercase() {
    assert!(http::header::from_lowercase(b"Content-Length").is_err());
}

#[test]
fn test_from_lowercase_invalid_header_with_special_character() {
    assert!(http::header::from_lowercase(b"content-length!").is_err());
}

#[test]
fn test_from_lowercase_empty_header() {
    assert!(http::header::from_lowercase(b"").is_err());
}

#[test]
fn test_from_lowercase_valid_custom_header() {
    let hdr = http::header::from_lowercase(b"x-custom-header").unwrap();
    assert_eq!(hdr.to_string(), "x-custom-header");
}

