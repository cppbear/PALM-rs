// Answer 0

#[test]
fn test_uri_path_empty_uri() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::Http },
        authority: Authority { data: ByteStr::new() },
        path_and_query: PathAndQuery::empty(),
    };
    assert_eq!(uri.path(), "");
}

#[test]
fn test_uri_path_no_path() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::Http },
        authority: Authority { data: ByteStr::new() },
        path_and_query: PathAndQuery::from_static(""),
    };
    assert_eq!(uri.path(), "");
}

#[test]
fn test_uri_path_only_query() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::Http },
        authority: Authority { data: ByteStr::new() },
        path_and_query: PathAndQuery::from_static("?key=value"),
    };
    assert_eq!(uri.path(), "");
}

#[test]
fn test_uri_path_slash_only() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::Http },
        authority: Authority { data: ByteStr::new() },
        path_and_query: PathAndQuery::slash(),
    };
    assert_eq!(uri.path(), "/");
}

