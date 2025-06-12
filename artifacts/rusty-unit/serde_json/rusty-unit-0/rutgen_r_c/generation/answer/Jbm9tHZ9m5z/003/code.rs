// Answer 0

#[test]
fn test_format_escaped_str_contents_escape_conditions() {
    use crate::io::{self, Write};
    use crate::ser::{format_escaped_str_contents, Formatter};
    use std::io::Cursor;

    struct MockFormatter {
        data: Vec<u8>,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { data: Vec::new() }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn Write, _char_escape: CharEscape) -> io::Result<()> {
            // For test purposes, just return Ok
            Ok(())
        }
    }

    // Test input string with valid characters that should not trigger escapes
    let input = "Hello, World!";
    let mut writer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str_contents(&mut writer, &mut formatter, input);
    assert!(result.is_ok());

    // Test with input string that includes a character requiring an escape
    let escaping_input = "Line1:\nLine2:\t";
    let mut writer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str_contents(&mut writer, &mut formatter, escaping_input);
    assert!(result.is_ok());

    // Test edge case with empty string input, should return Ok
    let empty_input = "";
    let mut writer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str_contents(&mut writer, &mut formatter, empty_input);
    assert!(result.is_ok());

    // Test input where the formatter fails to write, should return Err
    struct FailingFormatter;

    impl Formatter for FailingFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn Write, _fragment: &str) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "mock failure"))
        }

        fn write_char_escape(&mut self, _writer: &mut dyn Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let failing_input = "Hello, World!";
    let mut writer = Cursor::new(Vec::new());
    let mut formatter = FailingFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, failing_input);
    assert!(result.is_err());
}

