// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
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

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_null<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(b"null")
        }

        fn write_f64<W: io::Write>(&self, writer: &mut W, _value: f64) -> Result<()> {
            writer.write_all(b"not_null") // Just as a placeholder, we don't expect this to be called for NaN
        }
    }

    let writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    
    assert_eq!(serializer.serialize_f64(f64::NAN).is_ok(), true);
    assert_eq!(serializer.writer.output, b"null");
}

#[test]
fn test_serialize_f64_infinite() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
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

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_null<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(b"null")
        }

        fn write_f64<W: io::Write>(&self, writer: &mut W, _value: f64) -> Result<()> {
            writer.write_all(b"not_null") // Just as a placeholder, we don't expect this to be called for Infinite
        }
    }

    let writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    
    assert_eq!(serializer.serialize_f64(f64::INFINITY).is_ok(), true);
    assert_eq!(serializer.writer.output, b"null");
}

#[test]
fn test_serialize_f64_finite() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
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

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_null<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(b"null") // Placeholder, we don't expect this to be called
        }

        fn write_f64<W: io::Write>(&self, writer: &mut W, value: f64) -> Result<()> {
            writer.write_all(value.to_string().as_bytes())
        }
    }

    let writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };
    
    assert_eq!(serializer.serialize_f64(42.0).is_ok(), true);
    assert_eq!(serializer.writer.output, b"42");
}

