// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\u{0B}',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_space() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_bell() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\u{07}',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_line_feed() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_form_feed() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\u{0C}',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_x() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: 'A',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: 'B',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
    let mut output = Vec::new();
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: 'C',
    };
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: output };
    writer.fmt_literal(&ast);
}

