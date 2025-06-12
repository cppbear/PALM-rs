// Answer 0

#[test]
fn test_response_new() {
    struct Response<T> {
        head: Parts,
        body: T,
    }

    struct Parts;

    impl Parts {
        fn new() -> Self {
            Parts
        }
    }

    impl<T> Response<T> {
        fn new(body: T) -> Response<T> {
            Response {
                head: Parts::new(),
                body,
            }
        }

        fn status(&self) -> StatusCode {
            StatusCode::OK
        }

        fn body(&self) -> &T {
            &self.body
        }
    }

    #[derive(PartialEq, Debug)]
    enum StatusCode {
        OK,
    }

    let response = Response::new("hello world");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "hello world");
}

