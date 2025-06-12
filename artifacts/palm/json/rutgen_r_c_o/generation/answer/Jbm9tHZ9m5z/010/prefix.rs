// Answer 0

#[test]
fn test_format_escaped_str_contents_empty_string() {
    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let value = "";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_normal_string() {
    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let value = "normal text";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_long_string() {
    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let value = "a".repeat(1024); // a long string of 1024 "a" characters
    format_escaped_str_contents(&mut writer, &formatter, value);
}

// Helper structures and implementation for testing purposes
struct TestFormatter;

impl io::Write for Vec<u8> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Formatter for TestFormatter {
    fn write_string_fragment<W: io::Write>(&mut self, writer: &mut W, value: &str) -> io::Result<()> {
        writer.write(value.as_bytes())?;
        Ok(())
    }
    
    fn write_char_escape<W: io::Write>(&mut self, writer: &mut W, escape: CharEscape) -> io::Result<()> {
        match escape {
            CharEscape::Quote => writer.write(b"\\\"")?,
            CharEscape::ReverseSolidus => writer.write(b"\\\\")?,
            CharEscape::Solidus => writer.write(b"\\/")?,
            CharEscape::Backspace => writer.write(b"\\b")?,
            CharEscape::FormFeed => writer.write(b"\\f")?,
            CharEscape::LineFeed => writer.write(b"\\n")?,
            CharEscape::CarriageReturn => writer.write(b"\\r")?,
            CharEscape::Tab => writer.write(b"\\t")?,
            CharEscape::AsciiControl(byte) => {
                write!(writer, "\\u{:02x}", byte)?;
            }
        }
        Ok(())
    }
}

