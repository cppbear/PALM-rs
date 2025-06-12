// Answer 0

#[test]
fn test_path_and_query_with_standard_scheme() {
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path?query=1") }, query: 1 };
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Http) };
    let uri = Uri { scheme, authority, path_and_query };
    uri.path_and_query();
}

#[test]
fn test_path_and_query_with_other_scheme() {
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"example.com") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/path?query=2") }, query: 2 };
    let scheme = Scheme { inner: Scheme2::Other(Box::new(ByteStr { bytes: Bytes::from_static(b"custom") })) };
    let uri = Uri { scheme, authority, path_and_query };
    uri.path_and_query();
}

#[test]
fn test_path_and_query_with_non_empty_authority() {
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"localhost") } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/test?query=3") }, query: 3 };
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Https) };
    let uri = Uri { scheme, authority, path_and_query };
    uri.path_and_query();
}

#[test]
fn test_path_and_query_with_max_length_authority() {
    let authority_data = Bytes::from_static(b"a".repeat(MAX_LEN).as_slice());
    let authority = Authority { data: ByteStr { bytes: authority_data } };
    let path_and_query = PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/maxlen?query=4") }, query: 4 };
    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Ftp) };
    let uri = Uri { scheme, authority, path_and_query };
    uri.path_and_query();
}

