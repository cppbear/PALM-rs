// Answer 0

fn test_fmt_with_some_value() {
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
    cell.set(42); // Set a value for testing
    let output = format!("{:?}", cell); // Use the debug formatter

    assert_eq!(output, "OnceCell(42)"); // Check if the output matches the expected
}

#[test]
fn test_fmt_with_uninitialized() {
    use std::fmt;

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
    
    impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.get() {
                Some(v) => f.debug_tuple("OnceCell").field(v).finish(),
                None => f.write_str("OnceCell(Uninit)"),
            }
        }
    }

    let cell = OnceCell::<i32>::new(); // Create an uninitialized OnceCell
    let output = format!("{:?}", cell); // Use the debug formatter

    assert_eq!(output, "OnceCell(Uninit)"); // Check if the output matches the expected for uninitialized state
}

