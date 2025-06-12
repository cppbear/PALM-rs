// Answer 0

#[test]
fn test_parts_new() {
    struct Parts {
        status: StatusCode,
        version: Version,
        headers: HeaderMap,
        extensions: Extensions,
        _priv: (),
    }

    struct StatusCode; // Assume this is a struct.
    impl Default for StatusCode {
        fn default() -> Self {
            StatusCode {}
        }
    }

    struct Version; // Assume this is a struct.
    impl Default for Version {
        fn default() -> Self {
            Version {}
        }
    }

    struct HeaderMap; // Assume this is a struct.
    impl Default for HeaderMap {
        fn default() -> Self {
            HeaderMap {}
        }
    }

    struct Extensions; // Assume this is a struct.
    impl Default for Extensions {
        fn default() -> Self {
            Extensions {}
        }
    }

    let parts = Parts {
        status: StatusCode::default(),
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };

    assert_eq!(parts.status, StatusCode::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

