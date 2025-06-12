// Answer 0

#[test]
fn test_once_cell_debug_with_initialized_value() {
    // Define a helper struct to use with OnceCell
    struct TestValue {
        data: i32,
    }

    impl fmt::Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestValue({})", self.data)
        }
    }

    // Create an instance of OnceCell with a value
    let once_cell = OnceCell::new();
    once_cell.set(TestValue { data: 42 }).expect("Failed to set value");

    // Create a formatter to test the fmt function
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        once_cell.fmt(&mut formatter).expect("Formatting failed");
    }

    // Assert the output matches the expected format
    assert_eq!(output, "OnceCell(TestValue(42))");
}

#[test]
fn test_once_cell_debug_with_uninitialized_value() {
    // Create an instance of OnceCell without a value
    let once_cell = OnceCell::new();

    // Create a formatter to test the fmt function
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        once_cell.fmt(&mut formatter).expect("Formatting failed");
    }

    // Assert the output matches the expected format of uninitialized
    assert_eq!(output, "OnceCell(Uninit)");
}

