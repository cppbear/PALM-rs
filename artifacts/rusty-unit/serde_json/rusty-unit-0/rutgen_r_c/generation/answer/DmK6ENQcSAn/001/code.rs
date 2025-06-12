// Answer 0

#[test]
fn test_collect_str_write_error() {
    struct MockWriter {
        should_fail: bool,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            if self.should_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "write error")))
            } else {
                Ok(buf.len())
            }
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
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { should_fail: true };
    let formatter = &mut MockFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.collect_str(&"test string");
    assert!(result.is_err());
}

#[test]
fn test_collect_str_success() {
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
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = &mut MockFormatter;
    let mut serializer = &mut Serializer { writer, formatter };

    let result = serializer.collect_str(&"test string");
    assert!(result.is_ok());

    let mock_writer = &mut serializer.writer;
    assert_eq!(mock_writer.output, b"test string");
}

