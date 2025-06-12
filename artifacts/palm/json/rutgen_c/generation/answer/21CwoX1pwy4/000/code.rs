// Answer 0

#[test]
fn test_serialize_i64_positive() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i64(&mut self, _writer: &mut dyn io::Write, value: i64) -> Result<()> {
            let _ = write!(_writer, "{}", value);
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, "42");
}

#[test]
fn test_serialize_i64_negative() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i64(&mut self, _writer: &mut dyn io::Write, value: i64) -> Result<()> {
            let _ = write!(_writer, "{}", value);
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_i64(-42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, "-42");
}

#[test]
fn test_serialize_i64_zero() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i64(&mut self, _writer: &mut dyn io::Write, value: i64) -> Result<()> {
            let _ = write!(_writer, "{}", value);
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_i64(0);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, "0");
}

