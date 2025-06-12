// Answer 0

#[test]
fn test_format_escaped_str_contents_with_single_escape() {
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

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            match escape {
                CharEscape::Backspace => writer.write_all(b"\\b")?,
                CharEscape::Tab => writer.write_all(b"\\t")?,
                CharEscape::LineFeed => writer.write_all(b"\\n")?,
                CharEscape::FormFeed => writer.write_all(b"\\f")?,
                CharEscape::CarriageReturn => writer.write_all(b"\\r")?,
                CharEscape::Quote => writer.write_all(b"\\\"")?,
                CharEscape::ReverseSolidus => writer.write_all(b"\\\\")?,
                CharEscape::Solidus => writer.write_all(b"\\/")?,
                CharEscape::AsciiControl(byte) => writer.write_all(&[b'\\', b'u', byte])?,
            }
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = TestFormatter;
    let value = "Hello\tWorld"; // contains tab escape

    format_escaped_str_contents(&mut writer, &mut formatter, value).unwrap();
}

#[test]
fn test_format_escaped_str_contents_with_multiple_escapes() {
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

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            match escape {
                CharEscape::Backspace => writer.write_all(b"\\b")?,
                CharEscape::Tab => writer.write_all(b"\\t")?,
                CharEscape::LineFeed => writer.write_all(b"\\n")?,
                CharEscape::FormFeed => writer.write_all(b"\\f")?,
                CharEscape::CarriageReturn => writer.write_all(b"\\r")?,
                CharEscape::Quote => writer.write_all(b"\\\"")?,
                CharEscape::ReverseSolidus => writer.write_all(b"\\\\")?,
                CharEscape::Solidus => writer.write_all(b"\\/")?,
                CharEscape::AsciiControl(byte) => writer.write_all(&[b'\\', b'u', byte])?,
            }
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = TestFormatter;
    let value = "Hello\nWorld\tTest"; // contains newline and tab escapes

    format_escaped_str_contents(&mut writer, &mut formatter, value).unwrap();
}

#[test]
fn test_format_escaped_str_contents_with_control_characters() {
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

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            match escape {
                CharEscape::Backspace => writer.write_all(b"\\b")?,
                CharEscape::Tab => writer.write_all(b"\\t")?,
                CharEscape::LineFeed => writer.write_all(b"\\n")?,
                CharEscape::FormFeed => writer.write_all(b"\\f")?,
                CharEscape::CarriageReturn => writer.write_all(b"\\r")?,
                CharEscape::Quote => writer.write_all(b"\\\"")?,
                CharEscape::ReverseSolidus => writer.write_all(b"\\\\")?,
                CharEscape::Solidus => writer.write_all(b"\\/")?,
                CharEscape::AsciiControl(byte) => writer.write_all(&[b'\\', b'u', byte])?,
            }
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = TestFormatter;
    let value = "Hello\x01World\x02"; // contains control characters

    format_escaped_str_contents(&mut writer, &mut formatter, value).unwrap();
}

#[test]
fn test_format_escaped_str_contents_with_escaped_quote() {
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

    struct TestFormatter;
    impl Formatter for TestFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            match escape {
                CharEscape::Backspace => writer.write_all(b"\\b")?,
                CharEscape::Tab => writer.write_all(b"\\t")?,
                CharEscape::LineFeed => writer.write_all(b"\\n")?,
                CharEscape::FormFeed => writer.write_all(b"\\f")?,
                CharEscape::CarriageReturn => writer.write_all(b"\\r")?,
                CharEscape::Quote => writer.write_all(b"\\\"")?,
                CharEscape::ReverseSolidus => writer.write_all(b"\\\\")?,
                CharEscape::Solidus => writer.write_all(b"\\/")?,
                CharEscape::AsciiControl(byte) => writer.write_all(&[b'\\', b'u', byte])?,
            }
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = TestFormatter;
    let value = "Hello \"World\""; // contains escaped quotes

    format_escaped_str_contents(&mut writer, &mut formatter, value).unwrap();
}

