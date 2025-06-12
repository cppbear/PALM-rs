// Answer 0

#[test]
fn test_serialize_infinite() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_null(&mut self) -> Result<(), std::io::Error> {
            self.writer.push(b'n');
            self.writer.push(b'u');
            self.writer.push(b'l');
            self.writer.push(b'l');
            Ok(())
        }

        fn write_f32(&mut self, value: f32) -> Result<(), std::io::Error> {
            let str_value = value.to_string();
            self.writer.extend_from_slice(str_value.as_bytes());
            Ok(())
        }
    }

    struct TestSerial {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }

    impl TestSerial {
        fn new() -> Self {
            Self {
                formatter: TestFormatter { writer: Vec::new() },
                writer: Vec::new(),
            }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null() // Removed unnecessary self.writer usage
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(value)
                    .map_err(Error::io),
            }
        }
    }

    let serial = TestSerial::new();
    let result = serial.serialize_f32(f32::INFINITY);
    assert!(result.is_ok());
    assert_eq!(serial.formatter.writer, b"null");

    let result_nan = serial.serialize_f32(f32::NAN);
    assert!(result_nan.is_ok());
    assert_eq!(serial.formatter.writer, b"null");
}

#[test]
fn test_serialize_nan() {
    struct TestFormatter {
        writer: Vec<u8>,
    }

    impl TestFormatter {
        fn write_null(&mut self) -> Result<(), std::io::Error> {
            self.writer.push(b'n');
            self.writer.push(b'u');
            self.writer.push(b'l');
            self.writer.push(b'l');
            Ok(())
        }

        fn write_f32(&mut self, value: f32) -> Result<(), std::io::Error> {
            let str_value = value.to_string();
            self.writer.extend_from_slice(str_value.as_bytes());
            Ok(())
        }
    }

    struct TestSerial {
        formatter: TestFormatter,
    }

    impl TestSerial {
        fn new() -> Self {
            Self {
                formatter: TestFormatter { writer: Vec::new() },
            }
        }

        fn serialize_f32(self, value: f32) -> Result<(), std::io::Error> {
            match value.classify() {
                FpCategory::Nan | FpCategory::Infinite => self
                    .formatter
                    .write_null() // Removed unnecessary self.writer usage
                    .map_err(Error::io),
                _ => self
                    .formatter
                    .write_f32(value)
                    .map_err(Error::io),
            }
        }
    }

    let serial = TestSerial::new();
    let result_nan = serial.serialize_f32(f32::NAN);
    assert!(result_nan.is_ok());
    assert_eq!(serial.formatter.writer, b"null");
}

