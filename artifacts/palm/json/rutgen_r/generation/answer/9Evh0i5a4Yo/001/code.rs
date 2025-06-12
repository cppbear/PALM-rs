// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct TestFormatter {
        output: Vec<u8>,
    }

    impl TestFormatter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn write_bool(&mut self, _writer: &mut Vec<u8>, value: bool) -> Result<(), std::io::Error> {
            self.output.extend_from_slice(if value { b"true" } else { b"false" });
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }

    impl TestSerializer {
        fn new(formatter: TestFormatter) -> Self {
            Self {
                formatter,
                writer: Vec::new(),
            }
        }

        fn serialize_bool(self, value: bool) -> Result<()> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }
    
    let formatter = TestFormatter::new();
    let serializer = TestSerializer::new(formatter);

    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_bool_false() {
    struct TestFormatter {
        output: Vec<u8>,
    }

    impl TestFormatter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn write_bool(&mut self, _writer: &mut Vec<u8>, value: bool) -> Result<(), std::io::Error> {
            self.output.extend_from_slice(if value { b"true" } else { b"false" });
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }

    impl TestSerializer {
        fn new(formatter: TestFormatter) -> Self {
            Self {
                formatter,
                writer: Vec::new(),
            }
        }

        fn serialize_bool(self, value: bool) -> Result<()> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }
    
    let formatter = TestFormatter::new();
    let serializer = TestSerializer::new(formatter);

    let result = serializer.serialize_bool(false);
    assert!(result.is_ok());
}

