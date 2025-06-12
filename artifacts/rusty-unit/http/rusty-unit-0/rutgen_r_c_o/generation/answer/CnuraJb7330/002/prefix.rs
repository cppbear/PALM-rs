// Answer 0

#[test]
fn test_scheme_with_standard_scheme() {
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::HTTP) };
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path/data") }, query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = uri.scheme();
}

#[test]
fn test_scheme_with_other_scheme() {
    let scheme = Scheme { inner: Scheme2::Other(Box::new(ByteStr { bytes: Bytes::from_static(b"customscheme") })) };
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path/data") }, query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = uri.scheme();
}

#[test]
fn test_scheme_with_long_standard_scheme() {
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::FTP) };
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"long-authority.example.com") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/data/file") }, query: 1234 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = uri.scheme();
}

#[test]
fn test_scheme_with_uppercase_standard_scheme() {
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::HTTPS) };
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/secure/data") }, query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = uri.scheme();
}

#[test]
fn test_scheme_with_case_insensitive_scheme() {
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::HTTP) };
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"example.org") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/test/resource") }, query: 0 };
    let uri = Uri { scheme, authority, path_and_query };
    let _ = uri.scheme();
}

