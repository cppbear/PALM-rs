// Answer 0

#[test]
fn test_builder_new() {
    let builder = Builder::new();
}

#[test]
fn test_builder_method() {
    let builder = Builder::new().method("GET");
    let builder = builder.method("POST");
}

#[test]
fn test_builder_uri() {
    let builder = Builder::new().uri("http://example.com");
    let builder = builder.uri("https://example.org");
}

#[test]
fn test_builder_version() {
    let builder = Builder::new().version(Version::HTTP_10);
}

#[test]
fn test_builder_header() {
    let builder = Builder::new().header("Content-Type", "application/json");
    let builder = builder.header("Authorization", "Bearer token");
}

#[test]
fn test_builder_extensions() {
    let builder = Builder::new().extension(());
    let builder = builder.extension((1, 2));
    let builder = builder.extension("string");
}

#[test]
fn test_builder_complete() {
    let builder = Builder::new()
        .method("PUT")
        .uri("http://example.com")
        .version(Version::HTTP_11)
        .header("Custom-Header", "value")
        .extension("string");
}

