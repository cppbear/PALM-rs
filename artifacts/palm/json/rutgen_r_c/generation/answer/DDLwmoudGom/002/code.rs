// Answer 0

fn test_serialize_u16_success() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
        fn write_u16(&mut self, _writer: &mut MockWriter, _value: u16) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u16(42);
    assert!(result.is_ok());
}

fn test_serialize_u16_begin_string_error() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Err(Error)
        }
        fn write_u16(&mut self, _writer: &mut MockWriter, _value: u16) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u16(42);
    assert!(result.is_err());
}

fn test_serialize_u16_write_u16_error() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
        fn write_u16(&mut self, _writer: &mut MockWriter, _value: u16) -> Result<()> {
            Err(Error)
        }
        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u16(42);
    assert!(result.is_err());
}

fn test_serialize_u16_end_string_error() {
    struct MockWriter;
    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
        fn write_u16(&mut self, _writer: &mut MockWriter, _value: u16) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Err(Error)
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u16(42);
    assert!(result.is_err());
}

