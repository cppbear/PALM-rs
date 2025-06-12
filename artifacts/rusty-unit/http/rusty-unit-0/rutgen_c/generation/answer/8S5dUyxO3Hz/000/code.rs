// Answer 0

#[test]
fn test_path_and_query_with_scheme() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::Standard(Protocol) },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path?query") }, query: 0 },
    };
    assert!(uri.path_and_query().is_some());
}

#[test]
fn test_path_and_query_with_empty_authority() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::None },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path") }, query: 0 },
    };
    assert!(uri.path_and_query().is_some());
}

#[test]
fn test_path_and_query_with_no_scheme_and_non_empty_authority() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::None },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path") }, query: 0 },
    };
    assert!(uri.path_and_query().is_some());
}

#[test]
fn test_path_and_query_with_no_scheme_and_empty_authority() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::None },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"") }, query: 0 },
    };
    assert!(uri.path_and_query().is_none());
}

