// Answer 0

#[test]
fn test_expecting_non_empty_formatter() {
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    let string_visitor = StringVisitor;

    let result = string_visitor.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(output, "a string");
}

#[test]
fn test_expecting_with_non_ascii_characters() {
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    let string_visitor = StringVisitor;

    let result = string_visitor.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(output, "a string");
}

#[test]
fn test_expecting_empty_formatter() {
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    let string_visitor = StringVisitor;

    let result = string_visitor.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(output, "a string");
}

