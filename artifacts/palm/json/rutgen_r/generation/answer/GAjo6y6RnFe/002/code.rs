// Answer 0

fn test_serialize_i128_success() -> Result<()> {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
        
        fn write(&mut self, data: &[u8]) -> Result<()> {
            self.buffer.extend_from_slice(data);
            Ok(())
        }
    }

    struct MockFormatter<'a> {
        writer: &'a mut MockWriter,
        string_started: bool,
    }

    impl<'a> MockFormatter<'a> {
        fn begin_string(&mut self) -> Result<()> {
            self.string_started = true;
            Ok(())
        }

        fn write_i128(&mut self, _: &mut MockWriter, _: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self) -> Result<()> {
            if self.string_started {
                self.string_started = false;
                Ok(())
            } else {
                Err(Error::io)
            }
        }
    }

    struct Serializer<'a> {
        writer: MockWriter,
        formatter: MockFormatter<'a>,
    }

    impl<'a> Serializer<'a> {
        fn new() -> Self {
            let writer = MockWriter::new();
            let formatter = MockFormatter { writer: &mut writer, string_started: false };
            Serializer { writer, formatter }
        }

        fn serialize_i128(&mut self, value: i128) -> Result<()> {
            tri!(self
                .formatter
                .begin_string()
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i128(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string()
                .map_err(Error::io)
        }
    }

    let mut serializer = Serializer::new();
    let result = serializer.serialize_i128(123456789012345678901234567890i128);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_serialize_i128_begin_string_error() {
    struct FailingWriter;

    impl FailingWriter {
        fn write(&self, _: &[u8]) -> Result<()> {
            Err(Error::io)
        }
    }

    struct MockFormatter<'a> {
        writer: &'a FailingWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn begin_string(&self) -> Result<()> {
            Err(Error::io)
        }

        fn write_i128(&self, _: &mut FailingWriter, _: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&self) -> Result<()> {
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: FailingWriter,
        formatter: MockFormatter<'a>,
    }

    impl<'a> Serializer<'a> {
        fn new() -> Self {
            let writer = FailingWriter;
            let formatter = MockFormatter { writer: &writer };
            Serializer { writer, formatter }
        }

        fn serialize_i128(&mut self, value: i128) -> Result<()> {
            tri!(self
                .formatter
                .begin_string()
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i128(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string()
                .map_err(Error::io)
        }
    }

    let mut serializer = Serializer::new();
    let result = serializer.serialize_i128(123456789i128);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i128_write_i128_error() {
    struct MockWriter;

    impl MockWriter {
        fn write(&self, _: &[u8]) -> Result<()> {
            Ok(())
        }
    }

    struct FailingFormatter<'a> {
        writer: &'a MockWriter,
    }

    impl<'a> FailingFormatter<'a> {
        fn begin_string(&self) -> Result<()> {
            Ok(())
        }

        fn write_i128(&self, _: &mut MockWriter, _: i128) -> Result<()> {
            Err(Error::io)
        }

        fn end_string(&self) -> Result<()> {
            Ok(())
        }
    }

    struct Serializer<'a> {
        writer: MockWriter,
        formatter: FailingFormatter<'a>,
    }

    impl<'a> Serializer<'a> {
        fn new() -> Self {
            let writer = MockWriter;
            let formatter = FailingFormatter { writer: &writer };
            Serializer { writer, formatter }
        }

        fn serialize_i128(&mut self, value: i128) -> Result<()> {
            tri!(self
                .formatter
                .begin_string()
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i128(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string()
                .map_err(Error::io)
        }
    }

    let mut serializer = Serializer::new();
    let result = serializer.serialize_i128(123456789i128);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i128_end_string_error() {
    struct MockWriter {
        buffer: Vec<u8>
    };
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
        
        fn write(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }
    }

    struct FailingFormatter<'a> {
        writer: &'a mut MockWriter,
        string_started: bool,
    }

    impl<'a> FailingFormatter<'a> {
        fn begin_string(&mut self) -> Result<()> {
            self.string_started = true;
            Ok(())
        }

        fn write_i128(&mut self, _: &mut MockWriter, _: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self) -> Result<()> {
            if self.string_started {
                self.string_started = false;
                Err(Error::io)
            } else {
                Err(Error::io)
            }
        }
    }

    struct Serializer<'a> {
        writer: MockWriter,
        formatter: FailingFormatter<'a>,
    }

    impl<'a> Serializer<'a> {
        fn new() -> Self {
            let writer = MockWriter::new();
            let formatter = FailingFormatter { writer: &mut writer, string_started: false };
            Serializer { writer, formatter }
        }

        fn serialize_i128(&mut self, value: i128) -> Result<()> {
            tri!(self
                .formatter
                .begin_string()
                .map_err(Error::io));
            tri!(self
                .formatter
                .write_i128(&mut self.writer, value)
                .map_err(Error::io));
            self.formatter
                .end_string()
                .map_err(Error::io)
        }
    }

    let mut serializer = Serializer::new();
    let result = serializer.serialize_i128(123456789i128);
    assert!(result.is_err());
}

