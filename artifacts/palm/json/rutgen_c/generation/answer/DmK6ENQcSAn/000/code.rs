// Answer 0

#[test]
fn test_collect_str_with_valid_string() {
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

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    
    let serializer = &mut Serializer { writer: &mut writer, formatter };

    let result = serializer.collect_str("Hello, World!");
    
    assert!(result.is_ok());
    assert_eq!(writer.output, b"Hello, World!");
}

#[test]
fn test_collect_str_with_empty_string() {
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

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    
    let serializer = &mut Serializer { writer: &mut writer, formatter };

    let result = serializer.collect_str("");
    
    assert!(result.is_ok());
    assert_eq!(writer.output, b"");
}

#[should_panic]
#[test]
fn test_collect_str_with_error_condition() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
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
    let mut formatter = MockFormatter;
    
    let serializer = &mut Serializer { writer: &mut writer, formatter };

    let _ = serializer.collect_str("This will cause an error");
}

