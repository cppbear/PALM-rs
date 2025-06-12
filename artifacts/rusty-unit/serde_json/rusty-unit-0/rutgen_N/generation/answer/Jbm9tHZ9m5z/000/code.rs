// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape() {
    use std::io::{self, Cursor};

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter;
    let value = "Hello, World!";
    let result = format_escaped_str_contents(&mut buffer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(buffer.into_inner(), b"Hello, World!");
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    use std::io::{self, Cursor};

    struct MockFormatter {
        written: Vec<u8>,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { written: Vec::new() }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            self.written.extend_from_slice(fragment.as_bytes());
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()> {
            let byte = char_escape.to_byte(); // Assume CharEscape has a to_byte method
            self.written.push(byte);
            writer.write_all(&[byte])?;
            Ok(())
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter::new();
    let value = "Hello, \nWorld!";
    let result = format_escaped_str_contents(&mut buffer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(buffer.into_inner(), b"Hello, \\nWorld!");
}

