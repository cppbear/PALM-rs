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
    let mut printer = Printer { _priv: () };
    
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };

    fmt_writer.fmt_literal(&ast).unwrap();
    assert_eq!(fmt_writer.wtr.output, "a");
}

#[test]
fn test_fmt_literal_punctuation() {
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
    let mut printer = Printer { _priv: () };
    
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Punctuation,
        c: '!',
    };

    fmt_writer.fmt_literal(&ast).unwrap();
    assert_eq!(fmt_writer.wtr.output, r"\!");
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
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
    let mut printer = Printer { _priv: () };
    
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: 'é',
    };

    fmt_writer.fmt_literal(&ast).unwrap();
    assert_eq!(fmt_writer.wtr.output, r"\u{E9}");
}

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
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
    let mut printer = Printer { _priv: () };
    
    let mut fmt_writer = Writer { printer: &mut printer, wtr: writer };
    
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: '𐍈',
    };

    fmt_writer.fmt_literal(&ast).unwrap();
    assert_eq!(fmt_writer.wtr.output, r"\U{10348}");
}

