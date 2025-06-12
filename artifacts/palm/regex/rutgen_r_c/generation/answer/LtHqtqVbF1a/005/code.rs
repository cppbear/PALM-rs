// Answer 0

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, RepetitionRange, Ast};
    use std::fmt::Write;

    // Create a dummy Writer implementation
    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, str: &str) -> fmt::Result {
            self.output.push_str(str);
            Ok(())
        }
    }

    // Create a Writer instance
    let mut writer = TestWriter { output: String::new() };
    
    // Create a Repetition instance to test
    let repetition = Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(Ast::default()),  // Assuming Ast::default() is a valid expression
    };

    // Create a Writer wrapper
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    // Call the function under test
    assert!(fmt_writer.fmt_repetition(&repetition).is_ok());
    assert_eq!(fmt_writer.wtr.output, "+");
}

#[test]
fn test_fmt_repetition_one_or_more_not_greedy() {
    use ast::{Repetition, RepetitionKind, Ast};
    use std::fmt::Write;

    // Create a dummy Writer implementation
    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, str: &str) -> fmt::Result {
            self.output.push_str(str);
            Ok(())
        }
    }

    // Create a Writer instance
    let mut writer = TestWriter { output: String::new() };
    
    // Create a Repetition instance to test
    let repetition = Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: RepetitionKind::OneOrMore,
        },
        greedy: false,
        ast: Box::new(Ast::default()),  // Assuming Ast::default() is a valid expression
    };

    // Create a Writer wrapper
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    // Call the function under test
    assert!(fmt_writer.fmt_repetition(&repetition).is_ok());
    assert_eq!(fmt_writer.wtr.output, "+?");
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
    use ast::{Repetition, RepetitionKind, Ast};
    use std::fmt::Write;

    // Create a dummy Writer implementation
    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, str: &str) -> fmt::Result {
            self.output.push_str(str);
            Ok(())
        }
    }

    // Create a Writer instance
    let mut writer = TestWriter { output: String::new() };
    
    // Create a Repetition instance to test
    let repetition = Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: RepetitionKind::ZeroOrMore,
        },
        greedy: true,
        ast: Box::new(Ast::default()),  // Assuming Ast::default() is a valid expression
    };

    // Create a Writer wrapper
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    // Call the function under test
    assert!(fmt_writer.fmt_repetition(&repetition).is_ok());
    assert_eq!(fmt_writer.wtr.output, "*");
}

#[test]
fn test_fmt_repetition_zero_or_more_not_greedy() {
    use ast::{Repetition, RepetitionKind, Ast};
    use std::fmt::Write;

    // Create a dummy Writer implementation
    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, str: &str) -> fmt::Result {
            self.output.push_str(str);
            Ok(())
        }
    }

    // Create a Writer instance
    let mut writer = TestWriter { output: String::new() };
    
    // Create a Repetition instance to test
    let repetition = Repetition {
        span: Default::default(),
        op: ast::RepetitionOp {
            kind: RepetitionKind::ZeroOrMore,
        },
        greedy: false,
        ast: Box::new(Ast::default()),  // Assuming Ast::default() is a valid expression
    };

    // Create a Writer wrapper
    let mut fmt_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    // Call the function under test
    assert!(fmt_writer.fmt_repetition(&repetition).is_ok());
    assert_eq!(fmt_writer.wtr.output, "*?");
}

