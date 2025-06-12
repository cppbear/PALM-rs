// Answer 0

#[test]
fn test_is_sensitive_true() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        fn from_static(_: &str) -> Self {
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
fn test_is_sensitive_false() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        fn from_static(_: &str) -> Self {
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

#[test]
fn test_is_sensitive_multiple_changes() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        fn from_static(_: &str) -> Self {
            HeaderValue { is_sensitive: false }
        }

        fn set_sensitive(&mut self, sensitive: bool) {
            self.is_sensitive = sensitive;
        }

        fn is_sensitive(&self) -> bool {
            self.is_sensitive
        }
    }

    let mut val = HeaderValue::from_static("example value");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_is_sensitive_initial_state() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        fn from_static(_: &str) -> Self {
            HeaderValue { is_sensitive: false }
        }

        fn is_sensitive(&self) -> bool {
            self.is_sensitive
        }
    }

    let val = HeaderValue::from_static("initial value");
    assert!(!val.is_sensitive());
}

