// Answer 0

#[test]
fn test_request_new() {
    struct TestBody;
    
    impl Default for TestBody {
        fn default() -> Self {
            TestBody
        }
    }

    let body = TestBody::default();
    let request: Request<TestBody> = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
}

#[test]
fn test_request_new_with_string_body() {
    let body = String::from("hello world");
    let request: Request<String> = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), "hello world");
}

#[test]
fn test_request_new_with_integer_body() {
    let body = 42;
    let request: Request<i32> = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(*request.body(), 42);
}

#[test]
fn test_request_new_with_empty_string_body() {
    let body = String::from("");
    let request: Request<String> = Request::new(body);

    assert_eq!(*request.method(), Method::GET);
    assert_eq!(request.body(), "");
}

