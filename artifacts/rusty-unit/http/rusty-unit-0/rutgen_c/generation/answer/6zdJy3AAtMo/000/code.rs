// Answer 0

#[test]
fn test_into_parts_with_scheme_and_authority() {
    struct TestUri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    let test_uri = TestUri {
        scheme: Scheme { inner: Scheme2::Http },
        authority: Authority { data: ByteStr::from_static(b"example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b"/foo"), query: 0 },
    };

    let parts = test_uri.into_parts();

    assert_eq!(parts.path_and_query.unwrap().data, ByteStr::from_static(b"/foo"));
    assert!(parts.scheme.is_some());
    assert!(parts.authority.is_some());
}

#[test]
fn test_into_parts_without_scheme_and_authority() {
    let test_uri = Uri {
        scheme: Scheme { inner: Scheme2::None },
        authority: Authority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b"/foo"), query: 0 },
    };

    let parts = test_uri.into_parts();

    assert_eq!(parts.path_and_query.unwrap().data, ByteStr::from_static(b"/foo"));
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
} 

#[test]
fn test_into_parts_empty_path() {
    let test_uri = Uri {
        scheme: Scheme { inner: Scheme2::None },
        authority: Authority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b""), query: 0 },
    };

    let parts = test_uri.into_parts();

    assert_eq!(parts.path_and_query.unwrap().data, ByteStr::from_static(b""));
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
} 

#[test]
fn test_into_parts_with_query() {
    let test_uri = Uri {
        scheme: Scheme { inner: Scheme2::None },
        authority: Authority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b"/foo"), query: 1 },
    };

    let parts = test_uri.into_parts();

    assert_eq!(parts.path_and_query.unwrap().data, ByteStr::from_static(b"/foo"));
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
}

