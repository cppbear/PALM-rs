// Answer 0

#[test]
fn test_builder_new() {
    let builder = http::response::Builder::new();
    assert_eq!(builder.status(), None);
    assert_eq!(builder.body(), None);
}

#[test]
fn test_builder_status() {
    let mut builder = http::response::Builder::new();
    builder.status(200);
    assert_eq!(builder.status(), Some(200));
}

#[test]
fn test_builder_body() {
    let mut builder = http::response::Builder::new();
    builder.body(()).unwrap();
    assert!(builder.body().is_some());
}

#[test]
fn test_builder_complete_response() {
    let response = http::response::Builder::new()
        .status(200)
        .body(())
        .unwrap();
    assert_eq!(response.status(), 200);
}

