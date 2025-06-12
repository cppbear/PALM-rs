// Answer 0

#[test]
fn test_request_new_with_string() {
    let body = String::from("hello world");
    let request = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), &String::from("hello world"));
}

#[test]
fn test_request_new_with_empty_string() {
    let body = String::from("");
    let request = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), &String::from(""));
}

#[test]
fn test_request_new_with_integer() {
    let body = 42;
    let request = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), &42);
}

#[test]
fn test_request_new_with_float() {
    let body = 3.14;
    let request = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), &3.14);
}

#[test]
fn test_request_new_with_boolean() {
    let body = true;
    let request = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), &true);
}

