// Answer 0

#[test]
fn test_serialize_i32_success() {
    struct MockWriter {
        output: Vec<u8>,
    }

    struct MockFormatter<'a> {
        writer: &'a mut MockWriter,
    }

    impl MockFormatter<'_> {
        fn write_i32(&mut self, writer: &mut MockWriter, value: i32) -> Result<()> {
            writer.output.extend(value.to_string().as_bytes());
            Ok(())
        }
    }

    struct Serializer<'a> {
        formatter: MockFormatter<'a>,
        writer: &'a mut MockWriter,
    }

    impl<'a> Serializer<'a> {
        fn new(formatter: MockFormatter<'a>, writer: &'a mut MockWriter) -> Self {
            Self { formatter, writer }
        }
        
        fn serialize_i32(self, value: i32) -> Result<()> {
            self.formatter
                .write_i32(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter { writer: &mut writer };
    let serializer = Serializer::new(formatter, &mut writer);
    
    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"42");
}

#[test]
#[should_panic]
fn test_serialize_i32_failure() {
    struct MockFailingWriter;

    struct MockFormatter<'a> {
        writer: &'a MockFailingWriter,
    }

    impl MockFormatter<'_> {
        fn write_i32(&mut self, _writer: &mut MockFailingWriter, _value: i32) -> Result<()> {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "mock failure")))
        }
    }

    struct Serializer<'a> {
        formatter: MockFormatter<'a>,
        writer: &'a MockFailingWriter,
    }

    impl<'a> Serializer<'a> {
        fn new(formatter: MockFormatter<'a>, writer: &'a MockFailingWriter) -> Self {
            Self { formatter, writer }
        }

        fn serialize_i32(self, value: i32) -> Result<()> {
            self.formatter
                .write_i32(&mut self.writer, value)
                .map_err(|e| e)
        }
    }

    let writer = MockFailingWriter {};
    let formatter = MockFormatter { writer: &writer };
    let serializer = Serializer::new(formatter, &writer);
    
    let _ = serializer.serialize_i32(42);
}

