// Answer 0

#[test]
fn test_serialize_u128() {
    struct MockFormatter {
        output: Vec<u8>,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { output: Vec::new() }
        }

        fn write_u128(&mut self, _writer: &mut Vec<u8>, value: u128) -> Result<()> {
            self.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: MockFormatter::new(),
                writer: Vec::new(),
            }
        }

        fn serialize_u128(self, value: u128) -> Result<()> {
            self.formatter
                .write_u128(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Serialization Error"))
        }
    }

    let serializer = Serializer::new();
    let value: u128 = 12345678901234567890;

    let result = serializer.serialize_u128(value);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.len(), 16);
    assert_eq!(serializer.formatter.output.len(), 16);
    let bytes: [u8; 16] = value.to_le_bytes();
    assert_eq!(serializer.formatter.output, bytes);
}

