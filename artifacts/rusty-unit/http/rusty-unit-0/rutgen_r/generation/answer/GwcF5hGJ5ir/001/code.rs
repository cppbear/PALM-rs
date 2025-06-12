// Answer 0

#[test]
fn test_from_parts_valid_request() {
    struct Parts {
        method: Method,
    }

    struct Method {
        name: String,
    }

    struct Request<T> {
        head: Parts,
        body: T,
    }

    impl Method {
        fn new(name: &str) -> Self {
            Method {
                name: name.to_string(),
            }
        }
    }

    let method = Method::new("GET");
    let parts = Parts { method };
    let body = "This is a test body.";

    let request = from_parts(parts, body);
    
    assert_eq!(request.head.method.name, "GET");
    assert_eq!(request.body, "This is a test body.");
}

#[test]
fn test_from_parts_post_request() {
    struct Parts {
        method: Method,
    }

    struct Method {
        name: String,
    }

    struct Request<T> {
        head: Parts,
        body: T,
    }

    impl Method {
        fn new(name: &str) -> Self {
            Method {
                name: name.to_string(),
            }
        }
    }

    let method = Method::new("POST");
    let parts = Parts { method };
    let body = "Another test body.";

    let request = from_parts(parts, body);
    
    assert_eq!(request.head.method.name, "POST");
    assert_eq!(request.body, "Another test body.");
}

#[should_panic]
#[test]
fn test_from_parts_empty_body() {
    struct Parts {
        method: Method,
    }

    struct Method {
        name: String,
    }

    struct Request<T> {
        head: Parts,
        body: T,
    }

    impl Method {
        fn new(name: &str) -> Self {
            Method {
                name: name.to_string(),
            }
        }
    }

    let method = Method::new("DELETE");
    let parts = Parts { method };

    // Assuming from_parts panics on empty body
    let _request = from_parts(parts, "");
}

