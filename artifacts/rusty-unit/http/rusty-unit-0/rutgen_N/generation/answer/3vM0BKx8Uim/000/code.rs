// Answer 0

#[test]
fn test_uri_ref_default() {
    struct Request {
        inner: Result<Inner, ()>,
    }

    struct Inner {
        uri: Uri,
    }

    struct Uri {
        value: String,
    }

    impl Request {
        fn builder() -> Self {
            Request {
                inner: Ok(Inner {
                    uri: Uri {
                        value: "/".to_string(),
                    },
                }),
            }
        }

        pub fn uri_ref(&self) -> Option<&Uri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }

        pub fn uri(&mut self, uri: &str) -> &mut Self {
            if let Ok(inner) = &mut self.inner {
                inner.uri = Uri {
                    value: uri.to_string(),
                };
            }
            self
        }
    }

    let mut req = Request::builder();
    assert_eq!(req.uri_ref().unwrap().value, "/");

    req = req.uri("https://www.rust-lang.org/");
    assert_eq!(req.uri_ref().unwrap().value, "https://www.rust-lang.org/");
}

