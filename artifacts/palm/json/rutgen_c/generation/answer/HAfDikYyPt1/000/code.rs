// Answer 0

#[test]
fn test_serialize_unit_struct() {
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

    impl Formatter for TestFormatter {}

    let mut writer = TestWriter { output: Vec::new() };
    let formatter = TestFormatter;
    let mut serializer = Serializer { writer, formatter };

    assert!(serializer.serialize_unit_struct("test_unit").is_ok());
    assert_eq!(serializer.writer.output, b"");
}

