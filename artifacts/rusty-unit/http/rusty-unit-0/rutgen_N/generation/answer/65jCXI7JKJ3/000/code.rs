// Answer 0

#[test]
fn test_header_value_is_empty() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(v: &'static str) -> Self {
            HeaderValue { value: v }
        }

        fn len(&self) -> usize {
            self.value.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let val_empty = HeaderValue::from_static("");
    assert!(val_empty.is_empty());

    let val_non_empty = HeaderValue::from_static("hello");
    assert!(!val_non_empty.is_empty());
}

#[test]
fn test_header_value_is_empty_boundary() {
    struct HeaderValue {
        value: &'static str,
    }

    impl HeaderValue {
        fn from_static(v: &'static str) -> Self {
            HeaderValue { value: v }
        }

        fn len(&self) -> usize {
            self.value.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let val_empty = HeaderValue::from_static("");
    assert!(val_empty.is_empty());

    let val_non_empty = HeaderValue::from_static("A");
    assert!(!val_non_empty.is_empty());
    
    let val_non_empty_large = HeaderValue::from_static("This is a test string");
    assert!(!val_non_empty_large.is_empty());
}

