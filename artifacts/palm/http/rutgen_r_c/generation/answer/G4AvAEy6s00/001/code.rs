// Answer 0

#[test]
fn test_body_with_string() {
    let response: Response<String> = Response::new(String::from(""));
    assert_eq!(response.body(), "");
}

#[test]
fn test_body_with_non_empty_string() {
    let response: Response<String> = Response::new(String::from("Hello, world!"));
    assert_eq!(response.body(), "Hello, world!");
}

#[test]
fn test_body_with_vec_u8() {
    let response: Response<Vec<u8>> = Response::new(vec![1, 2, 3]);
    assert_eq!(response.body(), &[1, 2, 3]);
}

#[test]
fn test_body_with_empty_vector() {
    let response: Response<Vec<u8>> = Response::new(vec![]);
    assert!(response.body().is_empty());
}

#[test]
fn test_body_with_integer() {
    let response: Response<i32> = Response::new(42);
    assert_eq!(*response.body(), 42);
}

