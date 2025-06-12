// Answer 0

#[derive(Default)]
struct Request<T> {
    body: T,
}

impl<T> Request<T> {
    pub fn body_mut(&mut self) -> &mut T {
        &mut self.body
    }

    pub fn body(&self) -> &T {
        &self.body
    }
}

#[test]
fn test_body_mut() {
    let mut request: Request<String> = Request::default();
    request.body_mut().push_str("hello world");
    assert!(!request.body().is_empty());
}

#[test]
fn test_body_mut_empty() {
    let mut request: Request<String> = Request::default();
    assert_eq!(request.body().len(), 0);
    request.body_mut().push_str("");
    assert_eq!(request.body().len(), 0);
}

