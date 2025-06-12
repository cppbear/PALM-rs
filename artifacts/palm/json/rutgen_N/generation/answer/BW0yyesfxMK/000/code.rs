// Answer 0

#[test]
fn test_serialize_i128() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn write_i128(&mut self, writer: &mut Vec<u8>, value: i128) -> Result<(), std::io::Error> {
            let serialized = value.to_string();
            writer.extend_from_slice(serialized.as_bytes());
            Ok(())
        }
    }
    
    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }
    
    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: Vec::new(),
            }
        }
        
        fn serialize_i128(self, value: i128) -> Result<(), std::io::Error> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let serializer = TestSerializer::new();
    
    let result = serializer.serialize_i128(1234567890123456789i128);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "1234567890123456789");
}

#[test]
fn test_serialize_i128_negative() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn write_i128(&mut self, writer: &mut Vec<u8>, value: i128) -> Result<(), std::io::Error> {
            let serialized = value.to_string();
            writer.extend_from_slice(serialized.as_bytes());
            Ok(())
        }
    }
    
    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }
    
    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: Vec::new(),
            }
        }
        
        fn serialize_i128(self, value: i128) -> Result<(), std::io::Error> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let serializer = TestSerializer::new();
    
    let result = serializer.serialize_i128(-1234567890123456789i128);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "-1234567890123456789");
}

#[test]
fn test_serialize_i128_zero() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn write_i128(&mut self, writer: &mut Vec<u8>, value: i128) -> Result<(), std::io::Error> {
            let serialized = value.to_string();
            writer.extend_from_slice(serialized.as_bytes());
            Ok(())
        }
    }
    
    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }
    
    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: Vec::new(),
            }
        }
        
        fn serialize_i128(self, value: i128) -> Result<(), std::io::Error> {
            self.formatter
                .write_i128(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let serializer = TestSerializer::new();
    
    let result = serializer.serialize_i128(0i128);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "0");
}

