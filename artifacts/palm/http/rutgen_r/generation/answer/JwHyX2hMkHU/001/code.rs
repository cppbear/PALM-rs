// Answer 0

#[test]
fn test_map_function_with_string_body() {
    struct Response<T> {
        body: T,
        head: (),
    }

    impl<T> Response<T> {
        fn builder() -> Self {
            Response {
                body: (),
                head: (),
            }
        }

        fn body(&self) -> &T {
            &self.body
        }

        fn unwrap(self) -> Self {
            self
        }

        fn map<F, U>(self, f: F) -> Response<U>
        where
            F: FnOnce(T) -> U,
        {
            Response {
                body: f(self.body),
                head: self.head,
            }
        }
    }

    let response = Response::builder().body("some string").unwrap();
    let mapped_response: Response<&[u8]> = response.map(|b| {
        assert_eq!(b, "some string");
        b.as_bytes()
    });
    assert_eq!(mapped_response.body(), &"some string".as_bytes());
}

#[test]
fn test_map_function_with_empty_string_body() {
    struct Response<T> {
        body: T,
        head: (),
    }

    impl<T> Response<T> {
        fn builder() -> Self {
            Response {
                body: (),
                head: (),
            }
        }

        fn body(&self) -> &T {
            &self.body
        }

        fn unwrap(self) -> Self {
            self
        }

        fn map<F, U>(self, f: F) -> Response<U>
        where
            F: FnOnce(T) -> U,
        {
            Response {
                body: f(self.body),
                head: self.head,
            }
        }
    }

    let response = Response::builder().body("").unwrap();
    let mapped_response: Response<&[u8]> = response.map(|b| {
        assert_eq!(b, "");
        b.as_bytes()
    });
    assert_eq!(mapped_response.body(), &"".as_bytes());
}

#[test]
#[should_panic]
fn test_map_function_panic_condition() {
    struct Response<T> {
        body: T,
        head: (),
    }

    impl<T> Response<T> {
        fn builder() -> Self {
            Response {
                body: (),
                head: (),
            }
        }

        fn body(&self) -> &T {
            &self.body
        }

        fn unwrap(self) -> Self {
            self
        }

        fn map<F, U>(self, f: F) -> Response<U>
        where
            F: FnOnce(T) -> U,
        {
            Response {
                body: f(self.body),
                head: self.head,
            }
        }
    }

    let response = Response::builder().body("string").unwrap();
    let _: Response<&[u8]> = response.map(|b| {
        panic!("This is a panic condition");
    });
}

