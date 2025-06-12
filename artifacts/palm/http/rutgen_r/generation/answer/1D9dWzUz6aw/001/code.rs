// Answer 0

#[test]
fn test_body_empty_string() {
    struct Request<T> {
        body: T,
    }

    impl Default for Request<String> {
        fn default() -> Self {
            Request { body: String::new() }
        }
    }

    let request: Request<String> = Request::default();
    assert!(request.body().is_empty());
}

#[test]
fn test_body_non_empty_string() {
    struct Request<T> {
        body: T,
    }

    impl Default for Request<String> {
        fn default() -> Self {
            Request { body: "Hello".to_string() }
        }
    }

    let request: Request<String> = Request::default();
    assert_eq!(request.body(), "Hello");
}

#[test]
fn test_body_with_vec() {
    struct Request<T> {
        body: T,
    }

    impl Default for Request<Vec<u8>> {
        fn default() -> Self {
            Request { body: vec![] }
        }
    }

    let request: Request<Vec<u8>> = Request::default();
    assert!(request.body().is_empty());
}

#[test]
fn test_body_with_non_empty_vec() {
    struct Request<T> {
        body: T,
    }

    impl Default for Request<Vec<u8>> {
        fn default() -> Self {
            Request { body: vec![1, 2, 3] }
        }
    }

    let request: Request<Vec<u8>> = Request::default();
    assert_eq!(request.body(), &[1, 2, 3]);
}

