// Answer 0

#[test]
fn test_fmt_repetition_range_bounded_failure() {
    struct MockWriter {
        output: String,
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    let mut writer = MockWriter {
        output: String::new(),
        should_fail: true,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(5, 10)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let _ = fmt_writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_bounded_failure_with_zero() {
    struct MockWriter {
        output: String,
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    let mut writer = MockWriter {
        output: String::new(),
        should_fail: true,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(0, 0)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let _ = fmt_writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_bounded_failure_high_values() {
    struct MockWriter {
        output: String,
        should_fail: bool,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.should_fail {
                Err(fmt::Error)
            } else {
                self.output.push_str(s);
                Ok(())
            }
        }
    }

    let mut writer = MockWriter {
        output: String::new(),
        should_fail: true,
    };

    let mut fmt_writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(10, 10)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let _ = fmt_writer.fmt_repetition(&ast);
}

