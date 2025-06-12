// Answer 0

#[test]
fn test_parts_new() {
    struct Parts {
        method: Method,
        uri: Uri,
        version: Version,
        headers: HeaderMap,
        extensions: Extensions,
        _priv: (),
    }

    impl Default for Parts {
        fn default() -> Self {
            Parts {
                method: Method::default(),
                uri: Uri::default(),
                version: Version::default(),
                headers: HeaderMap::default(),
                extensions: Extensions::default(),
                _priv: (),
            }
        }
    }

    struct Method;
    impl Default for Method {
        fn default() -> Self {
            Method
        }
    }

    struct Uri;
    impl Default for Uri {
        fn default() -> Self {
            Uri
        }
    }

    struct Version;
    impl Default for Version {
        fn default() -> Self {
            Version
        }
    }

    struct HeaderMap;
    impl Default for HeaderMap {
        fn default() -> Self {
            HeaderMap
        }
    }

    struct Extensions;
    impl Default for Extensions {
        fn default() -> Self {
            Extensions
        }
    }

    let parts = Parts::default();

    assert_eq!(parts.method, Method::default());
    assert_eq!(parts.uri, Uri::default());
    assert_eq!(parts.version, Version::default());
    assert_eq!(parts.headers, HeaderMap::default());
    assert_eq!(parts.extensions, Extensions::default());
}

