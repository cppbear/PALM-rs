// Answer 0

#[test]
fn test_fmt_class_ascii_upper_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_not_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: false,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_edge_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

