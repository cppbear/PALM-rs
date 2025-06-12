// Answer 0

fn test_format_escaped_str_contents_success() -> Result<(), Box<dyn std::error::Error>> {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(
            &mut self,
            writer: &mut W,
            fragment: &str,
        ) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: ?Sized + io::Write>(
            &mut self,
            writer: &mut W,
            _char_escape: CharEscape,
        ) -> io::Result<()> {
            // Assuming the escaping is a no-op for this test
            Ok(())
        }
    }

    let mut writer = TestWriter { output: vec![] };
    let mut formatter = TestFormatter;

    // Test input with no characters requiring escaping
    let value = "Hello, World!";
    let result = format_escaped_str_contents(&mut writer, &mut formatter, value)?;
    assert_eq!(result, Ok(()));
    assert_eq!(String::from_utf8(writer.output).unwrap(), value);
    
    Ok(())
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(
            &mut self,
            writer: &mut W,
            fragment: &str,
        ) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())
        }

        fn write_char_escape<W: ?Sized + io::Write>(
            &mut self,
            writer: &mut W,
            _char_escape: CharEscape,
        ) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: vec![] };
    let mut formatter = TestFormatter;
    
    // Test value that includes a character which needs escaping
    let value = "Hello, \nWorld!";
    let result = format_escaped_str_contents(&mut writer, &mut formatter, value).unwrap();
    
    // Validate output and behavior
    assert_eq!(result, Ok(()));
    assert_eq!(String::from_utf8(writer.output).unwrap(), "Hello, World!"); // Newline removed
}

#[should_panic]
fn test_format_escaped_str_contents_panic() {
    struct BadFormatter;

    impl Formatter for BadFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(
            &mut self,
            _writer: &mut W,
            _fragment: &str,
        ) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to write")) // Induce panic
        }

        fn write_char_escape<W: ?Sized + io::Write>(
            &mut self,
            _writer: &mut W,
            _char_escape: CharEscape,
        ) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: vec![] };
    let mut formatter = BadFormatter;
    
    // This will trigger a panic due to the formatter error
    let value = "Hello, \nWorld!";
    let _result = format_escaped_str_contents(&mut writer, &mut formatter, value).unwrap(); // Unwrap to panic
}

fn main() {
    // Run the tests
    test_format_escaped_str_contents_success().unwrap();
    test_format_escaped_str_contents_with_escape();
    test_format_escaped_str_contents_panic();
}

