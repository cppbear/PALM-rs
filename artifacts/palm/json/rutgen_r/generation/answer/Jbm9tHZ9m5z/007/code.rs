// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape() {
    use std::io::{self, Write};

    struct TestWriter {
        output: Vec<u8>,
    }

    impl Write for TestWriter {
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
        fn write_string_fragment(&mut self, writer: &mut dyn Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn Write, _escape: CharEscape) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Test error"))
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let mut formatter = TestFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, "no escape");
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert_eq!(e.kind(), io::ErrorKind::Other);
    }
}

#[test]
fn test_format_escaped_str_contents_immediate_escape() {
    use std::io::{self, Write};

    struct TestWriter {
        output: Vec<u8>,
    }

    impl Write for TestWriter {
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
        fn write_string_fragment(&mut self, writer: &mut dyn Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn Write, _escape: CharEscape) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Test error"))
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let mut formatter = TestFormatter;

    let result = format_escaped_str_contents(&mut writer, &mut formatter, "\n");
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert_eq!(e.kind(), io::ErrorKind::Other);
    }
}

