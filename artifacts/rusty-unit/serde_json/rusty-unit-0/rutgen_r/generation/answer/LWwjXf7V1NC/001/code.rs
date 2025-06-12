// Answer 0

#[test]
fn test_serialize_u32_success() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u32(&mut self, writer: &mut Vec<u8>, value: u32) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct TestWriter {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                formatter: MockFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u32(self, value: u32) -> Result<()> {
            self.formatter
                .write_u32(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }
    }

    let test_writer = TestWriter::new();
    let result = test_writer.serialize_u32(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u32_zero() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u32(&mut self, writer: &mut Vec<u8>, value: u32) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct TestWriter {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                formatter: MockFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u32(self, value: u32) -> Result<()> {
            self.formatter
                .write_u32(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }
    }

    let test_writer = TestWriter::new();
    let result = test_writer.serialize_u32(0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u32_max_value() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u32(&mut self, writer: &mut Vec<u8>, value: u32) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct TestWriter {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                formatter: MockFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u32(self, value: u32) -> Result<()> {
            self.formatter
                .write_u32(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }
    }

    let test_writer = TestWriter::new();
    let result = test_writer.serialize_u32(u32::MAX);
    assert!(result.is_ok());
}

