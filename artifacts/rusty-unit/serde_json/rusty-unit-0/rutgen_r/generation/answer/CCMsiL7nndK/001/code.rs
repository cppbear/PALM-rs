// Answer 0

#[test]
fn test_format_escaped_str_with_formatter_error() {
    use std::io;
    use std::rc::Rc;

    struct FailingFormatter;

    impl FailingFormatter {
        fn new() -> Self {
            FailingFormatter
        }
    }

    impl Formatter for FailingFormatter {
        fn begin_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "formatter error"))
        }

        fn end_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestWriter(Vec<u8>);

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = FailingFormatter::new();
    let value = "test value";

    let result = format_escaped_str(&mut writer, &mut formatter, value);
    assert!(result.is_err());
}

