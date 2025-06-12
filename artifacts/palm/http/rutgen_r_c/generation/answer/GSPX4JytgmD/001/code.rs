// Answer 0

#[test]
fn test_response_new_with_string_body() {
    let response: Response<&str> = Response::new("hello world");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "hello world");
}

#[test]
fn test_response_new_with_integer_body() {
    let response: Response<i32> = Response::new(42);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), 42);
}

#[test]
fn test_response_new_with_empty_body() {
    let response: Response<&str> = Response::new("");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "");
}

#[test]
fn test_response_new_with_vector_body() {
    let response: Response<Vec<i32>> = Response::new(vec![1, 2, 3]);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), vec![1, 2, 3]);
}

#[test]
fn test_response_new_with_non_empty_body() {
    let response: Response<String> = Response::new(String::from("Non-empty body"));
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "Non-empty body");
}

