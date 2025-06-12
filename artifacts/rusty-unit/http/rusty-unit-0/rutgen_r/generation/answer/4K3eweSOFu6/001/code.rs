// Answer 0

#[test]
fn test_request_new_with_string() {
    let body = String::from("hello world");
    let request = Request::new(body.clone());

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(*request.body(), body);
}

#[test]
fn test_request_new_with_empty_string() {
    let body = String::from("");
    let request = Request::new(body.clone());

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(*request.body(), body);
}

#[test]
fn test_request_new_with_large_string() {
    let body = String::from("a".repeat(1000)); // Large body
    let request = Request::new(body.clone());

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(*request.body(), body);
}

#[test]
fn test_request_new_with_non_utf8_bytes() {
    let body = vec![240, 159, 152, 138]; // Non-UTF-8 byte sequence
    let request = Request::new(body.clone());

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(*request.body(), body);
}

