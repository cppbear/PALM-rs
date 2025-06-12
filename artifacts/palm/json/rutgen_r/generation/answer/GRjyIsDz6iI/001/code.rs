// Answer 0

#[test]
fn test_serialize_u64_success() {
    struct FakeFormatter;
    struct FakeWriter(Vec<u8>);

    impl FakeFormatter {
        fn write_u64(&self, writer: &mut FakeWriter, value: u64) -> Result<(), std::io::Error> {
            writer.0.extend(value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: FakeFormatter,
        writer: FakeWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FakeFormatter,
                writer: FakeWriter(vec![]),
            }
        }
        
        fn serialize_u64(self, value: u64) -> Result<()> {
            self.formatter
                .write_u64(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_u64(42);
    assert!(result.is_ok());

    let bytes = serializer.writer.0;
    assert_eq!(bytes, vec![42, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_serialize_u64_boundary_min() {
    struct FakeFormatter;
    struct FakeWriter(Vec<u8>);

    impl FakeFormatter {
        fn write_u64(&self, writer: &mut FakeWriter, value: u64) -> Result<(), std::io::Error> {
            writer.0.extend(value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: FakeFormatter,
        writer: FakeWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FakeFormatter,
                writer: FakeWriter(vec![]),
            }
        }
        
        fn serialize_u64(self, value: u64) -> Result<()> {
            self.formatter
                .write_u64(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_u64(0);
    assert!(result.is_ok());

    let bytes = serializer.writer.0;
    assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_serialize_u64_boundary_max() {
    struct FakeFormatter;
    struct FakeWriter(Vec<u8>);

    impl FakeFormatter {
        fn write_u64(&self, writer: &mut FakeWriter, value: u64) -> Result<(), std::io::Error> {
            writer.0.extend(value.to_le_bytes());
            Ok(())
        }
    }

    struct Serializer {
        formatter: FakeFormatter,
        writer: FakeWriter,
    }

    impl Serializer {
        fn new() -> Self {
            Serializer {
                formatter: FakeFormatter,
                writer: FakeWriter(vec![]),
            }
        }
        
        fn serialize_u64(self, value: u64) -> Result<()> {
            self.formatter
                .write_u64(&mut self.writer, value)
                .map_err(|e| e.into())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_u64(u64::MAX);
    assert!(result.is_ok());

    let bytes = serializer.writer.0;
    assert_eq!(bytes, (u64::MAX).to_le_bytes());
}

