// Answer 0

#[test]
fn test_fmt_class_ascii_xdigit() {
    use std::fmt::Write;

    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let ast = ast::ClassAscii {
        span: Span::new(0, 10), // assuming a valid Span implementation
        kind: ast::ClassAsciiKind::Xdigit,
        negated: false,
    };

    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(output, "[:xdigit:]");
}

#[test]
fn test_fmt_class_ascii_xdigit_negated() {
    use std::fmt::Write;

    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let ast = ast::ClassAscii {
        span: Span::new(0, 10),  // assuming a valid Span implementation
        kind: ast::ClassAsciiKind::Xdigit,
        negated: true,
    };

    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(output, "[:^xdigit:]");
}

#[test]
fn test_fmt_class_ascii_digit() {
    use std::fmt::Write;

    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let ast = ast::ClassAscii {
        span: Span::new(0, 10), // assuming a valid Span implementation
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };

    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(output, "[:digit:]");
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    use std::fmt::Write;

    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    let ast = ast::ClassAscii {
        span: Span::new(0, 10),  // assuming a valid Span implementation
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    let result = writer.fmt_class_ascii(&ast);
    
    assert!(result.is_ok());
    assert_eq!(output, "[:^digit:]");
}

