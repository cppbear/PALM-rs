// Answer 0

#[test]
fn test_map_function_with_string_body() {
    struct Request<T> {
        body: T,
        head: (),
    }

    impl<T> Request<T> {
        fn builder() -> RequestBuilder<T> {
            RequestBuilder::new()
        }

        fn body(&self) -> &T {
            &self.body
        }
    }

    struct RequestBuilder<T> {
        body: Option<T>,
    }

    impl<T> RequestBuilder<T> {
        fn new() -> Self {
            RequestBuilder { body: None }
        }

        fn body(self, body: T) -> Result<Request<T>, &'static str> {
            Ok(Request {
                body,
                head: (),
            })
        }
    }

    let request = Request::builder().body("some string").unwrap();
    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b, "some string");
        b.as_bytes()
    });
    assert_eq!(mapped_request.body(), &"some string".as_bytes());
}

#[test]
fn test_map_function_with_empty_string_body() {
    struct Request<T> {
        body: T,
        head: (),
    }

    impl<T> Request<T> {
        fn builder() -> RequestBuilder<T> {
            RequestBuilder::new()
        }

        fn body(&self) -> &T {
            &self.body
        }
    }

    struct RequestBuilder<T> {
        body: Option<T>,
    }

    impl<T> RequestBuilder<T> {
        fn new() -> Self {
            RequestBuilder { body: None }
        }

        fn body(self, body: T) -> Result<Request<T>, &'static str> {
            Ok(Request {
                body,
                head: (),
            })
        }
    }

    let request = Request::builder().body("").unwrap();
    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b, "");
        b.as_bytes()
    });
    assert_eq!(mapped_request.body(), &"".as_bytes());
}

