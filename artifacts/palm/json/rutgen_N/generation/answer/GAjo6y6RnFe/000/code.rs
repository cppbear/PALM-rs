// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct MockFormatter {
        called_begin_string: bool,
        called_write_i128: bool,
        called_end_string: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                called_begin_string: false,
                called_write_i128: false,
                called_end_string: false,
            }
        }

        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            self.called_begin_string = true;
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<()> {
            self.called_write_i128 = true;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            self.called_end_string = true;
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer {
                formatter: MockFormatter::new(),
                writer: Vec::new(),
            }
        }
        
        fn serialize_i128(&mut self, value: i128) -> Result<()> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mut serializer = MockSerializer::new();
    let result = serializer.serialize_i128(42_i128);
    assert!(result.is_ok());
    assert!(serializer.formatter.called_begin_string);
    assert!(serializer.formatter.called_write_i128);
    assert!(serializer.formatter.called_end_string);
}

#[test]
#[should_panic]
fn test_serialize_i128_fail_begin_string() {
    struct FailingFormatter;

    impl FailingFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Err(Error::io)
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    struct FailingSerializer {
        formatter: FailingFormatter,
        writer: Vec<u8>,
    }

    impl FailingSerializer {
        fn new() -> Self {
            FailingSerializer {
                formatter: FailingFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_i128(&mut self, value: i128) -> Result<()> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mut serializer = FailingSerializer::new();
    serializer.serialize_i128(42_i128).unwrap();
}

