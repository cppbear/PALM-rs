// Answer 0

#[test]
fn test_serialize_f32_infinite() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
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

    impl MockFormatter {
        fn new() -> Self {
            Self
        }
        
        fn write_null(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write_all(b"null")?;
            Ok(())
        }
        
        fn write_f32(&mut self, writer: &mut dyn io::Write, _value: f32) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = MockWriter::new();
    let formatter = MockFormatter::new();
    let mut serializer = &mut Serializer { writer, formatter };

    // Test with positive infinity
    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"null");

    // Reset the output for next case
    serializer.writer.output.clear();

    // Test with negative infinity
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"null");

    // Reset the output for next case
    serializer.writer.output.clear();

    // Test with NaN
    let result = serializer.serialize_f32(f32::NAN);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"null");
}

