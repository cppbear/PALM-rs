// Answer 0

#[test]
fn test_as_str_standard() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Custom(ByteStr);
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum Repr<T> {
        Standard(StandardHeader),
        Custom(T),
    }

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
    
    #[derive(Clone, Eq, PartialEq, Hash)]
    struct HeaderName {
        inner: Repr<Custom>,
    }

    let header_name = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };

    assert_eq!(header_name.as_str(), "accept");
}

#[test]
fn test_as_str_custom() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Custom(ByteStr);

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct ByteStr {
        bytes: Bytes,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum Repr<T> {
        Standard(StandardHeader),
        Custom(T),
    }

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

    #[derive(Clone, Eq, PartialEq, Hash)]
    struct HeaderName {
        inner: Repr<Custom>,
    }

    let custom_str = ByteStr {
        bytes: Bytes::from_static(b"custom-header"),
    };
    
    let header_name = HeaderName {
        inner: Repr::Custom(Custom(custom_str)),
    };

    assert_eq!(header_name.as_str(), "custom-header");
}

