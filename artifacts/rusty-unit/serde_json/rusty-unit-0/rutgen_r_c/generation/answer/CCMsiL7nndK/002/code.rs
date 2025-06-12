// Answer 0

#[test]
fn test_format_escaped_str_success() {
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

    struct MockFormatter {
        begin_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { begin_called: false }
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            self.begin_called = true;
            Ok(())
        }
        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }
        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str(&mut writer, &mut formatter, "test\\nvalue");
    
    assert!(result.is_ok());
    assert!(formatter.begin_called);
}

#[test]
fn test_format_escaped_str_contents_err() {
    struct MockFailingWriter;

    impl io::Write for MockFailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        begin_called: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { begin_called: false }
        }
    }

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            self.begin_called = true;
            Ok(())
        }
        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }
        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockFailingWriter;
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str(&mut writer, &mut formatter, "test\\nvalue");
    
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_format_escaped_str_panic() {
    struct PanicFormatter;

    impl Formatter for PanicFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            panic!("Panic in begin string");
        }
        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }
        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = vec![];
    let mut formatter = PanicFormatter;
    let _ = format_escaped_str(&mut writer, &mut formatter, "test panic");
}

