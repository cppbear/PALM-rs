// Answer 0

#[test]
fn test_as_bytes_static() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue { value }
        }

        fn as_bytes(&self) -> &[u8] {
            self.value.as_bytes()
        }
    }

    let val = HeaderValue::from_static("hello");
    assert_eq!(val.as_bytes(), b"hello");
}

#[test]
fn test_as_bytes_empty() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue { value }
        }

        fn as_bytes(&self) -> &[u8] {
            self.value.as_bytes()
        }
    }

    let val = HeaderValue::from_static("");
    assert_eq!(val.as_bytes(), b"");
}

#[test]
fn test_as_bytes_special_characters() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue { value }
        }

        fn as_bytes(&self) -> &[u8] {
            self.value.as_bytes()
        }
    }

    let val = HeaderValue::from_static("hello\nworld");
    assert_eq!(val.as_bytes(), b"hello\nworld");
}

