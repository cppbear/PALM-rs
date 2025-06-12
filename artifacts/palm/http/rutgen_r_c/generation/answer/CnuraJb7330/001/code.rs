// Answer 0

#[test]
fn test_scheme_none() {
    struct DummyScheme2;
    let scheme = Scheme {
        inner: Scheme2::None,
    };
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"example.com"),
        },
    };
    let path_and_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"/path/data"),
        },
        query: 0,
    };
    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };
    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_with_other() {
    struct DummyScheme2;
    let scheme = Scheme {
        inner: Scheme2::Other(Box::new(ByteStr {
            bytes: Bytes::from_static(b"custom"),
        })),
    };
    let authority = Authority {
        data: ByteStr {
            bytes: Bytes::from_static(b"example.com"),
        },
    };
    let path_and_query = PathAndQuery {
        data: ByteStr {
            bytes: Bytes::from_static(b"/path/data"),
        },
        query: 0,
    };
    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };
    assert!(uri.scheme().is_some());
}

