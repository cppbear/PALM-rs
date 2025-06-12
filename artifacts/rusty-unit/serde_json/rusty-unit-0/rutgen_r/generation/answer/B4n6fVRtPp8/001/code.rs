// Answer 0

#[test]
fn test_serialize_i8_success() {
    struct TestWriter {
        output: Vec<u8>,
    }

    struct TestFormatter {
        writer: TestWriter,
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }
    }

    impl TestFormatter {
        fn new(writer: TestWriter) -> Self {
            TestFormatter { writer }
        }

        fn write_i8(&mut self, writer: &mut TestWriter, value: i8) -> Result<()> {
            writer.output.push(value as u8);
            Ok(())
        }
    }

    impl TestSerializer {
        fn new(formatter: TestFormatter, writer: TestWriter) -> Self {
            TestSerializer { formatter, writer }
        }

        fn serialize_i8(self, value: i8) -> Result<()> {
            self.formatter
                .write_i8(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let writer = TestWriter::new();
    let formatter = TestFormatter::new(writer);
    let serializer = TestSerializer::new(formatter, writer);

    assert!(serializer.serialize_i8(42).is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i8_io_error() {
    struct ErrorWriter {
        should_error: bool,
    }

    struct ErrorFormatter {
        writer: ErrorWriter,
    }

    struct ErrorSerializer {
        formatter: ErrorFormatter,
        writer: ErrorWriter,
    }

    impl ErrorWriter {
        fn new(should_error: bool) -> Self {
            ErrorWriter { should_error }
        }
    }

    impl ErrorFormatter {
        fn new(writer: ErrorWriter) -> Self {
            ErrorFormatter { writer }
        }

        fn write_i8(&mut self, _: &mut ErrorWriter, _: i8) -> Result<()> {
            if self.writer.should_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "IO error"))
            } else {
                Ok(())
            }
        }
    }

    impl ErrorSerializer {
        fn new(formatter: ErrorFormatter, writer: ErrorWriter) -> Self {
            ErrorSerializer { formatter, writer }
        }

        fn serialize_i8(self, value: i8) -> Result<()> {
            self.formatter
                .write_i8(&mut self.writer, value)
                .map_err(|_| panic!("Serialization failed"))
        }
    }

    let writer = ErrorWriter::new(true);
    let formatter = ErrorFormatter::new(writer);
    let serializer = ErrorSerializer::new(formatter, writer);

    serializer.serialize_i8(100).unwrap(); // This should panic due to IO error
}

