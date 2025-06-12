// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
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
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();

    assert_eq!(writer.output, r"\r");
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
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
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();

    assert_eq!(writer.output, r"\v");
}

#[test]
fn test_fmt_literal_special_space() {
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
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();

    assert_eq!(writer.output, r"\ ");
}

#[test]
fn test_fmt_literal_special_form_feed() {
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
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\x0C',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();

    assert_eq!(writer.output, r"\f");
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
    let literal = ast::Literal {
        span: ast::Span::default(),
        kind: ast::LiteralKind::Punctuation,
        c: '!',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();

    assert_eq!(writer.output, r"\!");
}

