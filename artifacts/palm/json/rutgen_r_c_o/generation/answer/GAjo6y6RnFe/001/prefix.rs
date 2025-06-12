// Answer 0

#[test]
fn test_serialize_i128_err_begin_string() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(Error::io)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::io)
        }

        fn write_i128(&mut self, _writer: &mut dyn io::Write, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let result = serializer.serialize_i128(i128::MIN);
}

#[test]
fn test_serialize_i128_err_write_i128() {
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut dyn io::Write, _value: i128) -> Result<()> {
            Err(Error::io)
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };
    let result = serializer.serialize_i128(i128::MAX);
}

