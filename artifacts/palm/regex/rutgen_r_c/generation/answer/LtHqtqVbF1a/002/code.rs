// Answer 0

#[test]
fn test_fmt_repetition_zero_or_one_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::ZeroOrOne },
        greedy: true,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "?");
}

#[test]
fn test_fmt_repetition_zero_or_one_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::ZeroOrOne },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "??");
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "*?");
}

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "+?");
}

#[test]
fn test_fmt_repetition_range_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 5)) },
        greedy: true,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "{{2,5}}");
}

#[test]
fn test_fmt_repetition_range_not_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, RepetitionOp, Ast};
    use std::fmt::Write;

    struct MockWriter {
        buffer: String,
    }

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut printer = Printer { _priv: () };
    let ast = Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(3, 7)) },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let result = writer.fmt_repetition(&ast);
    assert_eq!(result, Ok(()));
    assert_eq!(writer.buffer, "{{3,7}}?");
}

