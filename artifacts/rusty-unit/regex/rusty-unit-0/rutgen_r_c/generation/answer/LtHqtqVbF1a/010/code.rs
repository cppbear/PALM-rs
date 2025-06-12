// Answer 0

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

    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };
    
    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "?");
}

fn test_fmt_repetition_zero_or_one_non_greedy() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrOne },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };
    
    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "??");
}

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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "*");
}

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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "*?");
}

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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "+");
}

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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "+?");
}

fn test_fmt_repetition_range_at_least() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(3)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "{{3}}");
}

fn test_fmt_repetition_range_bounded_non_greedy() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let repetition_ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let result = writer_instance.fmt_repetition(&repetition_ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "{{2,5}}?");
}

