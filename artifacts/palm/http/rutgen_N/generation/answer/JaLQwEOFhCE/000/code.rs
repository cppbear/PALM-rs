// Answer 0

#[derive(Debug)]
struct MyOption<T> {
    value: Option<T>,
}

impl<T> MyOption<T> {
    fn is_none(&self) -> bool {
        self.value.is_none()
    }

    fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[test]
fn test_is_some_when_some() {
    let opt = MyOption { value: Some(42) };
    assert!(opt.is_some());
}

#[test]
fn test_is_some_when_none() {
    let opt: MyOption<i32> = MyOption { value: None };
    assert!(!opt.is_some());
}

