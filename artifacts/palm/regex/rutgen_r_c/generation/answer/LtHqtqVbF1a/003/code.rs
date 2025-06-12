// Answer 0

#[test]
fn test_fmt_repetition_range_bounded() {
    struct MockWriter {
        output: String,
        should_error: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_error {
                return Err(fmt::Error);
            }
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new(), should_error: true };
    
    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = w.fmt_repetition(&repetition);
    
    assert!(result.is_err());
}

#[test]
fn test_fmt_repetition_range_exactly() {
    struct MockWriter {
        output: String,
        should_error: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_error {
                return Err(fmt::Error);
            }
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new(), should_error: true };
    
    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = w.fmt_repetition(&repetition);
    
    assert!(result.is_err());
}

#[test]
fn test_fmt_repetition_range_at_least() {
    struct MockWriter {
        output: String,
        should_error: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_error {
                return Err(fmt::Error);
            }
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new(), should_error: true };
    
    let repetition = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(1)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let mut w = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = w.fmt_repetition(&repetition);
    
    assert!(result.is_err());
}

