// Answer 0

#[derive(Default)]
struct MockWriter {
    consumed: String,
}

impl MockWriter {
    fn consume(&mut self, buf: &str) {
        self.consumed.push_str(buf);
    }
}

struct EncoderStringWriter {
    writer: MockWriter,
}

impl EncoderStringWriter {
    fn new() -> Self {
        Self {
            writer: MockWriter::default(),
        }
    }

    fn consume(&mut self, buf: &str) {
        self.writer.consume(buf);
    }
}

#[test]
fn test_consume_empty_string() {
    let mut encoder = EncoderStringWriter::new();
    encoder.consume("");
    assert_eq!(encoder.writer.consumed, "");
}

#[test]
fn test_consume_single_character() {
    let mut encoder = EncoderStringWriter::new();
    encoder.consume("a");
    assert_eq!(encoder.writer.consumed, "a");
}

#[test]
fn test_consume_multiple_characters() {
    let mut encoder = EncoderStringWriter::new();
    encoder.consume("hello");
    assert_eq!(encoder.writer.consumed, "hello");
}

#[test]
fn test_consume_special_characters() {
    let mut encoder = EncoderStringWriter::new();
    encoder.consume("!@#$%");
    assert_eq!(encoder.writer.consumed, "!@#$%");
}

#[test]
fn test_consume_large_string() {
    let mut encoder = EncoderStringWriter::new();
    let long_str = "a".repeat(1000);
    encoder.consume(&long_str);
    assert_eq!(encoder.writer.consumed, long_str);
}

