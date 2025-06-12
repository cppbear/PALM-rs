// Answer 0

#[test]
fn test_uri_ref_default() {
    struct Inner {
        uri: Uri,
    }

    struct Request {
        inner: Result<Inner, ()>,
    }

    impl Request {
        fn builder() -> Self {
            Request {
                inner: Ok(Inner {
                    uri: Uri::from_static("/"),
                }),
            }
        }

        fn uri_ref(&self) -> Option<&Uri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }
        
        fn uri(mut self, uri: &str) -> Self {
            self.inner = Ok(Inner {
                uri: Uri::from_static(uri),
            });
            self
        }
    }

    struct Uri {
        value: &'static str,
    }

    impl Uri {
        fn from_static(value: &'static str) -> Self {
            Uri { value }
        }
    }

    let mut req = Request::builder();
    assert_eq!(req.uri_ref().unwrap().value, "/");
}

#[test]
fn test_uri_ref_custom_uri() {
    struct Inner {
        uri: Uri,
    }

    struct Request {
        inner: Result<Inner, ()>,
    }

    impl Request {
        fn builder() -> Self {
            Request {
                inner: Ok(Inner {
                    uri: Uri::from_static("/"),
                }),
            }
        }

        fn uri_ref(&self) -> Option<&Uri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }
        
        fn uri(mut self, uri: &str) -> Self {
            self.inner = Ok(Inner {
                uri: Uri::from_static(uri),
            });
            self
        }
    }

    struct Uri {
        value: &'static str,
    }

    impl Uri {
        fn from_static(value: &'static str) -> Self {
            Uri { value }
        }
    }

    let mut req = Request::builder();
    req = req.uri("https://www.rust-lang.org/");
    assert_eq!(req.uri_ref().unwrap().value, "https://www.rust-lang.org/");
}

#[test]
fn test_uri_ref_empty_uri() {
    struct Inner {
        uri: Uri,
    }

    struct Request {
        inner: Result<Inner, ()>,
    }

    impl Request {
        fn builder() -> Self {
            Request {
                inner: Ok(Inner {
                    uri: Uri::from_static(""),
                }),
            }
        }

        fn uri_ref(&self) -> Option<&Uri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }
        
        fn uri(mut self, uri: &str) -> Self {
            self.inner = Ok(Inner {
                uri: Uri::from_static(uri),
            });
            self
        }
    }

    struct Uri {
        value: &'static str,
    }

    impl Uri {
        fn from_static(value: &'static str) -> Self {
            Uri { value }
        }
    }

    let req = Request::builder();
    assert_eq!(req.uri_ref().unwrap().value, "");
}

