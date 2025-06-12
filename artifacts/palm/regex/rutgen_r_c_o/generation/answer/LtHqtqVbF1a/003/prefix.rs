// Answer 0

#[test]
fn test_fmt_repetition_range_bound() {
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

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(3, 5)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let mut wtr = Writer { printer: &mut printer, wtr: &mut writer };
    let _ = wtr.fmt_repetition(&ast);
}

#[test]
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
    let mut printer = Printer { _priv: () };

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(2)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let mut wtr = Writer { printer: &mut printer, wtr: &mut writer };
    let _ = wtr.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_exactly() {
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

    let ast = ast::Repetition {
        span: ast::Span::default(),
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(5)),
        },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let mut wtr = Writer { printer: &mut printer, wtr: &mut writer };
    let _ = wtr.fmt_repetition(&ast);
}

