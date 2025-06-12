// Answer 0

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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\u{000B}',
    };
    
    formatter.fmt_literal(&literal).unwrap();
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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    
    formatter.fmt_literal(&literal).unwrap();
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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    
    formatter.fmt_literal(&literal).unwrap();
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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\u{0007}',
    };
    
    formatter.fmt_literal(&literal).unwrap();
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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\u{000C}',
    };
    
    formatter.fmt_literal(&literal).unwrap();
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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    
    formatter.fmt_literal(&literal).unwrap();
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
    let mut formatter = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let literal = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    
    formatter.fmt_literal(&literal).unwrap();
}

