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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Verbatim,
        c: 'a',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, "a");
}

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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, r"\v");
}

#[test]
fn test_fmt_literal_special_bell() {
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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\x07',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, r"\a");
}

#[test]
fn test_fmt_literal_special_line_feed() {
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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, r"\n");
} 

#[test]
fn test_fmt_literal_special_form_feed() {
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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\x0C',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, r"\f");
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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, r"\t");
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
    let mut literal = Literal {
        span: Span::default(), // Assuming a default implementation exists
        kind: LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
    assert_eq!(writer.output, r"\ ");
}

