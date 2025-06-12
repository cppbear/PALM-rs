// Answer 0

#[test]
fn test_header_name_debug_fmt_standard() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum TestHeader {
        Accept,
    }

    impl TestHeader {
        fn as_str(&self) -> &'static str {
            match *self {
                TestHeader::Accept => "accept",
            }
        }
    }

    let header_name = HeaderName {
        inner: Repr::Standard(TestHeader::Accept),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", header_name);
    assert!(result.is_ok());
    assert_eq!(buffer, "accept");
}

#[test]
fn test_header_name_debug_fmt_custom() {
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct CustomHeader(ByteStr);

    let custom_header = CustomHeader(ByteStr::from("custom-header"));
    let header_name = HeaderName {
        inner: Repr::Custom(custom_header.clone()),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", header_name);
    assert!(result.is_ok());
    assert_eq!(buffer, "custom-header");
}

