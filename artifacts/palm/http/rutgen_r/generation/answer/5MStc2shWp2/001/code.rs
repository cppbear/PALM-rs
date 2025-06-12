// Answer 0

#[test]
fn test_method_ref_default_get() {
    struct RequestBuilder {
        inner: Result<RequestInner, ()>, 
    }
    
    struct RequestInner {
        method: Method,
    }
    
    impl RequestBuilder {
        fn builder() -> Self {
            RequestBuilder {
                inner: Ok(RequestInner { method: Method::GET }),
            }
        }
        
        fn method(mut self, method: &str) -> Self {
            self.inner = Ok(RequestInner {
                method: Method::from_str(method).unwrap(),
            });
            self
        }
        
        fn method_ref(&self) -> Option<&Method> {
            self.inner.as_ref().ok().map(|h| &h.method)
        }
    }

    #[derive(Debug, PartialEq)]
    enum Method {
        GET,
        POST,
    }

    impl Method {
        fn from_str(s: &str) -> Result<Method, ()> {
            match s {
                "GET" => Ok(Method::GET),
                "POST" => Ok(Method::POST),
                _ => Err(()),
            }
        }
    }

    let req = RequestBuilder::builder();
    assert_eq!(req.method_ref(), Some(&Method::GET));
}

#[test]
fn test_method_ref_post() {
    struct RequestBuilder {
        inner: Result<RequestInner, ()>, 
    }
    
    struct RequestInner {
        method: Method,
    }
    
    impl RequestBuilder {
        fn builder() -> Self {
            RequestBuilder {
                inner: Ok(RequestInner { method: Method::GET }),
            }
        }
        
        fn method(mut self, method: &str) -> Self {
            self.inner = Ok(RequestInner {
                method: Method::from_str(method).unwrap(),
            });
            self
        }
        
        fn method_ref(&self) -> Option<&Method> {
            self.inner.as_ref().ok().map(|h| &h.method)
        }
    }

    #[derive(Debug, PartialEq)]
    enum Method {
        GET,
        POST,
    }

    impl Method {
        fn from_str(s: &str) -> Result<Method, ()> {
            match s {
                "GET" => Ok(Method::GET),
                "POST" => Ok(Method::POST),
                _ => Err(()),
            }
        }
    }

    let req = RequestBuilder::builder().method("POST");
    assert_eq!(req.method_ref(), Some(&Method::POST));
}

#[test]
fn test_method_ref_invalid_method() {
    struct RequestBuilder {
        inner: Result<RequestInner, ()>, 
    }
    
    struct RequestInner {
        method: Method,
    }
    
    impl RequestBuilder {
        fn builder() -> Self {
            RequestBuilder {
                inner: Err(()), // Simulate an error
            }
        }
        
        fn method(mut self, method: &str) -> Self {
            self.inner = Ok(RequestInner {
                method: Method::from_str(method).unwrap(),
            });
            self
        }
        
        fn method_ref(&self) -> Option<&Method> {
            self.inner.as_ref().ok().map(|h| &h.method)
        }
    }

    #[derive(Debug, PartialEq)]
    enum Method {
        GET,
        POST,
    }

    impl Method {
        fn from_str(s: &str) -> Result<Method, ()> {
            match s {
                "GET" => Ok(Method::GET),
                "POST" => Ok(Method::POST),
                _ => Err(()),
            }
        }
    }

    let req = RequestBuilder::builder();
    assert_eq!(req.method_ref(), None);
}

