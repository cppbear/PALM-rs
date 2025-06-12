// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };

    writer_instance.fmt_literal(&literal).unwrap();
    assert_eq!(writer_instance.wtr.output, "a");
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort),
        c: 'Ω',
    };

    writer_instance.fmt_literal(&literal).unwrap();
    assert_eq!(writer_instance.wtr.output, r#"\u{03A9}"#);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeLong),
        c: '𐍈',
    };

    writer_instance.fmt_literal(&literal).unwrap();
    assert_eq!(writer_instance.wtr.output, r#"\U{10348}"#);
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: 'A',
    };

    writer_instance.fmt_literal(&literal).unwrap();
    assert_eq!(writer_instance.wtr.output, r#"\x41"#);
}

