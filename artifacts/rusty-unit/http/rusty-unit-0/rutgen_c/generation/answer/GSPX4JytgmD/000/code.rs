// Answer 0

#[test]
fn test_response_new() {
    use http::{Response, StatusCode};

    let response: Response<&str> = Response::new("hello world");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "hello world");
}

#[test]
fn test_response_new_empty_body() {
    use http::{Response, StatusCode};

    let response: Response<&str> = Response::new("");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "");
}

#[test]
fn test_response_new_with_different_body_type() {
    use http::{Response, StatusCode};

    let response: Response<i32> = Response::new(42);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), 42);
}

