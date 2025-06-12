// Answer 0

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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: true,
        ast: Box::new(Default::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert!(result.is_ok());
    assert_eq!(writer.output, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_non_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: false,
        ast: Box::new(Default::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert!(result.is_ok());
    assert_eq!(writer.output, "*?");
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

    let mut writer = MockWriter { output: String::new() };
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(Default::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert!(result.is_ok());
    assert_eq!(writer.output, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_non_greedy() {
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
    let mut printer = Printer { _priv: () };

    let repetition = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: false,
        ast: Box::new(Default::default()),
    };

    let result = writer.fmt_repetition(&repetition);
    assert!(result.is_ok());
    assert_eq!(writer.output, "+?");
}

