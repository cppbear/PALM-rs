// Answer 0

#[test]
fn test_serialize_i64_success() {
    struct TestFormatter {
        output: Vec<u8>,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { output: Vec::new() }
        }

        fn write_i64(&self, writer: &mut Vec<u8>, value: i64) -> Result<()> {
            writer.extend(format!("{}", value).into_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: Vec::new(),
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            self.formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer, b"42");
}

#[test]
fn test_serialize_i64_negative_value() {
    struct TestFormatter {
        output: Vec<u8>,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { output: Vec::new() }
        }

        fn write_i64(&self, writer: &mut Vec<u8>, value: i64) -> Result<()> {
            writer.extend(format!("{}", value).into_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: Vec::new(),
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            self.formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_i64(-10);
    assert!(result.is_ok());
    assert_eq!(serializer.writer, b"-10");
}

#[test]
#[should_panic]
fn test_serialize_i64_panic_on_io_error() {
    struct TestFormatter {
        output: Vec<u8>,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { output: Vec::new() }
        }

        fn write_i64(&self, _writer: &mut Vec<u8>, _value: i64) -> Result<()> {
            panic!("IO error simulated");
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: Vec::new(),
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_i64(self, value: i64) -> Result<()> {
            self.formatter
                .write_i64(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let serializer = TestSerializer::new();
    let _ = serializer.serialize_i64(1);
}

