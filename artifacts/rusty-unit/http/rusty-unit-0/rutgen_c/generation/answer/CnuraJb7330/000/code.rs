// Answer 0

#[test]
fn test_scheme_with_absolute_uri() {
    struct TestScheme;
    
    let uri = Uri {
        scheme: Scheme {
            inner: Scheme2::Standard(TestScheme),
        },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"username:password@example.com:123") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 },
    };

    assert!(uri.scheme().is_some());
}

#[test]
fn test_scheme_with_relative_uri() {
    struct TestScheme;

    let uri = Uri {
        scheme: Scheme {
            inner: Scheme2::None,
        },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 },
    };

    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_with_different_schemes() {
    struct HttpScheme;
    struct FtpScheme;

    let http_uri = Uri {
        scheme: Scheme {
            inner: Scheme2::Standard(HttpScheme),
        },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path") }, query: 0 },
    };
    assert!(http_uri.scheme().is_some());

    let ftp_uri = Uri {
        scheme: Scheme {
            inner: Scheme2::Standard(FtpScheme),
        },
        authority: Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } },
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path") }, query: 0 },
    };
    assert!(ftp_uri.scheme().is_some());
}

