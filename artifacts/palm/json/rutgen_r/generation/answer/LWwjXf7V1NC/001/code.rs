// Answer 0

#[test]
fn test_serialize_u32_success() {
    struct MockFormatter {
        call_count: usize,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { call_count: 0 }
        }

        fn write_u32(&self, writer: &mut Vec<u8>, value: u32) -> Result<(), std::io::Error> {
            writer.extend_from_slice(&value.to_le_bytes());
            self.call_count += 1;
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new(formatter: MockFormatter) -> Self {
            Serializer {
                formatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u32(mut self, value: u32) -> Result<()> {
            self.formatter
                .write_u32(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let formatter = MockFormatter::new();
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_u32(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u32_panic_condition() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_u32(&self, _: &mut Vec<u8>, _: u32) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "mock panic condition"))
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new(formatter: MockFormatter) -> Self {
            Serializer {
                formatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u32(mut self, value: u32) -> Result<()> {
            self.formatter
                .write_u32(&mut self.writer, value)
                .map_err(Error::io)
        }
    }

    let formatter = MockFormatter {};
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_u32(100);
    assert!(result.is_err());
}

