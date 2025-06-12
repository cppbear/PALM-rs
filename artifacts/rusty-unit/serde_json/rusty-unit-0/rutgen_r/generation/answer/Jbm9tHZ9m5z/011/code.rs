// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape() {
    use std::io::Cursor;
    use std::io;
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()> {
            writer.write_all(&[char_escape.as_byte()]) // Assuming CharEscape has this method
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter;
    let value = "Hello, World!";

    let result = format_escaped_str_contents(&mut buffer, &mut formatter, value);
    assert!(result.is_ok());
    
    let output = String::from_utf8(buffer.into_inner()).unwrap();
    assert_eq!(output, value);
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    use std::io::Cursor;
    use std::io;
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()> {
            writer.write_all(&[char_escape.as_byte()]) // Assuming CharEscape has this method
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter;
    let value = "Hello, \nWorld!"; // \n is expected to be escaped

    let result = format_escaped_str_contents(&mut buffer, &mut formatter, value);
    assert!(result.is_ok());

    let output = String::from_utf8(buffer.into_inner()).unwrap();
    assert_eq!(output, "Hello, \\nWorld!"); // Assuming \n gets escaped to \\n
}

#[test]
fn test_format_escaped_str_contents_empty_string() {
    use std::io::Cursor;
    use std::io;
    
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()> {
            writer.write_all(&[char_escape.as_byte()]) // Assuming CharEscape has this method
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter;
    let value = ""; // Edge case: empty string

    let result = format_escaped_str_contents(&mut buffer, &mut formatter, value);
    assert!(result.is_ok());

    let output = String::from_utf8(buffer.into_inner()).unwrap();
    assert_eq!(output, value);
}

