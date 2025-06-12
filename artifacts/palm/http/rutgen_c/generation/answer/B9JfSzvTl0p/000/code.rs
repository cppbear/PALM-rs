// Answer 0

#[test]
fn test_uri_debug_fmt() {
    struct TestScheme {
        inner: Scheme2,
    }

    struct TestAuthority {
        data: ByteStr,
    }

    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    let scheme = TestScheme { inner: Scheme2::default() };
    let authority = TestAuthority { data: ByteStr::from("example.com") };
    let path_and_query = TestPathAndQuery { data: ByteStr::from("/path"), query: 0 };

    let uri = Uri {
        scheme: scheme.inner,
        authority: authority.data,
        path_and_query: path_and_query.data,
    };

    let mut output = String::new();
    let result = uri.fmt(&mut output);

    assert!(result.is_ok());
    assert!(!output.is_empty());
}

#[test]
fn test_uri_debug_fmt_empty() {
    struct TestScheme {
        inner: Scheme2,
    }

    struct TestAuthority {
        data: ByteStr,
    }

    struct TestPathAndQuery {
        data: ByteStr,
        query: u16,
    }

    let scheme = TestScheme { inner: Scheme2::default() };
    let authority = TestAuthority { data: ByteStr::from("") };
    let path_and_query = TestPathAndQuery { data: ByteStr::from(""), query: 0 };

    let uri = Uri {
        scheme: scheme.inner,
        authority: authority.data,
        path_and_query: path_and_query.data,
    };

    let mut output = String::new();
    let result = uri.fmt(&mut output);

    assert!(result.is_ok());
    assert!(output.is_empty());
}

