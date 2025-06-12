// Answer 0

#[test]
fn test_builder_empty() {
    let result = crate::Uri::builder().build();
    assert!(result.is_ok());
}

#[test]
fn test_builder_invalid_scheme() {
    let result = crate::Uri::builder().scheme("!@#%/^").build();
    assert!(result.is_err());
}

#[test]
fn test_builder_partial_uri() {
    let result = crate::Uri::builder().path("/example").build();
    assert!(result.is_err());
}

#[test]
fn test_builder_valid_uri() {
    let result = crate::Uri::builder()
        .scheme("http")
        .host("www.example.com")
        .path("/path")
        .build();
    assert!(result.is_ok());
}

#[test]
fn test_builder_with_query() {
    let result = crate::Uri::builder()
        .scheme("https")
        .host("www.example.com")
        .path("/path")
        .query(Some("key=value"))
        .build();
    assert!(result.is_ok());
}

