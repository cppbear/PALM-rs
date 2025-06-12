// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "?");
}

#[test]
fn test_fmt_repetition_zero_or_one_nongreedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "??");
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_nongreedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "*?");
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_nongreedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "+?");
}

#[test]
fn test_fmt_repetition_range_exceeding() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 3)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "{{2,3}}");
}

#[test]
fn test_fmt_repetition_range_nongreedy() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut mock_writer };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(mock_writer.output, "{{2,}}?");
}

