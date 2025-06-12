// Answer 0

#[test]
fn test_format_escaped_str_contents_with_escape_condition() {
    struct MockWriter {}
    struct MockFormatter {
        called: bool,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _value: &str) -> io::Result<()> {
            Err(Error::new(ErrorCode::CustomError, "error"))
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {};
    let mut formatter = MockFormatter { called: false };
    let value = "Sample\0Text"; // contains unescaped ASCII control character

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_contents_with_start_less_than_i() {
    struct MockWriter {}
    struct MockFormatter {
        called: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _value: &str) -> io::Result<()> {
            Err(Error::new(ErrorCode::CustomError, "error"))
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {};
    let mut formatter = MockFormatter { called: false };
    let value = "Hello\x02World"; // byte '\x02' causes escape == 0

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_contents_should_panic() {
    struct MockWriter {}
    struct MockFormatter {
        called: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _value: &str) -> io::Result<()> {
            panic!("Panic triggered!");
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {};
    let mut formatter = MockFormatter { called: false };
    let value = "Text with unescaped control \x03 char"; // byte '\x03' causes panic

    let result = std::panic::catch_unwind(|| {
        format_escaped_str_contents(&mut writer, &mut formatter, value);
    });
    
    assert!(result.is_err());
}

