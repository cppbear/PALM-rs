// Answer 0

#[test]
fn test_write_literal_byte_valid_character() {
    use std::fmt::Write;

    struct StringWriter {
        output: String,
    }

    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = StringWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let result = writer_instance.write_literal_byte(97); // 'a'
    assert!(result.is_ok());
    assert_eq!(writer.output, "a");
}

#[test]
fn test_write_literal_byte_control_character() {
    use std::fmt::Write;

    struct StringWriter {
        output: String,
    }

    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = StringWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let result = writer_instance.write_literal_byte(0); // control character
    assert!(result.is_ok());
    assert_eq!(writer.output, "(?-u:\\x00)");
}

#[test]
fn test_write_literal_byte_whitespace_character() {
    use std::fmt::Write;

    struct StringWriter {
        output: String,
    }

    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = StringWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let result = writer_instance.write_literal_byte(9); // tab character
    assert!(result.is_ok());
    assert_eq!(writer.output, "(?-u:\\x09)");
}

#[test]
fn test_write_literal_byte_non_ascii_character() {
    use std::fmt::Write;

    struct StringWriter {
        output: String,
    }

    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = StringWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };

    let result = writer_instance.write_literal_byte(255); // non-ASCII character
    assert!(result.is_ok());
    assert_eq!(writer.output, "(?-u:\\xFF)");
}

