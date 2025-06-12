// Answer 0

#[test]
fn test_fmt_literal_punctuation() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span { /* initialize span here */ },
        kind: ast::LiteralKind::Punctuation,
        c: '!'
    };
    let writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span { /* initialize span here */ },
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: 0x20
    };
    let writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_x() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span { /* initialize span here */ },
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: '$'
    };
    let writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let ast = ast::Literal {
        span: Span { /* initialize span here */ },
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: 0x7F
    };
    let writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&ast);
}

