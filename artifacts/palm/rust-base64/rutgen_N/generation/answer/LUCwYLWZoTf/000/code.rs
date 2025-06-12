// Answer 0

#[derive(Default)]
struct StringWriter {
    content: String,
}

impl StringWriter {
    fn consume(&mut self, buf: &str) {
        self.content.push_str(buf);
    }
}

#[test]
fn test_consume_empty_string() {
    let mut writer = StringWriter::default();
    writer.consume("");
    assert_eq!(writer.content, "");
}

#[test]
fn test_consume_non_empty_string() {
    let mut writer = StringWriter::default();
    writer.consume("Hello, ");
    writer.consume("World!");
    assert_eq!(writer.content, "Hello, World!");
}

#[test]
fn test_consume_whitespace_string() {
    let mut writer = StringWriter::default();
    writer.consume("   ");
    assert_eq!(writer.content, "   ");
}

#[test]
fn test_consume_special_characters() {
    let mut writer = StringWriter::default();
    writer.consume("!@#$%^&*()");
    assert_eq!(writer.content, "!@#$%^&*()");
}

#[test]
fn test_consume_multiple_times() {
    let mut writer = StringWriter::default();
    writer.consume("First part.");
    writer.consume(" Second part.");
    writer.consume(" Third part.");
    assert_eq!(writer.content, "First part. Second part. Third part.");
}

