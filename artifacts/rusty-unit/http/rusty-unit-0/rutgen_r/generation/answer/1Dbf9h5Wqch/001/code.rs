// Answer 0

#[test]
fn test_map_with_string_body() {
    struct Request<T> {
        body: T,
        head: (),
    }
    
    impl<T> Request<T> {
        fn builder() -> RequestBuilder<T> {
            RequestBuilder::default()
        }
        
        fn body(&self) -> &T {
            &self.body
        }
    }
    
    #[derive(Default)]
    struct RequestBuilder<T> {
        body: Option<T>,
    }
    
    impl<T> RequestBuilder<T> {
        fn body(mut self, body: T) -> Self {
            self.body = Some(body);
            self
        }
        
        fn unwrap(self) -> Request<T> {
            Request {
                body: self.body.expect("body should be set"),
                head: (),
            }
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
fn test_map_with_empty_string_body() {
    struct Request<T> {
        body: T,
        head: (),
    }
    
    impl<T> Request<T> {
        fn builder() -> RequestBuilder<T> {
            RequestBuilder::default()
        }
        
        fn body(&self) -> &T {
            &self.body
        }
    }
    
    #[derive(Default)]
    struct RequestBuilder<T> {
        body: Option<T>,
    }
    
    impl<T> RequestBuilder<T> {
        fn body(mut self, body: T) -> Self {
            self.body = Some(body);
            self
        }
        
        fn unwrap(self) -> Request<T> {
            Request {
                body: self.body.expect("body should be set"),
                head: (),
            }
        }
    }
    
    let request = Request::builder().body("").unwrap();
    let mapped_request: Request<&[u8]> = request.map(|b| {
        assert_eq!(b, "");
        b.as_bytes()
    });
    assert_eq!(mapped_request.body(), &"".as_bytes());
}

#[test]
fn test_map_with_numeric_body() {
    struct Request<T> {
        body: T,
        head: (),
    }

    impl<T> Request<T> {
        fn builder() -> RequestBuilder<T> {
            RequestBuilder::default()
        }

        fn body(&self) -> &T {
            &self.body
        }
    }

    #[derive(Default)]
    struct RequestBuilder<T> {
        body: Option<T>,
    }

    impl<T> RequestBuilder<T> {
        fn body(mut self, body: T) -> Self {
            self.body = Some(body);
            self
        }

        fn unwrap(self) -> Request<T> {
            Request {
                body: self.body.expect("body should be set"),
                head: (),
            }
        }
    }

    let request = Request::builder().body(42).unwrap();
    let mapped_request: Request<i32> = request.map(|b| {
        assert_eq!(b, 42);
        b * 2
    });
    assert_eq!(mapped_request.body(), &84);
}

