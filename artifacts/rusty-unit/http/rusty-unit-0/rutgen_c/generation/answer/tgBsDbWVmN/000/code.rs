// Answer 0

#[test]
fn test_as_str_standard_header() {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    enum StandardHeader {
        Accept,
    }

    impl StandardHeader {
        fn as_str(&self) -> &'static str {
            match *self {
                StandardHeader::Accept => "accept",
            }
        }
    }

    let header_name = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };
    assert_eq!(header_name.as_str(), "accept");
}

#[test]
fn test_as_str_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct CustomHeader(ByteStr);

    let custom_bytes = Bytes::from_static(b"custom-header");
    let custom_header = CustomHeader(ByteStr { bytes: custom_bytes });

    let header_name = HeaderName {
        inner: Repr::Custom(custom_header),
    };
    assert_eq!(header_name.as_str(), "custom-header");
}

