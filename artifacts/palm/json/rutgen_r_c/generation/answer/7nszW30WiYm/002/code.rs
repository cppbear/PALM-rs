// Answer 0

#[test]
fn test_serialize_i32_success() {
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
        fn write_i32(&mut self, _writer: &mut dyn io::Write, _value: i32) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i32_begin_string_error() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct ErrFormatter;
    impl Formatter for ErrFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "begin string error")))
        }
        fn write_i32(&mut self, _writer: &mut dyn io::Write, _value: i32) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = ErrFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i32_write_i32_error() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct ErrFormatter;
    impl Formatter for ErrFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_i32(&mut self, _writer: &mut dyn io::Write, _value: i32) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "write i32 error")))
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = ErrFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_i32_end_string_error() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct ErrFormatter;
    impl Formatter for ErrFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_i32(&mut self, _writer: &mut dyn io::Write, _value: i32) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "end string error")))
        }
    }

    let mut writer = MockWriter;
    let formatter = ErrFormatter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
}

