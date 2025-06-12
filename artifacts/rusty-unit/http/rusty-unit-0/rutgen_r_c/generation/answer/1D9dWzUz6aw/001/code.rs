// Answer 0

#[test]
fn test_body_empty_string() {
    let request: Request<String> = Request::new(String::new());
    assert!(request.body().is_empty());
}

#[test]
fn test_body_non_empty_string() {
    let request: Request<String> = Request::new("Hello, World!".to_string());
    assert_eq!(request.body(), "Hello, World!");
}

#[test]
fn test_body_numeric() {
    let request: Request<i32> = Request::new(42);
    assert_eq!(*request.body(), 42);
}

#[test]
fn test_body_struct() {
    #[derive(Debug, PartialEq)]
    struct TestBody {
        value: i32,
    }
    
    let body = TestBody { value: 10 };
    let request: Request<TestBody> = Request::new(body);
    assert_eq!(request.body().value, 10);
}

#[test]
fn test_body_mutability() {
    let mut request: Request<String> = Request::new("Initial".to_string());
    let body_mut = request.body_mut();
    body_mut.push_str(" Value");
    assert_eq!(request.body(), "Initial Value");
}

#[test]
fn test_body_transformation() {
    let request: Request<Vec<u8>> = Request::new(vec![1, 2, 3]);
    assert_eq!(request.body(), &vec![1, 2, 3]);
}

