// Answer 0

#[test]
fn test_fmt_literal_special_bell() {
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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\a");
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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\f");
}

#[test]
fn test_fmt_literal_special_tab() {
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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\t");
}

#[test]
fn test_fmt_literal_special_line_feed() {
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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\n");
}

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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
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

    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: '\0',
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    writer_instance.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\ ");
}

