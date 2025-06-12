// Answer 0

#[test]
fn test_serialize_u32() {
    struct TestWriter {
        output: String,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u32(&self, writer: &mut dyn io::Write, value: u32) -> Result<()> {
            write!(writer, "{}", value).map_err(|_| Error::from(ErrorCode::Io))?;
            Ok(())
        }

        fn end_string(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u32(42);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, "42");
}

#[test]
fn test_serialize_u32_zero() {
    struct TestWriter {
        output: String,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u32(&self, writer: &mut dyn io::Write, value: u32) -> Result<()> {
            write!(writer, "{}", value).map_err(|_| Error::from(ErrorCode::Io))?;
            Ok(())
        }

        fn end_string(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u32(0);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, "0");
}

#[test]
fn test_serialize_u32_max_value() {
    struct TestWriter {
        output: String,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u32(&self, writer: &mut dyn io::Write, value: u32) -> Result<()> {
            write!(writer, "{}", value).map_err(|_| Error::from(ErrorCode::Io))?;
            Ok(())
        }

        fn end_string(&self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u32(u32::MAX);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, format!("{}", u32::MAX));
}

