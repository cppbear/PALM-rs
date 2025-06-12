// Answer 0

#[test]
fn test_header_name_display_standard() {
    #[derive(Debug, Clone, Copy)]
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

    let mut output = String::new();
    let result = header_name.fmt(&mut output).map(|_| output);
    assert_eq!(result.unwrap(), "accept");
}

#[test]
fn test_header_name_display_custom() {
    let header_name = HeaderName {
        inner: Repr::Custom(Custom(ByteStr::from("custom-header".as_bytes()))),
    };

    let mut output = String::new();
    let result = header_name.fmt(&mut output).map(|_| output);
    assert_eq!(result.unwrap(), "custom-header");
}

