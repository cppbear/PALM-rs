// Answer 0

#[test]
fn test_fmt_class_ascii_alpha() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: Span::new(0, 5),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: Span::new(0, 5),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alnum() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: Span::new(0, 5),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: Span::new(0, 5),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: Span::new(0, 5),
        kind: ast::ClassAsciiKind::Upper,
        negated: false,
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_lower() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let ast = ast::ClassAscii {
        span: Span::new(0, 5),
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_class_ascii(&ast);
}

