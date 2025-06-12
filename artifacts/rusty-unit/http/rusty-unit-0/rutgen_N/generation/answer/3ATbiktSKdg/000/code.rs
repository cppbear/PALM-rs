// Answer 0

#[test]
fn test_method_returns_builder() {
    struct RequestBuilder {
        method: Option<String>,
    }

    impl RequestBuilder {
        fn method<T>(self, method: T) -> RequestBuilder
        where
            T: TryInto<Method, Error = crate::Error>,
        {
            let method = method.try_into().expect("Failed to convert");
            RequestBuilder {
                method: Some(method.to_string()),
            }
        }

        fn build(self) -> Result<Request, crate::Error> {
            // Placeholder for building the actual Request
            Ok(Request {
                method: self.method.ok_or(crate::Error::MissingMethod)?,
            })
        }
    }

    struct Request {
        method: String,
    }

    let req_builder = RequestBuilder {
        method: None,
    };

    let req = req_builder
        .method("POST")
        .build()
        .expect("Failed to build request");

    assert_eq!(req.method, "POST");
}

#[test]
fn test_method_default_get() {
    struct RequestBuilder {
        method: Option<String>,
    }

    impl RequestBuilder {
        fn method<T>(self, method: T) -> RequestBuilder
        where
            T: TryInto<Method, Error = crate::Error>,
        {
            let method = method.try_into().expect("Failed to convert");
            RequestBuilder {
                method: Some(method.to_string()),
            }
        }

        fn build(self) -> Result<Request, crate::Error> {
            // This implementation specifies that the default method is GET
            if self.method.is_none() {
                return Ok(Request {
                    method: "GET".to_string(),
                });
            }
            Ok(Request {
                method: self.method.ok_or(crate::Error::MissingMethod)?,
            })
        }
    }

    struct Request {
        method: String,
    }

    let req_builder = RequestBuilder {
        method: None,
    };

    let req = req_builder.build().expect("Failed to build request");

    assert_eq!(req.method, "GET");
}

#[test]
#[should_panic(expected = "Failed to convert")]
fn test_method_invalid() {
    struct RequestBuilder {
        method: Option<String>,
    }

    impl RequestBuilder {
        fn method<T>(self, method: T) -> RequestBuilder
        where
            T: TryInto<Method, Error = crate::Error>,
        {
            let method = method.try_into().expect("Failed to convert");
            RequestBuilder {
                method: Some(method.to_string()),
            }
        }

        fn build(self) -> Result<Request, crate::Error> {
            Ok(Request {
                method: self.method.ok_or(crate::Error::MissingMethod)?,
            })
        }
    }

    struct Request {
        method: String,
    }

    let req_builder = RequestBuilder {
        method: None,
    };

    // Attempting to call `method` with an invalid type to trigger panic
    let _req = req_builder
        .method(123) // This should fail
        .build()
        .expect("Failed to build request");
}

