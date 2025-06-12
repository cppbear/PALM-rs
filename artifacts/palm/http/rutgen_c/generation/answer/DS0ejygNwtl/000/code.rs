// Answer 0

#[test]
fn test_builder_new() {
    let builder = Builder::new();
    assert!(builder.inner.is_ok());
}

#[test]
fn test_builder_default_initialization() {
    let builder = Builder::default();
    assert!(builder.inner.is_ok());
    let parts = builder.inner.unwrap();
    assert!(parts.scheme.is_none());
    assert!(parts.authority.is_none());
    assert!(parts.path_and_query.is_none());
}

#[test]
fn test_builder_method() {
    let builder = Builder::new();
    let builder = builder.method("GET");
    assert!(builder.inner.is_ok());
    let parts = builder.inner.unwrap();
    assert_eq!(parts.method.to_string(), "GET");
}

#[test]
fn test_builder_uri() {
    let builder = Builder::new();
    let builder = builder.uri("http://example.com");
    assert!(builder.inner.is_ok());
    let parts = builder.inner.unwrap();
    assert_eq!(parts.uri.to_string(), "http://example.com");
}

#[test]
fn test_builder_version() {
    let builder = Builder::new();
    let version = Version::HTTP_11; // Assuming Version::HTTP_11 exists
    let builder = builder.version(version);
    assert!(builder.inner.is_ok());
    let parts = builder.inner.unwrap();
    assert_eq!(parts.version, version);
}

#[test]
fn test_builder_header() {
    let builder = Builder::new();
    let builder = builder.header("Content-Type", "application/json");
    assert!(builder.inner.is_ok());
    let parts = builder.inner.unwrap();
    assert!(parts.headers.get("Content-Type").is_some());
}

#[test]
fn test_builder_extensions() {
    let builder = Builder::new();
    let builder = builder.extension("Some extension");
    assert!(builder.inner.is_ok());
    let parts = builder.inner.unwrap();
    assert!(parts.extensions.is_some());
}

#[test]
#[should_panic]
fn test_builder_invalid_method() {
    let builder = Builder::new();
    let _ = builder.method("INVALID_METHOD"); // Assuming invalid method triggers panic
}

