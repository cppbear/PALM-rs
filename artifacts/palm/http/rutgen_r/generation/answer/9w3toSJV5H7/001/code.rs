// Answer 0

#[derive(Debug, PartialEq)]
struct Method {
    name: &'static str,
}

#[derive(Debug, PartialEq)]
struct Parts {
    method: Method,
}

struct Request<T> {
    head: Parts,
    body: T,
}

impl<T> Request<T> {
    fn new(body: T) -> Self {
        Self {
            head: Parts {
                method: Method { name: "GET" },
            },
            body,
        }
    }

    pub fn into_parts(self) -> (Parts, T) {
        (self.head, self.body)
    }
}

#[test]
fn test_into_parts_with_empty_body() {
    let request = Request::new(());
    let (parts, body) = request.into_parts();
    assert_eq!(parts.method, Method { name: "GET" });
    assert_eq!(body, ());
}

#[test]
fn test_into_parts_with_string_body() {
    let body = String::from("Hello, World!");
    let request = Request::new(body.clone());
    let (parts, extracted_body) = request.into_parts();
    assert_eq!(parts.method, Method { name: "GET" });
    assert_eq!(extracted_body, body);
}

#[test]
fn test_into_parts_with_numeric_body() {
    let body = 42;
    let request = Request::new(body);
    let (parts, extracted_body) = request.into_parts();
    assert_eq!(parts.method, Method { name: "GET" });
    assert_eq!(extracted_body, 42);
}

