// Answer 0

#[test]
fn test_fmt_literal_bell() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: 'a',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\a");
}

#[test]
fn test_fmt_literal_form_feed() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: 'b',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\f");
}

#[test]
fn test_fmt_literal_tab() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: 'c',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\t");
}

#[test]
fn test_fmt_literal_line_feed() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: 'd',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\n");
}

#[test]
fn test_fmt_literal_carriage_return() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: 'e',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\r");
}

#[test]
fn test_fmt_literal_vertical_tab() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: 'f',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\v");
}

#[test]
fn test_fmt_literal_space() {
    let mut wtr = Vec::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut wtr };
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: 'g',
    };
    writer.fmt_literal(&literal).unwrap();
    assert_eq!(String::from_utf8(wtr).unwrap(), r"\ ");
}

