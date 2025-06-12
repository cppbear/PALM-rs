// Answer 0

#[test]
fn test_authority_empty_data() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: Authority { data: ByteStr { bytes: Bytes::new() } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::new() }, query: 0 },
    };
    let result = uri.authority();
}

#[test]
fn test_authority_empty_data_large_uri() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path") }, query: 0 },
    };
    let result = uri.authority();
}

#[test]
fn test_authority_empty_data_full_uri() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: Authority { data: ByteStr { bytes: Bytes::new() } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/some/path") }, query: 1 },
    };
    let result = uri.authority();
}

