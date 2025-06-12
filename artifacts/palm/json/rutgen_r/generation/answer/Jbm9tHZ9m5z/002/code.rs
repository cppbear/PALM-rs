// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape_needed() {
    use std::io;
    
    struct TestFormatter;
    
    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;
    let value = "Hello World";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_some_escape_needed() {
    use std::io;
    
    struct TestFormatter;
    
    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;
    let value = "Hello \n World";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_all_escape_needed() {
    use std::io;
    
    struct TestFormatter;
    
    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;
    let value = "\x00\x01\x02\x03";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_empty_value() {
    use std::io;
    
    struct TestFormatter;
    
    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;
    let value = "";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_format_escaped_str_contents_panic_scenario() {
    use std::io;

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;
    let value = "Hello \u{10FFFF} World"; // This may cause panic if the value can't be escaped properly

    let _ = format_escaped_str_contents(&mut writer, &mut formatter, value);
}

