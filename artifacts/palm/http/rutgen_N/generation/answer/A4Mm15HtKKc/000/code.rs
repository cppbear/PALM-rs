// Answer 0

#[test]
fn test_len_with_static_string() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue { value }
        }

        fn len(&self) -> usize {
            self.value.len()
        }
    }

    let val = HeaderValue::from_static("hello");
    assert_eq!(val.len(), 5);
}

#[test]
fn test_len_with_empty_string() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue { value }
        }

        fn len(&self) -> usize {
            self.value.len()
        }
    }

    let val = HeaderValue::from_static("");
    assert_eq!(val.len(), 0);
}

#[test]
fn test_len_with_special_characters() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(value: &'static str) -> Self {
            HeaderValue { value }
        }

        fn len(&self) -> usize {
            self.value.len()
        }
    }

    let val = HeaderValue::from_static("!@#$%^&*()");
    assert_eq!(val.len(), 10);
}

