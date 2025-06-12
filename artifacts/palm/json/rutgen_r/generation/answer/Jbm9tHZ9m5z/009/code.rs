// Answer 0

#[test]
fn test_format_escaped_str_contents_empty_string() {
    let mut writer = Vec::new();
    let formatter = DummyFormatter;
    let value = "";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer, b"");
}

#[test]
fn test_format_escaped_str_contents_no_escape() {
    let mut writer = Vec::new();
    let formatter = DummyFormatter;
    let value = "Hello World!";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer, b"Hello World!");
}

#[test]
fn test_format_escaped_str_contents_with_escape() {
    let mut writer = Vec::new();
    let formatter = DummyFormatter;
    let value = "Hello\nWorld!";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer, b"Hello\\nWorld!");
}

#[test]
fn test_format_escaped_str_contents_multiple_escapes() {
    let mut writer = Vec::new();
    let formatter = DummyFormatter;
    let value = "A\tB\nC\"D\\E";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer, b"A\\tB\\nC\\\"D\\\\E");
}

#[test]
fn test_format_escaped_str_contents_just_escape() {
    let mut writer = Vec::new();
    let formatter = DummyFormatter;
    let value = "\n";

    let result = format_escaped_str_contents(&mut writer, &mut formatter, value);
    assert!(result.is_ok());
    assert_eq!(writer, b"\\n");
}

struct DummyFormatter;

impl Formatter for DummyFormatter {
    fn write_string_fragment<W: ?Sized + io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
        writer.write_all(fragment.as_bytes())
    }

    fn write_char_escape<W: ?Sized + io::Write>(&mut self, writer: &mut W, _char_escape: CharEscape) -> io::Result<()> {
        writer.write_all(b"\\n")
    }
}

trait Formatter {
    fn write_string_fragment<W: ?Sized + io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>;
    fn write_char_escape<W: ?Sized + io::Write>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()>;
}

enum CharEscape {
    Escape,
}

impl CharEscape {
    fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
        CharEscape::Escape
    }
}

