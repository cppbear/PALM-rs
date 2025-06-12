// Answer 0

fn test_fmt_literal_hex_brace_x() {
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let ast = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: 'A',
    };

    let result = w.fmt_literal(&ast);
    assert!(result.is_ok());
    assert_eq!(w.wtr.output, r"\x{41}");
}

fn test_fmt_literal_hex_brace_unicode_short() {
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let ast = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: 'Ω',
    };

    let result = w.fmt_literal(&ast);
    assert!(result.is_ok());
    assert_eq!(w.wtr.output, r"\u{03A9}");
}

fn test_fmt_literal_hex_brace_unicode_long() {
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let ast = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: '𐍈', // Gothic letter
    };

    let result = w.fmt_literal(&ast);
    assert!(result.is_ok());
    assert_eq!(w.wtr.output, r"\U00010348");
}

fn test_fmt_literal_octal() {
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let ast = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Octal,
        c: 'A',
    };

    let result = w.fmt_literal(&ast);
    assert!(result.is_ok());
    assert_eq!(w.wtr.output, r"\101"); // 'A' in octal
}

