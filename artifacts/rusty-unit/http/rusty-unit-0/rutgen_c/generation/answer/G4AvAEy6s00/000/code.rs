// Answer 0

#[test]
fn test_body_returns_reference_to_body() {
    let response: Response<String> = Response::new(String::from(""));
    assert_eq!(response.body(), &String::from(""));
}

#[test]
fn test_body_returns_non_empty_body() {
    let response: Response<String> = Response::new(String::from("Hello, World!"));
    assert_eq!(response.body(), &String::from("Hello, World!"));
}

#[test]
fn test_body_for_different_type() {
    let response: Response<i32> = Response::new(42);
    assert_eq!(response.body(), &42);
}

