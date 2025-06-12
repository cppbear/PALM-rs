// Answer 0

#[test]
fn test_builder_method_with_valid_string() {
    use crate::method::Method;
    use std::convert::TryInto;

    let builder = Builder::new();
    let req = builder
        .method("POST")
        .body(()); // Assuming body() returns Result<Request<T>>

    assert!(req.is_ok());
    let request = req.unwrap();
    assert_eq!(request.method, Method::POST);
}

#[test]
#[should_panic]
fn test_builder_method_with_invalid_method() {
    use crate::method::Method;
    use std::convert::TryInto;

    let builder = Builder::new();
    let req = builder
        .method("INVALID_METHOD") // This should cause TryInto to fail
        .body(()); // Assuming body() returns Result<Request<T>>

    req.unwrap(); // This should panic
}

#[test]
fn test_builder_method_default_is_get() {
    use crate::method::Method;

    let builder = Builder::new();
    let req = builder
        .body(()); // Assuming body() returns Result<Request<T>>

    assert!(req.is_ok());
    let request = req.unwrap();
    assert_eq!(request.method, Method::GET); // Ensure default method is GET
}

