// Answer 0

#[test]
fn test_serialize_char_valid() {
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

    impl ser::Formatter for MockFormatter {
        fn begin_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _writer: &mut dyn io::Write, _is_first: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;

    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_char('A');

    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"A");
}

#[test]
#[should_panic]
fn test_serialize_char_invalid() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0) // simulate failure
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

    impl ser::Formatter for MockFormatter {
        fn begin_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _writer: &mut dyn io::Write, _is_first: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;

    let serializer = &mut Serializer { writer, formatter };

    serializer.serialize_char('A').unwrap(); // This will panic
}

