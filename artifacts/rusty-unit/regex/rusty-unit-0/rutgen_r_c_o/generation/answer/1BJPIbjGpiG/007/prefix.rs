// Answer 0

#[test]
fn test_fmt_literal_special_bell() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\u{0007}',
    };
    
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_carriage_return() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\u{000B}',
    };
    
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_space() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_line_feed() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    
    writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_verbatim() {
    struct MockPrinter {
        output: String,
    }

    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    let ast = ast::Literal {
        span: Default::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    
    writer.fmt_literal(&ast);
}

