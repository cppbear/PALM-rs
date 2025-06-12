// Answer 0

#[test]
fn test_fmt_repetition_zero_or_more_greedy_false() {
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
        span: ast::Span { start: 0, end: 10 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::Placeholder), // Assuming Ast has a placeholder variant
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy_true() {
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
        span: ast::Span { start: 0, end: 10 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::Placeholder),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_greedy_false() {
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
        span: ast::Span { start: 0, end: 10 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::Placeholder),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_greedy_true() {
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
        span: ast::Span { start: 0, end: 10 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::Placeholder),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_greedy_false() {
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
        span: ast::Span { start: 0, end: 10 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 3)) },
        greedy: false,
        ast: Box::new(ast::Ast::Placeholder),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_greedy_true() {
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
        span: ast::Span { start: 0, end: 10 },
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(1, 3)) },
        greedy: true,
        ast: Box::new(ast::Ast::Placeholder),
    };

    writer.fmt_repetition(&ast).unwrap();
}

