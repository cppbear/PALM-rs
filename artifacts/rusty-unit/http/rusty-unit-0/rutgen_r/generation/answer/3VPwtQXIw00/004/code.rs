// Answer 0

#[test]
fn test_header_name_valid_case() {
    struct CustomHeaderName {
        inner: Repr,
    }

    enum Repr {
        Standard(StandardHeader),
        Custom(ByteStr),
    }

    struct StandardHeader;

    impl StandardHeader {
        fn from_bytes(_: &[u8]) -> Option<Self> {
            Some(StandardHeader)
        }
    }

    struct ByteStr;

    impl ByteStr {
        fn from_static(_: &'static str) -> Self {
            ByteStr
        }
    }

    const MAX_HEADER_NAME_LEN: usize = 128; // assuming some max length for testing
    const HEADER_CHARS_H2: [u8; 256] = [0; 256]; // assuming valid header chars initialized

    let valid_header_name = "valid-header-name-to-test"; // Length <= MAX_HEADER_NAME_LEN
    let hdr = from_static(valid_header_name);
    assert_eq!(hdr.inner, Repr::Custom(ByteStr::from_static(valid_header_name)));
}

#[test]
#[should_panic]
fn test_header_name_empty() {
    from_static(""); // Should panic due to being empty
}

#[test]
#[should_panic]
fn test_header_name_too_long() {
    let long_header_name = "a".repeat(129).as_str(); // Length exceeds MAX_HEADER_NAME_LEN
    from_static(long_header_name); // Should panic due to too long
}

#[test]
#[should_panic]
fn test_header_name_invalid_chars() {
    from_static("invalid{}header"); // Should panic due to invalid characters
}

#[test]
#[should_panic]
fn test_header_name_uppercase() {
    from_static("InvalidHeader"); // Should panic due to uppercase characters
}

