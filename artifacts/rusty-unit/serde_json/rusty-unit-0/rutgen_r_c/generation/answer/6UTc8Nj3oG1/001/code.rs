// Answer 0

#[test]
fn test_serialize_u16_valid() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u16(&mut self, writer: &mut Vec<u8>, value: u16) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct MockWriter {
        data: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    // Using the provided Serializer struct
    struct TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl TestSerializer {
        fn serialize_u16(self, value: u16) -> Result<()> {
            self.formatter
                .write_u16(&mut self.writer.data, value)
                .map_err(|_| Error)
        }
    }

    let serializer = TestSerializer {
        writer: MockWriter::new(),
        formatter: MockFormatter,
    };

    // Test with a valid u16 value
    let result = serializer.serialize_u16(1024);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.data, vec![0, 4]); // 1024 in little-endian
}

#[test]
fn test_serialize_u16_boundary_minimum() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u16(&mut self, writer: &mut Vec<u8>, value: u16) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct MockWriter {
        data: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    struct TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl TestSerializer {
        fn serialize_u16(self, value: u16) -> Result<()> {
            self.formatter
                .write_u16(&mut self.writer.data, value)
                .map_err(|_| Error)
        }
    }

    let serializer = TestSerializer {
        writer: MockWriter::new(),
        formatter: MockFormatter,
    };

    // Test with the minimum valid u16 value
    let result = serializer.serialize_u16(0);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.data, vec![0, 0]); // 0 in little-endian
}

#[test]
fn test_serialize_u16_boundary_maximum() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u16(&mut self, writer: &mut Vec<u8>, value: u16) -> Result<()> {
            writer.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct MockWriter {
        data: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    struct TestSerializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl TestSerializer {
        fn serialize_u16(self, value: u16) -> Result<()> {
            self.formatter
                .write_u16(&mut self.writer.data, value)
                .map_err(|_| Error)
        }
    }

    let serializer = TestSerializer {
        writer: MockWriter::new(),
        formatter: MockFormatter,
    };

    // Test with the maximum valid u16 value
    let result = serializer.serialize_u16(65535);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.data, vec![255, 255]); // 65535 in little-endian
}

