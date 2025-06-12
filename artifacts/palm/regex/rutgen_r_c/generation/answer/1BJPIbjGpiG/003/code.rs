// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
    use ast::{Literal, SpecialLiteralKind};

    let mut output = String::new();
    let literal = Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, r"\r");
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    use ast::{Literal, SpecialLiteralKind};

    let mut output = String::new();
    let literal = Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, r"\v");
}

#[test]
fn test_fmt_literal_special_space() {
    use ast::{Literal, SpecialLiteralKind};

    let mut output = String::new();
    let literal = Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(SpecialLiteralKind::Space),
        c: ' ',
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, r"\ ");
}

#[test]
fn test_fmt_literal_hexbrace_x() {
    use ast::{Literal, HexLiteralKind};

    let mut output = String::new();
    let literal = Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(HexLiteralKind::X),
        c: '\x41', // ASCII 'A'
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, r"\x{41}");
}

#[test]
fn test_fmt_literal_hexbrace_unicode_short() {
    use ast::{Literal, HexLiteralKind};

    let mut output = String::new();
    let literal = Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(HexLiteralKind::UnicodeShort),
        c: 'é',
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, r"\u{E9}");
}

#[test]
fn test_fmt_literal_hexbrace_unicode_long() {
    use ast::{Literal, HexLiteralKind};

    let mut output = String::new();
    let literal = Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(HexLiteralKind::UnicodeLong),
        c: '𐍈', // U+10340
    };
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, r"\U{10340}");
}

