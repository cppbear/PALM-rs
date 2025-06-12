// Answer 0

#[test]
fn test_scheme_some() {
    struct MockScheme {
        inner: Scheme2,
    }

    let scheme = MockScheme { inner: Scheme2::Standard(Protocol) };
    let uri = Uri {
        scheme: scheme,
        authority: Authority {
            data: ByteStr { bytes: Bytes::from_static(b"example.com") },
        },
        path_and_query: PathAndQuery {
            data: ByteStr { bytes: Bytes::from_static(b"/hello/world") },
            query: 0,
        },
    };

    assert!(uri.scheme().is_some());
}

#[test]
fn test_scheme_none() {
    struct MockScheme {
        inner: Scheme2,
    }

    let scheme = MockScheme { inner: Scheme2::None };
    let uri = Uri {
        scheme: scheme,
        authority: Authority {
            data: ByteStr { bytes: Bytes::from_static(b"example.com") },
        },
        path_and_query: PathAndQuery {
            data: ByteStr { bytes: Bytes::from_static(b"/hello/world") },
            query: 0,
        },
    };

    assert!(uri.scheme().is_none());
}

