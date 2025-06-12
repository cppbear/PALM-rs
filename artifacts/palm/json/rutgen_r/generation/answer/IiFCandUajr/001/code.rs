// Answer 0

#[test]
fn test_serialize_f64_normal_value() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_f64(&mut self, _writer: &mut Vec<u8>, value: f64) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }

        fn write_null(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(b"null");
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
                formatter: TestFormatter { writer: Vec::new() },
                writer: Vec::new(),
            }
        }

        fn serialize_f64(&mut self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => 
                    self.formatter.write_null(&mut self.writer),
                _ => 
                    self.formatter.write_f64(&mut self.writer, value),
            }
        }
    }

    let mut serializer = TestSerializer::new();
    
    // Test with a normal finite value
    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "3.14");
}

#[test]
fn test_serialize_f64_zero_value() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_f64(&mut self, _writer: &mut Vec<u8>, value: f64) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }

        fn write_null(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(b"null");
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
                formatter: TestFormatter { writer: Vec::new() },
                writer: Vec::new(),
            }
        }

        fn serialize_f64(&mut self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => 
                    self.formatter.write_null(&mut self.writer),
                _ => 
                    self.formatter.write_f64(&mut self.writer, value),
            }
        }
    }

    let mut serializer = TestSerializer::new();
    
    // Test with zero value
    let result = serializer.serialize_f64(0.0);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "0");
}

