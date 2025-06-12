// Answer 0

#[test]
fn test_write_literal_byte_valid() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };

    // Test with a valid byte that is not control or whitespace
    let result = writer.write_literal_byte(65); // 'A'
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "A");
}

#[test]
fn test_write_literal_byte_control_character() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };

    // Test with a control character (0x00)
    let result = writer.write_literal_byte(0);
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "(?-u:\\x00)");
}

#[test]
fn test_write_literal_byte_whitespace_character() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };

    // Test with a whitespace character (0x20)
    let result = writer.write_literal_byte(32); // ' '
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "(?-u:\\x20)");
}

#[test]
fn test_write_literal_byte_invalid_high_character() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };

    // Test with a valid byte that should lead to the formatted output
    let result = writer.write_literal_byte(255); // Outside of 0x7F limit
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, "(?-u:\\xFF)");
}

