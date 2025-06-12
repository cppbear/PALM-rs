// Answer 0

#[test]
fn test_fmt_flags_case_insensitive() {
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
    let ast = ast::Flags {
        span: ast::Span::default(), // Assuming a default for the example
        items: vec![
            ast::FlagsItem {
                span: ast::Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::CaseInsensitive)),
            },
        ],
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    fmt_writer.fmt_flags(&ast).unwrap();

    assert_eq!(writer.output, "i");
}

#[test]
fn test_fmt_flags_multi_line() {
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
    let ast = ast::Flags {
        span: ast::Span::default(),
        items: vec![
            ast::FlagsItem {
                span: ast::Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::MultiLine)),
            },
        ],
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    fmt_writer.fmt_flags(&ast).unwrap();

    assert_eq!(writer.output, "m");
}

#[test]
fn test_fmt_flags_ignore_whitespace() {
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
    let ast = ast::Flags {
        span: ast::Span::default(),
        items: vec![
            ast::FlagsItem {
                span: ast::Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::IgnoreWhitespace)),
            },
        ],
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    fmt_writer.fmt_flags(&ast).unwrap();

    assert_eq!(writer.output, "x");
}

#[test]
fn test_fmt_flags_combined() {
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
    let ast = ast::Flags {
        span: ast::Span::default(),
        items: vec![
            ast::FlagsItem {
                span: ast::Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::CaseInsensitive)),
            },
            ast::FlagsItem {
                span: ast::Span::default(),
                kind: ast::FlagsItemKind::Flag(Box::new(ast::Flag::IgnoreWhitespace)),
            },
        ],
    };

    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    fmt_writer.fmt_flags(&ast).unwrap();

    assert_eq!(writer.output, "ix");
}

