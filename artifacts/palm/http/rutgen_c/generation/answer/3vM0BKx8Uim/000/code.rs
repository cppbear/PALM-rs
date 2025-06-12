// Answer 0

#[test]
fn test_uri_ref_default() {
    struct MockUri {
        scheme: Option<Scheme>,
        authority: Option<Authority>,
        path_and_query: Option<PathAndQuery>,
    }

    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            let parts = Parts {
                method: Method::GET,
                uri: MockUri {
                    scheme: None,
                    authority: None,
                    path_and_query: Some(PathAndQuery::new("/")),
                },
                version: Version::HTTP_11,
                headers: HeaderMap::new(),
                extensions: Extensions::new(),
                _priv: (),
            };
            MockBuilder {
                inner: Ok(parts),
            }
        }

        fn uri_ref(&self) -> Option<&MockUri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }
    }

    let req = MockBuilder::new();
    assert_eq!(req.uri_ref().unwrap().path_and_query.as_ref().unwrap(), "/");
}

#[test]
fn test_uri_ref_custom_uri() {
    struct MockUri {
        scheme: Option<Scheme>,
        authority: Option<Authority>,
        path_and_query: Option<PathAndQuery>,
    }

    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            let parts = Parts {
                method: Method::GET,
                uri: MockUri {
                    scheme: Some(Scheme::HTTP),
                    authority: Some(Authority::new("www.rust-lang.org")),
                    path_and_query: Some(PathAndQuery::new("/")),
                },
                version: Version::HTTP_11,
                headers: HeaderMap::new(),
                extensions: Extensions::new(),
                _priv: (),
            };
            MockBuilder {
                inner: Ok(parts),
            }
        }

        fn uri_ref(&self) -> Option<&MockUri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }
    }

    let req = MockBuilder::new();
    assert_eq!(req.uri_ref().unwrap().authority.as_ref().unwrap(), "www.rust-lang.org");
}

#[test]
#[should_panic]
fn test_uri_ref_error_handling() {
    struct MockBuilder {
        inner: Result<Parts>,
    }

    impl MockBuilder {
        fn new() -> Self {
            MockBuilder {
                inner: Err(Error { inner: ErrorKind::BadRequest }),
            }
        }

        fn uri_ref(&self) -> Option<&Uri> {
            self.inner.as_ref().ok().map(|h| &h.uri)
        }
    }

    let req = MockBuilder::new();
    req.uri_ref().unwrap(); // This should panic as inner is an error
}

