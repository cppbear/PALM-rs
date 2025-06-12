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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "?");
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "*");
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "+");
} 

#[test]
fn test_fmt_repetition_zero_or_one_not_greedy() {
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "??");
}

#[test]
fn test_fmt_repetition_zero_or_more_not_greedy() {
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "*?");
}

#[test]
fn test_fmt_repetition_one_or_more_not_greedy() {
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "+?");
} 

#[test]
fn test_fmt_repetition_range() {
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)),
        },
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "{{2,5}}");
} 

#[test]
fn test_fmt_repetition_range_not_greedy() {
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

    let ast = ast::Repetition {
        span: Span::default(), // Assuming a default method exists
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Assuming a default method exists
    };

    writer.fmt_repetition(&ast).unwrap();
    assert_eq!(writer.output, "{{2,5}}?");
} 

