// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: '\u{007F}',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_x() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: '\u{7F}',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: '\u{FFFF}',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
    let mut output = String::new();
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: '\u{D7FF}',
    };
    writer.fmt_literal(&ast);
}

