// Answer 0

#[test]
fn test_fmt_with_initialized_once_cell() {
    use std::fmt;

    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T: fmt::Debug> OnceCell<T> {
        fn new(value: T) -> Self {
            OnceCell { value: Some(value) }
        }

        fn get(&self) -> &Option<T> {
            &self.value
        }
    }

    impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
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

    let cell_str = OnceCell::new("hello");
    let result_str = format!("{:?}", cell_str);
    assert_eq!(result_str, "OnceCell(\"hello\")");
}

#[test]
fn test_fmt_with_uninitialized_once_cell() {
    use std::fmt;

    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T: fmt::Debug> OnceCell<T> {
        fn new_uninit() -> Self {
            OnceCell { value: None }
        }

        fn get(&self) -> &Option<T> {
            &self.value
        }
    }

    impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let uninit_cell: OnceCell<i32> = OnceCell::new_uninit();
    let result = format!("{:?}", uninit_cell);
    assert_eq!(result, "OnceCell(Uninit)");
}

