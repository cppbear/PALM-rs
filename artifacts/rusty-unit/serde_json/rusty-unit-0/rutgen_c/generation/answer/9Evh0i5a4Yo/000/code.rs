// Answer 0

#[test]
fn test_serialize_bool_true() {
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

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter
        }

        fn write_bool(&self, writer: &mut dyn io::Write, value: bool) -> Result<()> {
            let output = if value { b"true" } else { b"false" };
            writer.write_all(output)
        }
    }

    let mock_writer = &mut MockWriter::new();
    let formatter = MockFormatter::new();

    let serializer = &mut Serializer {
        writer: mock_writer,
        formatter,
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
    assert_eq!(mock_writer.buffer, b"true");
}

#[test]
fn test_serialize_bool_false() {
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

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter
        }

        fn write_bool(&self, writer: &mut dyn io::Write, value: bool) -> Result<()> {
            let output = if value { b"true" } else { b"false" };
            writer.write_all(output)
        }
    }

    let mock_writer = &mut MockWriter::new();
    let formatter = MockFormatter::new();

    let serializer = &mut Serializer {
        writer: mock_writer,
        formatter,
    };

    let result = serializer.serialize_bool(false);
    assert!(result.is_ok());
    assert_eq!(mock_writer.buffer, b"false");
}

