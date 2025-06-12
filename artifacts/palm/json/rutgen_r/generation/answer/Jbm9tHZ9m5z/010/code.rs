// Answer 0

#[test]
fn test_format_escaped_str_contents_empty_string() {
    use std::io::{self, Write};

    struct TestFormatter;

    impl TestFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn Write, fragment: &str) -> io::Result<()> {
            assert_eq!(fragment, ""); // Expect empty fragment
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn Write, _char_escape: CharEscape) -> io::Result<()> {
            // No char escapes should be written for an empty string
            panic!("write_char_escape should not be called for an empty string");
        }
    }

    let mut output = Vec::new();
    let mut formatter = TestFormatter;
    let result = format_escaped_str_contents(&mut output, &mut formatter, "");
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_no_escape_characters() {
    use std::io::{self, Write};

    struct TestFormatter;

    impl TestFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn Write, fragment: &str) -> io::Result<()> {
            assert_eq!(fragment, "Hello World!"); // Expect the full fragment
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn Write, _char_escape: CharEscape) -> io::Result<()> {
            panic!("write_char_escape should not be called for no escape characters");
        }
    }

    let mut output = Vec::new();
    let mut formatter = TestFormatter;
    let result = format_escaped_str_contents(&mut output, &mut formatter, "Hello World!");
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_with_escape_character() {
    use std::io::{self, Write};

    struct TestFormatter;

    impl TestFormatter {
        fn write_string_fragment(&mut self, _writer: &mut dyn Write, fragment: &str) -> io::Result<()> {
            assert_eq!(fragment, "Hello "); // Expect fragment up to escape character
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn Write, _char_escape: CharEscape) -> io::Result<()> {
            // Expect this to be called for the escape character
            return Ok(());
        }
    }

    let mut output = Vec::new();
    let mut formatter = TestFormatter;
    let result = format_escaped_str_contents(&mut output, &mut formatter, "Hello \tWorld!");
    assert!(result.is_ok());
}

