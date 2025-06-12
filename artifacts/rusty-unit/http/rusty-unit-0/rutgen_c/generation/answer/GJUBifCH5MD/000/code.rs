// Answer 0

#[test]
fn test_into_bytes_with_standard_header() {
    struct CustomHeader(ByteStr);
    
    impl TryFrom<&[u8]> for CustomHeader {
        type Error = String; // Using String as a placeholder for the error type

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            Ok(CustomHeader(ByteStr::from(value)))
        }
    }

    let header_name = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };

    let bytes = header_name.into_bytes();
    assert_eq!(bytes, Bytes::from_static(b"accept")); // Assuming the default Bytes behavior
}

#[test]
fn test_into_bytes_with_custom_header() {
    struct CustomHeader(ByteStr);

    impl CustomHeader {
        fn from_bytes(bytes: &[u8]) -> CustomHeader {
            CustomHeader(ByteStr::from(bytes))
        }
    }

    let custom_header = CustomHeader::from_bytes(b"custom-header");
    
    let header_name = HeaderName {
        inner: Repr::Custom(custom_header),
    };

    let bytes = header_name.into_bytes();
    assert_eq!(bytes, Bytes::from_static(b"custom-header")); // Assuming the default Bytes behavior
}

