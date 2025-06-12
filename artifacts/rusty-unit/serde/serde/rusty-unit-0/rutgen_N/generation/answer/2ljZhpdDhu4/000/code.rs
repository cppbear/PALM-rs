// Answer 0

#[derive(Default)]
struct TestWriter {
    bytes: Vec<u8>,
    offset: usize,
}

impl TestWriter {
    fn new(size: usize) -> Self {
        TestWriter {
            bytes: vec![0; size],
            offset: 0,
        }
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.offset + s.len() > self.bytes.len() {
            Err(std::fmt::Error)
        } else {
            self.bytes[self.offset..self.offset + s.len()].copy_from_slice(s.as_bytes());
            self.offset += s.len();
            Ok(())
        }
    }
}

#[test]
fn test_write_str_valid() {
    let mut writer = TestWriter::new(10);
    let result = writer.write_str("hello");
    assert!(result.is_ok());
    assert_eq!(&writer.bytes[..writer.offset], b"hello");
}

#[test]
fn test_write_str_exceeds_capacity() {
    let mut writer = TestWriter::new(5);
    let result = writer.write_str("hello!");
    assert!(result.is_err());
    assert_eq!(writer.offset, 0); // Ensure offset was not updated
}

#[test]
fn test_write_str_empty() {
    let mut writer = TestWriter::new(10);
    let result = writer.write_str("");
    assert!(result.is_ok());
    assert_eq!(writer.offset, 0); // Offset should remain unchanged
}

#[test]
fn test_write_str_partial_fill() {
    let mut writer = TestWriter::new(8);
    let result = writer.write_str("abcd");
    assert!(result.is_ok());
    assert_eq!(&writer.bytes[..writer.offset], b"abcd");
    assert_eq!(writer.offset, 4);
}

