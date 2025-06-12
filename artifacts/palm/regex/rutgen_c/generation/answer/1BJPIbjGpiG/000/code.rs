// Answer 0

#[test]
fn test_fmt_literal_verbatim() {
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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };

    writer_ref.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, "a");
}

#[test]
fn test_fmt_literal_punctuation() {
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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Punctuation,
        c: '.',
    };

    writer_ref.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\.");
}

#[test]
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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Octal,
        c: 'a',
    };

    writer_ref.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\141");
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: 'A',
    };

    writer_ref.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\x41");
}

#[test]
fn test_fmt_literal_special() {
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
    let mut writer_ref = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };

    writer_ref.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\t");
}

