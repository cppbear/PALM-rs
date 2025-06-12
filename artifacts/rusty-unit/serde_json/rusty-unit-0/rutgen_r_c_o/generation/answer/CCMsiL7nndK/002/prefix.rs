// Answer 0

#[test]
fn test_format_escaped_str_empty_writer() {
    struct TestFormatter {
        should_fail: bool,
    }

    impl Formatter for TestFormatter {
        fn begin_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn end_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: ?Sized + io::Write>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = Vec::new();
    let mut formatter = TestFormatter { should_fail: false };
    let value = "Hello\nWorld";

    let _ = format_escaped_str(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_valid_writer() {
    struct TestFormatter {
        should_fail: bool,
    }

    impl Formatter for TestFormatter {
        fn begin_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn end_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: ?Sized + io::Write>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = Vec::new();
    let mut formatter = TestFormatter { should_fail: false };
    let value = "Hello\tWorld, \"Test\"";

    let _ = format_escaped_str(&mut writer, &mut formatter, value);
}

#[test]
fn test_format_escaped_str_formatter_error() {
    struct TestFormatter {
        should_fail: bool,
    }

    impl Formatter for TestFormatter {
        fn begin_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn end_string<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
            Ok(())
        }

        fn write_string_fragment<W: ?Sized + io::Write>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(&mut self, _writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "simulated error"))
        }
    }

    let mut writer = Vec::new();
    let mut formatter = TestFormatter { should_fail: true };
    let value = "Any string.";

    let _ = format_escaped_str(&mut writer, &mut formatter, value);
}

