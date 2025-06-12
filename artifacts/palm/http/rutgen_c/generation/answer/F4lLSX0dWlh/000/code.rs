// Answer 0

#[test]
fn test_uri_display_with_scheme_authority_path_and_query() {
    struct TestScheme;
    impl fmt::Display for TestScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "http")
        }
    }

    struct TestAuthority;
    impl fmt::Display for TestAuthority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "example.com")
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2 {} },
        authority: Authority { data: ByteStr::from_static(b"example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b"/path"), query: 1 },
    };

    let mut output = String::new();
    assert!(uri.fmt(&mut output).is_ok());
    assert_eq!(output, "http://example.com/path?1");
}

#[test]
fn test_uri_display_with_path_only() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2 {} },
        authority: Authority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b"/path"), query: 0 },
    };

    let mut output = String::new();
    assert!(uri.fmt(&mut output).is_ok());
    assert_eq!(output, "/path");
}

#[test]
fn test_uri_display_with_scheme_and_authority() {
    struct TestScheme;
    impl fmt::Display for TestScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ftp")
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: Scheme2 {} },
        authority: Authority { data: ByteStr::from_static(b"example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b""), query: 0 },
    };

    let mut output = String::new();
    assert!(uri.fmt(&mut output).is_ok());
    assert_eq!(output, "ftp://example.com");
}

#[test]
fn test_uri_display_without_scheme_or_authority() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2 {} },
        authority: Authority { data: ByteStr::from_static(b"") },
        path_and_query: PathAndQuery { data: ByteStr::from_static(b""), query: 0 },
    };

    let mut output = String::new();
    assert!(uri.fmt(&mut output).is_ok());
    assert_eq!(output, "");
}

