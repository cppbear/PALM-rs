// Answer 0

#[test]
fn test_method_default() {
    struct Request<T> {
        head: Head<T>,
    }
    
    struct Head<T> {
        method: Method,
        phantom: std::marker::PhantomData<T>,
    }

    #[derive(PartialEq, Debug)]
    enum Method {
        GET,
        POST,
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Request {
                head: Head {
                    method: Method::GET,
                    phantom: std::marker::PhantomData,
                },
            }
        }
    }

    let request: Request<()> = Request::default();
    assert_eq!(*request.method(), Method::GET);
}

#[test]
fn test_method_custom() {
    struct Request<T> {
        head: Head<T>,
    }
    
    struct Head<T> {
        method: Method,
        phantom: std::marker::PhantomData<T>,
    }

    #[derive(PartialEq, Debug)]
    enum Method {
        GET,
        POST,
    }

    impl<T> Request<T> {
        fn new(method: Method) -> Self {
            Request {
                head: Head {
                    method,
                    phantom: std::marker::PhantomData,
                },
            }
        }

        pub fn method(&self) -> &Method {
            &self.head.method
        }
    }

    let request: Request<()> = Request::new(Method::POST);
    assert_eq!(*request.method(), Method::POST);
}

