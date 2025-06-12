// Answer 0

#[test]
fn test_serialize_finite_f32() {
    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn begin_string(&mut self, writer: &mut String) -> Result<(), std::io::Error> {
            writer.push('"');
            Ok(())
        }

        fn write_f32(&mut self, writer: &mut String, value: f32) -> Result<(), std::io::Error> {
            writer.push_str(&value.to_string());
            Ok(())
        }

        fn end_string(&mut self, writer: &mut String) -> Result<(), std::io::Error> {
            writer.push('"');
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: String,
    }

    impl MockSerializer {
        fn new() -> Self {
            Self {
                formatter: MockFormatter { output: String::new() },
                writer: String::new(),
            }
        }

        fn serialize_f32(&mut self, value: f32) -> Result<(), std::io::Error> {
            if !value.is_finite() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
            }

            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_f32(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let mut serializer = MockSerializer::new();
    let result = serializer.serialize_f32(3.14);
    assert!(result.is_ok());
    assert_eq!(serializer.writer, "\"3.14\"");
}

#[test]
#[should_panic(expected = "value must be finite")]
fn test_serialize_infinite_f32() {
    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut String) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut String, _value: f32) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut String) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new() -> Self {
            Self {
                formatter: MockFormatter { output: String::new() },
            }
        }

        fn serialize_f32(&mut self, value: f32) -> Result<(), std::io::Error> {
            if !value.is_finite() {
                panic!("value must be finite");
            }
            Ok(())
        }
    }

    let mut serializer = MockSerializer::new();
    serializer.serialize_f32(f32::INFINITY);
}

