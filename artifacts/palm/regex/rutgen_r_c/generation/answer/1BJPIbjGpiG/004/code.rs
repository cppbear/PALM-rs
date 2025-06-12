// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
    use std::fmt::Write;
    use ast::{self, Literal, SpecialLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\r");
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    use std::fmt::Write;
    use ast::{self, Literal, SpecialLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };

    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\v");
}

#[test]
fn test_fmt_literal_special_space() {
    use std::fmt::Write;
    use ast::{self, Literal, SpecialLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::Space),
        c: ' ',
    };

    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\ ");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    use std::fmt::Write;
    use ast::{self, Literal, HexLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexFixed(HexLiteralKind::UnicodeShort),
        c: 'A', // represents 65 in Unicode
    };

    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\u0041");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    use std::fmt::Write;
    use ast::{self, Literal, HexLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexFixed(HexLiteralKind::UnicodeLong),
        c: 'A', // represents 65 in Unicode
    };

    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\U00000041");
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    use std::fmt::Write;
    use ast::{self, Literal, HexLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'A', // represents 65 in Unicode
    };

    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\x41");
}

#[test]
fn test_fmt_literal_special_line_feed() {
    use std::fmt::Write;
    use ast::{self, Literal, SpecialLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let literal = Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::LineFeed),
        c: '\n',
    };

    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.fmt_literal(&literal).unwrap();
    assert_eq!(output, r"\n");
}

