// Answer 0

#[test]
fn test_serialize_i128() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { buffer: Vec::new() }
        }
        
        fn get_buffer(&self) -> &[u8] {
            &self.buffer
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

    impl MockFormatter {
        fn write_i128(&self, _writer: &mut dyn io::Write, value: i128) -> Result<()> {
            let str_value = value.to_string().into_bytes();
            _writer.write_all(&str_value)
        }
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    // Test with a simple positive i128 value
    let result = serializer.serialize_i128(123456789012345678901234567890i128);
    assert!(result.is_ok());

    // Test with a negative i128 value
    let result = serializer.serialize_i128(-123456789012345678901234567890i128);
    assert!(result.is_ok());

    // Check the output in the mock writer
    let output = serializer.writer.get_buffer();
    assert_eq!(output, b"-123456789012345678901234567890"); // check the final buffer
}

#[test]
fn test_serialize_i128_zero() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { buffer: Vec::new() }
        }
        
        fn get_buffer(&self) -> &[u8] {
            &self.buffer
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

    impl MockFormatter {
        fn write_i128(&self, _writer: &mut dyn io::Write, value: i128) -> Result<()> {
            let str_value = value.to_string().into_bytes();
            _writer.write_all(&str_value)
        }
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    // Test with zero
    let result = serializer.serialize_i128(0i128);
    assert!(result.is_ok());

    // Check the output in the mock writer
    let output = serializer.writer.get_buffer();
    assert_eq!(output, b"0"); // check the final buffer
}

