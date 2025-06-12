// Answer 0

fn test_collect_str_success() -> Result<()> {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        started: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self { started: false }
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            self.started = true;
            writer.write_all(b"\"")?;
            Ok(())
        }

        fn end_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            if self.started {
                writer.write_all(b"\"")?;
            }
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: String::new(),
    };
    let mut formatter = MockFormatter::new();

    collect_str(&mut writer, &formatter, "Hello, World!")?;

    assert_eq!(writer.output, "\"Hello, World!\"");

    Ok(())
}

#[test]
fn test_collect_str_io_error() {
    struct ErrorWriter;

    impl io::Write for ErrorWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Write error"))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn new() -> Self {
            Self {}
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = ErrorWriter;
    let formatter = MockFormatter::new();

    let result = collect_str(&mut writer, &formatter, "This should fail");
    assert!(result.is_err());
}

#[test]
fn test_collect_str_no_formatter_start() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct BrokenFormatter;

    impl Formatter for BrokenFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Formatter error"))
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: String::new(),
    };
    let formatter = BrokenFormatter;

    let result = collect_str(&mut writer, &formatter, "Hello");
    assert!(result.is_err());
}

