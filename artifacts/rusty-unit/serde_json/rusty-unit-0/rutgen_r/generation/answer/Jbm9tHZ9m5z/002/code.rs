// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape_sequences() {
    use std::io::{self, Cursor};

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            // Mock implementation that does nothing for escapes
            Ok(())
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut formatter = TestFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello, world!");

    assert!(result.is_ok());
    assert_eq!(writer.into_inner(), b"Hello, world!");
}

#[test]
fn test_format_escaped_str_contents_with_escape_sequences() {
    use std::io::{self, Cursor};

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            writer.write_all(b"\\")?;  // Mocking escape write with a backslash
            Ok(())
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut formatter = TestFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello\tworld");

    assert!(result.is_ok());
    assert_eq!(writer.into_inner(), b"Hello\\world");
}

#[test]
#[should_panic]
fn test_format_escaped_str_contents_panic_on_out_of_bounds() {
    use std::io::{self, Cursor};

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            // Mock implementation that does nothing for escapes
            Ok(())
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut formatter = TestFormatter;
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, ""); // This should panic due to value[start..]
}

#[test]
fn test_format_escaped_str_contents_edge_cases() {
    use std::io::{self, Cursor};

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            writer.write_all(b"\\")?;  // Mocking escape write with a backslash
            Ok(())
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut formatter = TestFormatter;
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "\n\t\r");

    assert!(result.is_ok());
    assert_eq!(writer.into_inner(), b"\\n\\t\\r");
}

