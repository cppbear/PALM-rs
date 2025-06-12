// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }

        fn write_null(&mut self) -> std::io::Result<()> {
            self.output.extend(b"null");
            Ok(())
        }

        fn write_f64(&mut self, value: f64) -> std::io::Result<()> {
            self.output.extend(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestFormatter {
        writer: TestWriter,
    }

    impl TestFormatter {
        fn new(writer: TestWriter) -> Self {
            Self { writer }
        }

        fn write_null(&mut self, _writer: &mut TestWriter) -> std::io::Result<()> {
            self.writer.write_null()
        }

        fn write_f64(&mut self, _writer: &mut TestWriter, value: f64) -> std::io::Result<()> {
            self.writer.write_f64(value)
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

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(|e| e),
                _ => self
                    .formatter
                    .write_f64(&mut self.writer, value)
                    .map_err(|e| e),
            }
        }
    }

    let writer = TestWriter::new();
    let formatter = TestFormatter::new(writer);
    let serializer = TestSerializer::new(formatter, TestWriter::new());

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.get_output(), b"null");
}

#[test]
fn test_serialize_f64_infinite() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }

        fn write_null(&mut self) -> std::io::Result<()> {
            self.output.extend(b"null");
            Ok(())
        }

        fn write_f64(&mut self, value: f64) -> std::io::Result<()> {
            self.output.extend(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestFormatter {
        writer: TestWriter,
    }

    impl TestFormatter {
        fn new(writer: TestWriter) -> Self {
            Self { writer }
        }

        fn write_null(&mut self, _writer: &mut TestWriter) -> std::io::Result<()> {
            self.writer.write_null()
        }

        fn write_f64(&mut self, _writer: &mut TestWriter, value: f64) -> std::io::Result<()> {
            self.writer.write_f64(value)
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

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(|e| e),
                _ => self
                    .formatter
                    .write_f64(&mut self.writer, value)
                    .map_err(|e| e),
            }
        }
    }

    let writer = TestWriter::new();
    let formatter = TestFormatter::new(writer);
    let serializer = TestSerializer::new(formatter, TestWriter::new());

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.get_output(), b"null");
}

#[test]
fn test_serialize_f64_regular_value() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }

        fn write_null(&mut self) -> std::io::Result<()> {
            self.output.extend(b"null");
            Ok(())
        }

        fn write_f64(&mut self, value: f64) -> std::io::Result<()> {
            self.output.extend(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestFormatter {
        writer: TestWriter,
    }

    impl TestFormatter {
        fn new(writer: TestWriter) -> Self {
            Self { writer }
        }

        fn write_null(&mut self, _writer: &mut TestWriter) -> std::io::Result<()> {
            self.writer.write_null()
        }

        fn write_f64(&mut self, _writer: &mut TestWriter, value: f64) -> std::io::Result<()> {
            self.writer.write_f64(value)
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

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null(&mut self.writer)
                    .map_err(|e| e),
                _ => self
                    .formatter
                    .write_f64(&mut self.writer, value)
                    .map_err(|e| e),
            }
        }
    }

    let writer = TestWriter::new();
    let formatter = TestFormatter::new(writer);
    let serializer = TestSerializer::new(formatter, TestWriter::new());

    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.get_output(), b"3.14");
}

