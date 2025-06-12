// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape_characters() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            // No-op for this test
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;
    let value = "Hello World";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer.output, "Hello World");
}

#[test]
fn test_format_escaped_str_contents_with_escape_characters() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            match escape {
                CharEscape::Quote => writer.write_all(b"\\\"")?,
                CharEscape::ReverseSolidus => writer.write_all(b"\\\\")?,
                CharEscape::Solidus => writer.write_all(b"\\/")?,
                CharEscape::Backspace => writer.write_all(b"\\b")?,
                CharEscape::FormFeed => writer.write_all(b"\\f")?,
                CharEscape::LineFeed => writer.write_all(b"\\n")?,
                CharEscape::CarriageReturn => writer.write_all(b"\\r")?,
                CharEscape::Tab => writer.write_all(b"\\t")?,
                CharEscape::AsciiControl(byte) => writer.write_all(&format!("\\u{:04x}", byte).as_bytes())?,
            }
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;
    let value = "Hello\nWorld";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer.output, "Hello\\nWorld");
}

#[test]
fn test_format_escaped_str_contents_empty_string() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            // No-op for this test
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;
    let value = "";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

#[test]
#[should_panic] 
fn test_format_escaped_str_contents_invalid_range() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            // No-op for this test
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;
    let value = "Hello World";

    // Intentionally induce an invalid range panic
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, &value[1..]);
}

