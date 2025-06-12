// Answer 0

#[test]
fn test_set_sensitive_true() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        pub fn from_static(_: &str) -> Self {
            HeaderValue { is_sensitive: false }
        }

        pub fn set_sensitive(&mut self, val: bool) {
            self.is_sensitive = val;
        }

        pub fn is_sensitive(&self) -> bool {
            self.is_sensitive
        }
    }

    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(true);
    assert!(val.is_sensitive());
}

#[test]
fn test_set_sensitive_false() {
    struct HeaderValue {
        is_sensitive: bool,
    }

    impl HeaderValue {
        pub fn from_static(_: &str) -> Self {
            HeaderValue { is_sensitive: false }
        }

        pub fn set_sensitive(&mut self, val: bool) {
            self.is_sensitive = val;
        }

        pub fn is_sensitive(&self) -> bool {
            self.is_sensitive
        }
    }

    let mut val = HeaderValue::from_static("my secret");
    val.set_sensitive(false);
    assert!(!val.is_sensitive());
}

