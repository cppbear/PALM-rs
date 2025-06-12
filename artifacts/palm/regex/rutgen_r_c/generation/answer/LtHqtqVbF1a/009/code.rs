// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    use ast::{self, Repetition, RepetitionOp, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span implementation
        op: RepetitionOp { kind: RepetitionKind::ZeroOrOne }, // Assuming RepetitionOp struct exists
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "?");
}

#[test]
fn test_fmt_repetition_zero_or_one_non_greedy() {
    use ast::{self, Repetition, RepetitionOp, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span
        op: RepetitionOp { kind: RepetitionKind::ZeroOrOne }, // Assuming RepetitionOp struct exists
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "??");
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    use ast::{self, Repetition, RepetitionOp, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore }, // Assuming RepetitionOp struct exists
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_non_greedy() {
    use ast::{self, Repetition, RepetitionOp, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore }, // Assuming RepetitionOp struct exists
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "*?");
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    use ast::{self, Repetition, RepetitionOp, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span
        op: RepetitionOp { kind: RepetitionKind::OneOrMore }, // Assuming RepetitionOp struct exists
        greedy: true,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_non_greedy() {
    use ast::{self, Repetition, RepetitionOp, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span
        op: RepetitionOp { kind: RepetitionKind::OneOrMore }, // Assuming RepetitionOp struct exists
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "+?");
}

#[test]
fn test_fmt_repetition_range() {
    use ast::{self, Repetition, RepetitionOp, RepetitionRange, RepetitionKind};

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

    let repetition = Repetition {
        span: ast::Span { start: 0, end: 1 }, // Placeholder for actual span
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 5)) }, // Assuming RepetitionOp struct exists
        greedy: false,
        ast: Box::new(ast::Ast::default()), // Use a suitable default constructor for the Ast struct
    };

    writer.fmt_repetition(&repetition).expect("Formatting failed");

    assert_eq!(mock_writer.output, "{{2,5}}?");
}

