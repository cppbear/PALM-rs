// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape() {
    use std::io::{self, Write};

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, value: &str) -> io::Result<()> {
            writer.write_all(value.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            Ok(()) // For testing, we do not need real implementation
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    let value = "This string has no escapes.";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(writer.buffer).unwrap(), value);
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    use std::io::{self, Write};

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, value: &str) -> io::Result<()> {
            writer.write_all(value.as_bytes())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            Ok(()) // For testing, we do not need real implementation
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    let value = "This string has a newline\ncharacter.";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_ok());
    assert!(String::from_utf8(writer.buffer).unwrap().contains("This string has a newline\\ncharacter."));
}

#[test]
#[should_panic]
fn test_format_escaped_str_contents_panics_on_out_of_bounds() {
    use std::io::{self, Write};

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, _value: &str) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;
    let value = "A string that causes panic";
    
    // Assuming that this value will cause out-of-bounds access
    let bad_index = value.len() + 1;
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, &value[0..bad_index]);
}

