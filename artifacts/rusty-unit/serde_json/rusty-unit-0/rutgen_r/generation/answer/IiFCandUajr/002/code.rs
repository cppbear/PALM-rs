// Answer 0

#[test]
fn test_serialize_f64_infinite() {
    struct TestFormatter<'a> {
        writer: &'a mut Vec<u8>,
    }

    impl<'a> TestFormatter<'a> {
        fn write_null(&mut self) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(b"null");
            Ok(())
        }

        fn write_f64(&mut self, value: f64) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        formatter: TestFormatter<'a>,
        writer: &'a mut Vec<u8>,
    }

    impl<'a> Serializer<'a> {
        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null()
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f64(value)
                    .map_err(Error::io),
            }
        }
    }

    let mut buffer = Vec::new();
    let formatter = TestFormatter { writer: &mut buffer };
    let serializer = Serializer {
        formatter,
        writer: &mut buffer,
    };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
    assert_eq!(buffer, b"null");
}

#[test]
fn test_serialize_f64_nan() {
    struct TestFormatter<'a> {
        writer: &'a mut Vec<u8>,
    }

    impl<'a> TestFormatter<'a> {
        fn write_null(&mut self) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(b"null");
            Ok(())
        }

        fn write_f64(&mut self, value: f64) -> Result<(), std::io::Error> {
            self.writer.extend_from_slice(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        formatter: TestFormatter<'a>,
        writer: &'a mut Vec<u8>,
    }

    impl<'a> Serializer<'a> {
        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null()
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f64(value)
                    .map_err(Error::io),
            }
        }
    }

    let mut buffer = Vec::new();
    let formatter = TestFormatter { writer: &mut buffer };
    let serializer = Serializer {
        formatter,
        writer: &mut buffer,
    };

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
    assert_eq!(buffer, b"null");
}

