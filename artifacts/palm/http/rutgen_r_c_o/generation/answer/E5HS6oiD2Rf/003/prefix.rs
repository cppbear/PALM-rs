// Answer 0

#[test]
fn test_from_parts_with_valid_absolute_uri() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) });
    parts.authority = Some(Authority::from_static("example.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/path/query"));

    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_with_valid_scheme_and_empty_authority() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Https) });
    parts.authority = Some(Authority::empty()); // valid, but empty
    parts.path_and_query = Some(PathAndQuery::from_static("/"));

    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_with_valid_scheme_and_path_query() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Ftp) });
    parts.authority = Some(Authority::from_static("ftp.server.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/uploads"));

    let uri = Uri::from_parts(parts);
}

#[test]
#[should_panic]
fn test_from_parts_with_missing_authority() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::File) });
    parts.authority = None; // Missing authority
    parts.path_and_query = Some(PathAndQuery::from_static("/local/file"));

    let _uri = Uri::from_parts(parts);
}

#[test]
#[should_panic]
fn test_from_parts_with_missing_path_and_query() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Mailto) });
    parts.authority = Some(Authority::from_static("user@example.com"));
    parts.path_and_query = None; // Missing path_and_query

    let _uri = Uri::from_parts(parts);
}

#[test]
#[should_panic]
fn test_from_parts_with_scheme_missing() {
    let mut parts = Parts::default();
    parts.scheme = None; // Missing scheme
    parts.authority = Some(Authority::from_static("foo.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/bar"));

    let _uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_with_valid_non_standard_scheme() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Other(Box::new(ByteStr::from_static("custom-scheme"))) });
    parts.authority = Some(Authority::from_static("custom.host.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/resource"));

    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_with_query_in_path_and_query() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) });
    parts.authority = Some(Authority::from_static("www.example.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/search?q=test"));

    let uri = Uri::from_parts(parts);
}

