// Answer 0

#[test]
fn test_format_escaped_str_contents_empty_string() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "");
    assert!(result.is_ok());
    assert_eq!(writer.output, b"");
}

#[test]
fn test_format_escaped_str_contents_no_escapes() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello World");
    assert!(result.is_ok());
    assert_eq!(writer.output, b"Hello World");
}

#[test]
fn test_format_escaped_str_contents_with_escapes() {
    struct MockWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            // For testing purposes, we will just encode the escape in a simple manner
            let escape_char = match escape {
                CharEscape::Quote => b'\"',
                CharEscape::ReverseSolidus => b'\\',
                CharEscape::Solidus => b'/',
                CharEscape::Backspace => b'\x08',
                CharEscape::FormFeed => b'\x0C',
                CharEscape::LineFeed => b'\n',
                CharEscape::CarriageReturn => b'\r',
                CharEscape::Tab => b'\t',
                CharEscape::AsciiControl(byte) => byte,
            };
            writer.write(&[escape_char])?;
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello \"World\"\n");
    assert!(result.is_ok());
    assert_eq!(writer.output, b"Hello \"World\"\n");
}

