// Answer 0

#[test]
fn test_fmt_with_valid_scheme_authority_and_path() {
    let scheme = Scheme { inner: Scheme2::new("http").unwrap() };
    let authority = Authority { data: ByteStr::from_static("www.example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from_static("/path/to/resource"), query: 0 };
    let uri = Uri { scheme, authority, path_and_query };

    let mut output = String::new();
    let res = write!(output, "{}", uri);
}

#[test]
fn test_fmt_with_valid_scheme_and_authority_with_empty_path() {
    let scheme = Scheme { inner: Scheme2::new("https").unwrap() };
    let authority = Authority { data: ByteStr::from_static("www.example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from_static(""), query: 0 };
    let uri = Uri { scheme, authority, path_and_query };

    let mut output = String::new();
    let res = write!(output, "{}", uri);
}

#[test]
fn test_fmt_with_valid_scheme_authority_and_empty_query() {
    let scheme = Scheme { inner: Scheme2::new("ftp").unwrap() };
    let authority = Authority { data: ByteStr::from_static("ftp.example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from_static("/path"), query: 0 };
    let uri = Uri { scheme, authority, path_and_query };

    let mut output = String::new();
    let res = write!(output, "{}", uri);
}

#[test]
fn test_fmt_with_valid_scheme_authority_and_non_empty_query() {
    let scheme = Scheme { inner: Scheme2::new("mailto").unwrap() };
    let authority = Authority { data: ByteStr::from_static("user@example.com") };
    let path_and_query = PathAndQuery { data: ByteStr::from_static(""), query: 1 };
    let uri = Uri { scheme, authority, path_and_query };

    let mut output = String::new();
    let res = write!(output, "{}", uri);
}

