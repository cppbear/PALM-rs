// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape() {
    struct MockWriter;
    struct MockFormatter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            assert_eq!(buf, b"Hello, World!");
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, buf: &str) -> io::Result<()> {
            assert_eq!(buf, "Hello, World!");
            Ok(())
        }
        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            unreachable!()
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello, World!");
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    struct MockWriter;
    struct MockFormatter {
        fragments: Vec<String>,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { fragments: Vec::new() }
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(_buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, buf: &str) -> io::Result<()> {
            self.fragments.push(buf.to_string());
            Ok(())
        }
        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
            match escape {
                CharEscape::Quote => self.fragments.push("\\\"".to_string()),
                CharEscape::ReverseSolidus => self.fragments.push("\\\\".to_string()),
                CharEscape::Solidus => self.fragments.push("\\/".to_string()),
                CharEscape::Backspace => self.fragments.push("\\b".to_string()),
                CharEscape::FormFeed => self.fragments.push("\\f".to_string()),
                CharEscape::LineFeed => self.fragments.push("\\n".to_string()),
                CharEscape::CarriageReturn => self.fragments.push("\\r".to_string()),
                CharEscape::Tab => self.fragments.push("\\t".to_string()),
                CharEscape::AsciiControl(byte) => self.fragments.push(format!("\\u{:02x}", byte)),
            }
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello, \"World!\n\"");
    assert!(result.is_ok());
    assert_eq!(formatter.fragments.len(), 5);
    assert_eq!(formatter.fragments[0], "Hello, ");
    assert_eq!(formatter.fragments[1], "\\\"");
    assert_eq!(formatter.fragments[2], "World!");
    assert_eq!(formatter.fragments[3], "\\n");
    assert_eq!(formatter.fragments[4], "\\\"");
}

