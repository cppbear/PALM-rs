// Answer 0

#[test]
fn test_from_static_valid_standard_header() {
    let hdr = HeaderName::from_static("content-length");
    assert_eq!(CONTENT_LENGTH, hdr);
}

#[test]
#[should_panic]
fn test_from_static_invalid_empty_string() {
    HeaderName::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_invalid_uppercase() {
    HeaderName::from_static("UPPERCASE");
}

#[test]
#[should_panic]
fn test_from_static_invalid_symbols() {
    HeaderName::from_static("content{}length");
}

#[test]
fn test_from_static_valid_custom_header() {
    let custom_header: &'static str = "custom-header";
    let a = HeaderName::from_lowercase(b"custom-header").unwrap();
    let b = HeaderName::from_static(custom_header);
    assert_eq!(a, b);
}

