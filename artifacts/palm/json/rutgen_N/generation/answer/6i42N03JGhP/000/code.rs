// Answer 0

#[test]
fn test_serialize_u128() {
    struct MockFormatter {
        written_data: Vec<u8>,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            self.written_data.push(b'"');
            Ok(())
        }

        fn write_u128(&mut self, _writer: &mut Vec<u8>, value: u128) -> Result<()> {
            self.written_data.extend(value.to_string().as_bytes());
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            self.written_data.push(b'"');
            Ok(())
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: MockFormatter {
                    written_data: Vec::new(),
                },
            }
        }

        fn serialize_u128(&mut self, value: u128) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_u128(&mut self.writer, value)
                .map_err(Error::io));
            self
                .formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    let mut serializer = MockSerializer::new();
    let result = serializer.serialize_u128(1234567890123456789012345678901234567890u128);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "\"1234567890123456789012345678901234567890\"");
}

#[test]
fn test_serialize_u128_empty() {
    struct MockFormatter {
        written_data: Vec<u8>,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            self.written_data.push(b'"');
            Ok(())
        }

        fn write_u128(&mut self, _writer: &mut Vec<u8>, _value: u128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            self.written_data.push(b'"');
            Ok(())
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl MockSerializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: MockFormatter {
                    written_data: Vec::new(),
                },
            }
        }

        fn serialize_u128(&mut self, value: u128) -> Result<()> {
            tri!(self
                .formatter
                .begin_string(&mut self.writer)
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_u128(&mut self.writer, value)
                .map_err(Error::io));
            self
                .formatter
                .end_string(&mut self.writer)
                .map_err(Error::io)
        }
    }

    let mut serializer = MockSerializer::new();
    let result = serializer.serialize_u128(0u128);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer).unwrap(), "\"0\"");
}

