// Answer 0

#[test]
fn test_fmt_literal_hex_fixed_unicode_short_A() {
    let mut buffer = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::new(0, 1),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort),
        c: 'A',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short_Z() {
    let mut buffer = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::new(0, 1),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort),
        c: 'Z',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_x_a() {
    let mut buffer = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::new(0, 1),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: 'a',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_x_f() {
    let mut buffer = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::new(0, 1),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: 'f',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_punctuation_exclamation() {
    let mut buffer = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::new(0, 1),
        kind: ast::LiteralKind::Punctuation,
        c: '!',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_punctuation_hash() {
    let mut buffer = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut buffer };
    let ast = ast::Literal {
        span: Span::new(0, 1),
        kind: ast::LiteralKind::Punctuation,
        c: '#',
    };
    writer.fmt_literal(&ast);
}

