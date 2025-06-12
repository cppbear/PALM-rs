// Answer 0

#[test]
fn test_serialize_i16_positive_value() {
    struct TestFormatter;
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }
    }

    impl TestFormatter {
        fn write_i16(&self, writer: &mut TestWriter, value: i16) -> std::io::Result<()> {
            writer.output.extend(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new(formatter: TestFormatter, writer: TestWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_i16(self, value: i16) -> Result<(), std::io::Error> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let formatter = TestFormatter;
    let writer = TestWriter::new();
    let serializer = Serializer::new(formatter, writer);

    let result = serializer.serialize_i16(12345);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i16_negative_value() {
    struct TestFormatter;
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }
    }

    impl TestFormatter {
        fn write_i16(&self, writer: &mut TestWriter, value: i16) -> std::io::Result<()> {
            writer.output.extend(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new(formatter: TestFormatter, writer: TestWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_i16(self, value: i16) -> Result<(), std::io::Error> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let formatter = TestFormatter;
    let writer = TestWriter::new();
    let serializer = Serializer::new(formatter, writer);

    let result = serializer.serialize_i16(-12345);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i16_zero_value() {
    struct TestFormatter;
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }
    }

    impl TestFormatter {
        fn write_i16(&self, writer: &mut TestWriter, value: i16) -> std::io::Result<()> {
            writer.output.extend(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new(formatter: TestFormatter, writer: TestWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_i16(self, value: i16) -> Result<(), std::io::Error> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let formatter = TestFormatter;
    let writer = TestWriter::new();
    let serializer = Serializer::new(formatter, writer);

    let result = serializer.serialize_i16(0);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i16_invalid_write() {
    struct PanickingFormatter;
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }
    }

    impl PanickingFormatter {
        fn write_i16(&self, _: &mut TestWriter, _: i16) -> std::io::Result<()> {
            panic!("Intentional panic for testing!");
        }
    }

    struct Serializer {
        formatter: PanickingFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new(formatter: PanickingFormatter, writer: TestWriter) -> Self {
            Serializer { formatter, writer }
        }

        fn serialize_i16(self, value: i16) -> Result<(), std::io::Error> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let formatter = PanickingFormatter;
    let writer = TestWriter::new();
    let serializer = Serializer::new(formatter, writer);

    let _ = serializer.serialize_i16(123);
}

