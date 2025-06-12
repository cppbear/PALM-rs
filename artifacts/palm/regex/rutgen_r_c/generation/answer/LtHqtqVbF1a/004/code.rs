// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(), // assuming a default implementation exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // assuming a default implementation exists
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "?");
}

#[test]
fn test_fmt_repetition_zero_or_one_non_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "??");
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_non_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "*?");
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_non_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "+?");
}

#[test]
fn test_fmt_repetition_range_exclusively() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 3)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "{{2,3}}?");
}

