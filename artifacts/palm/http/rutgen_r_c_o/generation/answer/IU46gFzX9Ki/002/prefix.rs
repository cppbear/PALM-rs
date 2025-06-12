// Answer 0

#[test]
fn test_scheme_str_http() {
    let uri = Uri {
        scheme: Scheme {
            inner: Scheme2::Standard(Protocol::Http),
        },
        authority: Authority {
            data: ByteStr {
                bytes: Bytes::from_static(b"example.org"),
            },
        },
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/hello/world"),
            },
            query: 0,
        },
    };
    let _ = uri.scheme_str();
}

#[test]
fn test_scheme_str_https() {
    let uri = Uri {
        scheme: Scheme {
            inner: Scheme2::Standard(Protocol::Https),
        },
        authority: Authority {
            data: ByteStr {
                bytes: Bytes::from_static(b"example.org"),
            },
        },
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/hello/world"),
            },
            query: 0,
        },
    };
    let _ = uri.scheme_str();
}

#[test]
fn test_scheme_str_empty_scheme() {
    let uri = Uri {
        scheme: Scheme::empty(),
        authority: Authority {
            data: ByteStr {
                bytes: Bytes::from_static(b"example.org"),
            },
        },
        path_and_query: PathAndQuery {
            data: ByteStr {
                bytes: Bytes::from_static(b"/hello/world"),
            },
            query: 0,
        },
    };
    let _ = uri.scheme_str();
}

