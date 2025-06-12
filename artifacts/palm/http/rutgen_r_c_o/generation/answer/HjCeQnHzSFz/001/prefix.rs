// Answer 0

#[test]
fn test_builder_with_none_parts() {
    let builder = Builder::new();
    let _result = builder.build();
}

#[test]
fn test_builder_with_err_parts() {
    let builder = Builder { parts: Err(Error { inner: ErrorKind::Unknown }) };
    let _result = builder.build();
}

#[test]
fn test_builder_with_missing_authority() {
    let valid_scheme = Scheme { inner: Scheme2::Http };
    let builder = Builder::new().scheme(valid_scheme);
    let _result = builder.build();
}

#[test]
fn test_builder_with_missing_path_and_query() {
    let valid_scheme = Scheme { inner: Scheme2::Http };
    let valid_authority = Authority::from_str("example.com").unwrap();
    let builder = Builder::new()
        .scheme(valid_scheme)
        .authority(valid_authority);
    let _result = builder.build();
}

#[test]
fn test_builder_with_missing_scheme() {
    let valid_authority = Authority::from_str("example.com").unwrap();
    let valid_path_and_query = PathAndQuery::from_str("/path?query=value").unwrap();
    let builder = Builder::new()
        .authority(valid_authority)
        .path_and_query(valid_path_and_query);
    let _result = builder.build();
}

#[test]
fn test_builder_with_all_parts() {
    let valid_scheme = Scheme { inner: Scheme2::Http };
    let valid_authority = Authority::from_str("example.com").unwrap();
    let valid_path_and_query = PathAndQuery::from_str("/path?query=value").unwrap();
    let builder = Builder::new()
        .scheme(valid_scheme)
        .authority(valid_authority)
        .path_and_query(valid_path_and_query);
    let _result = builder.build();
}

#[test]
fn test_builder_with_none_scheme_and_authority() {
    let valid_path_and_query = PathAndQuery::from_str("/path?query=value").unwrap();
    let builder = Builder::new().path_and_query(valid_path_and_query);
    let _result = builder.build();
}

