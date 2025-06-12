// Answer 0

#[test]
fn test_serialize_i64() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf).map(|_| ())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i64(&self, writer: &mut dyn io::Write, value: i64) -> Result<()> {
            writer.write(value.to_string().as_bytes())?;
            Ok(())
        }
    }

    let writer = TestWriter { output: Vec::new() };
    let formatter = TestFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    // Test serializing a negative value
    let result = serializer.serialize_i64(-42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"-42");

    // Test serializing a positive value
    serializer.writer.output.clear();
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"42");

    // Test serializing zero
    serializer.writer.output.clear();
    let result = serializer.serialize_i64(0);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"0");
}

