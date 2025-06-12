// Answer 0

#[test]
fn test_uri_path_empty_string() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: Authority { data: ByteStr::new() },
        path_and_query: PathAndQuery::empty(),
    };
    let _ = uri.path();
}

#[test]
fn test_uri_path_no_path() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::default() },
        authority: Authority { data: ByteStr::new() },
        path_and_query: PathAndQuery {
            data: ByteStr::new(),
            query: 0,
        },
    };
    let _ = uri.path();
}

