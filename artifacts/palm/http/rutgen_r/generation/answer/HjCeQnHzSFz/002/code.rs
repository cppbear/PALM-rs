// Answer 0

#[test]
fn test_build_success() {
    struct Builder {
        parts: Option<UriParts>,
    }

    impl Builder {
        fn new() -> Self {
            Builder { parts: None }
        }

        fn parts(mut self, parts: UriParts) -> Self {
            self.parts = Some(parts);
            self
        }

        fn build(self) -> Result<Uri, crate::Error> {
            let parts = self.parts?;
            Uri::from_parts(parts).map_err(Into::into)
        }
    }

    struct UriParts {
        // Dummy fields for the parts, replace with actual ones if available
        scheme: String,
        path: String,
    }

    struct Uri {
        // Dummy struct for Uri, replace with actual implementation if available
        uri_string: String,
    }

    impl Uri {
        fn from_parts(parts: UriParts) -> Result<Self, crate::Error> {
            // Dummy implementation, replace with actual logic for constructing Uri
            if parts.scheme.is_empty() || parts.path.is_empty() {
                Err(crate::Error::InvalidUri)
            } else {
                Ok(Uri {
                    uri_string: format!("{}://{}", parts.scheme, parts.path),
                })
            }
        }
    }

    let valid_parts = UriParts {
        scheme: "http".into(),
        path: "example.com".into(),
    };

    let uri = Builder::new()
        .parts(valid_parts)
        .build()
        .expect("Failed to build URI");

    assert_eq!(uri.uri_string, "http://example.com");
}

#[test]
#[should_panic]
fn test_build_with_no_parts() {
    struct Builder {
        parts: Option<UriParts>,
    }

    impl Builder {
        fn new() -> Self {
            Builder { parts: None }
        }

        fn build(self) -> Result<Uri, crate::Error> {
            let parts = self.parts?;
            Uri::from_parts(parts).map_err(Into::into)
        }
    }

    struct UriParts {
        scheme: String,
        path: String,
    }

    struct Uri {
        uri_string: String,
    }

    impl Uri {
        fn from_parts(parts: UriParts) -> Result<Self, crate::Error> {
            if parts.scheme.is_empty() || parts.path.is_empty() {
                Err(crate::Error::InvalidUri)
            } else {
                Ok(Uri {
                    uri_string: format!("{}://{}", parts.scheme, parts.path),
                })
            }
        }
    }

    let uri = Builder::new().build().expect("Expected to panic but did not");
}

#[test]
#[should_panic]
fn test_build_with_empty_scheme() {
    struct Builder {
        parts: Option<UriParts>,
    }

    impl Builder {
        fn new() -> Self {
            Builder { parts: None }
        }

        fn parts(mut self, parts: UriParts) -> Self {
            self.parts = Some(parts);
            self
        }

        fn build(self) -> Result<Uri, crate::Error> {
            let parts = self.parts?;
            Uri::from_parts(parts).map_err(Into::into)
        }
    }

    struct UriParts {
        scheme: String,
        path: String,
    }

    struct Uri {
        uri_string: String,
    }

    impl Uri {
        fn from_parts(parts: UriParts) -> Result<Self, crate::Error> {
            if parts.scheme.is_empty() || parts.path.is_empty() {
                Err(crate::Error::InvalidUri)
            } else {
                Ok(Uri {
                    uri_string: format!("{}://{}", parts.scheme, parts.path),
                })
            }
        }
    }

    let invalid_parts = UriParts {
        scheme: "".into(),
        path: "example.com".into(),
    };

    let uri = Builder::new()
        .parts(invalid_parts)
        .build()
        .expect("Expected to panic but did not");
}

