// Answer 0

fn test_format_escaped_str_contents_panic() -> io::Result<()> {
    struct MockFormatter;
    
    impl Formatter for MockFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Mock error"))
        }

        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(()) // No error for writing char escape
        }
    }

    let value = "Escape this!"; // Sample string to test against

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = MockFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_err(), "Expected an error result");
    match result {
        Err(e) => assert_eq!(e.kind(), io::ErrorKind::Other),
        _ => {}
    }

    Ok(())
}

fn test_format_escaped_str_contents_no_escape() -> io::Result<()> {
    struct MockFormatter;
    
    impl Formatter for MockFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()> {
            Ok(()) // No error for writing string fragment
        }

        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(()) // No error for writing char escape
        }
    }

    let value = "NoEscape"; // Sample string with no escape characters

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = MockFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_ok(), "Expected a successful result");

    Ok(())
}

fn test_format_escaped_str_contents_part_escape() -> io::Result<()> {
    struct MockFormatter;
    
    impl Formatter for MockFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _writer: &mut W, _fragment: &str) -> io::Result<()> {
            Ok(()) // No error for writing string fragment
        }

        fn write_char_escape<W: io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(()) // No error for writing char escape
        }
    }

    let value = "Hello\nWorld"; // Sample string with an escape character

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = MockFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_ok(), "Expected a successful result");

    Ok(())
}

