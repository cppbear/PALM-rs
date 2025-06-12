// Answer 0

#[test]
fn test_uri_default_request() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        uri: Uri,
        _marker: std::marker::PhantomData<T>,
    }

    struct Uri(String);

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Self {
                head: Head {
                    uri: Uri("/".to_string()),
                    _marker: std::marker::PhantomData,
                }
            }
        }
    }

    impl Uri {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    let request: Request<()> = Request::default();
    assert_eq!(request.uri().as_str(), "/");
}

#[test]
fn test_uri_custom_request() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        uri: Uri,
        _marker: std::marker::PhantomData<T>,
    }

    struct Uri(String);

    impl<T> Request<T> {
        pub fn new(uri: &str) -> Self {
            Self {
                head: Head {
                    uri: Uri(uri.to_string()),
                    _marker: std::marker::PhantomData,
                }
            }
        }
    }

    impl Uri {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    let request: Request<()> = Request::new("/custom-uri");
    assert_eq!(request.uri().as_str(), "/custom-uri");
}

#[should_panic]
#[test]
fn test_uri_empty_request_panics() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        uri: Uri,
        _marker: std::marker::PhantomData<T>,
    }

    struct Uri(String);

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Self {
                head: Head {
                    uri: Uri("".to_string()),
                    _marker: std::marker::PhantomData,
                }
            }
        }
    }

    impl Uri {
        pub fn as_str(&self) -> &str {
            if self.0.is_empty() {
                panic!("URI should not be empty");
            }
            &self.0
        }
    }

    let request: Request<()> = Request::default();
    let _ = request.uri().as_str(); // this should panic
}

