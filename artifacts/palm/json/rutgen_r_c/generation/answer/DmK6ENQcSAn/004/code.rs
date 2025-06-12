// Answer 0

fn collect_str_test_success() -> Result<()> {
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

    let mut writer = MockWriter { output: vec![] };
    let mut formatter = MockFormatter;

    let serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    assert!(serializer.collect_str(&"test string").is_ok());
    Ok(())
}

#[test]
fn test_collect_str_success() {
    assert!(collect_str_test_success().is_ok());
}

fn collect_str_test_write_format_error() -> Result<()> {
    struct MockWriterWithError;

    impl io::Write for MockWriterWithError {
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

    let mut writer = MockWriterWithError {};
    let mut formatter = MockFormatter;

    let serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.collect_str(&"test string");
    assert!(result.is_err());
}

#[test]
fn test_collect_str_write_format_error() {
    assert!(collect_str_test_write_format_error().is_err());
}

fn collect_str_test_adapter_error() -> Result<()> {
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

    struct MockFormatterWithError;

    impl Formatter for MockFormatterWithError {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: vec![] };
    let mut formatter = MockFormatterWithError;

    let serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let result = serializer.collect_str(&12); // An integer should cause formatting error
    assert!(result.is_err());
}

#[test]
fn test_collect_str_adapter_error() {
    assert!(collect_str_test_adapter_error().is_err());
}

