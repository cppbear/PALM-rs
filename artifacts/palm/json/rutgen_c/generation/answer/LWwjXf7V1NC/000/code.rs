// Answer 0

#[test]
fn test_serialize_u32() {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
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

    impl Formatter for MockFormatter {
        fn write_u32(&mut self, writer: &mut impl io::Write, value: u32) -> Result<()> {
            writer.write(&value.to_le_bytes())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_u32(42);
    assert!(result.is_ok());

    let expected_bytes = 42u32.to_le_bytes();
    assert_eq!(serializer.writer.buffer, expected_bytes);
}

#[test]
fn test_serialize_u32_zero() {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
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

    impl Formatter for MockFormatter {
        fn write_u32(&mut self, writer: &mut impl io::Write, value: u32) -> Result<()> {
            writer.write(&value.to_le_bytes())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_u32(0);
    assert!(result.is_ok());

    let expected_bytes = 0u32.to_le_bytes();
    assert_eq!(serializer.writer.buffer, expected_bytes);
}

