// Answer 0

#[test]
fn test_scheme_str_with_http() {
    struct Uri {
        scheme: Scheme,
    }

    struct Scheme {
        inner: Option<String>,
    }

    impl Uri {
        pub fn scheme_str(&self) -> Option<&str> {
            if self.scheme.inner.is_none() {
                None
            } else {
                Some(self.scheme.inner.as_ref().unwrap().as_str())
            }
        }
    }

    let uri = Uri {
        scheme: Scheme {
            inner: Some("http".to_string()),
        },
    };

    assert_eq!(uri.scheme_str(), Some("http"));
}

#[test]
fn test_scheme_str_with_none() {
    struct Uri {
        scheme: Scheme,
    }

    struct Scheme {
        inner: Option<String>,
    }

    impl Uri {
        pub fn scheme_str(&self) -> Option<&str> {
            if self.scheme.inner.is_none() {
                None
            } else {
                Some(self.scheme.inner.as_ref().unwrap().as_str())
            }
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: None },
    };

    assert_eq!(uri.scheme_str(), None);
}

