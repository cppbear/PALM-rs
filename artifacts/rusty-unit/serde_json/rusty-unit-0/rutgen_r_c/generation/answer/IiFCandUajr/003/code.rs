// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_null<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(b"null")?;
            Ok(())
        }

        fn write_f64<W: io::Write>(&self, _writer: &mut W, _value: f64) -> Result<()> {
            // Here we don't need to implement actual serialization logic
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let formatter = TestFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    // Test for NaN
    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"null");
}

#[test]
fn test_serialize_f64_infinite() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_null<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(b"null")?;
            Ok(())
        }

        fn write_f64<W: io::Write>(&self, _writer: &mut W, _value: f64) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let formatter = TestFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    // Test for positive infinity
    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"null");

    // Test for negative infinity
    let result = serializer.serialize_f64(f64::NEG_INFINITY);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"null");
}

