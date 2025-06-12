// Answer 0

#[test]
fn test_fmt_class_ascii_lower_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let class_ascii = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: true,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    writer.fmt_class_ascii(&class_ascii);
}

#[test]
fn test_fmt_class_ascii_lower_non_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let class_ascii = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    writer.fmt_class_ascii(&class_ascii);
}

