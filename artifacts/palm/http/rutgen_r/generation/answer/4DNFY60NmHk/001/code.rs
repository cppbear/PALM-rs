// Answer 0

#[test]
fn test_builder_creates_new_builder() {
    let builder = http::builder();
    assert!(builder.is_instance_of::<http::Builder>());
}

#[test]
fn test_builder_can_set_status() {
    let builder = http::builder();
    let response = builder.status(200).body(()).unwrap();
    assert_eq!(response.status(), 200);
}

#[test]
fn test_builder_can_add_header() {
    let builder = http::builder();
    let response = builder.header("X-Custom-Foo", "Bar").body(()).unwrap();
    assert_eq!(response.headers()["X-Custom-Foo"], "Bar");
}

#[test]
fn test_builder_unwrap_without_body() {
    let builder = http::builder();
    let response = builder.status(204).body(()).unwrap();
    assert_eq!(response.status(), 204);
}

#[should_panic]
fn test_builder_unwrap_without_status() {
    let builder = http::builder();
    // Attempting to unwrap with no status should panic
    builder.body(()).unwrap();
}

