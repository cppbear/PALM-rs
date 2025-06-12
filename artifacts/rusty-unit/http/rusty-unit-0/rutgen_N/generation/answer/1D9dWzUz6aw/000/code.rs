// Answer 0

#[test]
fn test_body_empty_string_request() {
    struct Request<T> {
        body: T,
    }

    impl<T: Default> Default for Request<T> {
        fn default() -> Self {
            Request {
                body: T::default(),
            }
        }
    }

    let request: Request<String> = Request::default();
    assert!(request.body().is_empty());
}

#[test]
fn test_body_empty_vec_request() {
    struct Request<T> {
        body: T,
    }

    impl<T: Default> Default for Request<T> {
        fn default() -> Self {
            Request {
                body: T::default(),
            }
        }
    }

    let request: Request<Vec<u8>> = Request::default();
    assert!(request.body().is_empty());
}

