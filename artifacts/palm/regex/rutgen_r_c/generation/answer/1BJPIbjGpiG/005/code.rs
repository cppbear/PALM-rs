// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
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
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\r");
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
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
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\v");
}

#[test]
fn test_fmt_literal_special_space() {
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
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\ ");
}

#[test]
fn test_fmt_literal_special_tab() {
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
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\t");
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
    let ast = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Octal,
        c: 'A', // ASCII 'A' -> 65 in decimal -> 101 in octal
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&ast).unwrap();

    assert_eq!(writer.output, r"\101");
}

