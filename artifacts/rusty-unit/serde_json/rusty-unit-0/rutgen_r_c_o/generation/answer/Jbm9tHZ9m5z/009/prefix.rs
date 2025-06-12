// Answer 0

#[test]
fn test_format_escaped_str_contents_empty_string() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "").unwrap();
}

#[test]
fn test_format_escaped_str_contents_single_control_character() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "\u{01}").unwrap();
}

#[test]
fn test_format_escaped_str_contents_multiple_control_characters() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "\u{01}\u{02}\u{03}").unwrap();
}

#[test]
fn test_format_escaped_str_contents_backslash_and_quote() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "\\\"").unwrap();
}

#[test]
fn test_format_escaped_str_contents_all_control_characters() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "\u{00}\u{01}\u{02}\u{03}\u{04}\u{05}\u{06}\u{07}\u{08}\u{09}\u{0A}\u{0B}\u{0C}\u{0D}\u{0E}\u{0F}").unwrap();
}

#[test]
fn test_format_escaped_str_contents_non_control_characters() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "Hello World!").unwrap();
}

#[test]
fn test_format_escaped_str_contents_special_characters() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "\n\t\\\"").unwrap();
}

#[test]
fn test_format_escaped_str_contents_combined_string() {
    let mut writer = Vec::new();
    let formatter = SimpleFormatter;
    format_escaped_str_contents(&mut writer, &mut formatter, "Line1\nLine2\tWith \\ Escapes \"Quotes\"").unwrap();
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn write_string_fragment<W: ?Sized + io::Write>(&mut self, _: &mut W, _: &str) -> io::Result<()> {
        Ok(())
    }

    fn write_char_escape<W: ?Sized + io::Write>(&mut self, _: &mut W, _: CharEscape) -> io::Result<()> {
        Ok(())
    }
}

