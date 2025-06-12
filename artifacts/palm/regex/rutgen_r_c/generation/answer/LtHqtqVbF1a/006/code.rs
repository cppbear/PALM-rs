// Answer 0

fn test_fmt_repetition_one_or_more_greedy() {
    use std::fmt::Write;
    use ast::{Ast, Repetition, RepetitionOp, RepetitionKind, RepetitionRange};

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

    let op = RepetitionOp { kind: RepetitionKind::OneOrMore };
    let repetition = Repetition {
        span: Span { start: 0, end: 0 },
        op,
        greedy: false,
        ast: Box::new(Ast::Dummy), // Assuming there's a Dummy variant for testing purposes
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_repetition(&repetition);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "+?");
}

fn test_fmt_repetition_one_or_more_greedy_true() {
    use std::fmt::Write;
    use ast::{Ast, Repetition, RepetitionOp, RepetitionKind};

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

    let op = RepetitionOp { kind: RepetitionKind::OneOrMore };
    let repetition = Repetition {
        span: Span { start: 0, end: 0 },
        op,
        greedy: true,
        ast: Box::new(Ast::Dummy),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = writer_instance.fmt_repetition(&repetition);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "+");
}

fn test_fmt_repetition_zero_or_more() {
    use std::fmt::Write;
    use ast::{Ast, Repetition, RepetitionOp, RepetitionKind};

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

    let op = RepetitionOp { kind: RepetitionKind::ZeroOrMore };
    let repetition = Repetition {
        span: Span { start: 0, end: 0 },
        op,
        greedy: false,
        ast: Box::new(Ast::Dummy),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = writer_instance.fmt_repetition(&repetition);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "*?");
} 

fn test_fmt_repetition_range() {
    use std::fmt::Write;
    use ast::{Ast, Repetition, RepetitionOp, RepetitionKind, RepetitionRange};

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

    let op = RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 4)) };
    let repetition = Repetition {
        span: Span { start: 0, end: 0 },
        op,
        greedy: false,
        ast: Box::new(Ast::Dummy),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = writer_instance.fmt_repetition(&repetition);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "{2,4}?");
} 

fn test_fmt_repetition_exactly() {
    use std::fmt::Write;
    use ast::{Ast, Repetition, RepetitionOp, RepetitionKind, RepetitionRange};

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

    let op = RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Exactly(3)) };
    let repetition = Repetition {
        span: Span { start: 0, end: 0 },
        op,
        greedy: false,
        ast: Box::new(Ast::Dummy),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = writer_instance.fmt_repetition(&repetition);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "{3}?");
} 

fn test_fmt_repetition_at_least() {
    use std::fmt::Write;
    use ast::{Ast, Repetition, RepetitionOp, RepetitionKind, RepetitionRange};

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

    let op = RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::AtLeast(5)) };
    let repetition = Repetition {
        span: Span { start: 0, end: 0 },
        op,
        greedy: false,
        ast: Box::new(Ast::Dummy),
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let result = writer_instance.fmt_repetition(&repetition);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "{5}?");
}

