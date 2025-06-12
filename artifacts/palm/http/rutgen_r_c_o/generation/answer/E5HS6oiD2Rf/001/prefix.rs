// Answer 0

#[test]
fn test_from_parts_scheme_missing_authority() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Http) }); // valid scheme
    parts.authority = None; // authority is missing
    parts.path_and_query = Some(PathAndQuery::from_static("/foo")); // valid path

    let result = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_scheme_missing_authority_empty_path() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Https) }); // valid scheme
    parts.authority = None; // authority is missing
    parts.path_and_query = Some(PathAndQuery::empty()); // empty path

    let result = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_scheme_missing_authority_large_path() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::Standard(Protocol::Ftp) }); // valid scheme
    parts.authority = None; // authority is missing
    parts.path_and_query = Some(PathAndQuery::from_static(&"/".repeat(MAX_LEN))); // maximum length path

    let result = Uri::from_parts(parts);
}

