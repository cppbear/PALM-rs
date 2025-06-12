// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct MyType {
        value: i32,
    }

    struct Lazy<T> {
        inner: Option<T>,
    }

    impl<T> Lazy<T> {
        fn new() -> Self {
            Lazy { inner: None }
        }

        fn force(&self) -> &T {
            self.inner.as_ref().unwrap()
        }
    }

    impl MyType {
        fn new(value: i32) -> Self {
            MyType { value }
        }
    }

    #[test]
    fn test_deref() {
        let my_value = MyType::new(42);
        let lazy_instance = Lazy { inner: Some(my_value) };

        assert_eq!(lazy_instance.force().value, 42);
    }

    #[test]
    #[should_panic]
    fn test_deref_should_panic_when_none() {
        let lazy_instance: Lazy<MyType> = Lazy::new();

        // This should panic since there's no value inside
        let _ = lazy_instance.force();
    }
}

