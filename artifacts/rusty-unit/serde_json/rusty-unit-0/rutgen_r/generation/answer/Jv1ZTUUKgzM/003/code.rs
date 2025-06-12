// Answer 0

#[test]
fn test_serialize_f32_nan() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_null(&mut self) -> Result<()> {
            self.writer.extend_from_slice(b"null");
            Ok(())
        }

        fn write_f32(&mut self, value: f32) -> Result<()> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter { writer: Vec::new() },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null()
                    .map_err(|e| e), // simplifying Error for the test
                _ => self
                    .formatter
                    .write_f32(value)
                    .map_err(|e| e),
            }
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_f32(f32::NAN);
    assert!(result.is_ok());
    assert_eq!(serializer.formatter.writer, b"null");
}

#[test]
fn test_serialize_f32_infinite() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_null(&mut self) -> Result<()> {
            self.writer.extend_from_slice(b"null");
            Ok(())
        }

        fn write_f32(&mut self, value: f32) -> Result<()> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter { writer: Vec::new() },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null()
                    .map_err(|e| e),
                _ => self
                    .formatter
                    .write_f32(value)
                    .map_err(|e| e),
            }
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_ok());
    assert_eq!(serializer.formatter.writer, b"null");
}

#[test]
fn test_serialize_f32_normal_value() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_null(&mut self) -> Result<()> {
            self.writer.extend_from_slice(b"null");
            Ok(())
        }

        fn write_f32(&mut self, value: f32) -> Result<()> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter { writer: Vec::new() },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<()> {
            match value.classify() {
                std::num::FpCategory::Nan | std::num::FpCategory::Infinite => self
                    .formatter
                    .write_null()
                    .map_err(|e| e),
                _ => self
                    .formatter
                    .write_f32(value)
                    .map_err(|e| e),
            }
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_f32(3.14);
    assert!(result.is_ok());
    assert_eq!(serializer.formatter.writer, b"3.14");
}

