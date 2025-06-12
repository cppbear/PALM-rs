// Answer 0

#[test]
fn test_serialize_f64_finite_value() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_f64(&mut self, _writer: &mut dyn io::Write, _value: f64) -> Result<()> {
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
    let result = serializer.serialize_f64(42.0);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_f64_nan_value() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn write_f64(&mut self, _writer: &mut dyn io::Write, _value: f64) -> Result<()> {
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
    let result = serializer.serialize_f64(f64::NAN);
    
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_infinite_value() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0))
        }
        fn write_f64(&mut self, _writer: &mut dyn io::Write, _value: f64) -> Result<()> {
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
    let result = serializer.serialize_f64(f64::INFINITY);

    assert!(result.is_err());
}

