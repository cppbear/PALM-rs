// Answer 0

#[test]
fn test_serialize_f32_normal() {
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
        fn write_f32<W: io::Write>(&self, writer: &mut W, value: f32) -> Result<()> {
            write!(writer, "{}", value).map_err(Error::io)
        }
        fn write_null<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write_all(b"null")
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    // Test with a normal f32 value
    let result = serializer.serialize_f32(1.23);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.output).unwrap(), "1.23");

    // Test with zero
    serializer.writer.output.clear();
    let result = serializer.serialize_f32(0.0);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.output).unwrap(), "0");

    // Test with negative value
    serializer.writer.output.clear();
    let result = serializer.serialize_f32(-2.5);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.output).unwrap(), "-2.5");
}

