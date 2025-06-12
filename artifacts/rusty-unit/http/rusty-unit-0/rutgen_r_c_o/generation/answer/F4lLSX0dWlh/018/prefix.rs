// Answer 0

#[test]
fn test_uri_display_with_valid_scheme_authority_path_and_query() {
    let scheme = Scheme { inner: Scheme2::from_str("http").unwrap() };
    let authority = Authority { data: ByteStr::from_static(b"example.com") };
    let path_and_query = PathAndQuery {
        data: ByteStr::from_static(b"/path/to/resource"),
        query: 1,
    };

    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
fn test_uri_display_with_valid_scheme_authority_path_no_query() {
    let scheme = Scheme { inner: Scheme2::from_str("https").unwrap() };
    let authority = Authority { data: ByteStr::from_static(b"example.com") };
    let path_and_query = PathAndQuery {
        data: ByteStr::from_static(b"/path/to/resource"),
        query: 0,
    };

    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
fn test_uri_display_with_scheme_and_authority_only() {
    let scheme = Scheme { inner: Scheme2::from_str("ftp").unwrap() };
    let authority = Authority { data: ByteStr::from_static(b"ftp.example.com") };
    let path_and_query = PathAndQuery {
        data: ByteStr::from_static(b""),
        query: 0,
    };

    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
#[should_panic]
fn test_uri_display_with_valid_scheme_authority_and_invalid_query() {
    let scheme = Scheme { inner: Scheme2::from_str("http").unwrap() };
    let authority = Authority { data: ByteStr::from_static(b"example.com") };
    let path_and_query = PathAndQuery {
        data: ByteStr::from_static(b"/path/to/resource"),
        query: u16::MAX,
    };

    let uri = Uri {
        scheme,
        authority,
        path_and_query,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

