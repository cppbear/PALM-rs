// Answer 0

#[test]
fn test_from_static_valid_header() {
    let result = HdrName::from_static("accept", |hdr| hdr);
    assert!(result.inner.is_standard()); // Assuming an appropriate method like `is_standard`
}

#[test]
fn test_from_static_invalid_header_empty() {
    let result = std::panic::catch_unwind(|| {
        HdrName::from_static("", |hdr| hdr);
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_invalid_header_long_name() {
    let long_header = "a".repeat(65); // Assuming MAX_HEADER_NAME_LEN is 64
    let result = std::panic::catch_unwind(|| {
        HdrName::from_static(&long_header, |hdr| hdr);
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_invalid_header_with_null_byte() {
    let result = std::panic::catch_unwind(|| {
        HdrName::from_static("accept\0", |hdr| hdr);
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_custom_header() {
    let result = HdrName::from_static("custom-header", |hdr| hdr);
    assert!(!result.inner.is_standard()); // Assuming an appropriate method like `is_standard`
}

