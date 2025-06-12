// Answer 0

#[test]
fn test_format_escaped_str() {
    use std::io::{self, Cursor, Write};
    
    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string<W: io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
            write!(writer, "\"")?;
            Ok(())
        }

        fn end_string<W: io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
            write!(writer, "\"")?;
            Ok(())
        }
    }

    let mut output = Cursor::new(Vec::new());
    let mut formatter = TestFormatter;

    // Test normal string
    let result = format_escaped_str(&mut output, &mut formatter, "hello");
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"\"hello\"");

    // Reset output for next test
    output.set_position(0);
    output.get_mut().clear();
    
    // Test string with escape characters
    let result = format_escaped_str(&mut output, &mut formatter, "hello\nworld");
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"\"hello\nworld\"");
    
    // Reset output for next test
    output.set_position(0);
    output.get_mut().clear();
    
    // Test empty string
    let result = format_escaped_str(&mut output, &mut formatter, "");
    assert!(result.is_ok());
    assert_eq!(output.into_inner(), b"\"\"");
}

