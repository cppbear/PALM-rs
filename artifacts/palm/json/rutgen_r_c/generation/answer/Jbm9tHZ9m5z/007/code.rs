// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape_characters() {
    struct DummyWriter {
        output: String,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter {
        error_on_escape: bool,
    }

    impl super::Formatter for DummyFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _fragment: &str) -> io::Result<()> {
            Ok(())
        }
        
        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: super::CharEscape) -> io::Result<()> {
            if self.error_on_escape {
                return Err(io::Error::new(io::ErrorKind::Other, "Forced error"));
            }
            Ok(())
        }
    }
    
    let test_string = "Hello, World!";
    let mut writer = DummyWriter { output: String::new() };
    let mut formatter = DummyFormatter { error_on_escape: false };
    
    let result = format_escaped_str_contents(&mut writer, &mut formatter, test_string);
    
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_with_escape_character() {
    struct DummyWriter {
        output: String,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter {
        error_on_escape: bool,
    }

    impl super::Formatter for DummyFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _fragment: &str) -> io::Result<()> {
            Ok(())
        }
        
        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: super::CharEscape) -> io::Result<()> {
            if self.error_on_escape {
                return Err(io::Error::new(io::ErrorKind::Other, "Forced error"));
            }
            Ok(())
        }
    }
    
    let test_string = "Hello, \"World!\"";
    let mut writer = DummyWriter { output: String::new() };
    
    // This formatter will simulate an error while escaping
    let mut formatter = DummyFormatter { error_on_escape: true };
    
    let result = format_escaped_str_contents(&mut writer, &mut formatter, test_string);
    
    assert!(result.is_err());
    assert_eq!(writer.output, "Hello, ");
}

