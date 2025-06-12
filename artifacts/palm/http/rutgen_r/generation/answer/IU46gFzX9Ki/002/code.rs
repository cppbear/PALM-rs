// Answer 0

#[test]
fn test_scheme_str_valid_scheme() {
    struct Inner {
        inner: Option<String>,
    }

    struct Scheme {
        inner: Inner,
        scheme: String,
    }

    struct Uri {
        scheme: Scheme,
    }

    impl Uri {
        fn scheme_str(&self) -> Option<&str> {
            if self.scheme.inner.inner.is_none() {
                None
            } else {
                Some(&self.scheme.scheme)
            }
        }
    }

    let uri = Uri {
        scheme: Scheme {
            inner: Inner {
                inner: Some("valid".to_string()),
            },
            scheme: "http".to_string(),
        },
    };

    assert_eq!(uri.scheme_str(), Some("http"));
}

#[test]
fn test_scheme_str_non_empty_option() {
    struct Inner {
        inner: Option<String>,
    }

    struct Scheme {
        inner: Inner,
        scheme: String,
    }

    struct Uri {
        scheme: Scheme,
    }

    impl Uri {
        fn scheme_str(&self) -> Option<&str> {
            if self.scheme.inner.inner.is_none() {
                None
            } else {
                Some(&self.scheme.scheme)
            }
        }
    }

    let uri = Uri {
        scheme: Scheme {
            inner: Inner {
                inner: Some("another_valid".to_string()),
            },
            scheme: "https".to_string(),
        },
    };

    assert_eq!(uri.scheme_str(), Some("https"));
}

