// Answer 0

#[derive(Debug)]
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

impl std::fmt::Write for TestWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
    
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.output.push(c);
        Ok(())
    }
}

#[test]
fn test_write_literal_class_byte_valid_ascii() {
    let mut writer = TestWriter::new();
    let result = writer.write_literal_class_byte(b'A'); // 'A' is ASCII and valid
    assert!(result.is_ok());
    assert_eq!(writer.output, "A");
}

#[test]
fn test_write_literal_class_byte_control_character() {
    let mut writer = TestWriter::new();
    let result = writer.write_literal_class_byte(b'\n'); // Newline is a control character
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x0A");
}

#[test]
fn test_write_literal_class_byte_whitespace_character() {
    let mut writer = TestWriter::new();
    let result = writer.write_literal_class_byte(b' '); // Space is a whitespace character
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\x20");
}

#[test]
fn test_write_literal_class_byte_non_ascii() {
    let mut writer = TestWriter::new();
    let result = writer.write_literal_class_byte(0xFF); // Non-ASCII byte
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\xFF");
}

