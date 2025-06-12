// Answer 0

#[test]
fn test_char_01() {
    let c = '\u{0000}';
    let mut formatter = std::fmt::Formatter::new();
    CharVisitor.expecting(&mut formatter);
}

#[test]
fn test_char_02() {
    let c = '\u{007F}';
    let mut formatter = std::fmt::Formatter::new();
    CharVisitor.expecting(&mut formatter);
}

#[test]
fn test_char_03() {
    let c = '\u{00FF}';
    let mut formatter = std::fmt::Formatter::new();
    CharVisitor.expecting(&mut formatter);
}

#[test]
fn test_char_04() {
    let c = '\u{01F600}'; // emoji character
    let mut formatter = std::fmt::Formatter::new();
    CharVisitor.expecting(&mut formatter);
}

#[test]
fn test_char_05() {
    let c = '\u{10FFFF}'; // maximum valid Unicode scalar value
    let mut formatter = std::fmt::Formatter::new();
    CharVisitor.expecting(&mut formatter);
}

