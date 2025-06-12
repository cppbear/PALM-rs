// Answer 0

#[test]
fn test_fmt_uri_valid_minimal() {
    let scheme = Scheme { inner: Scheme2::from_str("http").unwrap() };
    let authority = Authority { data: ByteStr::from("example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from("/path"), query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = fmt(&uri, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_uri_valid_maximum_length() {
    let scheme = Scheme { inner: Scheme2::from_str("https").unwrap() };
    let authority = Authority { data: ByteStr::from("a".repeat(URI_CHARS.len() - 1).as_str()) };
    let path_and_query = PathAndQuery { data: ByteStr::from("/path/to/resource"), query: 65535 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = fmt(&uri, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_uri_valid_with_special_chars() {
    let scheme = Scheme { inner: Scheme2::from_str("ftp").unwrap() };
    let authority = Authority { data: ByteStr::from("user:password@ftp.example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from("/file.txt?param1=value1&param2=value2"), query: 50 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = fmt(&uri, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_uri_valid_edge_case_large_query() {
    let scheme = Scheme { inner: Scheme2::from_str("mailto").unwrap() };
    let authority = Authority { data: ByteStr::from("user@example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from("/?q=large_query_param").repeat(100).as_str(), query: 65535 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = fmt(&uri, &mut fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_fmt_uri_empty_scheme() {
    let scheme = Scheme { inner: Scheme2::from_str("").unwrap() };
    let authority = Authority { data: ByteStr::from("example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from("/path"), query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = fmt(&uri, &mut fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_fmt_uri_exceeding_length() {
    let scheme = Scheme { inner: Scheme2::from_str("http").unwrap() };
    let authority = Authority { data: ByteStr::from("a".repeat(65534)) }; // exceeds the maximum length of ByteStr
    let path_and_query = PathAndQuery { data: ByteStr::from("/valid/path"), query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = fmt(&uri, &mut fmt::Formatter::new());
}

