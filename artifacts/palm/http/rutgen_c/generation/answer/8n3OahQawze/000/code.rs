// Answer 0

#[test]
fn test_builder_creation() {
    let builder = http::Uri::builder();
    assert!(builder.parts.is_ok());
}

#[test]
fn test_builder_scheme() {
    let builder = http::Uri::builder().scheme("https");
    assert!(builder.parts.as_ref().unwrap().scheme.is_some());
}

#[test]
fn test_builder_authority() {
    let builder = http::Uri::builder().authority("hyper.rs");
    assert!(builder.parts.as_ref().unwrap().authority.is_some());
}

#[test]
fn test_builder_path_and_query() {
    let builder = http::Uri::builder().path_and_query("/");
    assert!(builder.parts.as_ref().unwrap().path_and_query.is_some());
}

#[test]
fn test_builder_complete_uri_creation() {
    let uri = http::Uri::builder()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("/")
        .build()
        .unwrap();
    assert_eq!(uri.scheme().unwrap().inner, "https");
    assert_eq!(uri.authority().unwrap().data, "hyper.rs");
    assert_eq!(uri.path_and_query().unwrap().data, "/");
}

