// Answer 0

#[test]
fn test_scheme_str_http() {
    let uri = Uri {
        scheme: Scheme::HTTP,
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.org") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 },
    };
    assert_eq!(uri.scheme_str(), Some("http"));
}

#[test]
fn test_scheme_str_https() {
    let uri = Uri {
        scheme: Scheme::HTTPS,
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.org") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 },
    };
    assert_eq!(uri.scheme_str(), Some("https"));
}

#[test]
fn test_scheme_str_none() {
    let uri = Uri {
        scheme: Scheme::empty(),
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.org") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 },
    };
    assert_eq!(uri.scheme_str(), None);
}

