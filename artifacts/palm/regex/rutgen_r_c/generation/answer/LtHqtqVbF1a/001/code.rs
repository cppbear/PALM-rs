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

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    writer_instance.fmt_repetition(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "?");
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

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    writer_instance.fmt_repetition(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "??");
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

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    writer_instance.fmt_repetition(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "*");
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

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    writer_instance.fmt_repetition(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "*?");
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

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    writer_instance.fmt_repetition(&ast).unwrap();

    assert_eq!(writer_instance.wtr.output, "+");
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

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    writer_instance.fmt_repetition(&ast).unwrap();
    
    assert_eq!(writer_instance.wtr.output, "+?");
}

#[test]
fn test_fmt_repetition_range_bound() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    impl Writer<'_, TestWriter> {
        fn fmt_repetition_range(&mut self, _ast: &ast::RepetitionRange) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let ast = ast::Repetition {
        span: Span { start: 0, end: 1 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)) },
        greedy: false,
        ast: Box::new(ast::Group {}),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = writer_instance.fmt_repetition(&ast);
    
    assert!(result.is_err());
}

