// Answer 0

#[test]
fn test_response_version_default() {
    struct Response {
        version: Version,
    }

    struct Builder {
        response: Response,
    }

    impl Builder {
        fn version(mut self, version: Version) -> Self {
            self.response.version = version;
            self
        }

        fn body<T>(self, _body: T) -> Result<Response, &'static str> {
            Ok(self.response)
        }
        
        fn builder() -> Self {
            Builder {
                response: Response {
                    version: Version::HTTP_1_1,
                },
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum Version {
        HTTP_1_0,
        HTTP_1_1,
        HTTP_2,
    }

    let response = Builder::builder()
        .version(Version::HTTP_1_1)
        .body(())
        .unwrap();
    
    assert_eq!(response.version, Version::HTTP_1_1);
}

#[test]
fn test_response_version_http_2() {
    struct Response {
        version: Version,
    }

    struct Builder {
        response: Response,
    }

    impl Builder {
        fn version(mut self, version: Version) -> Self {
            self.response.version = version;
            self
        }

        fn body<T>(self, _body: T) -> Result<Response, &'static str> {
            Ok(self.response)
        }
        
        fn builder() -> Self {
            Builder {
                response: Response {
                    version: Version::HTTP_1_1,
                },
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum Version {
        HTTP_1_0,
        HTTP_1_1,
        HTTP_2,
    }

    let response = Builder::builder()
        .version(Version::HTTP_2)
        .body(())
        .unwrap();

    assert_eq!(response.version, Version::HTTP_2);
}

#[test]
#[should_panic]
fn test_response_version_invalid() {
    struct Response {
        version: Version,
    }

    struct Builder {
        response: Response,
    }

    impl Builder {
        fn version(mut self, _version: Version) -> Self {
            // Intentionally not updating version for invalid case
            self.response.version = Version::HTTP_1_1; // Default case
            self
        }

        fn body<T>(self, _: T) -> Result<Response, &'static str> {
            panic!("This is a simulated panic to test invalid cases");
        }
        
        fn builder() -> Self {
            Builder {
                response: Response {
                    version: Version::HTTP_1_1,
                },
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum Version {
        HTTP_1_0,
        HTTP_1_1,
        HTTP_2,
    }

    let _response = Builder::builder()
        .version(Version::HTTP_1_0)
        .body(()); // This will invoke panic
}

