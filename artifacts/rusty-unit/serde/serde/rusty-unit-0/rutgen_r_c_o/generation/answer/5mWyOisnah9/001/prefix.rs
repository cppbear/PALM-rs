// Answer 0

#[test]
fn test_expecting_true() {
    use std::fmt::Formatter;
    let mut formatter = std::fmt::Formatter::default();
    let visitor = BoolVisitor;
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_false() {
    use std::fmt::Formatter;
    let mut formatter = std::fmt::Formatter::default();
    let visitor = BoolVisitor;
    visitor.expecting(&mut formatter);
}

