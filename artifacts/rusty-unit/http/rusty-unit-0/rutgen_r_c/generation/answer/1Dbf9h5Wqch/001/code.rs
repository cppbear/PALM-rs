// Answer 0

#[test]
fn test_map_with_string_body() {
    struct TestBody(String);
    let request = Request::new(TestBody("some string".to_string()));
    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b.0, "some string");
        b.0.as_bytes()
    });
    assert_eq!(mapped_request.body(), &"some string".as_bytes());
}

#[test]
fn test_map_with_integer_body() {
    struct TestBody(i32);
    let request = Request::new(TestBody(42));
    let mapped_request: Request<String> = request.map(|b| {
        assert_eq!(b.0, 42);
        b.0.to_string()
    });
    assert_eq!(mapped_request.body(), "42");
}

#[test]
fn test_map_with_empty_body() {
    struct TestBody(Vec<u8>);
    let request = Request::new(TestBody(vec![]));
    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b.0, vec![]);
        b.0.as_slice()
    });
    assert_eq!(mapped_request.body(), &[]);
}

#[test]
fn test_map_with_empty_string_body() {
    struct TestBody(String);
    let request = Request::new(TestBody("".to_string()));
    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b.0, "");
        b.0.as_bytes()
    });
    assert_eq!(mapped_request.body(), &[]);
}

#[test]
fn test_map_with_composite_body() {
    struct TestBody { a: i32, b: String }
    let request = Request::new(TestBody { a: 10, b: "Hello".to_string() });
    let mapped_request: Request<(i32, Vec<u8>)> = request.map(|b| {
        assert_eq!(b.a, 10);
        (b.a, b.b.as_bytes().to_vec())
    });
    assert_eq!(mapped_request.body(), &(10, b"Hello".to_vec()));
}

