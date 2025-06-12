// Answer 0

fn test_serialize_bool_should_return_err_when_begin_string_fails() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Err(Error::new(ErrorCode::Other, "formatter error"))
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_bool(true);
    assert!(result.is_err());
} 

fn test_serialize_bool_should_return_err_when_write_bool_fails() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
            Ok(1)
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<(), Error> {
            Err(Error::new(ErrorCode::Other, "formatter error"))
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_bool(true);
    assert!(result.is_err());
} 

fn test_serialize_bool_should_return_err_when_end_string_fails() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
            Ok(1)
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }

        fn write_bool(&mut self, _writer: &mut dyn io::Write, _value: bool) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Err(Error::new(ErrorCode::Other, "formatter error"))
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;

    let serializer = Serializer { writer, formatter };
    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let result = map_key_serializer.serialize_bool(true);
    assert!(result.is_err());
}

