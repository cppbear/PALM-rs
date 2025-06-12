// Answer 0

#[test]
fn test_serialize_u16_success() {
    struct TestWriter {
        output: Vec<u8>,
    }
    
    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn write_u16(&mut self, value: &u16) -> std::io::Result<()> {
            self.output.extend(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Formatter {
        writer: TestWriter,
    }

    struct Serializer {
        formatter: Formatter,
    }

    impl Serializer {
        fn new(writer: TestWriter) -> Self {
            Serializer {
                formatter: Formatter { writer },
            }
        }

        fn serialize_u16(self, value: u16) -> Result<(), std::io::Error> {
            self.formatter
                .writer
                .write_u16(&value)
                .map_err(|e| e)
        }
    }

    let writer = TestWriter::new();
    let serializer = Serializer::new(writer);
    let result = serializer.serialize_u16(42);

    assert!(result.is_ok());
    assert_eq!(serializer.formatter.writer.output, vec![42, 0]);
}

#[test]
fn test_serialize_u16_io_error() {
    struct FailingWriter;

    impl FailingWriter {
        fn new() -> Self {
            FailingWriter
        }

        fn write_u16(&mut self, _: &u16) -> std::io::Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
    }

    struct Formatter {
        writer: FailingWriter,
    }

    struct Serializer {
        formatter: Formatter,
    }

    impl Serializer {
        fn new(writer: FailingWriter) -> Self {
            Serializer {
                formatter: Formatter { writer },
            }
        }

        fn serialize_u16(self, value: u16) -> Result<(), std::io::Error> {
            self.formatter
                .writer
                .write_u16(&value)
                .map_err(|e| e)
        }
    }

    let writer = FailingWriter::new();
    let serializer = Serializer::new(writer);
    let result = serializer.serialize_u16(42);

    assert!(result.is_err());
}

