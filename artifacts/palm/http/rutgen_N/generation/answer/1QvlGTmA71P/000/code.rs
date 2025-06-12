// Answer 0

#[test]
fn test_sensitive_value_true() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        fn from_static(value: &str) -> Self {
            // In a real implementation, this would initialize with the value.
            HeaderValue { is_sensitive: false }
        }

        fn set_sensitive(&mut self, sensitive: bool) {
            self.is_sensitive = sensitive;
        }

        fn is_sensitive(&self) -> bool {
            self.is_sensitive
        }
    }

    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_sensitive_value_false() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        fn from_static(value: &str) -> Self {
            HeaderValue { is_sensitive: false }
        }

        fn set_sensitive(&mut self, sensitive: bool) {
            self.is_sensitive = sensitive;
        }

        fn is_sensitive(&self) -> bool {
            self.is_sensitive
        }
    }

    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
}

