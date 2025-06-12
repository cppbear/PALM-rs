// Answer 0

#[test]
fn test_fmt_literal_special_tab() {
    let mut output = String::new();
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_line_feed() {
    let mut output = String::new();
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_carriage_return() {
    let mut output = String::new();
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_space() {
    let mut output = String::new();
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_bell() {
    let mut output = String::new();
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\u{07}',
    };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_octal() {
    let mut output = String::new();
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Octal,
        c: 'A',
    };
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    writer.fmt_literal(&literal);
}

