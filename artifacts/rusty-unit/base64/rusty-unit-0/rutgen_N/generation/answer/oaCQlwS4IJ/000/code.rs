// Answer 0

#[derive(Default)]
struct MockStrConsumer {
    consumed: String,
}

impl MockStrConsumer {
    fn consume(&mut self, s: &str) {
        self.consumed.push_str(s);
    }
}

struct EncoderStringWriter<'a> {
    str_consumer: &'a mut MockStrConsumer,
}

impl<'a> EncoderStringWriter<'a> {
    fn new(str_consumer: &'a mut MockStrConsumer) -> Self {
        Self { str_consumer }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
        self.str_consumer.consume(s);
        Ok(buf.len())
    }
}

#[test]
fn test_write_valid_utf8() {
    let mut consumer = MockStrConsumer::default();
    let mut writer = EncoderStringWriter::new(&mut consumer);
    let input = b"Hello, World!";
    let result = writer.write(input).unwrap();
    assert_eq!(result, input.len());
    assert_eq!(consumer.consumed, "Hello, World!");
}

#[test]
#[should_panic(expected = "Input must be valid UTF-8")]
fn test_write_invalid_utf8() {
    let mut consumer = MockStrConsumer::default();
    let mut writer = EncoderStringWriter::new(&mut consumer);
    let input = &[0xFF]; // Invalid UTF-8 byte
    writer.write(input).unwrap();
}

#[test]
fn test_write_empty_input() {
    let mut consumer = MockStrConsumer::default();
    let mut writer = EncoderStringWriter::new(&mut consumer);
    let input: &[u8] = b"";
    let result = writer.write(input).unwrap();
    assert_eq!(result, input.len());
    assert_eq!(consumer.consumed, "");
}

