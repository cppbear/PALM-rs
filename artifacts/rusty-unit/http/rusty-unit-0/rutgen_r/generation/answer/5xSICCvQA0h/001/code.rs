// Answer 0

#[test]
fn test_builder_creation() {
    let builder = http::response::Builder::new();
    assert_eq!(builder.status(), None);
    assert_eq!(builder.body(), None);
}

#[test]
fn test_builder_default() {
    let builder: http::response::Builder = http::response::Builder::new();
    assert!(builder.is_default());
}

#[test]
fn test_builder_with_status() {
    let builder = http::response::Builder::new()
        .status(200);
    assert_eq!(builder.status(), Some(200));
}

#[test]
fn test_builder_with_body() {
    let builder = http::response::Builder::new()
        .body("Hello, world!")
        .unwrap();
    assert_eq!(builder.body(), Some("Hello, world!"));
}

#[test]
#[should_panic]
fn test_builder_invalid_status() {
    let builder = http::response::Builder::new();
    builder.status(-1); // Assuming negative status is invalid
}

#[test]
fn test_builder_chain_methods() {
    let builder = http::response::Builder::new()
        .status(404)
        .body("Not Found")
        .unwrap();
    assert_eq!(builder.status(), Some(404));
    assert_eq!(builder.body(), Some("Not Found"));
}

