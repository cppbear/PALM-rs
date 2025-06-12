// Answer 0

#[test]
fn test_into_body_with_integer() {
    let response = Response::new(42);
    let body = response.into_body();
    assert_eq!(body, 42);
}

#[test]
fn test_into_body_with_string() {
    let response = Response::new(String::from("Hello, world!"));
    let body = response.into_body();
    assert_eq!(body, "Hello, world!");
}

#[test]
fn test_into_body_with_empty_vector() {
    let response = Response::new(vec![]);
    let body: Vec<i32> = response.into_body();
    assert_eq!(body.len(), 0);
}

#[test]
fn test_into_body_with_non_empty_vector() {
    let response = Response::new(vec![1, 2, 3]);
    let body = response.into_body();
    assert_eq!(body, vec![1, 2, 3]);
}

