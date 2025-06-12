// Answer 0

#[test]
fn test_serialize_u128() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_u128(&mut self, writer: &mut Vec<u8>, value: u128) -> Result<()> {
            // Just for testing, we will simulate writing u128 value as bytes
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { buffer: Vec::new() }
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;
    
    let serializer = &mut Serializer { writer, formatter };

    // Test with a regular u128 value
    let result = serializer.serialize_u128(123456789012345678901234567890123456789).unwrap();
    assert!(result.is_ok());

    // Test with 0
    let result_zero = serializer.serialize_u128(0).unwrap();
    assert!(result_zero.is_ok());

    // Test with maximum u128 value
    let result_max = serializer.serialize_u128(u128::MAX).unwrap();
    assert!(result_max.is_ok());
}

