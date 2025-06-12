// Answer 0

#[test]
fn test_format_escaped_str_contents_with_escaped_newline() {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.buffer.len() + buf.len() > 10 {
                return Err(io::Error::new(io::ErrorKind::Other, "buffer overflow"));
            }
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        write_fragment_called: bool,
        write_escape_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                write_fragment_called: false,
                write_escape_called: false,
            }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, value: &str) -> io::Result<()> {
            self.write_fragment_called = true;
            if value.is_empty() {
                return Ok(());
            }
            writer.write(value.as_bytes())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            self.write_escape_called = true;
            Err(io::Error::new(io::ErrorKind::Other, "escape error"))
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter::new();
    let value = "abc\\nxyz";
    
    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    // Uncomment to view result
    // assert!(result.is_err());
}

#[test]
fn test_format_escaped_str_contents_with_empty_string() {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        write_fragment_called: bool,
        write_escape_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                write_fragment_called: false,
                write_escape_called: false,
            }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _: &mut dyn io::Write, _: &str) -> io::Result<()> {
            self.write_fragment_called = true;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            self.write_escape_called = true;
            Err(io::Error::new(io::ErrorKind::Other, "escape error"))
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter::new();
    let value = "";
    
    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    // Uncomment to view result
    // assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_with_trailing_escape() {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.buffer.len() + buf.len() > 10 {
                return Err(io::Error::new(io::ErrorKind::Other, "buffer overflow"));
            }
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        write_fragment_called: bool,
        write_escape_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                write_fragment_called: false,
                write_escape_called: false,
            }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, value: &str) -> io::Result<()> {
            self.write_fragment_called = true;
            writer.write(value.as_bytes())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            self.write_escape_called = true;
            Err(io::Error::new(io::ErrorKind::Other, "escape error"))
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter::new();
    let value = "abc\\";
    
    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    // Uncomment to view result
    // assert!(result.is_err());
}

