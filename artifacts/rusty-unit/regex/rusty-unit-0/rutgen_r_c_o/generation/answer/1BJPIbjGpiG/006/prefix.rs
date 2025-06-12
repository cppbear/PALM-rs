// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_space() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_bell() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\x07',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_line_feed() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_form_feed() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\x0C',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_tab() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_punctuation() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Punctuation,
        c: '.',
    };
    writer.fmt_literal(&ast);
}

