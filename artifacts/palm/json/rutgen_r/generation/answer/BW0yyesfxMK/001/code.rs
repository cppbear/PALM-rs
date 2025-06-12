// Answer 0

#[test]
fn test_serialize_i128_positive_value() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i128(&self, writer: &mut TestWriter, value: i128) -> Result<(), std::io::Error> {
            writer.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_i128(123456789012345678901234567890_i128);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i128_negative_value() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i128(&self, writer: &mut TestWriter, value: i128) -> Result<(), std::io::Error> {
            writer.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_i128(-123456789012345678901234567890_i128);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i128_zero() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_i128(&self, writer: &mut TestWriter, value: i128) -> Result<(), std::io::Error> {
            writer.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_i128(self, value: i128) -> Result<()> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_i128(0_i128);
    assert!(result.is_ok());
}

