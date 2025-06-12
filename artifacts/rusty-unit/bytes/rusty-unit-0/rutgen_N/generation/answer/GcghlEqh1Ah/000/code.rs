// Answer 0

#[derive(Debug)]
struct Limit<T> {
    inner: T,
}

impl<T> Limit<T> {
    pub fn new(inner: T) -> Self {
        Limit { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

#[test]
fn test_into_inner() {
    let limit = Limit::new(42);
    assert_eq!(limit.into_inner(), 42);
}

#[test]
fn test_into_inner_string() {
    let limit = Limit::new(String::from("Hello, world!"));
    assert_eq!(limit.into_inner(), "Hello, world!");
}

