// Answer 0

#[test]
fn test_is_empty_with_empty_header_value() {
    struct HeaderValue {
        data: &'static str,
    }
    
    impl HeaderValue {
        fn from_static(data: &'static str) -> Self {
            HeaderValue { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let val = HeaderValue::from_static("");
    assert!(val.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_header_value() {
    struct HeaderValue {
        data: &'static str,
    }
    
    impl HeaderValue {
        fn from_static(data: &'static str) -> Self {
            HeaderValue { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let val = HeaderValue::from_static("hello");
    assert!(!val.is_empty());
}

#[test]
fn test_is_empty_with_whitespace_header_value() {
    struct HeaderValue {
        data: &'static str,
    }
    
    impl HeaderValue {
        fn from_static(data: &'static str) -> Self {
            HeaderValue { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let val = HeaderValue::from_static("   ");
    assert!(!val.is_empty());
}

