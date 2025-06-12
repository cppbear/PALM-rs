// Answer 0

#[test]
fn test_into_bytes_standard_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct CustomHeader(ByteStr);

    let header_name = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };
    let bytes = header_name.into_bytes();
    let expected = Bytes::from_static(b"accept");
    assert_eq!(bytes, expected);
}

#[test]
fn test_into_bytes_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct CustomHeader(ByteStr);

    let custom_bytes = ByteStr::from_static(b"x-custom-header");
    let header_name = HeaderName {
        inner: Repr::Custom(CustomHeader(custom_bytes)),
    };
    let bytes = header_name.into_bytes();
    
    let expected = Bytes::from_static(b"x-custom-header");
    assert_eq!(bytes, expected);
}

#[test]
fn test_into_bytes_empty_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct CustomHeader(ByteStr);

    let custom_bytes = ByteStr::from_static(b"");
    let header_name = HeaderName {
        inner: Repr::Custom(CustomHeader(custom_bytes)),
    };
    let bytes = header_name.into_bytes();
    
    let expected = Bytes::from_static(b"");
    assert_eq!(bytes, expected);
}

