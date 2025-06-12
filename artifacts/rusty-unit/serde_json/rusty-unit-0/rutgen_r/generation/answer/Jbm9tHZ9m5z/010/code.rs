// Answer 0

#[test]
fn test_format_escaped_str_contents_empty() {
    use std::io;
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            // Simulate an escape write
            writer.write_all(b"\\uXXXX")?;
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = "";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, b"");
}

#[test]
fn test_format_escaped_str_contents_no_escape() {
    use std::io;
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = "Hello, World!";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, b"Hello, World!");
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    use std::io;
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
            writer.write_all(fragment.as_bytes())?;
            Ok(())
        }

        fn write_char_escape(&mut self, writer: &mut dyn io::Write, _escape: CharEscape) -> io::Result<()> {
            writer.write_all(b"\\uXXXX")?;
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;
    let value = "Hello, \"World\"!";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, b"Hello, \\uXXXXWorld!"); // Adjust expected output based on escape handling
}

