// Answer 0

#[derive(Default)]
struct Response<T> {
    body: T,
}

impl<T> Response<T> {
    pub fn body(&self) -> &T {
        &self.body
    }
}

#[test]
fn test_body_empty_string() {
    let response: Response<String> = Response::default();
    assert!(response.body().is_empty());
}

#[test]
fn test_body_non_empty_string() {
    let mut response: Response<String> = Response::default();
    response.body = String::from("Hello, world!");
    assert_eq!(response.body(), "Hello, world!");
}

#[test]
fn test_body_default_value() {
    let response: Response<u32> = Response::default();
    assert_eq!(*response.body(), 0);
}

