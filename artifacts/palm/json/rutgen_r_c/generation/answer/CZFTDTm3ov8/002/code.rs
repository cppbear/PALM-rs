// Answer 0

#[test]
fn test_serialize_u8_success() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_u8(&mut self, _writer: &mut dyn io::Write, _value: u8) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = key_serializer.serialize_u8(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u8_write_error() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_u8(&mut self, _writer: &mut dyn io::Write, _value: u8) -> Result<()> {
            Err(Error::io("Write error".to_string()))
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = key_serializer.serialize_u8(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u8_begin_string_error() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::io("Begin string error".to_string()))
        }
        fn write_u8(&mut self, _writer: &mut dyn io::Write, _value: u8) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = key_serializer.serialize_u8(42);
    assert!(result.is_err());
}

