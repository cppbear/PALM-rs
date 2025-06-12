// Answer 0

#[test]
fn test_once_cell_initialized() {
    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T: std::fmt::Debug> OnceCell<T> {
        fn new(value: T) -> Self {
            OnceCell { value: Some(value) }
        }

        fn get(&self) -> &Option<T> {
            &self.value
        }
    }

    use std::fmt;

    impl<T: std::fmt::Debug> fmt::Debug for OnceCell<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let cell = OnceCell::new(42);
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(42)");
}

#[test]
fn test_once_cell_uninitialized() {
    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn get(&self) -> &Option<T> {
            &self.value
        }
    }

    use std::fmt;

    impl<T> fmt::Debug for OnceCell<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(_) => f.write_str("OnceCell(Initialized)"),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let cell: OnceCell<i32> = OnceCell::new();
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(Uninit)");
}

