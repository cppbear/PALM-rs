// Answer 0

#[test]
fn test_serialize_bool_success() {
    struct MockFormatter;
    struct MockWriter {
        output: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: Vec::new() }
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, value: bool) -> Result<()> {
            if value {
                _writer.write(b"true")?;
            } else {
                _writer.write(b"false")?;
            }
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let mut formatter = MockFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer: &mut writer, formatter } };

    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"true");

    writer.output.clear();
    let result = serializer.serialize_bool(false);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"false");
}

#[test]
#[should_panic]
fn test_serialize_bool_write_bool_error() {
    struct MockFormatter;
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Err(Error::new("formatter write boolean error"))
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer: &writer, formatter } };
    
    let _ = serializer.serialize_bool(true); // This should panic
} 

#[test]
#[should_panic]
fn test_serialize_bool_begin_string_error() {
    struct MockFormatter;
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::new("begin string error"))
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer: &writer, formatter } };
    
    let _ = serializer.serialize_bool(true); // This should panic
} 

#[test]
#[should_panic]
fn test_serialize_bool_end_string_error() {
    struct MockFormatter;
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::new("end string error"))
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = MapKeySerializer { ser: &mut Serializer { writer: &writer, formatter } };
    
    let _ = serializer.serialize_bool(true); // This should panic
}

