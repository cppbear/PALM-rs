// Answer 0

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    use std::fmt::Write;
    use ast::{Literal, LiteralKind, HexLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let literal = Literal {
        span: Span::default(),
        kind: LiteralKind::HexFixed(HexLiteralKind::UnicodeShort),
        c: 'A',
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, "\\u{0041}");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    use std::fmt::Write;
    use ast::{Literal, LiteralKind, HexLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let literal = Literal {
        span: Span::default(),
        kind: LiteralKind::HexFixed(HexLiteralKind::UnicodeLong),
        c: 'Ω',
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, "\\U{03A9}");
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    use std::fmt::Write;
    use ast::{Literal, LiteralKind, HexLiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let literal = Literal {
        span: Span::default(),
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'B',
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, "\\x{42}");
}

#[test]
fn test_fmt_literal_octal() {
    use std::fmt::Write;
    use ast::{Literal, LiteralKind};

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };

    let literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Octal,
        c: 'C',
    };

    let result = writer.fmt_literal(&literal);
    assert!(result.is_ok());
    assert_eq!(output, "\\03");
}

