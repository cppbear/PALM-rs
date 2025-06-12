// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct MockFormatter;
    
    impl MockFormatter {
        fn write_bool(&self, _writer: &mut dyn std::io::Write, value: bool) -> std::io::Result<()> {
            if value == true {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected true"))
            }
        }
    }

    struct MockSerializer<'a> {
        formatter: MockFormatter,
        writer: &'a mut Vec<u8>,
    }

    impl<'a> MockSerializer<'a> {
        fn new(writer: &'a mut Vec<u8>) -> Self {
            MockSerializer {
                formatter: MockFormatter,
                writer,
            }
        }

        fn serialize_bool(self, value: bool) -> std::io::Result<()> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed"))
        }
    }

    let mut buffer = Vec::new();
    let serializer = MockSerializer::new(&mut buffer);
    let result = serializer.serialize_bool(true);

    assert!(result.is_ok());
}

#[test]
fn test_serialize_bool_false() {
    struct MockFormatter;
    
    impl MockFormatter {
        fn write_bool(&self, _writer: &mut dyn std::io::Write, value: bool) -> std::io::Result<()> {
            if value == false {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected false"))
            }
        }
    }

    struct MockSerializer<'a> {
        formatter: MockFormatter,
        writer: &'a mut Vec<u8>,
    }

    impl<'a> MockSerializer<'a> {
        fn new(writer: &'a mut Vec<u8>) -> Self {
            MockSerializer {
                formatter: MockFormatter,
                writer,
            }
        }

        fn serialize_bool(self, value: bool) -> std::io::Result<()> {
            self.formatter
                .write_bool(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed"))
        }
    }

    let mut buffer = Vec::new();
    let serializer = MockSerializer::new(&mut buffer);
    let result = serializer.serialize_bool(false);

    assert!(result.is_ok());
}

