// Answer 0

#[test]
fn test_format_escaped_str_with_empty_string() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(&String::from_utf8_lossy(buf));
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: io::Write>(
            &mut self,
            writer: &mut W,
            fragment: &str,
        ) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(
            &mut self,
            _writer: &mut W,
            _char_escape: CharEscape,
        ) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;

    let result = format_escaped_str(&mut writer, &mut formatter, "");
    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

#[test]
fn test_format_escaped_str_with_simple_string() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(&String::from_utf8_lossy(buf));
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: io::Write>(
            &mut self,
            writer: &mut W,
            fragment: &str,
        ) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(
            &mut self,
            _writer: &mut W,
            _char_escape: CharEscape,
        ) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;

    let result = format_escaped_str(&mut writer, &mut formatter, "hello");
    assert!(result.is_ok());
    assert_eq!(writer.output, "hello");
}

#[test]
fn test_format_escaped_str_with_escape() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(&String::from_utf8_lossy(buf));
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: io::Write>(
            &mut self,
            writer: &mut W,
            fragment: &str,
        ) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(
            &mut self,
            writer: &mut W,
            char_escape: CharEscape,
        ) -> io::Result<()> {
            writer.write_all(char_escape.as_bytes())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;

    let result = format_escaped_str(&mut writer, &mut formatter, "Hello \"World\"");
    assert!(result.is_ok());
    assert_eq!(writer.output, "Hello \"World\"");
}

