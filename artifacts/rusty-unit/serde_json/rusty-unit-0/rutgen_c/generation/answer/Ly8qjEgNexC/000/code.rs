// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: Vec::new() }
        }
    }

    impl io::Write for TestWriter {
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

    struct TestFormatter;

    impl TestFormatter {
        fn write_byte_array(&mut self, writer: &mut TestWriter, value: &[u8]) -> Result<()> {
            writer.write_all(value)?;
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_bytes(&[]);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_bytes_non_empty() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: Vec::new() }
        }
    }

    impl io::Write for TestWriter {
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

    struct TestFormatter;

    impl TestFormatter {
        fn write_byte_array(&mut self, writer: &mut TestWriter, value: &[u8]) -> Result<()> {
            writer.write_all(value)?;
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let data = [1, 2, 3, 4];
    let result = serializer.serialize_bytes(&data);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.buffer.as_slice(), &data);
}

#[test]
fn test_serialize_bytes_write_error() {
    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::IoError)
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Err(Error::IoError)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_byte_array(&mut self, _: &mut FailingWriter, _: &[u8]) -> Result<()> {
            Err(Error::IoError)
        }
    }

    let writer = FailingWriter;
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_bytes(&[1, 2, 3]);
    assert!(result.is_err());
}

