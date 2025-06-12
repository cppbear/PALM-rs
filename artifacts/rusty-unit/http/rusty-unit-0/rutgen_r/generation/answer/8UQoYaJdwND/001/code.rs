// Answer 0

#[derive(Default)]
struct Request<T> {
    head: Head<T>,
}

#[derive(Default)]
struct Head<T> {
    extensions: Extensions,
}

struct Extensions;

impl Extensions {
    pub fn get<T>(&self) -> Option<&T> {
        None
    }
}

impl<T> Request<T> {
    pub fn extensions(&self) -> &Extensions {
        &self.head.extensions
    }
}

#[test]
fn test_extensions_none() {
    let request: Request<()> = Request::default();
    assert!(request.extensions().get::<i32>().is_none());
}

#[test]
fn test_extensions_empty() {
    let request: Request<()> = Request::default();
    let extensions_ref = request.extensions();
    assert!(extensions_ref.get::<String>().is_none());
}

#[test]
fn test_extensions_reference() {
    let request: Request<()> = Request::default();
    let extensions_ref = request.extensions();
    let result: Option<&i32> = extensions_ref.get();
    assert!(result.is_none()); // No panic expected
}

