// Answer 0

#[test]
fn test_write_literal_class_byte_valid() {
    use std::fmt::Write; // Make sure to import the Write trait for use.

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }
    }

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    let result = write_literal_class_byte(&mut writer, 0x41); // 'A'
    assert!(result.is_ok());
    assert_eq!(writer.output, "A");
}

#[test]
fn test_write_literal_class_byte_boundary() {
    use std::fmt::Write;

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }
    }

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    let result = write_literal_class_byte(&mut writer, 0x7F); // 0x7F is the boundary
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x7F");
}

#[test]
fn test_write_literal_class_byte_control_character() {
    use std::fmt::Write;

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }
    }

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    let result = write_literal_class_byte(&mut writer, 0x00); // Control character
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x00");
}

#[test]
fn test_write_literal_class_byte_whitespace_character() {
    use std::fmt::Write;

    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer { output: String::new() }
        }
    }

    impl std::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    let result = write_literal_class_byte(&mut writer, 0x20); // Whitespace character ' '
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x20");
}

