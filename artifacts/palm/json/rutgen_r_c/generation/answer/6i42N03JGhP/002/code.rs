// Answer 0

#[test]
fn test_serialize_u128_success() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter;
    impl Formatter for DummyFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_u128(&mut self, _writer: &mut dyn io::Write, _value: u128) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = DummyFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let result = serializer.serialize_u128(1234567890123456789012345678901234567890);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u128_failure_begin_string() {
    struct FailingWriter;
    impl io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct FailingFormatter;
    impl Formatter for FailingFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::from(ErrorCode::CustomError))
        }
        fn write_u128(&mut self, _writer: &mut dyn io::Write, _value: u128) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = FailingWriter;
    let formatter = FailingFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let result = serializer.serialize_u128(123);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u128_failure_write_u128() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct FailingFormatter;
    impl Formatter for FailingFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_u128(&mut self, _writer: &mut dyn io::Write, _value: u128) -> Result<()> {
            Err(Error::from(ErrorCode::CustomError))
        }
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = DummyWriter;
    let formatter = FailingFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let result = serializer.serialize_u128(123);
    assert!(result.is_err());
}

