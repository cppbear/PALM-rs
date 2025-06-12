// Answer 0

fn test_format_escaped_str_contents_escape_zero() -> Result<(), Box<dyn std::error::Error>> {
    use std::io;

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, s: &str) -> Result<(), io::Error> {
            // Simulate a successful write
            writer.write_all(s.as_bytes()).map_err(|e| e)
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, _char_escape: CharEscape) -> Result<(), io::Error> {
            // Simulate an error condition
            Err(io::Error::new(io::ErrorKind::Other, "write_char_escape error"))
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let value = "abc"; // 'a', 'b', 'c' are valid bytes, and hence ESCAPE for them is 0.

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_err());

    Ok(())
}

fn test_format_escaped_str_contents_start_equals_i() -> Result<(), Box<dyn std::error::Error>> {
    use std::io;

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_string_fragment(&mut self, writer: &mut dyn io::Write, s: &str) -> Result<(), io::Error> {
            // Simulate a successful write
            writer.write_all(s.as_bytes()).map_err(|e| e)
        }
        
        fn write_char_escape(&mut self, writer: &mut dyn io::Write, _char_escape: CharEscape) -> Result<(), io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "write_char_escape error"))
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;

    let value = "\x01"; // ESCAPE[1] would cause the start to equal i condition.

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);

    assert!(result.is_err());

    Ok(())
}

