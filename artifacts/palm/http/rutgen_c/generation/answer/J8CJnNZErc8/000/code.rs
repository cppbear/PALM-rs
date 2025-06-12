// Answer 0

#[test]
fn test_borrow_standard_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum StandardHeader {
        Accept,
        Host,
    }
    
    #[derive(Clone, Eq, PartialEq, Hash)]
    struct HeaderName {
        inner: Repr<Custom>,
    }
    
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Custom(ByteStr);
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum Repr<T> {
        Standard(StandardHeader),
        Custom(T),
    }
    
    impl StandardHeader {
        fn as_str(&self) -> &'static str {
            match *self {
                StandardHeader::Accept => "accept",
                StandardHeader::Host => "host",
            }
        }
    }
    
    impl HeaderName {
        #[inline]
        pub fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(v) => v.as_str(),
                Repr::Custom(ref v) => &v.0,
            }
        }
    }

    let standard_header = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };
    assert_eq!(standard_header.borrow(), "accept");
}

#[test]
fn test_borrow_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Custom(ByteStr);

    #[derive(Clone, Eq, PartialEq, Hash)]
    struct HeaderName {
        inner: Repr<Custom>,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    enum Repr<T> {
        Standard(StandardHeader),
        Custom(T),
    }

    impl HeaderName {
        #[inline]
        pub fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(v) => v.as_str(),
                Repr::Custom(ref v) => &v.0,
            }
        }
    }

    let custom_header = HeaderName {
        inner: Repr::Custom(Custom(ByteStr::from("X-Custom-Header"))),
    };
    assert_eq!(custom_header.borrow(), "X-Custom-Header");
}

