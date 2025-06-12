// Answer 0

#[test]
fn test_response_new_with_string() {
    let body = String::from("hello world");
    let response = Response::new(body);
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "hello world");
}

#[test]
fn test_response_new_with_empty_string() {
    let body = String::from("");
    let response = Response::new(body);
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "");
}

#[test]
fn test_response_new_with_integer_body() {
    let body = 42;
    let response = Response::new(body);
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), 42);
}

#[test]
fn test_response_new_with_float_body() {
    let body = 3.14;
    let response = Response::new(body);
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), 3.14);
}

#[test]
#[should_panic]
fn test_response_new_with_panic_condition() {
    let body = std::panic::panic_any("panic");
    let _response = Response::new(body);
}

