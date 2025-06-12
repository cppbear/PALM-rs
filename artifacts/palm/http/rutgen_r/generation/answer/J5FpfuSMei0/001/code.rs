// Answer 0

#[derive(Default)]
struct Head {
    version: Version,
}

#[derive(Default)]
struct Request<T> {
    head: Head,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Request<T> {
    pub fn default() -> Self {
        Request {
            head: Head {
                version: Version::HTTP_11,
            },
            _marker: std::marker::PhantomData,
        }
    }

    pub fn version(&self) -> Version {
        self.head.version
    }
}

#[derive(Debug, PartialEq)]
enum Version {
    HTTP_10,
    HTTP_11,
    HTTP_2,
}

#[test]
fn test_request_default_version() {
    let request: Request<()> = Request::default();
    assert_eq!(request.version(), Version::HTTP_11);
}

#[test]
fn test_request_version_http_10() {
    struct CustomRequest;
    
    impl CustomRequest {
        fn new_http_10() -> Request<()> {
            Request {
                head: Head {
                    version: Version::HTTP_10,
                },
                _marker: std::marker::PhantomData,
            }
        }
    }

    let request = CustomRequest::new_http_10();
    assert_eq!(request.version(), Version::HTTP_10);
}

#[test]
fn test_request_version_http_2() {
    struct CustomRequest;
    
    impl CustomRequest {
        fn new_http_2() -> Request<()> {
            Request {
                head: Head {
                    version: Version::HTTP_2,
                },
                _marker: std::marker::PhantomData,
            }
        }
    }

    let request = CustomRequest::new_http_2();
    assert_eq!(request.version(), Version::HTTP_2);
}

