// Answer 0

#[test]
fn test_write_literal_class_byte_high_value() {
    use std::fmt::{self, Write};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    // Testing with a byte greater than 0x7F
    let result = writer.write_literal_class_byte(0x80);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x80");

    // Testing with a byte equal to 0xFF (max value)
    writer.output.clear(); // Resetting output for the next test
    let result = writer.write_literal_class_byte(0xFF);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\xFF");
}

