// Answer 0

#[test]
fn test_fmt_literal_hex_brace_x() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: '\u{0041}', // 'A'
    };
    
    writer.fmt_literal(&ast).unwrap();
}

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: '\u{1F600}', // 😀 (grinning face)
    };
    
    writer.fmt_literal(&ast).unwrap();
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: '\u{0041}', // 'A'
    };
    
    writer.fmt_literal(&ast).unwrap();
}

#[test]
fn test_fmt_literal_octal() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Octal,
        c: '7', // valid octal character
    };
    
    writer.fmt_literal(&ast).unwrap();
}

