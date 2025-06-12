// Answer 0

#[test]
fn test_valid_standard_header() {
    let hdr = HeaderName::from_static("accept");
}

#[test]
fn test_empty_header_name() {
    let hdr = HeaderName::from_static("");
}

#[should_panic]
fn test_invalid_header_with_uppercase() {
    let _ = HeaderName::from_static("InvalidUppercase");
}

#[should_panic]
fn test_invalid_header_with_special_characters() {
    let _ = HeaderName::from_static("invalid@header");
}

