// Answer 0

#[test]
fn test_write_literal_class_byte_boundary_condition() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    // Test input that satisfies: c == 0x7F as char (which is a control character)
    let result = write_literal_class_byte(&mut writer, 0x7F);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x7F"); // Expected output as it is a control character

    // Test input that satisfies: c.is_control() and c <= 0x7F as char
    let control_char = 0x00; // NUL is a control character
    writer = TestWriter::new();
    let result = write_literal_class_byte(&mut writer, control_char);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x00"); // Expected output for control character NUL
}

#[test]
fn test_write_literal_class_byte_non_control() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
    }

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();

    // Test input that does not trigger any panics and is < 0x7F as char
    let regular_char = 0x41; // 'A'
    let result = write_literal_class_byte(&mut writer, regular_char);
    assert!(result.is_ok());
    assert_eq!(writer.output, "A"); // Expected output for character 'A'
}

