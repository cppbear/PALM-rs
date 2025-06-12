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
fn test_body_with_empty_string() {
    let response: Response<String> = Response::default();
    assert!(response.body().is_empty());
}

#[test]
fn test_body_with_non_empty_string() {
    let mut response: Response<String> = Response::default();
    response.body = String::from("Hello, World!");
    assert_eq!(response.body(), &String::from("Hello, World!"));
}

#[test]
fn test_body_with_integer() {
    #[derive(Default)]
    struct IntResponse {
        body: i32,
    }

    let response: IntResponse = IntResponse::default();
    assert_eq!(*response.body(), 0);
}

#[test]
fn test_body_with_float() {
    #[derive(Default)]
    struct FloatResponse {
        body: f64,
    }

    let response: FloatResponse = FloatResponse::default();
    assert_eq!(*response.body(), 0.0);
}

