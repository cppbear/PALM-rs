// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort),
        c: '\u{007F}',
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeLong),
        c: '\u{FFFF}',
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: '\u{00A0}',
    };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

