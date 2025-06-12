// Answer 0

#[test]
fn test_request_new_with_string() {
    struct Request<T> {
        head: Parts,
        body: T,
    }

    struct Parts;

    impl Parts {
        fn new() -> Self {
            Parts
        }
    }

    enum Method {
        GET,
    }

    impl Request<String> {
        fn method(&self) -> &Method {
            &Method::GET
        }

        fn body(&self) -> &T {
            &self.body
        }
    }

    fn new(body: String) -> Request<String> {
        Request {
            head: Parts::new(),
            body,
        }
    }

    let request = new("hello world".to_string());

    if let Method::GET = *request.method() {
        assert_eq!(request.body(), "hello world");
    } else {
        panic!("Expected method to be GET");
    }
}

#[test]
fn test_request_new_with_empty_string() {
    struct Request<T> {
        head: Parts,
        body: T,
    }

    struct Parts;

    impl Parts {
        fn new() -> Self {
            Parts
        }
    }

    enum Method {
        GET,
    }

    impl Request<String> {
        fn method(&self) -> &Method {
            &Method::GET
        }

        fn body(&self) -> &T {
            &self.body
        }
    }

    fn new(body: String) -> Request<String> {
        Request {
            head: Parts::new(),
            body,
        }
    }

    let request = new("".to_string());

    if let Method::GET = *request.method() {
        assert_eq!(request.body(), "");
    } else {
        panic!("Expected method to be GET");
    }
}

