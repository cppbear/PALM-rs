// Answer 0

#[test]
fn test_serialize_i32() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl ser::Formatter for MockFormatter {
        fn write_i32(&mut self, writer: &mut impl io::Write, value: i32) -> Result<()> {
            write!(writer, "{}", value).map_err(Error::io)
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    // Test serializing a positive i32
    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.buffer, b"42");

    // Clear the buffer for the next test
    serializer.writer.buffer.clear();

    // Test serializing a negative i32
    let result = serializer.serialize_i32(-42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.buffer, b"-42");
}

