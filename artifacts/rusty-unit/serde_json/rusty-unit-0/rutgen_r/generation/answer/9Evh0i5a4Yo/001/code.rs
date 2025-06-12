// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write(&mut self, data: &str) {
            self.output.push_str(data);
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_bool(&self, writer: &mut TestWriter, value: bool) -> std::io::Result<()> {
            writer.write(if value { "true" } else { "false" });
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_bool(self, value: bool) -> Result<(), std::io::Error> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|e| e)
        }

        fn get_output(&self) -> &str {
            &self.writer.output
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
    assert_eq!(serializer.get_output(), "true");
}

#[test]
fn test_serialize_bool_false() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write(&mut self, data: &str) {
            self.output.push_str(data);
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_bool(&self, writer: &mut TestWriter, value: bool) -> std::io::Result<()> {
            writer.write(if value { "true" } else { "false" });
            Ok(())
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_bool(self, value: bool) -> Result<(), std::io::Error> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|e| e)
        }

        fn get_output(&self) -> &str {
            &self.writer.output
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_bool(false);
    assert!(result.is_ok());
    assert_eq!(serializer.get_output(), "false");
}

#[test]
#[should_panic]
fn test_serialize_bool_panic() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write(&mut self, _: &str) -> std::io::Result<()> {
            panic!("Intentional panic");
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn write_bool(&self, writer: &mut TestWriter, value: bool) -> std::io::Result<()> {
            writer.write(if value { "true" } else { "false" })
        }
    }

    struct Serializer {
        formatter: TestFormatter,
        writer: TestWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: TestFormatter,
                writer: TestWriter::new(),
            }
        }

        fn serialize_bool(self, value: bool) -> Result<(), std::io::Error> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let serializer = Serializer::new();
    let _result = serializer.serialize_bool(true);
}

