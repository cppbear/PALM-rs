// Answer 0

fn test_serialize_i16_success() {
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
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_i16<W: io::Write>(&self, _writer: &mut W, _value: i16) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_i16(42);
    assert!(result.is_ok());
}

fn test_serialize_i16_begin_string_error() {
    struct ErrorWriter;

    impl io::Write for ErrorWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Err(Error::io())
        }

        fn write_i16<W: io::Write>(&self, _writer: &mut W, _value: i16) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = ErrorWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_i16(42);
    assert!(result.is_err());
}

fn test_serialize_i16_write_i16_error() {
    struct MockWriter {
        error_condition: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            if self.error_condition {
                Err(Error::io())
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_i16<W: io::Write>(&self, _writer: &mut W, _value: i16) -> Result<()> {
            Err(Error::io())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { error_condition: false };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_i16(42);
    assert!(result.is_err());
}

