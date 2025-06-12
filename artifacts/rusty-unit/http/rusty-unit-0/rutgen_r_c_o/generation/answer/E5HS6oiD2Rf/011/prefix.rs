// Answer 0

#[test]
fn test_from_parts_scheme_missing() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_static("example.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let result = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_scheme_missing_empty_authority() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::empty());
    parts.path_and_query = Some(PathAndQuery::from_static("/foo"));

    let result = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_scheme_missing_valid_authority() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_static("valid.com"));
    parts.path_and_query = Some(PathAndQuery::from_static("/bar"));

    let result = Uri::from_parts(parts);
}

#[test]
fn test_from_parts_scheme_missing_with_special_characters() {
    let mut parts = Parts::default();
    parts.authority = Some(Authority::from_static("foo.bar:8080"));
    parts.path_and_query = Some(PathAndQuery::from_static("/baz?query=value"));

    let result = Uri::from_parts(parts);
}

