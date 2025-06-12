// Answer 0

fn test_collect_str_success() {
    struct MockWriter {
        buffer: Vec<u8>,
        should_fail: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.should_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "write error")))
            } else {
                self.buffer.extend_from_slice(buf);
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"").map(|_| ())
        }
        
        fn end_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"").map(|_| ())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new(), should_fail: false };
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.collect_str(&"test").unwrap();
    assert_eq!(writer.buffer, b"\"test\"");
}

fn test_collect_str_failure() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::io(io::Error::new(io::ErrorKind::Other, "write error")))
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

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.collect_str(&"test");
    assert!(result.is_err());
}

fn test_collect_str_adapter_error() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.collect_str(&"test");
    assert!(result.is_ok());
}

