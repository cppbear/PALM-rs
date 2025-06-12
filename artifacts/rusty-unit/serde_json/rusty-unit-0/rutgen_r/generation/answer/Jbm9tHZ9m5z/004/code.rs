// Answer 0

fn test_format_escaped_str_contents_success() -> io::Result<()> {
    struct TestFormatter {}
    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            writer.write_all(b"\\")?; // Simulate escaping
            Ok(())
        }
    }

    let mut output: Vec<u8> = Vec::new();
    let value = "Hello, world!";
    let mut formatter = TestFormatter {};

    assert!(format_escaped_str_contents(&mut output, &mut formatter, value).is_ok());
    assert_eq!(String::from_utf8(output).expect("Invalid UTF-8"), "Hello, world!");
    Ok(())
}

#[test]
fn test_format_escaped_str_contents_escape() {
    struct TestFormatter {
        should_fail: bool,
    }

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            if self.should_fail {
                return Err(io::Error::new(io::ErrorKind::Other, "String fragment failure"));
            }
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            writer.write_all(b"\\")?; // Simulate escaping
            Ok(())
        }
    }

    let mut output: Vec<u8> = Vec::new();
    let value = "Escape me! \n";
    let mut formatter = TestFormatter { should_fail: true };

    assert!(format_escaped_str_contents(&mut output, &mut formatter, value).is_err());
}

#[test]
fn test_format_escaped_str_contents_boundary() {
    struct TestFormatter {}

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }
        
        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()> {
            writer.write_all(b"\\")?; // Simulate escaping
            Ok(())
        }
    }

    let mut output: Vec<u8> = Vec::new();
    let value = ""; // Testing empty string
    let mut formatter = TestFormatter {};

    assert!(format_escaped_str_contents(&mut output, &mut formatter, value).is_ok());
    assert_eq!(String::from_utf8(output).expect("Invalid UTF-8"), "");
    Ok(())
}

