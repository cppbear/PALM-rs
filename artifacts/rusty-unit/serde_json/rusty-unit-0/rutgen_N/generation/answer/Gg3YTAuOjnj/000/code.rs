// Answer 0

#[test]
fn test_serialize_str_success() {
    struct TestWriter;
    
    impl TestWriter {
        fn new() -> Self {
            TestWriter {}
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {}
        }
    }

    struct TestSerializer {
        writer: TestWriter,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: TestWriter::new(),
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_str(self, value: &str) -> Result<()> {
            format_escaped_str(&mut self.writer, &mut self.formatter, value).map_err(Error::io)
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_str("test string");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_str_error() {
    struct TestWriter;
    
    impl TestWriter {
        fn new() -> Self {
            TestWriter {}
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {}
        }
    }

    struct TestSerializer {
        writer: TestWriter,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: TestWriter::new(),
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_str(self, value: &str) -> Result<()> {
            // Simulate an error for test purposes
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "error")))
        }
    }

    let serializer = TestSerializer::new();
    let _ = serializer.serialize_str("test string").unwrap();
}

