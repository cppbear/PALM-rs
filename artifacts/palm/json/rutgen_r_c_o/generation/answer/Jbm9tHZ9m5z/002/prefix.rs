// Answer 0

#[test]
fn test_format_escaped_str_contents_basic() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    let value = "Hello, World!\nThis is a test string with a tab\tcharacter.";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_with_quotes() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    let value = "This \"string\" contains escaped quotes.";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_with_backslashes() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    let value = "This string contains a backslash \\ character.";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_with_control_chars() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    let value = "Control chars: \x00\x01\x02";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_empty_string() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    let value = "";
    format_escaped_str_contents(&mut writer, &formatter, value);
}

#[test]
fn test_format_escaped_str_contents_max_length() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    let value = "A".repeat(255);
    format_escaped_str_contents(&mut writer, &formatter, &value);
}

struct SimpleFormatter;

impl io::Write for SimpleFormatter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Formatter for SimpleFormatter {
    fn write_string_fragment(&mut self, writer: &mut dyn io::Write, fragment: &str) -> io::Result<()> {
        writer.write_all(fragment.as_bytes())?;
        Ok(())
    }

    fn write_char_escape(&mut self, writer: &mut dyn io::Write, escape: CharEscape) -> io::Result<()> {
        let escaped = match escape {
            CharEscape::Quote => b"\\\"",
            CharEscape::ReverseSolidus => b"\\\\",
            CharEscape::Solidus => b"\\/",
            CharEscape::Backspace => b"\\b",
            CharEscape::FormFeed => b"\\f",
            CharEscape::LineFeed => b"\\n",
            CharEscape::CarriageReturn => b"\\r",
            CharEscape::Tab => b"\\t",
            CharEscape::AsciiControl(byte) => format!("\\u{:04x}", byte).as_bytes(),
        };
        writer.write_all(escaped)?;
        Ok(())
    }
}

