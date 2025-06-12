// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    let mut buffer = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut buffer,
    };
    let ast = ast::ClassAscii {
        span: ast::Span::new(0, 1),
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    let mut buffer = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut buffer,
    };
    let ast = ast::ClassAscii {
        span: ast::Span::new(0, 1),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_space_negated() {
    let mut buffer = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut buffer,
    };
    let ast = ast::ClassAscii {
        span: ast::Span::new(0, 1),
        kind: ast::ClassAsciiKind::Space,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    let mut buffer = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut buffer,
    };
    let ast = ast::ClassAscii {
        span: ast::Span::new(0, 1),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_graph_negated() {
    let mut buffer = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut buffer,
    };
    let ast = ast::ClassAscii {
        span: ast::Span::new(0, 1),
        kind: ast::ClassAsciiKind::Graph,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

