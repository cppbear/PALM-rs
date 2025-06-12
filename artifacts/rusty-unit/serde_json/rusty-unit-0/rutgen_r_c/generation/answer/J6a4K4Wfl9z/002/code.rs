// Answer 0

#[test]
fn test_serialize_i8_success() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
        fn write_i8(&mut self, _writer: &mut dyn io::Write, _value: i8) -> Result<()> { Ok(()) }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
    }

    let mock_writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer: mock_writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_i8(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i8_write_error() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::io())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
        fn write_i8(&mut self, _writer: &mut dyn io::Write, _value: i8) -> Result<()> { Err(Error::io()) }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> { Ok(()) }
    }

    let mock_writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer: mock_writer,
        formatter,
    };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_i8(42);
    assert!(result.is_err());
}

