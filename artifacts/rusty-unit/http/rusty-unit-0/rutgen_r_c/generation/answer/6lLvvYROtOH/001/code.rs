// Answer 0

#[test]
fn test_from_static_valid_header() {
    let result = HdrName::from_static("Content-Type", |hdr| hdr);
    assert!(result.is_some());
}

#[test]
fn test_from_static_empty_header() {
    let result = std::panic::catch_unwind(|| {
        HdrName::from_static("", |hdr| hdr);
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_invalid_header_with_null() {
    let result = std::panic::catch_unwind(|| {
        HdrName::from_static("Invalid\x00Header", |hdr| hdr);
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_standard_header() {
    let result = HdrName::from_static("Accept", |hdr| hdr);
    assert!(result.is_some());
}

#[test]
fn test_from_static_long_header() {
    let long_header = "a".repeat(super::MAX_HEADER_NAME_LEN + 1);
    let result = std::panic::catch_unwind(|| {
        HdrName::from_static(&long_header, |hdr| hdr);
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_valid_custom_header() {
    let result = HdrName::from_static("X-Custom-Header", |hdr| hdr);
    assert!(result.is_some());
}

