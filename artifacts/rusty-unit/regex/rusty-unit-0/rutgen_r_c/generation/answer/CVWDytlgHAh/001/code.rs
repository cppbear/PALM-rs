// Answer 0

#[test]
fn test_write_literal_byte_valid_char() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    // Test with a valid ASCII character (e.g., 'A' = 65)
    let result = writer.write_literal_byte(65);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "A");
}

#[test]
fn test_write_literal_byte_boundary_condition() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    // Test with boundary value (c == 0x7F)
    let result = writer.write_literal_byte(127);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "(?-u:\\x7F)");
}

#[test]
fn test_write_literal_byte_control_character() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    // Test with a control character (e.g., 0x00)
    let result = writer.write_literal_byte(0);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "(?-u:\\x00)");
}

#[test]
fn test_write_literal_byte_whitespace_character() {
    use std::fmt::Write;

    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut mock_writer };

    // Test with a whitespace character (e.g., space = 0x20)
    let result = writer.write_literal_byte(32);
    assert!(result.is_ok());
    assert_eq!(mock_writer.output, "(?-u:\\x20)");
}

