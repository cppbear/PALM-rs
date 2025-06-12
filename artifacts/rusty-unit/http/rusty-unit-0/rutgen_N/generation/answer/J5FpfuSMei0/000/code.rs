// Answer 0

#[test]
fn test_version_default() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        version: Version,
        _marker: std::marker::PhantomData<T>,
    }

    #[derive(Debug, PartialEq)]
    enum Version {
        HTTP_10,
        HTTP_11,
    }

    impl<T> Default for Request<T> {
        fn default() -> Self {
            Request {
                head: Head {
                    version: Version::HTTP_11,
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    let request: Request<()> = Request::default();
    assert_eq!(request.version(), Version::HTTP_11);
}

#[test]
fn test_version_http_10() {
    struct Request<T> {
        head: Head<T>,
    }

    struct Head<T> {
        version: Version,
        _marker: std::marker::PhantomData<T>,
    }

    #[derive(Debug, PartialEq)]
    enum Version {
        HTTP_10,
        HTTP_11,
    }

    impl<T> Request<T> {
        fn new(version: Version) -> Self {
            Request {
                head: Head {
                    version,
                    _marker: std::marker::PhantomData,
                },
            }
        }

        pub fn version(&self) -> Version {
            self.head.version
        }
    }

    let request: Request<()> = Request::new(Version::HTTP_10);
    assert_eq!(request.version(), Version::HTTP_10);
}

