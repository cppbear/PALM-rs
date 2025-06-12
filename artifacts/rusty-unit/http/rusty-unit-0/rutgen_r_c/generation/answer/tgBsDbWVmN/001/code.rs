// Answer 0

#[test]
fn test_as_str_custom() {
    struct MyCustomHeader {
        bytes: Bytes,
    }

    impl MyCustomHeader {
        fn new(value: &str) -> Self {
            MyCustomHeader {
                bytes: Bytes::from(value),
            }
        }
    }

    let custom_header = MyCustomHeader::new("my-custom-header");
    let header_name = HeaderName {
        inner: Repr::Custom(Custom(ByteStr { bytes: custom_header.bytes })),
    };
    
    assert_eq!(header_name.as_str(), "my-custom-header");
}

#[test]
fn test_as_str_empty_custom() {
    struct MyCustomHeader {
        bytes: Bytes,
    }

    impl MyCustomHeader {
        fn new(value: &str) -> Self {
            MyCustomHeader {
                bytes: Bytes::from(value),
            }
        }
    }

    let custom_header = MyCustomHeader::new("");
    let header_name = HeaderName {
        inner: Repr::Custom(Custom(ByteStr { bytes: custom_header.bytes })),
    };

    assert_eq!(header_name.as_str(), "");
}

#[test]
fn test_as_str_character_case_custom() {
    struct MyCustomHeader {
        bytes: Bytes,
    }

    impl MyCustomHeader {
        fn new(value: &str) -> Self {
            MyCustomHeader {
                bytes: Bytes::from(value),
            }
        }
    }

    let custom_header = MyCustomHeader::new("My-Custom-Header");
    let header_name = HeaderName {
        inner: Repr::Custom(Custom(ByteStr { bytes: custom_header.bytes })),
    };

    assert_eq!(header_name.as_str(), "My-Custom-Header");
}

