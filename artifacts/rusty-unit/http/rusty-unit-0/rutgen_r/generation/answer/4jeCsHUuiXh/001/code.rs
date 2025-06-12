// Answer 0

#[test]
fn test_new_parts() {
    struct Method {
        // Assuming Method has some fields
    }

    impl Default for Method {
        fn default() -> Self {
            Method {
                // Initialize fields as necessary
            }
        }
    }

    struct Uri {
        // Assuming Uri has some fields
    }

    impl Default for Uri {
        fn default() -> Self {
            Uri {
                // Initialize fields as necessary
            }
        }
    }

    struct Version {
        // Assuming Version has some fields
    }

    impl Default for Version {
        fn default() -> Self {
            Version {
                // Initialize fields as necessary
            }
        }
    }

    struct HeaderMap {
        // Assuming HeaderMap has some fields
    }

    impl Default for HeaderMap {
        fn default() -> Self {
            HeaderMap {
                // Initialize fields as necessary
            }
        }
    }

    struct Extensions {
        // Assuming Extensions has some fields
    }

    impl Default for Extensions {
        fn default() -> Self {
            Extensions {
                // Initialize fields as necessary
            }
        }
    }

    struct Parts {
        method: Method,
        uri: Uri,
        version: Version,
        headers: HeaderMap,
        extensions: Extensions,
        _priv: (),
    }

    fn new() -> Parts {
        Parts {
            method: Method::default(),
            uri: Uri::default(),
            version: Version::default(),
            headers: HeaderMap::default(),
            extensions: Extensions::default(),
            _priv: (),
        }
    }

    let parts = new();
    assert_eq!(parts.method, Method::default());
    assert_eq!(parts.uri, Uri::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

