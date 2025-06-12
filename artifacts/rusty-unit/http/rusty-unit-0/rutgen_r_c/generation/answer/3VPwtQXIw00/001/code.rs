// Answer 0

#[test]
fn test_from_static_standard_header() {
    let hdr = HeaderName::from_static("accept");
    assert_eq!(hdr.inner, Repr::Standard(StandardHeader::Accept));
}

#[test]
fn test_from_static_empty() {
    let result = std::panic::catch_unwind(|| {
        HeaderName::from_static("");
    });
    assert!(result.is_err());
}

#[test]
fn test_from_static_custom_header() {
    let custom_header = "custom-header";
    let hdr = HeaderName::from_static(custom_header);
    assert_eq!(hdr.inner, Repr::Custom(Custom(ByteStr::from_static(custom_header))));
}

#[test]
#[should_panic]
fn test_from_static_invalid_symbols() {
    HeaderName::from_static("content{}{}length"); // This should panic
}

#[test]
#[should_panic]
fn test_from_static_invalid_uppercase() {
    HeaderName::from_static("FOOBAR"); // This should panic
}

