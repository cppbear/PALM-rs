// Answer 0

#[test]
fn test_fmt() {
    // Define a struct that implements the required trait
    struct TestStruct;

    impl TestStruct {
        fn description(&self) -> &str {
            "Test description"
        }
    }

    let test_object = TestStruct;

    // Call the fmt method to ensure it works correctly
    let mut output = String::new();
    let result = write!(&mut output, "{}", test_object);

    // Check if the write was successful and the output is as expected
    assert!(result.is_ok());
    assert_eq!(output, "Test description");
}

#[test]
#[should_panic(expected = "write_str called on a failed formatting")]
fn test_fmt_panics_on_write_error() {
    struct PanicOnWrite;

    impl PanicOnWrite {
        fn description(&self) -> &str {
            "Panic description"
        }
    }

    let panic_object = PanicOnWrite;

    // Create a writer that fails on write
    struct FailingWriter;

    impl std::fmt::Write for FailingWriter {
        fn write_str(&mut self, _s: &str) -> std::fmt::Result {
            Err(std::fmt::Error) // Simulate a write failure
        }
    }

    let mut failing_writer = FailingWriter;
    
    // This should panic because of write failure
    let _ = write!(&mut failing_writer, "{}", panic_object);
}

