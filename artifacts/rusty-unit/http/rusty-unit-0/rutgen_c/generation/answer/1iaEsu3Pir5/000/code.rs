// Answer 0

#[test]
fn test_into_body() {
    struct TestRequestBody {
        value: i32,
    }

    let request = Request::new(TestRequestBody { value: 10 });
    let body = request.into_body();
    assert_eq!(body.value, 10);
}

#[test]
fn test_into_body_empty_request() {
    struct EmptyRequestBody;

    let request = Request::new(EmptyRequestBody {});
    let body = request.into_body();
    // Since the body is empty, we can only assert its type or check for some property.
    assert!(std::mem::size_of_val(&body) == 0);
}

