// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_ascii_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Ascii,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_blank_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_graph_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Graph,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_lower_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_word_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Word,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_xdigit_negated() {
    let mut printer = Printer { _priv: () };
    let mut result = String::new();
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Xdigit,
        negated: false,
    };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut result,
    };
    writer.fmt_class_ascii(&ast);
}

