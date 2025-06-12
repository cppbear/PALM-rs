// Answer 0

#[test]
fn test_body_mut() {
    struct Response<T> {
        body: T,
    }

    impl<T: Default> Default for Response<T> {
        fn default() -> Self {
            Response {
                body: T::default(),
            }
        }
    }

    let mut response: Response<String> = Response::default();
    response.body_mut().push_str("hello world");
    assert!(!response.body.is_empty());
}

