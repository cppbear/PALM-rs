// Answer 0

#[test]
fn test_serialize_i64_success() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn written(&self) -> &[u8] {
            &self.output
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i64(&self, writer: &mut TestWriter, value: i64) -> Result<()> {
            writer.output.extend(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl TestSerializer {
        fn new(formatter: TestFormatter, writer: TestWriter) -> Self {
            Self { formatter, writer }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            self.formatter
                .write_i64(&mut self.writer, value)
                .map_err(|_| ())
        }
    }

    let formatter = TestFormatter;
    let writer = TestWriter::new();
    let serializer = TestSerializer::new(formatter, writer);
    
    let result = serializer.serialize_i64(42);
    
    assert!(result.is_ok());
    assert_eq!(serializer.writer.written(), &b"42"[..]);
}

#[test]
#[should_panic]
fn test_serialize_i64_fail() {
    struct FailingFormatter;

    impl FailingFormatter {
        fn write_i64(&self, _writer: &mut TestWriter, _value: i64) -> Result<()> {
            Err(())
        }
    }

    struct TestSerializerFail {
        formatter: FailingFormatter,
        writer: TestWriter,
    }

    impl TestSerializerFail {
        fn new(formatter: FailingFormatter, writer: TestWriter) -> Self {
            Self { formatter, writer }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            self.formatter
                .write_i64(&mut self.writer, value)
                .map_err(|_| ())
        }
    }

    let formatter = FailingFormatter;
    let writer = TestWriter::new();
    let serializer = TestSerializerFail::new(formatter, writer);
    
    serializer.serialize_i64(42).unwrap();
}

