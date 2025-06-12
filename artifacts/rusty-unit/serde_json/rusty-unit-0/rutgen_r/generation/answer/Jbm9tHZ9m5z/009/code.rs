// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape() {
    use std::io::{self, Write};

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello, World!");
    
    assert!(result.is_ok());
    assert_eq!(writer, b"Hello, World!");
}

#[test]
fn test_format_escaped_str_contents_with_escapes() {
    use std::io::{self, Write};

    struct TestFormatter {
        wrote_escapes: bool,
    }

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            self.wrote_escapes = true; // simulating that we wrote something
            Ok(())
        }
    }

    let mut writer = Vec::new();
    let mut formatter = TestFormatter { wrote_escapes: false };

    // Assume ESCAPE maps to an escape for '\n' (newline character)
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello\nWorld");

    assert!(result.is_ok());
    assert!(formatter.wrote_escapes);
    assert_eq!(writer, b"Hello"); // Assuming '\n' would be escaped but not in output.
}

#[test]
#[should_panic]
fn test_format_escaped_str_contents_start_panic() {
    use std::io::{self, Write};

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = TestFormatter;

    // This input should cause a panic since it starts at an invalid index 
    let result = format_escaped_str_contents(&mut Vec::new(), &mut formatter, "");
    assert!(result.is_err());
}

