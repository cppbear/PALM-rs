// Answer 0

#[test]
fn test_fmt_char_null() {
    let unexpected = Unexpected::Char('\0');
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", unexpected);
}

#[test]
fn test_fmt_char_a() {
    let unexpected = Unexpected::Char('a');
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", unexpected);
}

#[test]
fn test_fmt_char_z() {
    let unexpected = Unexpected::Char('z');
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", unexpected);
}

#[test]
fn test_fmt_char_uppercase() {
    let unexpected = Unexpected::Char('A');
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", unexpected);
}

#[test]
fn test_fmt_char_special() {
    let unexpected = Unexpected::Char('!');
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", unexpected);
}

#[test]
fn test_fmt_char_unicode() {
    let unexpected = Unexpected::Char('\u{1F600}'); // Grinning face emoji
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", unexpected);
}

