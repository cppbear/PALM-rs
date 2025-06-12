// Answer 0

#[test]
fn test_into_parts_empty_response() {
    struct TestBody;

    let response: Response<TestBody> = Response::new(TestBody);
    let (parts, body) = response.into_parts();

    assert_eq!(body, TestBody);
    // Assuming default values for Parts are set correctly
    assert!(parts.status == StatusCode::OK);
}

#[test]
fn test_into_parts_non_empty_response() {
    struct TestBody(String);

    let body = TestBody("Hello, World!".to_string());
    let parts = Parts {
        status: StatusCode::CREATED,
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let response: Response<TestBody> = Response::from_parts(parts.clone(), body.clone());
    let (returned_parts, returned_body) = response.into_parts();

    assert_eq!(returned_body, body);
    assert_eq!(returned_parts, parts);
}

#[test]
fn test_into_parts_response_with_different_body() {
    struct TestBody(i32);

    let body = TestBody(42);
    let parts = Parts {
        status: StatusCode::ACCEPTED,
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let response: Response<TestBody> = Response::from_parts(parts.clone(), body.clone());
    let (returned_parts, returned_body) = response.into_parts();

    assert_eq!(returned_body, body);
    assert_eq!(returned_parts.status, StatusCode::ACCEPTED);
}

