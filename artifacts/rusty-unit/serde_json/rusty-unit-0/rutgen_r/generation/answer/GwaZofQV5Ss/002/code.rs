// Answer 0

fn test_serialize_f64_finite_success() {
    struct MockWriter {
        output: Vec<u8>,
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: Vec::new() }
        }

        fn write(&mut self, _data: f64) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                writer: MockWriter::new(),
            }
        }

        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut MockWriter, _value: f64) -> Result<(), std::io::Error> {
            self.writer.write(_value)
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                formatter: MockFormatter::new(),
            }
        }

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            if !value.is_finite() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
            }

            self.formatter.writer.writer.write(value)?;
            Ok(())
        }
    }

    let serializer = TestStruct {
        ser: MockSerializer::new(),
    };

    let result = serializer.ser.serialize_f64(42.0);
    assert!(result.is_ok());
}

fn test_serialize_f64_nan() {
    struct MockWriter {
        output: Vec<u8>,
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: Vec::new() }
        }

        fn write(&mut self, _data: f64) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                writer: MockWriter::new(),
            }
        }

        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut MockWriter, _value: f64) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "write failed"))
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                formatter: MockFormatter::new(),
            }
        }

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            if !value.is_finite() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
            }

            self.formatter.writer.writer.write(value)?;
            Ok(())
        }
    }

    let serializer = TestStruct {
        ser: MockSerializer::new(),
    };

    let result = serializer.ser.serialize_f64(f64::NAN);
    assert!(result.is_err());
}

fn test_serialize_f64_infinite() {
    struct MockWriter {
        output: Vec<u8>,
    }

    struct MockFormatter {
        writer: MockWriter,
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: Vec::new() }
        }

        fn write(&mut self, _data: f64) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                writer: MockWriter::new(),
            }
        }

        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut MockWriter, _value: f64) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "write failed"))
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                formatter: MockFormatter::new(),
            }
        }

        fn serialize_f64(self, value: f64) -> Result<(), std::io::Error> {
            if !value.is_finite() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
            }

            self.formatter.writer.writer.write(value)?;
            Ok(())
        }
    }

    let serializer = TestStruct {
        ser: MockSerializer::new(),
    };

    let result = serializer.ser.serialize_f64(f64::INFINITY);
    assert!(result.is_err());
}

