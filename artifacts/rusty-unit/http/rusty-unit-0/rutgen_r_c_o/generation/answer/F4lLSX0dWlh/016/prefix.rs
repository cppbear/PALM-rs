// Answer 0

#[test]
fn test_uri_fmt_with_empty_path_and_query() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::from_str("http").unwrap() },
        authority: Authority { data: ByteStr::from("example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from(""), query: 0 },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
fn test_uri_fmt_with_non_empty_query() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::from_str("https").unwrap() },
        authority: Authority { data: ByteStr::from("example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from(""), query: 100 },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
fn test_uri_fmt_with_empty_authority() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::from_str("ftp").unwrap() },
        authority: Authority { data: ByteStr::from("") },
        path_and_query: PathAndQuery { data: ByteStr::from("path"), query: 0 },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
fn test_uri_fmt_with_large_query_number() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::from_str("mailto").unwrap() },
        authority: Authority { data: ByteStr::from("user@example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from("path"), query: 65535 },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

#[test]
fn test_uri_fmt_with_non_empty_path_and_zero_query() {
    let uri = Uri {
        scheme: Scheme { inner: Scheme2::from_str("geo").unwrap() },
        authority: Authority { data: ByteStr::from("maps.example.com") },
        path_and_query: PathAndQuery { data: ByteStr::from("41.40338,2.17403"), query: 0 },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", uri);
}

