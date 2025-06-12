// Answer 0

#[test]
fn test_serialize_i16_success() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i16(&self, writer: &mut TestWriter, value: i16) -> Result<()> {
            let serialized = value.to_string(); // Simplified serialization for testing
            writer.output.extend_from_slice(serialized.as_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_i16(self, value: i16) -> Result<()> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_i16(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"42");
}

#[test]
#[should_panic]
fn test_serialize_i16_failure() {
    struct FailingWriter;

    impl FailingWriter {
        fn new() -> Self {
            FailingWriter {}
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i16(&self, _writer: &mut FailingWriter, _value: i16) -> Result<()> {
            // Simulate failure
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
    }

    struct FailingSerializer {
        formatter: TestFormatter,
        writer: FailingWriter,
    }

    impl FailingSerializer {
        fn new() -> Self {
            FailingSerializer {
                formatter: TestFormatter,
                writer: FailingWriter::new(),
            }
        }

        fn serialize_i16(self, value: i16) -> Result<()> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let serializer = FailingSerializer::new();
    serializer.serialize_i16(42).unwrap(); // This should panic
}

