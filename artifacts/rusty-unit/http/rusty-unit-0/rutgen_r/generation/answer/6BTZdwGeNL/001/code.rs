// Answer 0

#[test]
fn test_response_builder_with_unit_body() {
    let response = Response::builder()
        .body(())
        .unwrap();
    assert_eq!(response.body, ());
}

#[test]
fn test_response_builder_with_string_body() {
    let body_content = String::from("Hello, world!");
    let response = Response::builder()
        .body(body_content.clone())
        .unwrap();
    assert_eq!(response.body, body_content);
}

#[test]
fn test_response_builder_with_numeric_body() {
    let numeric_body = 42;
    let response = Response::builder()
        .body(numeric_body)
        .unwrap();
    assert_eq!(response.body, 42);
}

#[should_panic]
fn test_response_builder_with_invalid_header() {
    let invalid_header_response = Response::builder()
        .header("Foo", "Bar\r\n")
        .body(())
        .unwrap();
}

