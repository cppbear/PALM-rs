// Answer 0

#[test]
fn test_key_function() {
    struct HeaderName {
        name: String,
    }

    struct HeaderMap {
        key: HeaderName,
    }

    impl HeaderMap {
        pub fn new() -> Self {
            HeaderMap {
                key: HeaderName {
                    name: String::from("x-hello"),
                },
            }
        }

        pub fn entry(&self, _name: &str) -> &HeaderName {
            &self.key
        }
    }

    let map = HeaderMap::new();
    assert_eq!(map.entry("x-hello").key().name, "x-hello");
}

#[test]
fn test_key_function_empty_key() {
    struct HeaderName {
        name: String,
    }

    struct HeaderMap {
        key: HeaderName,
    }

    impl HeaderMap {
        pub fn new() -> Self {
            HeaderMap {
                key: HeaderName {
                    name: String::from(""),
                },
            }
        }

        pub fn entry(&self, _name: &str) -> &HeaderName {
            &self.key
        }
    }

    let map = HeaderMap::new();
    assert_eq!(map.entry("").key().name, "");
}

#[test]
#[should_panic]
fn test_key_function_invalid_key() {
    struct HeaderName {
        name: String,
    }

    struct HeaderMap {
        key: HeaderName,
    }

    impl HeaderMap {
        pub fn new() -> Self {
            HeaderMap {
                key: HeaderName {
                    name: String::from("x-hello"),
                },
            }
        }

        pub fn entry(&self, _name: &str) -> &HeaderName {
            panic!("Invalid key access!");
        }
    }

    let map = HeaderMap::new();
    let _ = map.entry("invalid-key").key(); // This should trigger a panic
}

