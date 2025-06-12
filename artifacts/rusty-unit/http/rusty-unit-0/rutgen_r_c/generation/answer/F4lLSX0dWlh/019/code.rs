// Answer 0

#[test]
fn test_fmt_with_complete_uri() {
    use crate::byte_str::ByteStr;

    struct MockScheme {
        inner: Scheme2,
    }

    struct MockAuthority {
        data: ByteStr,
    }

    struct MockPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl fmt::Display for MockScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "http")
        }
    }

    impl fmt::Display for MockAuthority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "www.example.com")
        }
    }

    impl MockPathAndQuery {
        fn path(&self) -> &str {
            "path/to/resource"
        }

        fn query(&self) -> Option<&str> {
            Some("query=1")
        }
    }

    let uri = Uri {
        scheme: MockScheme { inner: Scheme2::new() },
        authority: MockAuthority { data: ByteStr::from_static(b"www.example.com") },
        path_and_query: MockPathAndQuery {
            data: ByteStr::from_static(b"path/to/resource"),
            query: 1,
        },
    };

    let mut output = String::new();
    let result = uri.fmt(&mut fmt::Formatter::new(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "http://www.example.com/path/to/resource?query=1");
}

#[test]
fn test_fmt_with_no_query() {
    use crate::byte_str::ByteStr;

    struct MockScheme {
        inner: Scheme2,
    }

    struct MockAuthority {
        data: ByteStr,
    }

    struct MockPathAndQuery {
        data: ByteStr,
        query: Option<u16>,
    }

    impl fmt::Display for MockScheme {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "https")
        }
    }

    impl fmt::Display for MockAuthority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "api.example.com")
        }
    }

    impl MockPathAndQuery {
        fn path(&self) -> &str {
            "api/resource"
        }

        fn query(&self) -> Option<&str> {
            None
        }
    }

    let uri = Uri {
        scheme: MockScheme { inner: Scheme2::new() },
        authority: MockAuthority { data: ByteStr::from_static(b"api.example.com") },
        path_and_query: MockPathAndQuery {
            data: ByteStr::from_static(b"api/resource"),
            query: None,
        },
    };

    let mut output = String::new();
    let result = uri.fmt(&mut fmt::Formatter::new(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "https://api.example.com/api/resource");
}

#[test]
fn test_fmt_without_scheme() {
    use crate::byte_str::ByteStr;

    struct MockAuthority {
        data: ByteStr,
    }

    struct MockPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    impl fmt::Display for MockAuthority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "localhost")
        }
    }

    impl MockPathAndQuery {
        fn path(&self) -> &str {
            "local/path"
        }

        fn query(&self) -> Option<&str> {
            Some("id=123")
        }
    }

    let uri = Uri {
        scheme: MockScheme { inner: Scheme2::new() },
        authority: MockAuthority { data: ByteStr::from_static(b"localhost") },
        path_and_query: MockPathAndQuery {
            data: ByteStr::from_static(b"local/path"),
            query: 123,
        },
    };

    let mut output = String::new();
    let result = uri.fmt(&mut fmt::Formatter::new(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "localhost/local/path?id=123");
}

#[should_panic]
#[test]
fn test_fmt_with_invalid_uri() {
    struct BrokenScheme {}

    impl fmt::Display for BrokenScheme {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("Invalid scheme")
        }
    }

    let uri = Uri {
        scheme: BrokenScheme {},
        authority: MockAuthority { data: ByteStr::from_static(b"localhost") },
        path_and_query: MockPathAndQuery {
            data: ByteStr::from_static(b"local"),
            query: 0,
        },
    };

    let mut output = String::new();
    let _ = uri.fmt(&mut fmt::Formatter::new(&mut output));
}

