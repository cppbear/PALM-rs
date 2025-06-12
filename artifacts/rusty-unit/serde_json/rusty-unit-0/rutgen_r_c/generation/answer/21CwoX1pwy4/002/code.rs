// Answer 0

#[test]
fn test_serialize_i64_success() {
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
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i64(&mut self, _writer: &mut dyn io::Write, _value: i64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i64_begin_string_failure() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFailingFormatter;

    impl Formatter for MockFailingFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::io())
        }

        fn write_i64(&mut self, _writer: &mut dyn io::Write, _value: i64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter;
    let formatter = MockFailingFormatter;
    let mut serializer = Serializer { writer, formatter };

    let _result = serializer.serialize_i64(42);
}

#[test]
#[should_panic]
fn test_serialize_i64_write_i64_failure() {
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

    struct MockFailingFormatter;

    impl Formatter for MockFailingFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i64(&mut self, _writer: &mut dyn io::Write, _value: i64) -> Result<()> {
            Err(Error::io())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFailingFormatter;
    let mut serializer = Serializer { writer, formatter };

    let _result = serializer.serialize_i64(42);
}

