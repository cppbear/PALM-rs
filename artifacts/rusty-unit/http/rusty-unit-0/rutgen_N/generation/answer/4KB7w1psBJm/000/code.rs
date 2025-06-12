// Answer 0

#[test]
fn test_uri_mut() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        uri: Uri,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Request {
                head: Head {
                    uri: Uri::default(),
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    #[derive(Default, Clone, PartialEq)]
    struct Uri(String);

    impl Uri {
        fn parse(input: &str) -> Result<Self, &'static str> {
            Ok(Uri(input.to_string()))
        }
    }

    let mut request: Request<()> = Request::default();
    *request.uri_mut() = Uri::parse("/hello").unwrap();
    assert_eq!(request.head.uri, Uri("/hello".to_string()));
}

#[test]
fn test_uri_mut_with_empty_string() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        uri: Uri,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Request {
                head: Head {
                    uri: Uri::default(),
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    #[derive(Default, Clone, PartialEq)]
    struct Uri(String);

    impl Uri {
        fn parse(input: &str) -> Result<Self, &'static str> {
            Ok(Uri(input.to_string()))
        }
    }

    let mut request: Request<()> = Request::default();
    *request.uri_mut() = Uri::parse("").unwrap();
    assert_eq!(request.head.uri, Uri("".to_string()));
}

