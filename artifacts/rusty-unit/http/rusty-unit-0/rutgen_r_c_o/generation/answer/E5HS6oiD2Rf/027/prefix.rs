// Answer 0

#[test]
fn test_from_parts_empty() {
    let parts = Parts::default();
    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_only_scheme() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::None });
    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_only_authority() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::empty());
    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_only_path_and_query() {
    let mut parts = Parts::default();
    parts.path_and_query = Some(PathAndQuery::empty());
    let uri = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_valid() {
    let mut parts = Parts::default();
    parts.scheme = Some(Scheme { inner: Scheme2::None });
    parts.authority = Some(Authority::empty());
    parts.path_and_query = Some(PathAndQuery::empty());
    let uri = Uri::from_parts(parts);
}

