// Answer 0

#[test]
fn test_header_name_display_standard_header() {
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct TestByteStr(Vec<u8>);

    impl Borrow<[u8]> for TestByteStr {
        fn borrow(&self) -> &[u8] {
            &self.0
        }
    }

    #[allow(dead_code)]
    struct TestHeaderName {
        inner: Repr<Custom>,
    }

    impl TestHeaderName {
        fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(header) => header.as_str(),
                Repr::Custom(custom) => std::str::from_utf8(&custom.0).unwrap(),
            }
        }
    }

    let header = TestHeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };

    let fmt = &mut std::fmt::Formatter::default();
    assert!(header.fmt(fmt).is_ok());
}

#[test]
fn test_header_name_display_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct TestByteStr(Vec<u8>);

    impl Borrow<[u8]> for TestByteStr {
        fn borrow(&self) -> &[u8] {
            &self.0
        }
    }

    #[allow(dead_code)]
    struct TestHeaderName {
        inner: Repr<Custom>,
    }

    impl TestHeaderName {
        fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(header) => header.as_str(),
                Repr::Custom(custom) => std::str::from_utf8(&custom.0).unwrap(),
            }
        }
    }

    let custom_bytes = TestByteStr(b"custom-header-name".to_vec());
    let header = TestHeaderName {
        inner: Repr::Custom(custom_bytes),
    };

    let fmt = &mut std::fmt::Formatter::default();
    assert!(header.fmt(fmt).is_ok());
}

#[test]
#[should_panic]
fn test_header_name_display_invalid_utf8_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct TestByteStr(Vec<u8>);

    impl Borrow<[u8]> for TestByteStr {
        fn borrow(&self) -> &[u8] {
            &self.0
        }
    }

    #[allow(dead_code)]
    struct TestHeaderName {
        inner: Repr<Custom>,
    }

    impl TestHeaderName {
        fn as_str(&self) -> &str {
            match &self.inner {
                Repr::Standard(header) => header.as_str(),
                Repr::Custom(custom) => std::str::from_utf8(&custom.0).unwrap(),
            }
        }
    }

    let invalid_utf8_bytes = TestByteStr(vec![0, 159, 146, 150]);
    let header = TestHeaderName {
        inner: Repr::Custom(invalid_utf8_bytes),
    };

    let fmt = &mut std::fmt::Formatter::default();
    header.fmt(fmt);
}

