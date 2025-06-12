// Answer 0

#[test]
fn test_fmt_class_ascii_lower() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_lower_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alpha() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

