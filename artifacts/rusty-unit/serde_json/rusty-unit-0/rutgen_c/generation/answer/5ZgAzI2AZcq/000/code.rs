// Answer 0

#[test]
fn test_serialize_i16() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl TestFormatter {
        fn begin_string<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }

        fn write_i16<W: io::Write>(&self, writer: &mut W, value: i16) -> Result<()> {
            let bytes = value.to_string().into_bytes();
            writer.write(&bytes)?;
            Ok(())
        }

        fn end_string<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write(&[b'"'])?;
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let formatter = TestFormatter;
    let serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i16(42);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.buffer).unwrap(), "\"42\"");
}

