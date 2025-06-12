// Answer 0

#[test]
fn test_serialize_i16_success() {
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
            self.write(buf).map(|_| ())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_i16(&mut self, writer: &mut MockWriter, value: i16) -> Result<()> {
            writer.write(&value.to_le_bytes())?;
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    
    let result = serializer.serialize_i16(42);
    
    assert!(result.is_ok());
    let expected = 42_i16.to_le_bytes();
    assert_eq!(serializer.writer.buffer, expected);
}

#[test]
fn test_serialize_i16_failure() {
    struct FailingMockWriter;

    impl io::Write for FailingMockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::new(ErrorCode::IoError))
        }

        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            self.write(_buf).map(|_| ())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_i16(&mut self, writer: &mut FailingMockWriter, _value: i16) -> Result<()> {
            writer.write(&0u8)?;
            Ok(())
        }
    }

    let writer = FailingMockWriter;
    let formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    
    let result = serializer.serialize_i16(42);
    
    assert!(result.is_err());
}

