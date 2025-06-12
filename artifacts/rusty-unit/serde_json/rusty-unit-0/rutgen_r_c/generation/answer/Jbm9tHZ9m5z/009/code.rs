// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape_characters() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, value: &str) -> io::Result<()> {
            writer.write(value.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello World");
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(writer.buffer).unwrap(), "Hello World");
}

#[test]
fn test_format_escaped_str_contents_with_escape_characters() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, value: &str) -> io::Result<()> {
            writer.write(value.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, char_escape: CharEscape) -> io::Result<()> {
            match char_escape {
                CharEscape::Quote => writer.write(b"\\\"")?,
                CharEscape::Backspace => writer.write(b"\\b")?,
                CharEscape::Tab => writer.write(b"\\t")?,
                CharEscape::LineFeed => writer.write(b"\\n")?,
                CharEscape::CarriageReturn => writer.write(b"\\r")?,
                CharEscape::FormFeed => writer.write(b"\\f")?,
                CharEscape::ReverseSolidus => writer.write(b"\\\\")?,
                CharEscape::AsciiControl(_) => writer.write(b"\\u00XX")?, // Simplified control byte handling
            }
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello\tWorld\nQuotes:\"");
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(writer.buffer).unwrap(), "Hello\\tWorld\\nQuotes: \\\"");
}

#[test]
fn test_format_escaped_str_contents_only_escape_characters() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, value: &str) -> io::Result<()> {
            writer.write(value.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn io::Write, char_escape: CharEscape) -> io::Result<()> {
            match char_escape {
                CharEscape::Quote => writer.write(b"\\\"")?,
                CharEscape::Backspace => writer.write(b"\\b")?,
                CharEscape::Tab => writer.write(b"\\t")?,
                CharEscape::LineFeed => writer.write(b"\\n")?,
                CharEscape::CarriageReturn => writer.write(b"\\r")?,
                CharEscape::FormFeed => writer.write(b"\\f")?,
                CharEscape::ReverseSolidus => writer.write(b"\\\\")?,
                CharEscape::AsciiControl(x) => write!(writer, "\\u{:02X}", x)?,
            }
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, "\t\n\"\\");

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(writer.buffer).unwrap(), "\\t\\n\\\"\\\\");
}

