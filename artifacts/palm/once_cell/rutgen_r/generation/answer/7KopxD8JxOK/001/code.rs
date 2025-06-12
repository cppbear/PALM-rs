// Answer 0

#[test]
fn test_once_cell_with_initialized_value() {
    use std::fmt;

    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn set(&mut self, value: T) {
            self.value = Some(value);
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

    let mut cell = OnceCell::new();
    cell.set(42); // Using an integer for testing

    // Invoke the function under test
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(42)"); // Check that the formatted result is as expected
}

#[test]
fn test_once_cell_with_debug_string() {
    use std::fmt;

    struct OnceCell<T> {
        value: Option<T>,
    }

    impl<T> OnceCell<T> {
        fn new() -> Self {
            OnceCell { value: None }
        }

        fn set(&mut self, value: T) {
            self.value = Some(value);
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

    let mut cell = OnceCell::new();
    cell.set("Hello, World!"); // Using a string for testing

    // Invoke the function under test
    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(\"Hello, World!\")"); // Check that the formatted result is as expected
}

