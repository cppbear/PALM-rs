// Answer 0

#[test]
fn test_fmt_literal_hex_fixed_x() {
    let mut s = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut s,
    };
    let ast = ast::Literal {
        span: ast::Span { start: 0, end: 1 },
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: '\u{FF}' // valid Unicode scalar
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_octal() {
    let mut s = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut s,
    };
    let ast = ast::Literal {
        span: ast::Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Octal,
        c: '\u{21}' // valid Unicode scalar
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    let mut s = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut s,
    };
    let ast = ast::Literal {
        span: ast::Span { start: 0, end: 1 },
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeLong),
        c: '\u{1F600}' // valid Unicode scalar, grinning face emoji
    };
    writer.fmt_literal(&ast);
}

