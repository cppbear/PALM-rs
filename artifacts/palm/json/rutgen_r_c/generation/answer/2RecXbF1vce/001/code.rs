// Answer 0

#[test]
fn test_serialize_i64_positive() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_i64(&self, writer: &mut Vec<u8>, value: i64) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

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

    let mut buffer = Vec::new();
    let formatter = MockFormatter;
    let writer = MockWriter { buffer };

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i64(1234567890123456789);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i64_negative() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_i64(&self, writer: &mut Vec<u8>, value: i64) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

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

    let mut buffer = Vec::new();
    let formatter = MockFormatter;
    let writer = MockWriter { buffer };

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i64(-1234567890123456789);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i64_zero() {
    struct MockFormatter;
    impl MockFormatter {
        fn write_i64(&self, writer: &mut Vec<u8>, value: i64) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

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

    let mut buffer = Vec::new();
    let formatter = MockFormatter;
    let writer = MockWriter { buffer };

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i64(0);
    assert!(result.is_ok());
}

