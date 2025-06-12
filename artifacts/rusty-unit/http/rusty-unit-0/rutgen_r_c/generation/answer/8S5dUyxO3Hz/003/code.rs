// Answer 0

#[test]
fn test_path_and_query_with_standard_scheme_and_non_empty_authority() {
    struct TestScheme {
        inner: Scheme2,
    }

    let test_scheme = TestScheme {
        inner: Scheme2::Standard(Protocol::Http),
    };

    let test_authority = Authority {
        data: ByteStr {
            bytes: Bytes::from("example.com"),
        },
    };

    let test_path_and_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from("/path?query=value"),
        },
        query: 1,
    };

    let uri = Uri {
        scheme: Scheme {
            inner: test_scheme.inner,
        },
        authority: test_authority,
        path_and_query: test_path_and_query,
    };

    assert!(uri.path_and_query().is_some());
}

#[test]
fn test_path_and_query_with_none_scheme_and_non_empty_authority() {
    struct TestScheme {
        inner: Scheme2,
    }

    let test_scheme = TestScheme {
        inner: Scheme2::None,
    };

    let test_authority = Authority {
        data: ByteStr {
            bytes: Bytes::from("example.com"),
        },
    };

    let test_path_and_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from("/path?query=value"),
        },
        query: 1,
    };

    let uri = Uri {
        scheme: Scheme {
            inner: test_scheme.inner,
        },
        authority: test_authority,
        path_and_query: test_path_and_query,
    };

    assert!(uri.path_and_query().is_some());
}

