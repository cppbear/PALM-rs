// Answer 0

#[test]
fn test_unexpected_bytes_display() {
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // This implementation does nothing, just for testing purposes
            Ok(())
        }
    }

    let unexpected_value = Unexpected::Bytes(&[1, 2, 3, 4]);
    let mut formatter = TestFormatter;
    let result = unexpected_value.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_unexpected_bytes_display_empty_array() {
    struct TestFormatter;
    
    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // This implementation does nothing, just for testing purposes
            Ok(())
        }
    }

    let unexpected_value = Unexpected::Bytes(&[]);
    let mut formatter = TestFormatter;
    let result = unexpected_value.fmt(&mut formatter);
    assert!(result.is_ok());
}

