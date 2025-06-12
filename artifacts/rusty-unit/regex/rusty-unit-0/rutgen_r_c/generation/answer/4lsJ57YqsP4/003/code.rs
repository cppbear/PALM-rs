// Answer 0

#[test]
fn test_write_literal_class_byte_normal_character() {
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

    let mut test_writer = TestWriter {
        output: String::new(),
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut test_writer,
    };

    let result = writer.write_literal_class_byte(65); // 'A'
    assert!(result.is_ok());
    assert_eq!(test_writer.output, "A");
}

#[test]
fn test_write_literal_class_byte_control_character() {
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

    let mut test_writer = TestWriter {
        output: String::new(),
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut test_writer,
    };

    let result = writer.write_literal_class_byte(7); // Bell character
    assert!(result.is_ok());
    assert_eq!(test_writer.output, "\\x07");
}

#[test]
fn test_write_literal_class_byte_whitespace_character() {
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

    let mut test_writer = TestWriter {
        output: String::new(),
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut test_writer,
    };

    let result = writer.write_literal_class_byte(32); // Space character
    assert!(result.is_ok());
    assert_eq!(test_writer.output, "\\x20");
}

#[test]
fn test_write_literal_class_byte_boundary_character() {
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

    let mut test_writer = TestWriter {
        output: String::new(),
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut test_writer,
    };

    let result = writer.write_literal_class_byte(127); // Delete character
    assert!(result.is_ok());
    assert_eq!(test_writer.output, "\\x7F");
}

