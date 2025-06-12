// Answer 0

#[test]
fn test_format_escaped_str_contents_empty_string() {
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
    
    struct MockFormatter {
        written_fragments: Vec<String>,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                written_fragments: Vec::new(),
            }
        }
    }
    
    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            self.written_fragments.push(fragment.to_string());
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            // Just simulate writing an escaped character
            writer.write_all(b"\\u")?;
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter::new();
    let value = "";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_ok());
    assert_eq!(writer.output, "");
}

#[test]
fn test_format_escaped_str_contents_no_escapes() {
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
    
    struct MockFormatter{
        written_fragments: Vec<String>,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                written_fragments: Vec::new(),
            }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            self.written_fragments.push(fragment.to_string());
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            panic!("should not write char escape here")
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter::new();
    let value = "hello world";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_ok());
    assert_eq!(writer.output, "hello world");
}

#[test]
fn test_format_escaped_str_contents_multiple_escapes() {
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
    
    struct MockFormatter {
        written_fragments: Vec<String>,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                written_fragments: Vec::new(),
            }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            self.written_fragments.push(fragment.to_string());
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, char_escape: CharEscape) -> io::Result<()> {
            // Simulate writing the escape
            match char_escape {
                CharEscape::Quote => writer.write_all(b"\\\"")?,
                CharEscape::ReverseSolidus => writer.write_all(b"\\\\")?,
                CharEscape::Solidus => writer.write_all(b"\\/")?,
                _ => {}
            }
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter::new();
    let value = "\"backslash\\tab\ntest\"";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_ok());
    assert_eq!(writer.output, "\\\"backslash\\\\tab\\ntest\\\"");
}

